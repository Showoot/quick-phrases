use crate::models::phrase::{CreatePhraseRequest, Phrase, UpdatePhraseRequest};
use crate::services::clipboard;
use crate::services::storage::{AppData, Storage};
use crate::StorageState;
use chrono::Utc;
use serde::Deserialize;
use tauri::State;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ImportPhraseItem {
    pub title: String,
    pub text_content: String,
    pub category_name: Option<String>,
    pub tags: Option<Vec<String>>,
}

fn load_data(storage: &Storage) -> Result<AppData, String> {
    storage.load().map_err(|e| e.to_string())
}

fn save_data(storage: &Storage, data: &AppData) -> Result<(), String> {
    storage.save(data).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_phrases(
    state: State<'_, StorageState>,
    category_id: Option<String>,
    search: Option<String>,
) -> Result<Vec<Phrase>, String> {
    let data = load_data(&state.0)?;
    let mut phrases = data.phrases;

    if let Some(cat_id) = category_id {
        phrases.retain(|p| p.category_id == cat_id);
    }

    if let Some(query) = search {
        let q = query.to_lowercase();
        phrases.retain(|p| {
            p.title.to_lowercase().contains(&q)
                || p.text_content.to_lowercase().contains(&q)
                || p.tags.iter().any(|t| t.to_lowercase().contains(&q))
        });
    }

    // Sort by newest first
    phrases.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

    Ok(phrases)
}

#[tauri::command]
pub fn get_phrase(
    state: State<'_, StorageState>,
    id: String,
) -> Result<Phrase, String> {
    let data = load_data(&state.0)?;
    data.phrases
        .iter()
        .find(|p| p.id == id)
        .cloned()
        .ok_or_else(|| format!("Phrase not found: {}", id))
}

#[tauri::command]
pub fn create_phrase(
    state: State<'_, StorageState>,
    request: CreatePhraseRequest,
) -> Result<Phrase, String> {
    let mut data = load_data(&state.0)?;
    let now = Utc::now().timestamp_millis();
    let phrase = Phrase {
        id: Uuid::new_v4().to_string(),
        category_id: request.category_id,
        title: request.title,
        text_content: request.text_content,
        image_paths: request.image_paths,
        tags: request.tags,
        is_favorite: false,
        usage_count: 0,
        created_at: now,
        updated_at: now,
    };
    data.phrases.push(phrase.clone());
    save_data(&state.0, &data)?;
    Ok(phrase)
}

#[tauri::command]
pub fn update_phrase(
    state: State<'_, StorageState>,
    id: String,
    request: UpdatePhraseRequest,
) -> Result<Phrase, String> {
    let mut data = load_data(&state.0)?;
    let phrase = data
        .phrases
        .iter_mut()
        .find(|p| p.id == id)
        .ok_or_else(|| format!("Phrase not found: {}", id))?;

    if let Some(cat_id) = request.category_id {
        phrase.category_id = cat_id;
    }
    if let Some(title) = request.title {
        phrase.title = title;
    }
    if let Some(content) = request.text_content {
        phrase.text_content = content;
    }
    if let Some(paths) = request.image_paths {
        phrase.image_paths = paths;
    }
    if let Some(tags) = request.tags {
        phrase.tags = tags;
    }
    if let Some(fav) = request.is_favorite {
        phrase.is_favorite = fav;
    }
    phrase.updated_at = Utc::now().timestamp_millis();

    let result = phrase.clone();
    save_data(&state.0, &data)?;
    Ok(result)
}

#[tauri::command]
pub fn delete_phrase(
    state: State<'_, StorageState>,
    id: String,
) -> Result<(), String> {
    let mut data = load_data(&state.0)?;
    if let Some(phrase) = data.phrases.iter().find(|p| p.id == id) {
        for img_path in &phrase.image_paths {
            let _ = clipboard::delete_image(&state.0, img_path);
        }
    }
    data.phrases.retain(|p| p.id != id);
    save_data(&state.0, &data)?;
    Ok(())
}

#[tauri::command]
pub fn increment_usage(
    state: State<'_, StorageState>,
    id: String,
) -> Result<Phrase, String> {
    let mut data = load_data(&state.0)?;
    let phrase = data
        .phrases
        .iter_mut()
        .find(|p| p.id == id)
        .ok_or_else(|| format!("Phrase not found: {}", id))?;
    phrase.usage_count += 1;
    phrase.updated_at = Utc::now().timestamp_millis();
    let result = phrase.clone();
    save_data(&state.0, &data)?;
    Ok(result)
}

#[tauri::command]
pub fn toggle_favorite(
    state: State<'_, StorageState>,
    id: String,
) -> Result<Phrase, String> {
    let mut data = load_data(&state.0)?;
    let phrase = data
        .phrases
        .iter_mut()
        .find(|p| p.id == id)
        .ok_or_else(|| format!("Phrase not found: {}", id))?;
    phrase.is_favorite = !phrase.is_favorite;
    phrase.updated_at = Utc::now().timestamp_millis();
    let result = phrase.clone();
    save_data(&state.0, &data)?;
    Ok(result)
}

#[tauri::command]
pub fn import_phrases(
    state: State<'_, StorageState>,
    items: Vec<ImportPhraseItem>,
) -> Result<Vec<Phrase>, String> {
    let mut data = load_data(&state.0)?;
    let now = Utc::now().timestamp_millis();
    let mut imported = Vec::new();

    for item in items {
        // Resolve category: by name or use first available
        let category_id = if let Some(ref cat_name) = item.category_name {
            data.categories
                .iter()
                .find(|c| c.name.eq_ignore_ascii_case(cat_name))
                .map(|c| c.id.clone())
                .unwrap_or_else(|| {
                    data.categories
                        .first()
                        .map(|c| c.id.clone())
                        .unwrap_or_default()
                })
        } else {
            data.categories
                .first()
                .map(|c| c.id.clone())
                .unwrap_or_default()
        };

        if category_id.is_empty() {
            continue; // Skip if no categories exist at all
        }

        // Skip duplicates by title
        if data.phrases.iter().any(|p| p.title == item.title) {
            continue;
        }

        let phrase = Phrase {
            id: Uuid::new_v4().to_string(),
            category_id,
            title: item.title.clone(),
            text_content: item.text_content.clone(),
            image_paths: Vec::new(),
            tags: item.tags.clone().unwrap_or_default(),
            is_favorite: false,
            usage_count: 0,
            created_at: now,
            updated_at: now,
        };

        data.phrases.push(phrase.clone());
        imported.push(phrase);
    }

    save_data(&state.0, &data)?;
    Ok(imported)
}

use crate::models::category::Category;
use crate::services::storage::{AppData, Storage};
use crate::StorageState;
use chrono::Utc;
use tauri::State;
use uuid::Uuid;

fn load_data(storage: &Storage) -> Result<AppData, String> {
    storage.load().map_err(|e| e.to_string())
}

fn save_data(storage: &Storage, data: &AppData) -> Result<(), String> {
    storage.save(data).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_categories(
    state: State<'_, StorageState>,
) -> Result<Vec<Category>, String> {
    let data = load_data(&state.0)?;
    let mut categories = data.categories;
    categories.sort_by(|a, b| a.sort_order.cmp(&b.sort_order));
    Ok(categories)
}

#[tauri::command]
pub fn create_category(
    state: State<'_, StorageState>,
    name: String,
    icon: Option<String>,
) -> Result<Category, String> {
    if name.trim().is_empty() {
        return Err("Category name cannot be empty".to_string());
    }
    let mut data = load_data(&state.0)?;
    let max_order = data
        .categories
        .iter()
        .map(|c| c.sort_order)
        .max()
        .unwrap_or(0);

    let category = Category {
        id: Uuid::new_v4().to_string(),
        name: name.trim().to_string(),
        icon,
        sort_order: max_order + 1,
        created_at: Utc::now().timestamp_millis(),
    };
    data.categories.push(category.clone());
    save_data(&state.0, &data)?;
    Ok(category)
}

#[tauri::command]
pub fn update_category(
    state: State<'_, StorageState>,
    id: String,
    name: String,
) -> Result<Category, String> {
    if name.trim().is_empty() {
        return Err("Category name cannot be empty".to_string());
    }
    let mut data = load_data(&state.0)?;
    let category = data
        .categories
        .iter_mut()
        .find(|c| c.id == id)
        .ok_or_else(|| format!("Category not found: {}", id))?;
    category.name = name.trim().to_string();
    let result = category.clone();
    save_data(&state.0, &data)?;
    Ok(result)
}

#[tauri::command]
pub fn delete_category(
    state: State<'_, StorageState>,
    id: String,
) -> Result<(), String> {
    let mut data = load_data(&state.0)?;
    // Collect phrase IDs in this category
    let phrase_ids: Vec<String> = data
        .phrases
        .iter()
        .filter(|p| p.category_id == id)
        .map(|p| p.id.clone())
        .collect();

    // Delete associated images
    for pid in &phrase_ids {
        if let Some(phrase) = data.phrases.iter().find(|p| &p.id == pid) {
            for img_path in &phrase.image_paths {
                let _ = crate::services::clipboard::delete_image(&state.0, img_path);
            }
        }
    }

    data.phrases.retain(|p| p.category_id != id);
    data.categories.retain(|c| c.id != id);
    save_data(&state.0, &data)?;
    Ok(())
}

#[tauri::command]
pub fn reorder_categories(
    state: State<'_, StorageState>,
    ids: Vec<String>,
) -> Result<(), String> {
    let mut data = load_data(&state.0)?;
    for (i, id) in ids.iter().enumerate() {
        if let Some(cat) = data.categories.iter_mut().find(|c| &c.id == id) {
            cat.sort_order = i as u32;
        }
    }
    save_data(&state.0, &data)?;
    Ok(())
}

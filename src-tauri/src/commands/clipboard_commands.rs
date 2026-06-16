use crate::services::clipboard;
use crate::StorageState;
use std::fs;
use tauri::State;

#[tauri::command]
pub fn copy_to_clipboard(
    state: State<'_, StorageState>,
    _text: String,
    image_paths: Vec<String>,
) -> Result<(), String> {
    // Validate all image paths exist
    for path in &image_paths {
        let full_path = state.0.image_path(path);
        if !full_path.exists() {
            return Err(format!("Image not found: {}", path));
        }
    }
    Ok(())
}

#[tauri::command]
pub fn import_image(
    state: State<'_, StorageState>,
    source_path: String,
) -> Result<String, String> {
    clipboard::import_image(&state.0, &source_path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn read_image_as_base64(
    state: State<'_, StorageState>,
    filename: String,
) -> Result<String, String> {
    clipboard::read_image_base64(&state.0, &filename).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_image_file(
    state: State<'_, StorageState>,
    filename: String,
) -> Result<(), String> {
    clipboard::delete_image(&state.0, &filename).map_err(|e| e.to_string())
}

/// Read text content of any file (for import feature)
#[tauri::command]
pub fn read_text_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {}", e))
}

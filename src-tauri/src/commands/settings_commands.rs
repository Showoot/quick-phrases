use crate::models::settings::AppSettings;
use crate::services::settings_storage::SettingsStorage;
use crate::StorageState;
use auto_launch::AutoLaunch;
use std::fs;
use std::path::PathBuf;
use tauri::State;
use tauri_plugin_opener::OpenerExt;

#[tauri::command]
pub fn get_settings(
    state: State<'_, SettingsStorage>,
) -> Result<AppSettings, String> {
    state.load().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_settings(
    settings_state: State<'_, SettingsStorage>,
    settings: AppSettings,
) -> Result<(), String> {
    // Handle auto-start toggle using auto-launch directly
    let current_exe = std::env::current_exe().unwrap_or_default();
    let exe_path = current_exe.to_string_lossy().to_string();
    let auto = AutoLaunch::new("quick-phrases", &exe_path, &[] as &[&str]);

    if settings.auto_start {
        match auto.enable() {
            Ok(_) => {}
            Err(e) => eprintln!("Failed to enable autostart: {}", e),
        }
    } else {
        match auto.disable() {
            Ok(_) => {}
            Err(e) => eprintln!("Failed to disable autostart: {}", e),
        }
    }

    settings_state.save(&settings).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_storage_path(
    settings_state: State<'_, SettingsStorage>,
    storage_state: State<'_, StorageState>,
) -> Result<String, String> {
    let settings = settings_state.load().map_err(|e| e.to_string())?;
    Ok(settings
        .custom_storage_path
        .unwrap_or_else(|| storage_state.0.data_dir().to_string_lossy().to_string()))
}

#[tauri::command]
pub async fn set_storage_path(
    storage_state: State<'_, StorageState>,
    settings_state: State<'_, SettingsStorage>,
    new_path: String,
) -> Result<(), String> {
    let mut settings = settings_state.load().map_err(|e| e.to_string())?;
    let old_dir = storage_state.0.data_dir().to_path_buf();
    let new_dir = PathBuf::from(&new_path);

    if new_dir == old_dir {
        return Ok(());
    }

    // Create new directory
    fs::create_dir_all(&new_dir).map_err(|e| e.to_string())?;

    // Migrate data.json
    let old_data = old_dir.join("data.json");
    let new_data = new_dir.join("data.json");
    if old_data.exists() {
        fs::copy(&old_data, &new_data).map_err(|e| e.to_string())?;
    }

    // Migrate images directory
    let old_images = old_dir.join("images");
    let new_images = new_dir.join("images");
    if old_images.exists() {
        fs::create_dir_all(&new_images).map_err(|e| e.to_string())?;
        for entry in fs::read_dir(&old_images).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let dest = new_images.join(entry.file_name());
            fs::copy(entry.path(), &dest).map_err(|e| e.to_string())?;
        }
    }

    // Save new path in settings (takes effect on next app launch)
    settings.custom_storage_path = Some(new_path);
    settings_state.save(&settings).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn open_storage_dir(
    app: tauri::AppHandle,
    settings_state: State<'_, SettingsStorage>,
    storage_state: State<'_, StorageState>,
) -> Result<(), String> {
    let settings = settings_state.load().map_err(|e| e.to_string())?;
    let dir = settings
        .custom_storage_path
        .unwrap_or_else(|| storage_state.0.data_dir().to_string_lossy().to_string());
    app.opener().open_path(dir, None::<&str>).map_err(|e| e.to_string())?;
    Ok(())
}

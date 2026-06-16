mod commands;
mod error;
mod models;
mod services;

use models::settings::CloseBehavior;
use services::settings_storage::SettingsStorage;
use services::storage::Storage;
use std::path::PathBuf;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    Manager,
};

/// Managed state holding the storage instance
pub struct StorageState(pub Storage);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(Default::default(), None::<Vec<&str>>))
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Resolve config directory (fixed, for settings)
            let config_dir: PathBuf = app
                .path()
                .app_config_dir()
                .expect("Failed to resolve config directory");

            let settings_storage = SettingsStorage::new(config_dir);
            let settings = settings_storage.load().unwrap_or_default();

            // Resolve data directory (may be custom from settings)
            let app_data_dir: PathBuf = settings
                .custom_storage_path
                .map(PathBuf::from)
                .unwrap_or_else(|| {
                    app.path()
                        .app_data_dir()
                        .expect("Failed to resolve app data directory")
                });

            let storage = Storage::new(app_data_dir);
            storage.init().expect("Failed to initialize storage");

            app.manage(StorageState(storage));
            app.manage(settings_storage);

            // Build tray menu
            let show_item = MenuItemBuilder::with_id("show", "显示窗口").build(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "退出").build(app)?;
            let tray_menu = MenuBuilder::new(app)
                .item(&show_item)
                .separator()
                .item(&quit_item)
                .build()?;

            // Build tray icon
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("快捷话术")
                .menu(&tray_menu)
                .show_menu_on_left_click(false)
                .on_menu_event(move |app, event| {
                    match event.id().as_ref() {
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::DoubleClick { .. } = event {
                        if let Some(window) = tray.app_handle().get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::phrase_commands::list_phrases,
            commands::phrase_commands::get_phrase,
            commands::phrase_commands::create_phrase,
            commands::phrase_commands::update_phrase,
            commands::phrase_commands::delete_phrase,
            commands::phrase_commands::increment_usage,
            commands::phrase_commands::toggle_favorite,
            commands::phrase_commands::import_phrases,
            commands::category_commands::list_categories,
            commands::category_commands::create_category,
            commands::category_commands::update_category,
            commands::category_commands::delete_category,
            commands::category_commands::reorder_categories,
            commands::clipboard_commands::copy_to_clipboard,
            commands::clipboard_commands::import_image,
            commands::clipboard_commands::read_image_as_base64,
            commands::clipboard_commands::delete_image_file,
            commands::clipboard_commands::read_text_file,
            commands::settings_commands::get_settings,
            commands::settings_commands::update_settings,
            commands::settings_commands::get_storage_path,
            commands::settings_commands::set_storage_path,
            commands::settings_commands::open_storage_dir,
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // Check close behavior from settings
                let app = window.app_handle();
                if let Some(settings_state) = app.try_state::<SettingsStorage>() {
                    if let Ok(settings) = settings_state.load() {
                        if settings.close_behavior == CloseBehavior::Quit {
                            // Allow the window to close (app will exit if last window)
                            return;
                        }
                    }
                }
                // Default: hide to tray
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

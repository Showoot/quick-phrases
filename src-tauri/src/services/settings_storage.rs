use crate::error::AppError;
use crate::models::settings::AppSettings;
use std::fs;
use std::path::PathBuf;

pub struct SettingsStorage {
    file_path: PathBuf,
}

impl SettingsStorage {
    pub fn new(config_dir: PathBuf) -> Self {
        let file_path = config_dir.join("settings.json");
        Self { file_path }
    }

    pub fn load(&self) -> Result<AppSettings, AppError> {
        if !self.file_path.exists() {
            return Ok(AppSettings::default());
        }
        let content = fs::read_to_string(&self.file_path)?;
        if content.trim().is_empty() {
            return Ok(AppSettings::default());
        }
        let settings: AppSettings = serde_json::from_str(&content)?;
        Ok(settings)
    }

    pub fn save(&self, settings: &AppSettings) -> Result<(), AppError> {
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let tmp = self.file_path.with_extension("json.tmp");
        let json = serde_json::to_string_pretty(settings)?;
        fs::write(&tmp, json)?;
        fs::rename(&tmp, &self.file_path)?;
        Ok(())
    }
}

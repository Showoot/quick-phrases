use crate::error::AppError;
use crate::models::category::Category;
use crate::models::phrase::Phrase;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppData {
    pub categories: Vec<Category>,
    pub phrases: Vec<Phrase>,
}

pub struct Storage {
    data_dir: PathBuf,
    data_file: PathBuf,
    images_dir: PathBuf,
}

impl Storage {
    pub fn new(app_data_dir: PathBuf) -> Self {
        let data_file = app_data_dir.join("data.json");
        let images_dir = app_data_dir.join("images");
        Self {
            data_dir: app_data_dir,
            data_file,
            images_dir,
        }
    }

    /// Initialize storage: create directories and default data file if not exists
    pub fn init(&self) -> Result<(), AppError> {
        fs::create_dir_all(&self.data_dir)?;
        fs::create_dir_all(&self.images_dir)?;
        if !self.data_file.exists() {
            self.save(&AppData::default())?;
        }
        Ok(())
    }

    /// Load all data from JSON file
    pub fn load(&self) -> Result<AppData, AppError> {
        if !self.data_file.exists() {
            return Ok(AppData::default());
        }
        let content = fs::read_to_string(&self.data_file)?;
        if content.trim().is_empty() {
            return Ok(AppData::default());
        }
        let data: AppData = serde_json::from_str(&content)?;
        Ok(data)
    }

    /// Atomically save data to JSON file (write to tmp, then rename)
    pub fn save(&self, data: &AppData) -> Result<(), AppError> {
        let tmp_file = self.data_file.with_extension("json.tmp");
        let json = serde_json::to_string_pretty(data)?;
        fs::write(&tmp_file, json)?;
        fs::rename(&tmp_file, &self.data_file)?;
        Ok(())
    }

    pub fn images_dir(&self) -> &Path {
        &self.images_dir
    }

    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    /// Get absolute path for an image filename
    pub fn image_path(&self, filename: &str) -> PathBuf {
        self.images_dir.join(filename)
    }
}

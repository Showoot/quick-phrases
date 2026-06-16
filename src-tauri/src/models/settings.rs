use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CloseBehavior {
    MinimizeToTray,
    Quit,
}

impl Default for CloseBehavior {
    fn default() -> Self {
        CloseBehavior::MinimizeToTray
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    #[serde(default)]
    pub auto_start: bool,
    #[serde(default)]
    pub close_behavior: CloseBehavior,
    #[serde(default)]
    pub custom_storage_path: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            auto_start: false,
            close_behavior: CloseBehavior::MinimizeToTray,
            custom_storage_path: None,
        }
    }
}

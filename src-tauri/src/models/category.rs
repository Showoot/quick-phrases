use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub sort_order: u32,
    pub created_at: i64,
}

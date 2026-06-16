use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phrase {
    pub id: String,
    pub category_id: String,
    pub title: String,
    pub text_content: String,
    pub image_paths: Vec<String>,
    pub tags: Vec<String>,
    pub is_favorite: bool,
    pub usage_count: u32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePhraseRequest {
    pub category_id: String,
    pub title: String,
    pub text_content: String,
    pub image_paths: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePhraseRequest {
    pub category_id: Option<String>,
    pub title: Option<String>,
    pub text_content: Option<String>,
    pub image_paths: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub is_favorite: Option<bool>,
}

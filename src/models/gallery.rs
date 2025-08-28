use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct GalleryEntity {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub cover_url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct GalleryResponse {
    pub name: String,
    pub title: String,
    pub description: String,
    pub cover_image: String,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

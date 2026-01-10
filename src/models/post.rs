use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct BlogPostType {
    pub type_key: String,
    pub name: String,
    pub parent_type: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct BlogPost {
    pub id: i64,
    pub title: String,
    pub r#type: Option<String>,
    pub description: Option<String>,
    pub file_url: String,
    pub cover_image_url: Option<String>,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub published_at: Option<chrono::NaiveDateTime>,
    pub slug: String,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub keywords: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ArticleContentParams {
    pub article_name: String,
}

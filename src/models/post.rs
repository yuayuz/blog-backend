use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct BlogPostType {
    pub type_key: String,
    pub name: String,
    pub parent_type: Option<String>,
}

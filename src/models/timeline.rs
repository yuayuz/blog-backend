use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Timeline {
    pub id: i64,
    pub happened_at: NaiveDate,
    pub title: String,
    pub note: Option<String>,
    pub r#type: String,
    pub mood: Option<String>,
    pub created_at: DateTime<Utc>,
}

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 时间线条目（与 `timelines` 表对应）。
///
/// 记录某一天发生的一件事，可标记类型和心情。
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Timeline {
    pub id: i64,
    /// 事件发生的日期
    pub happened_at: NaiveDate,
    pub title: String,
    /// 补充说明，可选
    pub note: Option<String>,
    /// 事件类型，如 `"blog"`、`"life"`、`"project"`
    pub r#type: String,
    /// 心情标记，如 `"😊"`、`"😢"`
    pub mood: Option<String>,
    /// 记录创建时间
    pub created_at: DateTime<Utc>,
}

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 图库实体（与 `galleries` 表对应）。
#[derive(Debug, Serialize, FromRow)]
pub struct GalleryEntity {
    pub id: i32,
    pub title: String,
    pub description: String,
    /// 封面图 OSS 路径
    pub cover_url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// 图库的 API 返回结构。
#[derive(Debug, Serialize)]
pub struct GalleryResponse {
    /// 图库名，也是 OSS 上的目录名
    pub name: String,
    pub title: String,
    pub description: String,
    pub cover_image: String,
}

/// 分页查询参数。
///
/// 示例: `?page=1&page_size=20`
#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

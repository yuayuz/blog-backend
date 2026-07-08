use serde::{Deserialize, Serialize};

/// 照片 API 返回结构。
///
/// `width` / `height` 要么都传要么都不传；
/// 不传时前端随机选一个展示比例。
#[derive(Debug, Clone, Serialize)]
pub struct PhotoResponse {
    pub id: i32,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
}

/// 照片墙 API 顶层响应。
#[derive(Debug, Serialize)]
pub struct PhotosResponse {
    pub photos: Vec<PhotoResponse>,
}

/// 查询参数。
#[derive(Deserialize)]
pub struct PhotoQueryParams {
    pub count: Option<i64>,
}


use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

#[derive(Deserialize)]
pub struct UploadArticleRequest {
    /// 文件名（不含扩展名），如 "test_article"
    pub file_name: String,
    /// Markdown 原始内容
    pub content: String,
}

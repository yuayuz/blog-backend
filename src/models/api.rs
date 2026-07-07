use serde::{Deserialize, Serialize};

/// 统一的 API 响应结构。
///
/// 虽然当前代码中未大量使用，但可作为未来统一封装的模板。
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

/// 文章上传的 JSON 请求体（备用，当前使用 multipart 方式）。
///
/// 实际项目中上传接口已改用 `multipart/form-data`，
/// 此结构体保留作为 JSON 上传方式的备选。
#[derive(Deserialize)]
pub struct UploadArticleRequest {
    /// 文件名（不含扩展名），如 "test_article"
    pub file_name: String,
    /// Markdown 原始内容
    pub content: String,
}

//! 图片代理服务：从 OSS 读取图片并原样返回。

use crate::AppState;
use axum::extract::{Path, State};
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use bytes::Bytes;

/// 图片服务根路径。
pub async fn image_root() -> impl IntoResponse {
    (StatusCode::OK, "Image API")
}

/// 通过 OSS 路径获取图片。
///
/// `{*path}` 对应 OSS 上的对象 key，
/// 返回 `image/webp` 类型，前端可直接用作 `<img src="...">`。
pub async fn get_image(
    Path(path): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let bucket = &state.bucket;
    match bucket.get_object(&path).await {
        Ok(obj) => {
            let body: Bytes = obj.bytes().clone();
            let content_type = "image/webp";

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, content_type)
                .body(body.into())
                .unwrap()
        }
        Err(_) => (StatusCode::NOT_FOUND, "图片不存在").into_response(),
    }
}

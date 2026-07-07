//! 图片代理 API 路由。

use crate::AppState;
use crate::service::image_service::{get_image, image_root};
use axum::Router;
use axum::routing::get;

/// 构建 `/rust/image/*` 下的子路由。
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(image_root))
        .route("/{*path}", get(get_image))
}

//! 顶层路由聚合，所有 API 统一挂载在 `/rust` 路径下。

use crate::AppState;
use crate::handlers::{blog_handler, gallery_handler, image_handler, timeline_handler};
use axum::Router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;

/// 构建整个应用的路由树：
///
/// ```text
/// /rust
///   ├── /             → "Hello,Rust!"
///   ├── /gallery/...  → 图库相关接口
///   ├── /image/...    → 图片代理
///   ├── /blog/...     → 博客文章 CRUD
///   └── /timeline/... → 时间线
/// ```
pub fn router() -> Router<AppState> {
    Router::new().nest(
        "/rust",
        Router::new()
            .route("/", get(root))
            .nest("/gallery", gallery_handler::router())
            .nest("/image", image_handler::router())
            .nest("/blog", blog_handler::router())
            .nest("/timeline", timeline_handler::router()),
    )
}

/// 根路径，用于健康检查。
async fn root() -> impl IntoResponse {
    (StatusCode::OK, "Hello,Rust!")
}

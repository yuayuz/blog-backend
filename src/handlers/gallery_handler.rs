//! 图库相关 API 路由。

use crate::AppState;
use crate::models::gallery::PaginationParams;
use crate::service::gallery_service::{get_gallery_images, list_galleries};
use axum::Router;
use axum::extract::{Path, Query, State};
use axum::routing::get;
use sqlx::PgPool;
use std::sync::Arc;

/// 构建 `/gallery/*` 下的子路由。
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_galleries_handler))
        .route("/{name}/images", get(get_gallery_images_handler))
}

/// `GET /gallery/` — 列出所有图库。
async fn list_galleries_handler(
    State(state): State<AppState>,
) -> impl axum::response::IntoResponse {
    let pool: PgPool = state.pool.clone();
    let bucket: Arc<_> = state.bucket.clone();

    list_galleries(pool, bucket).await
}

/// `GET /gallery/{name}/images?page=1&page_size=9` — 获取图库内图片列表。
async fn get_gallery_images_handler(
    Path(name): Path<String>,
    Query(params): Query<PaginationParams>,
    State(state): State<AppState>,
) -> impl axum::response::IntoResponse {
    let bucket: Arc<_> = state.bucket.clone();

    get_gallery_images(name, params, bucket).await
}

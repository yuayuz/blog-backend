//! 照片墙 API 路由。

use crate::AppState;
use crate::models::photo::PhotoQueryParams;
use crate::service::photo_service::get_photos;
use axum::Router;
use axum::extract::{Query, State};
use axum::routing::get;

/// 构建 `/rust/photos/*` 下的子路由。
pub fn router() -> Router<AppState> {
    Router::new().route("/", get(photos_handler))
}

/// `GET /rust/photos/?count=20` — 随机获取照片列表。
async fn photos_handler(
    Query(params): Query<PhotoQueryParams>,
    State(state): State<AppState>,
) -> impl axum::response::IntoResponse {
    get_photos(state.pool.clone(), state.bucket.clone(), params, state.photo_cache).await
}

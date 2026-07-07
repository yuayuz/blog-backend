//! 时间线 API 路由。

use axum::extract::State;
use axum::response::IntoResponse;
use crate::AppState;
use axum::Router;
use axum::routing::get;
use sqlx::PgPool;
use crate::service::timeline_service::get_all_timeline_service;

/// 构建 `/rust/timeline` 子路由。
pub fn router() -> Router<AppState> {
    Router::new().route("/", get(get_timeline_handler))
}

/// `GET /rust/timeline/` — 获取全部时间线条目。
async fn get_timeline_handler(State(state): State<AppState>) -> impl IntoResponse {
    let pool: PgPool = state.pool.clone();

    get_all_timeline_service(pool).await
}

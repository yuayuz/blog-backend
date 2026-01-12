use axum::extract::State;
use axum::response::IntoResponse;
use crate::AppState;
use axum::Router;
use axum::routing::get;
use sqlx::PgPool;
use crate::service::timeline_service::get_all_timeline_service;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(get_timeline_handler))
}

async fn get_timeline_handler(State(state): State<AppState>) -> impl IntoResponse {
    let pool: PgPool = state.pool.clone();

    get_all_timeline_service(pool).await
}

use crate::AppState;
use crate::service::blog_service::{
    get_all_types_service, get_child_types_service, get_primary_types_service,
};
use axum::Router;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use sqlx::PgPool;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/allTypes", get(get_all_types_handler))
        .route("/primaryTypes", get(get_primary_types_handler))
        .route("/childTypes/{parent}", get(get_child_types_handler))
}

async fn get_all_types_handler(State(state): State<AppState>) -> impl IntoResponse {
    let pool: PgPool = state.pool.clone();

    get_all_types_service(pool).await
}

async fn get_primary_types_handler(State(state): State<AppState>) -> impl IntoResponse {
    let pool: PgPool = state.pool.clone();

    get_primary_types_service(pool).await
}

pub async fn get_child_types_handler(
    State(state): State<AppState>,
    Path(parent): Path<String>,
) -> impl IntoResponse {
    let pool = state.pool.clone();
    get_child_types_service(pool, parent).await
}

use crate::AppState;
use crate::models::post::ArticleContentParams;
use crate::repository::blog_repository::get_posts_by_tag;
use crate::service::blog_service::{
    get_all_posts_service, get_all_types_service, get_article_content_service,
    get_child_types_service, get_posts_by_tag_service, get_posts_service,
    get_primary_types_service,
};
use axum::Router;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::routing::get;
use sqlx::PgPool;
use std::sync::Arc;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/allTypes", get(get_all_types_handler))
        .route("/primaryTypes", get(get_primary_types_handler))
        .route("/childTypes/{parent}", get(get_child_types_handler))
        .route("/allPosts", get(get_all_posts_handler))
        .route("/posts/type/{type_key}", get(get_posts_handler))
        .route("/posts/tag/{tag}", get(get_posts_by_tag_handler))
        .route("/article", get(get_article_content_handler))
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

async fn get_all_posts_handler(State(state): State<AppState>) -> impl IntoResponse {
    let pool: PgPool = state.pool.clone();

    get_all_posts_service(pool).await
}

async fn get_posts_handler(
    State(state): State<AppState>,
    Path(type_key): Path<String>,
) -> impl IntoResponse {
    let pool: PgPool = state.pool.clone();

    get_posts_service(pool, type_key).await
}

async fn get_posts_by_tag_handler(
    State(state): State<AppState>,
    Path(tag): Path<String>,
) -> impl IntoResponse {
    let pool: PgPool = state.pool.clone();

    get_posts_by_tag_service(pool, tag).await
}
async fn get_article_content_handler(
    Query(params): Query<ArticleContentParams>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let bucket: Arc<_> = state.bucket.clone();
    get_article_content_service(params, bucket).await
}

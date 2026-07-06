use crate::AppState;
use crate::models::post::ArticleContentParams;
use crate::service::blog_service::{
    get_all_posts_service, get_all_types_service, get_article_content_service,
    get_child_types_service, get_posts_by_tag_service, get_posts_service,
    get_primary_types_service, upload_post_service,
};
use axum::Router;
use axum::extract::{Multipart, Path, Query, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use sqlx::PgPool;
use std::sync::Arc;
use tracing::error;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/allTypes", get(get_all_types_handler))
        .route("/primaryTypes", get(get_primary_types_handler))
        .route("/childTypes/{parent}", get(get_child_types_handler))
        .route("/allPosts", get(get_all_posts_handler))
        .route("/posts/type/{type_key}", get(get_posts_handler))
        .route("/posts/tag/{tag}", get(get_posts_by_tag_handler))
        .route("/article", get(get_article_content_handler))
        .route("/upload", post(upload_post_handler))
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

async fn upload_post_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    use axum::http::StatusCode;

    // 从 multipart 中读取第一个字段（即上传的 .md 文件）
    while let Ok(Some(field)) = multipart.next_field().await {
        let file_name = field.file_name().unwrap_or("untitled.md").to_string();

        let data = match field.bytes().await {
            Ok(d) => d,
            Err(e) => {
                error!("读取上传文件失败: {}", e);
                return (StatusCode::BAD_REQUEST, "读取文件失败").into_response();
            }
        };

        let raw_md = match String::from_utf8(data.to_vec()) {
            Ok(s) => s,
            Err(e) => {
                error!("文件编码错误: {}", e);
                return (StatusCode::BAD_REQUEST, "文件编码错误，需要 UTF-8").into_response();
            }
        };

        return upload_post_service(state.pool.clone(), state.bucket.clone(), file_name, raw_md)
            .await
            .into_response();
    }

    (StatusCode::BAD_REQUEST, "未收到文件").into_response()
}

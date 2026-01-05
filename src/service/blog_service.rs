use crate::models::post::{BlogPost, BlogPostType};
use crate::repository::blog_repository::{
    get_all_posts, get_all_types, get_child_types, get_posts, get_primary_types,
};
use axum::Json;
use axum::response::IntoResponse;
use sqlx::PgPool;
use tracing::error;

pub async fn get_all_types_service(pool: PgPool) -> impl IntoResponse {
    let rows: Vec<BlogPostType> = get_all_types(pool).await.unwrap_or_else(|e| {
        error!("数据库查询失败: {}", e);
        Vec::new()
    });
    Json(rows)
}
pub async fn get_primary_types_service(pool: PgPool) -> impl IntoResponse {
    let rows: Vec<BlogPostType> = get_primary_types(pool).await.unwrap_or_else(|e| {
        error!("数据库查询失败: {}", e);
        Vec::new()
    });
    Json(rows)
}
pub async fn get_child_types_service(pool: PgPool, parent: String) -> impl IntoResponse {
    let rows: Vec<BlogPostType> = get_child_types(pool, &parent).await.unwrap_or_else(|e| {
        error!("数据库查询失败: {}", e);
        Vec::new()
    });
    Json(rows)
}

pub async fn get_all_posts_service(pool: PgPool) -> impl IntoResponse {
    let rows: Vec<BlogPost> = get_all_posts(pool).await.unwrap_or_else(|e| {
        error!("数据库查询失败: {}", e);
        Vec::new()
    });
    Json(rows)
}

pub async fn get_posts_service(pool: PgPool, type_key: String) -> impl IntoResponse {
    let rows: Vec<BlogPost> = get_posts(pool, &type_key).await.unwrap_or_else(|e| {
        error!("数据库查询失败: {}", e);
        Vec::new()
    });
    Json(rows)
}

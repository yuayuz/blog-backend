use crate::models::post::BlogPostType;
use crate::repository::blog_repository::{get_child_types, get_primary_types,get_all_types};
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

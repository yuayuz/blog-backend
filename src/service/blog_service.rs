use crate::models::md_parser::parse_md;
use crate::models::post::{ArticleContentParams, BlogPost, BlogPostType};
use crate::repository::blog_repository::{
    get_all_posts, get_all_types, get_child_types, get_posts, get_posts_by_tag, get_primary_types,
};
use axum::Json;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use s3::Bucket;
use serde_json;
use sqlx::PgPool;
use std::sync::Arc;
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
pub async fn get_posts_by_tag_service(pool: PgPool, tag: String) -> impl IntoResponse {
    let rows: Vec<BlogPost> = get_posts_by_tag(pool, &tag).await.unwrap_or_else(|e| {
        error!("数据库查询失败: {}", e);
        Vec::new()
    });
    Json(rows)
}

pub async fn get_article_content_service(
    params: ArticleContentParams,
    bucket: Arc<Bucket>,
) -> impl IntoResponse {
    let object_key = format!("article/{}.md", params.article_name);

    let obj = match bucket.get_object(&object_key).await {
        Ok(obj) => obj,
        Err(_) => return (StatusCode::NOT_FOUND, "文章不存在").into_response(),
    };

    let body = obj.bytes().clone();

    let content = match String::from_utf8(body.to_vec()) {
        Ok(c) => c,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "文章编码错误").into_response();
        }
    };

    match parse_md(&content) {
        Ok((front_matter, markdown_content)) => Json(serde_json::json!({
            "front_matter": front_matter,
            "content": markdown_content
        }))
        .into_response(),
        Err(_) => Json(serde_json::json!({
            "front_matter": null,
            "content": content
        }))
        .into_response(),
    }
}

use crate::models::md_parser::parse_md;
use crate::models::post::{ArticleContentParams, BlogPost, BlogPostType};
use crate::repository::blog_repository::{
    get_all_posts, get_all_types, get_child_types, get_posts, get_posts_by_tag, get_primary_types,
    insert_post,
};
use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use regex::Regex;
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

pub async fn upload_post_service(
    pool: PgPool,
    bucket: Arc<Bucket>,
    file_name: String,
    raw_md: String,
) -> impl IntoResponse {
    // 1. 解析 Markdown
    let (front_matter, md_content) = match parse_md(&raw_md) {
        Ok(v) => v,
        Err(e) => {
            error!("解析 Markdown 失败: {}", e);
            return (StatusCode::BAD_REQUEST, format!("解析失败: {}", e)).into_response();
        }
    };

    let category = front_matter.category;
    let tags = front_matter.tags;
    let description = front_matter.description;
    let date = front_matter
        .date
        .unwrap_or_else(|| chrono::Utc::now().format("%Y-%m-%d").to_string());

    // 去掉文件名中的 .md 后缀，作为 title
    let title = file_name
        .strip_suffix(".md")
        .unwrap_or(&file_name)
        .to_string();
    let oss_key = format!("article/{}.md", title);
    let file_url = format!("/{}", title);

    // 2. 上传 Markdown 到 OSS
    let response = match bucket
        .put_object_with_content_type(&oss_key, md_content.as_bytes(), "text/markdown")
        .await
    {
        Ok(r) => r,
        Err(e) => {
            error!("OSS 上传失败: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "OSS 上传失败").into_response();
        }
    };

    if response.status_code() != 200 {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("OSS 上传失败，状态码: {}", response.status_code()),
        )
            .into_response();
    }

    // 3. 生成 slug
    let slug = generate_slug(&title);

    // 4. 写入数据库
    match insert_post(
        &pool,
        &title,
        &slug,
        &file_url,
        category.as_deref(),
        description.as_deref(),
        tags.as_ref(),
    )
    .await
    {
        Ok(post) => Json(serde_json::json!({
            "id": post.id,
            "slug": post.slug,
            "title": post.title,
        }))
        .into_response(),
        Err(e) => {
            error!("数据库写入失败: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "数据库写入失败").into_response()
        }
    }
}

/// 根据标题生成 URL 友好的 slug
fn generate_slug(title: &str) -> String {
    let slug = title
        .to_lowercase()
        .chars()
        .map(|c| match c {
            'a'..='z' | '0'..='9' => c,
            ' ' | '-' | '_' => '-',
            _ => '-',
        })
        .collect::<String>();

    let slug = Regex::new("-+")
        .unwrap()
        .replace_all(&slug, "-")
        .to_string();

    slug.trim_matches('-').to_string()
}

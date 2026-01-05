use crate::models::post::{BlogPost, BlogPostType};
use sqlx::PgPool;
use std::mem::take;

pub async fn get_all_types(pool: PgPool) -> Result<Vec<BlogPostType>, sqlx::Error> {
    sqlx::query_as::<_, BlogPostType>(
        "SELECT type AS type_key,name,parent_type FROM blog_post_types",
    )
    .fetch_all(&pool)
    .await
}

pub async fn get_primary_types(pool: PgPool) -> Result<Vec<BlogPostType>, sqlx::Error> {
    sqlx::query_as::<_, BlogPostType>(
        "SELECT type AS type_key,name,parent_type FROM blog_post_types WHERE parent_type IS NULL",
    )
    .fetch_all(&pool)
    .await
}
pub async fn get_child_types(
    pool: PgPool,
    parent: &String,
) -> Result<Vec<BlogPostType>, sqlx::Error> {
    sqlx::query_as::<_, BlogPostType>(
        "SELECT type AS type_key,name,parent_type FROM blog_post_types WHERE parent_type = $1",
    )
    .bind(parent)
    .fetch_all(&pool)
    .await
}

pub async fn get_all_posts(pool: PgPool) -> Result<Vec<BlogPost>, sqlx::Error> {
    sqlx::query_as::<_, BlogPost>("SELECT * FROM blog_posts")
        .fetch_all(&pool)
        .await
}
pub async fn get_posts(pool: PgPool, type_key: &String) -> Result<Vec<BlogPost>, sqlx::Error> {
    // 首先查询所有子类型
    let child_types = sqlx::query_as::<_, BlogPostType>(
        "SELECT type AS type_key, name, parent_type FROM blog_post_types WHERE parent_type = $1",
    )
    .bind(type_key)
    .fetch_all(&pool)
    .await?;

    // 收集所有相关的类型键（父类型和子类型）
    let mut all_types = Vec::new();
    all_types.push(type_key.clone()); // 添加父类型
    for child_type in child_types {
        all_types.push(child_type.type_key);
    }

    // 构建 IN 查询的占位符
    let placeholders: Vec<String> = (1..=all_types.len())
        .map(|i| format!("${}", i))
        .collect();
    let query_placeholders = placeholders.join(",");

    // 构建最终查询语句
    let query = format!(
        "SELECT * FROM blog_posts WHERE type IN ({})",
        query_placeholders
    );

    // 绑定所有类型参数
    let mut query_builder = sqlx::query_as::<_, BlogPost>(&query);
    for type_val in all_types {
        query_builder = query_builder.bind(type_val);
    }

    query_builder.fetch_all(&pool).await
}


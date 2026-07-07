use crate::models::post::{BlogPost, BlogPostType};
use sqlx::PgPool;

/// 查询所有文章分类（一级 + 子级全部返回）。
pub async fn get_all_types(pool: PgPool) -> Result<Vec<BlogPostType>, sqlx::Error> {
    sqlx::query_as::<_, BlogPostType>(
        "SELECT type AS type_key,name,parent_type FROM blog_post_types",
    )
    .fetch_all(&pool)
    .await
}

/// 查询所有一级分类（`parent_type IS NULL`）。
pub async fn get_primary_types(pool: PgPool) -> Result<Vec<BlogPostType>, sqlx::Error> {
    sqlx::query_as::<_, BlogPostType>(
        "SELECT type AS type_key,name,parent_type FROM blog_post_types WHERE parent_type IS NULL",
    )
    .fetch_all(&pool)
    .await
}

/// 查询某父分类下的所有子分类。
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

/// 查询所有博客文章（无分页，适合数据量不大的场景）。
pub async fn get_all_posts(pool: PgPool) -> Result<Vec<BlogPost>, sqlx::Error> {
    sqlx::query_as::<_, BlogPost>("SELECT * FROM blog_posts")
        .fetch_all(&pool)
        .await
}

/// 按分类查询文章：不仅查直接属于该分类的文章，
/// 还会一并查出其所有子分类下的文章。
pub async fn get_posts(pool: PgPool, type_key: &String) -> Result<Vec<BlogPost>, sqlx::Error> {
    // 首先查询所有子类型
    let child_types = sqlx::query_as::<_, BlogPostType>(
        "SELECT type AS type_key, name, parent_type FROM blog_post_types WHERE parent_type = $1",
    )
    .bind(type_key)
    .fetch_all(&pool)
    .await?;

    // 收集父类型 + 所有子类型的 type_key
    let mut all_types = Vec::new();
    all_types.push(type_key.clone());
    for child_type in child_types {
        all_types.push(child_type.type_key);
    }

    // 动态构建 IN ($1, $2, ...) 占位符
    let placeholders: Vec<String> = (1..=all_types.len()).map(|i| format!("${}", i)).collect();
    let query_placeholders = placeholders.join(",");

    let query = format!(
        "SELECT * FROM blog_posts WHERE type IN ({})",
        query_placeholders
    );

    // 逐个绑定参数
    let mut query_builder = sqlx::query_as::<_, BlogPost>(&query);
    for type_val in all_types {
        query_builder = query_builder.bind(type_val);
    }

    query_builder.fetch_all(&pool).await
}

/// 按标签查询文章：使用 PostgreSQL 数组包含操作符 `@>`。
pub async fn get_posts_by_tag(pool: PgPool, tag: &String) -> Result<Vec<BlogPost>, sqlx::Error> {
    sqlx::query_as::<_, BlogPost>("SELECT * FROM blog_posts WHERE tags @> ARRAY[$1]")
        .bind(tag)
        .fetch_all(&pool)
        .await
}

/// 插入一条新文章记录，状态默认设为 `"published"`。
///
/// 使用 `RETURNING *` 返回插入后的完整记录（包含自增 id 和时间戳）。
pub async fn insert_post(
    pool: &PgPool,
    title: &str,
    slug: &str,
    file_url: &str,
    post_type: Option<&str>,
    description: Option<&str>,
    tags: Option<&Vec<String>>,
) -> Result<BlogPost, sqlx::Error> {
    sqlx::query_as::<_, BlogPost>(
        "INSERT INTO blog_posts (title, slug, file_url, type, description, tags, status) 
         VALUES ($1, $2, $3, $4, $5, $6, 'published')
         RETURNING *"
    )
    .bind(title)
    .bind(slug)
    .bind(file_url)
    .bind(post_type)
    .bind(description)
    .bind(tags)
    .fetch_one(pool)
    .await
}

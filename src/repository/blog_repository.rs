use crate::models::post::BlogPostType;
use sqlx::PgPool;

pub async fn get_primary_types(pool: PgPool) -> Result<Vec<BlogPostType>, sqlx::Error> {
    sqlx::query_as::<_, BlogPostType>(
        "SELECT type AS type_key,name,parent_type FROM blog_post_types WHERE parent_type IS NULL",
    )
    .fetch_all(&pool)
    .await
}
pub async fn get_child_types(pool: PgPool, parent: &String) -> Result<Vec<BlogPostType>, sqlx::Error> {
    sqlx::query_as::<_, BlogPostType>(
        "SELECT type AS type_key,name,parent_type FROM blog_post_types WHERE parent_type = $1",
    )
    .bind(parent)
    .fetch_all(&pool)
    .await
}

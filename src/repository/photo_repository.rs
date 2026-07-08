use crate::models::photo::PhotoEntity;
use sqlx::PgPool;

/// 随机选取 count 条照片记录。
///
/// PostgreSQL 使用 `ORDER BY RANDOM()` 每次返回不同的结果。
/// `limit` 会被钳制在 15~30 之间。
pub async fn get_random_photos(pool: &PgPool, count: i64) -> Result<Vec<PhotoEntity>, sqlx::Error> {
    // 钳制到 15~30 的合理范围
    let limit = count.clamp(15, 30);

    sqlx::query_as::<_, PhotoEntity>(
        "SELECT id, path, title, width, height FROM photos ORDER BY RANDOM() LIMIT $1",
    )
    .bind(limit)
    .fetch_all(pool)
    .await
}
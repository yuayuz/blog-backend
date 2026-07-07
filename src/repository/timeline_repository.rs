use sqlx::PgPool;
use crate::models::timeline::Timeline;

/// 查询所有时间线条目。
pub async fn get_all_timeline(pool: PgPool) -> Result<Vec<Timeline>, sqlx::Error> {
    sqlx::query_as::<_, Timeline>("SELECT * FROM timeline")
        .fetch_all(&pool)
        .await
}

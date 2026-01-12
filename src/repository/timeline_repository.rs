use sqlx::PgPool;
use crate::models::timeline::Timeline;

pub async fn get_all_timeline(pool: PgPool) -> Result<Vec<Timeline>, sqlx::Error> {
    sqlx::query_as::<_, Timeline>("SELECT * FROM timeline")
        .fetch_all(&pool)
        .await
}
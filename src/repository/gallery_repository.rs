use sqlx::PgPool;
use crate::models::gallery::GalleryEntity;

pub async fn get_all_galleries(pool: PgPool) -> Result<Vec<GalleryEntity>, sqlx::Error> {
    sqlx::query_as::<_, GalleryEntity>("SELECT * FROM galleries")
        .fetch_all(&pool)
        .await
}
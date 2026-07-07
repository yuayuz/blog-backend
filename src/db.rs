use sqlx::{PgPool, postgres::PgPoolOptions};

/// 创建 PostgreSQL 连接池。
///
/// 从环境变量 `DATABASE_URL` 读取连接串，
/// 最大连接数固定为 5（适合个人博客的小规模并发）。
///
/// # Panics
/// 如果 `DATABASE_URL` 未设置或数据库无法连接，直接 panic。
pub async fn create_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL")
}

mod handlers;
mod routes;
mod service;

pub use blog_backend::db;
pub use blog_backend::models;
pub use blog_backend::repository;
pub use blog_backend::s3_client;
use crate::routes::router as routes_router;
use crate::service::photo_service::PhotoCache;
use axum::Router;
use axum::http::Method;
use db::create_pool;
use dotenvy::dotenv;
use s3::Bucket;
use sqlx::PgPool;
use std::env;
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

/// 应用全局共享状态，所有 handler 通过 axum 的 `State` 提取器获取。
///
/// - `pool`: PostgreSQL 连接池，读写数据库时从这里获取连接。
/// - `bucket`: 阿里云 OSS Bucket 实例，文章和图片都存储在 OSS 上。
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub bucket: Arc<Bucket>,
    pub photo_cache: Arc<Mutex<PhotoCache>>,
}

/// 程序入口：加载配置 → 初始化 OSS/DB → 构建路由 → 启动 HTTP 服务
///
/// 启动前需要确保 `.env` 文件存在且配置正确，否则会 panic。
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载 .env 文件
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let bucket = s3_client::init_bucket().await?;

    let pool = create_pool().await;

    let photo_cache = Arc::new(Mutex::new(PhotoCache::new(180))); // 3 minutes

    let state = AppState {
        pool,
        bucket,
        photo_cache,
    };

    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:8000".to_string());

    let cors = CorsLayer::new()
        .allow_origin(Any) // 允许所有来源，生产环境建议限定具体域名
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    let app = Router::new()
        .merge(routes_router())
        .with_state(state)
        .layer(cors);

    info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

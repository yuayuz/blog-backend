mod db;
mod routes;
mod s3_client;

use routes::{gallery, image};

use axum::http::{Method, StatusCode};
use axum::response::{IntoResponse};
use axum::{Router, routing::get};
use db::create_pool;
use dotenvy::dotenv;
use s3::Bucket;
use sqlx::PgPool;
use std::env;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub bucket: Arc<Bucket>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载 .env 文件
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let bucket = s3_client::init_bucket().await?;

    let pool = create_pool().await;

    let state = AppState { pool, bucket };

    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:8000".to_string());

    let cors = CorsLayer::new()
        .allow_origin(Any) // 允许所有来源，生产环境建议限定具体域名
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root))
        .nest("/gallery", gallery::router())
        .nest("/image", image::router())
        .with_state(state) // ✅ 整个 AppState 传进来
        .layer(cors);

    info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn root() -> impl IntoResponse {
    (StatusCode::OK, "hello")
}

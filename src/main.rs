mod routes;
mod s3_client;
use routes::{gallery, image};

use axum::body::Body;
use axum::extract::Extension;
use axum::http::{Method, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::{Router, routing::get};
use bytes::Bytes;
use dotenvy::dotenv;
use s3::Bucket;
use std::env;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载 .env 文件
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let bucket = s3_client::init_bucket().await?;

    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:8000".to_string());

    let cors = CorsLayer::new()
        .allow_origin(Any) // 允许所有来源，生产环境建议限定具体域名
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root))
        .nest("/gallery", gallery::router(bucket.clone()))
        .nest("/image", image::router(bucket.clone()))
        .layer(cors)
        .layer(Extension(bucket));

    info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn root(Extension(bucket): Extension<Arc<Bucket>>) -> impl IntoResponse {
    match bucket.get_object("test.txt").await {
        Ok(obj) => {
            let body = obj.bytes();
            let response: Response<Body> = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "text/plain")
                .body(Body::from(body.clone()))
                .unwrap();
            response
        }
        Err(_) => {
            let body = Bytes::from("文件不存在");
            let response: Response<Body> = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header(header::CONTENT_TYPE, "text/plain")
                .body(Body::from(body))
                .unwrap();
            response
        }
    }
}

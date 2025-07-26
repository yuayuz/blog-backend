mod s3_client;
mod routes;
use routes::{gallery};

use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use axum::{routing::get, Router};
use axum::body::Body;
use axum::extract::Extension;
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use s3::Bucket;
use tracing::{info};
use bytes::Bytes;

#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>> {

    // 加载 .env 文件
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let bucket = s3_client::init_bucket().await?;

    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:8000".to_string());

    let app = Router::new()
        .route("/", get(root))
        .nest("/gallery",gallery::router(bucket.clone()))
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

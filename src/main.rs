use dotenvy::dotenv;
use std::env;
use axum::{
    routing::get,
    Router,
};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::{info};

#[tokio::main]
async fn main() {
    
    // 加载 .env 文件
    dotenv().ok();
    
    tracing_subscriber::fmt::init();

    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:8000".to_string());

    let app = Router::new()
        .route("/", get(root));

    info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

async fn root()  -> impl IntoResponse {
    (StatusCode::OK,"Hello, world!")
}

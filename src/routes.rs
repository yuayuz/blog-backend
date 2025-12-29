use crate::AppState;
use crate::handlers::{blog_handler, gallery_handler, image_handler};
use axum::Router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .nest("/gallery", gallery_handler::router())
        .nest("/image", image_handler::router())
        .nest("/blog", blog_handler::router())
}

async fn root() -> impl IntoResponse {
    (StatusCode::OK, "Hello,Rust!")
}

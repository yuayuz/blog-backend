use crate::AppState;
use axum::routing::get;
use axum::Router;
use crate::service::image_service::{get_image, image_root};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(image_root))
        .route("/{*path}", get(get_image))
}

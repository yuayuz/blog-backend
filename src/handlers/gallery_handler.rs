use crate::AppState;
use crate::service::gallery_service::{get_galleries, get_gallery_images, list_galleries};
use axum::{Router, routing::get};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_galleries))
        .route("/{name}", get(get_galleries))
        .route("/{name}/images", get(get_gallery_images))
}

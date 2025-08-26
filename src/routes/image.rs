use crate::AppState;
use axum::extract::{Path, State};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use bytes::Bytes;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .route("/{*path}", get(get_image))
}

async fn root() -> impl IntoResponse {
    { StatusCode::OK }.into_response()
}

async fn get_image(Path(path): Path<String>, State(state): State<AppState>) -> impl IntoResponse {
    let bucket = &state.bucket;
    match bucket.get_object(&path).await {
        Ok(obj) => {
            let body: Bytes = obj.bytes().clone();
            let content_type = "image/webp";

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, content_type)
                .body(body.into())
                .unwrap()
        }
        Err(_) => (StatusCode::NOT_FOUND, "图片不存在").into_response(),
    }
}

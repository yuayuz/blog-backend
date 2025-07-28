use axum::extract::Path;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Extension, Router};
use bytes::Bytes;
use s3::Bucket;
use std::sync::Arc;

pub fn router(bucket: Arc<Bucket>) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/{*path}", get(get_image))
        .layer(Extension(bucket))
}

async fn root() -> impl IntoResponse {
    { StatusCode::OK }.into_response()
}

async fn get_image(
    Path(path): Path<String>,
    Extension(bucket): Extension<Arc<Bucket>>,
) -> impl IntoResponse {
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

use crate::models::gallery::{GalleryEntity, GalleryResponse, PaginationParams};
use crate::repository::gallery_repository::get_all_galleries;
use axum::Json;
use axum::response::IntoResponse;
use base64::Engine;
use base64::engine::general_purpose;
use s3::Bucket;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::error;

pub async fn list_galleries(pool: PgPool, bucket: Arc<Bucket>) -> impl IntoResponse {
    let rows: Vec<GalleryEntity> = match get_all_galleries(pool).await {
        Ok(rows) => rows,
        Err(e) => {
            error!("DB query failed: {}", e);
            return Json(Vec::new());
        }
    };

    let mut galleries: Vec<GalleryResponse> = Vec::new();

    for entity in rows {
        match bucket.get_object(&entity.cover_url).await {
            Ok(obj) => {
                let cover_bytes = obj.bytes();
                let cover_image_base64 = format!(
                    "data:image/webp;base64,{}",
                    general_purpose::STANDARD.encode(cover_bytes)
                );

                galleries.push(GalleryResponse {
                    name: entity
                        .cover_url
                        .rsplit_once('/')
                        .map(|(left, _)| left)
                        .unwrap_or(&entity.cover_url)
                        .to_string(),
                    title: entity.title,
                    description: entity.description,
                    cover_image: cover_image_base64,
                });
            }
            Err(e) => error!("Error fetching object {}: {}", entity.cover_url, e),
        }
    }

    Json(galleries)
}

pub async fn get_gallery_images(
    name: String,
    params: PaginationParams,
    bucket: Arc<Bucket>,
) -> impl IntoResponse {
    let prefix = format!("gallery/{}/", name);
    let page_size = params.page_size.unwrap_or(9);
    let page = params.page.unwrap_or(1);

    let all_results = bucket
        .list(prefix, Some("/".to_string()))
        .await
        .unwrap_or_default();

    let mut all_files = vec![];
    for result in all_results {
        all_files.extend(result.contents.into_iter());
    }

    let start = (page - 1) * page_size;
    let paged_files: Vec<_> = all_files
        .iter()
        .skip(start as usize)
        .take(page_size as usize)
        .map(|obj| obj.key.clone())
        .collect();

    Json(paged_files)
}

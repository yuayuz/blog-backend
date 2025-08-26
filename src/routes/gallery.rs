use crate::AppState;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, Router, routing::get};
use base64::Engine;
use base64::engine::general_purpose;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tracing::error;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_galleries))
        .route("/{name}", get(get_galleries))
        .route("/{name}/images", get(get_gallery_images))
}

#[derive(Serialize, Debug)]
struct GalleryItem {
    name: String,
    title: String,
    description: String,
    cover_image: String,
}

#[derive(Debug, serde::Deserialize)]
struct MetaData {
    title: Option<String>,
    description: Option<String>,
}

fn parse_meta(content: &str) -> Result<MetaData, Box<dyn Error>> {
    let re = Regex::new(r"(?s)---\s*(.*?)\s*---")?;
    if let Some(caps) = re.captures(content) {
        let yaml_str = &caps[1];
        let meta: MetaData = serde_yaml::from_str(yaml_str)?;
        Ok(meta)
    } else {
        Err("未找到 YAML front matter".into())
    }
}

pub async fn list_galleries(State(state): State<AppState>) -> impl IntoResponse {
    let bucket = &state.bucket;
    let list_result = bucket
        .list("gallery/".to_string(), Some("/".to_string()))
        .await;

    match list_result {
        Ok(results) => {
            let mut galleries = vec![];

            for result in results {
                for prefix in result.common_prefixes.unwrap_or_default() {
                    let dir = prefix.prefix;
                    let files_result = bucket.list(dir.clone(), None).await;

                    if let Ok(file_pages) = files_result {
                        let mut title = String::new();
                        let mut description = String::new();
                        let mut cover_image = String::new();

                        for page in file_pages {
                            for obj in page.contents {
                                let key = obj.key.to_lowercase();

                                if (key.ends_with("meta.md")) && description.is_empty() {
                                    if let Ok(meta_response) = bucket.get_object(&obj.key).await {
                                        let meta_bytes = meta_response.bytes();
                                        let content =
                                            String::from_utf8_lossy(&meta_bytes).to_string();

                                        match parse_meta(&content) {
                                            Ok(meta) => {
                                                title = meta.title.unwrap_or("".to_string());
                                                description =
                                                    meta.description.unwrap_or("".to_string());
                                            }
                                            Err(err) => {
                                                error!("Failed to parse meta: {}", err);
                                            }
                                        }
                                    }
                                }

                                if key.ends_with("cover.webp") && cover_image.is_empty() {
                                    if let Ok(cover_response) = bucket.get_object(&obj.key).await {
                                        let cover_bytes = cover_response.bytes(); // 读取字节
                                        let base64_str =
                                            general_purpose::STANDARD.encode(&cover_bytes);
                                        let cover_image_base64 =
                                            format!("data:image/webp;base64,{}", base64_str);

                                        cover_image = cover_image_base64
                                    }
                                }

                                if !description.is_empty() && !cover_image.is_empty() {
                                    break;
                                }
                            }
                        }

                        if !cover_image.is_empty() {
                            galleries.push(GalleryItem {
                                name: dir.trim_end_matches('/').to_string(),
                                cover_image,
                                title,
                                description,
                            });
                        }
                    }
                }
            }

            Json(galleries).into_response()
        }
        Err(e) => {
            error!("Error listing galleries: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn get_galleries(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let bucket = &state.bucket;
    let prefix = format!("gallery/{}/", name);
    let list_result = bucket.list(prefix, Some("/".to_string())).await;
    match list_result {
        Ok(results) => {
            let mut files = Vec::new();
            for result in results {
                for obj in result.contents {
                    if !obj.key.ends_with("meta.md") {
                        files.push(obj.key);
                    }
                }
            }

            (StatusCode::OK, Json(files)).into_response()
        }
        Err(e) => {
            error!("Error listing galleries: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

async fn get_gallery_images(
    Path(name): Path<String>,
    Query(params): Query<PaginationParams>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let bucket = &state.bucket;
    let prefix = format!("gallery/{}/", name);
    let page_size = params.page_size.unwrap_or(9);
    let page = params.page.unwrap_or(1);

    let all_results = bucket
        .list(prefix, Some("/".to_string()))
        .await
        .unwrap_or_default();

    // 合并所有文件
    let mut all_files = vec![];
    for result in all_results {
        all_files.extend(
            result
                .contents
                .into_iter()
                .filter(|obj| !obj.key.ends_with("meta.md")),
        );
    }

    // 分页
    let start = (page - 1) * page_size;
    let paged_files: Vec<_> = all_files
        .iter()
        .skip(start as usize)
        .take(page_size as usize)
        .map(|obj| obj.key.clone())
        .collect();

    Json(paged_files).into_response()
}

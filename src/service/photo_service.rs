//! 照片墙服务：从所有图库中随机选取照片并短时缓存。
//!
//! 缓存时长 3 分钟，减轻 DB + OSS 压力。
//! 空查询结果返回 `{ "photos": [] }`，前端自动降级为占位图。

use crate::models::photo::{PhotoQueryParams, PhotoResponse, PhotosResponse};
use crate::repository::gallery_repository::get_all_galleries;
use axum::Json;
use axum::response::IntoResponse;
use rand::seq::SliceRandom;
use rand::thread_rng;
use s3::Bucket;
use sqlx::PgPool;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tracing::{error, warn};

/// 缓存条目：记录随机结果和生成时间。
struct CacheEntry {
    photos: Vec<PhotoResponse>,
    created_at: Instant,
}

/// 带 TTL 的内存缓存。
pub struct PhotoCache {
    entry: Option<CacheEntry>,
    ttl_secs: u64,
}

impl PhotoCache {
    pub fn new(ttl_secs: u64) -> Self {
        Self {
            entry: None,
            ttl_secs,
        }
    }

    /// 获取缓存，如果过期返回 None。
    pub fn get(&self) -> Option<&[PhotoResponse]> {
        self.entry.as_ref().and_then(|e| {
            if e.created_at.elapsed().as_secs() < self.ttl_secs {
                Some(e.photos.as_slice())
            } else {
                None
            }
        })
    }

    /// 写入缓存。
    pub fn set(&mut self, photos: Vec<PhotoResponse>) {
        self.entry = Some(CacheEntry {
            photos,
            created_at: Instant::now(),
        });
    }
}

/// 从 cover_url 提取 OSS 目录名。
///
/// 例如 `gallery/sunset/cover.webp` → `gallery/sunset`
fn extract_gallery_dir(cover_url: &str) -> &str {
    cover_url
        .rsplit_once('/')
        .map(|(dir, _file)| dir)
        .unwrap_or(cover_url)
}

/// 获取照片墙数据。
///
/// 流程：
/// 1. 查所有图库
/// 2. 对每个图库 OSS list 获取所有图片 key
/// 3. 打乱后取 count 条
///
/// 缓存策略：
/// - 首次查询时随机获取并缓存 3 分钟
/// - 缓存期内忽略 count 参数变化（参数跟随缓存）
/// - 缓存过期后下一次查询重新随机
pub async fn get_photos(
    pool: PgPool,
    bucket: Arc<Bucket>,
    params: PhotoQueryParams,
    cache: Arc<Mutex<PhotoCache>>,
) -> impl IntoResponse {
    let count = params.count.unwrap_or(20).clamp(15, 30) as usize;

    // 尝试读缓存
    {
        let cache_guard = cache.lock().unwrap();
        if let Some(cached) = cache_guard.get() {
            return Json(PhotosResponse {
                photos: cached.to_vec(),
            });
        }
    }

    // 1. 获取所有图库
    let galleries = match get_all_galleries(pool).await {
        Ok(g) => g,
        Err(e) => {
            error!("Failed to query galleries: {}", e);
            return Json(PhotosResponse { photos: vec![] });
        }
    };

    if galleries.is_empty() {
        return Json(PhotosResponse { photos: vec![] });
    }

    // 2. 从每个图库 OSS list 收集所有图片 key
    let mut all_keys: Vec<String> = Vec::new();

    for gallery in &galleries {
        let dir = extract_gallery_dir(&gallery.cover_url);
        let prefix = format!("{}/", dir);

        let results = match bucket.list(prefix, Some("/".to_string())).await {
            Ok(r) => r,
            Err(e) => {
                warn!("Failed to list OSS prefix {}: {}", dir, e);
                continue;
            }
        };

        for result in results {
            for obj in result.contents {
                all_keys.push(obj.key);
            }
        }
    }

    if all_keys.is_empty() {
        return Json(PhotosResponse { photos: vec![] });
    }

    // 3. 打乱后取 count 条
    let mut rng = thread_rng();
    all_keys.shuffle(&mut rng);

    let photos: Vec<PhotoResponse> = all_keys
        .into_iter()
        .take(count)
        .enumerate()
        .map(|(i, path)| PhotoResponse {
            id: i as i32 + 1, // 从 1 开始的自增 id
            path,
            title: None,
            width: None,
            height: None,
        })
        .collect();

    // 写入缓存
    {
        let mut cache_guard = cache.lock().unwrap();
        if cache_guard.get().is_none() {
            cache_guard.set(photos.clone());
        }
    }

    Json(PhotosResponse { photos })
}
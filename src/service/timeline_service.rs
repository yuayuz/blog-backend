//! 时间线服务。

use crate::models::timeline::Timeline;
use crate::repository::timeline_repository::get_all_timeline;
use axum::Json;
use axum::response::IntoResponse;
use sqlx::PgPool;
use tracing::error;

/// 获取所有时间线条目，按日期排序。
pub async fn get_all_timeline_service(pool: PgPool) -> impl IntoResponse {
    let rows: Vec<Timeline> = get_all_timeline(pool).await.unwrap_or_else(|e| {
        error!("数据库查询失败: {}", e);
        Vec::new()
    });
    Json(rows)
}

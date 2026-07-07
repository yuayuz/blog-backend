//! 阿里云 OSS 对象存储客户端初始化。
//!
//! 使用 `rust-s3` 库通过自定义 Endpoint 连接阿里云 OSS，
//! 用于存储 Markdown 文章原文和图片等静态资源。

use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use std::env;
use std::sync::Arc;

/// 初始化阿里云 OSS Bucket 实例。
///
/// 从环境变量读取凭证和配置，通过自定义 Endpoint 连接阿里云 OSS。
/// 返回 `Arc<Bucket>` 以便在多个 handler 间安全共享。
///
/// # 需要的环境变量
///
/// | 变量名 | 说明 | 示例 |
/// |--------|------|------|
/// | `ALIYUN_OSS_ACCESS_KEY` | AccessKey | `LTAI5t...` |
/// | `ALIYUN_OSS_SECRET_KEY` | SecretKey | `7O1dt...` |
/// | `AWS_REGION` | OSS 区域，作为 Region::Custom 的 region 参数 | `oss-cn-beijing` |
/// | `AWS_BUCKET` | Bucket 名称 | `blog-media-yu` |
/// | `AWS_ENDPOINT` | OSS Endpoint 完整 URL | `https://oss-cn-beijing.aliyuncs.com` |
pub async fn init_bucket() -> Result<Arc<Bucket>, Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok(); // 加载 .env 文件
    let access_key = env::var("ALIYUN_OSS_ACCESS_KEY")?;
    let secret_key = env::var("ALIYUN_OSS_SECRET_KEY")?;
    let region_str = env::var("AWS_REGION")?;
    let bucket_name = env::var("AWS_BUCKET")?;
    let endpoint = env::var("AWS_ENDPOINT")?;

    // 阿里云 OSS 使用自定义 Region（区别于 AWS 标准 region）
    let region = Region::Custom {
        region: region_str,
        endpoint,
    };

    let credentials = Credentials::new(Some(&access_key), Some(&secret_key), None, None, None)?;

    let bucket = Bucket::new(&bucket_name, region.clone(), credentials.clone())?;

    Ok(Arc::new(*bucket))
}

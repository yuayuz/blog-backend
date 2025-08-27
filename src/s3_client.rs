use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use std::env;
use std::sync::Arc;

pub async fn init_bucket() -> Result<Arc<Bucket>, Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok(); // 加载 .env 文件
    let access_key = env::var("ALIYUN_OSS_ACCESS_KEY")?;
    let secret_key = env::var("ALIYUN_OSS_SECRET_KEY")?;
    let region_str = env::var("AWS_REGION")?;
    let bucket_name = env::var("AWS_BUCKET")?;
    let endpoint = env::var("AWS_ENDPOINT")?;

    // 阿里云 OSS 的 Endpoint（注意：这里 Region 是手动拼的）
    let region = Region::Custom {
        region: region_str, // 自定义区域名
        endpoint,
    };

    let credentials = Credentials::new(Some(&access_key), Some(&secret_key), None, None, None)?;

    let bucket = Bucket::new(&bucket_name, region.clone(), credentials.clone())?;

    Ok(Arc::new(*bucket))
}

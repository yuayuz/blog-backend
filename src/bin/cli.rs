use blog_backend::db::create_pool;
use blog_backend::models::md_parser::parse_md;
use blog_backend::repository::blog_repository::insert_post;
use blog_backend::s3_client::init_bucket;
use clap::Parser;
use regex::Regex;
use std::path::Path;

/// Blog CLI - 博客文章发布工具
#[derive(Parser)]
#[command(name = "blog", about = "博客 CLI 工具", version)]
struct Cli {
    /// Markdown 文件路径
    file_path: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();

    let path = Path::new(&cli.file_path);
    if !path.exists() {
        eprintln!("❌ 文件不存在: {}", cli.file_path);
        return Ok(());
    }

    // 1. 读取 Markdown 文件
    let raw_md = std::fs::read_to_string(&cli.file_path)?;
    println!("📄 读取文件: {}", cli.file_path);

    // 2. 解析 Front Matter
    let (front_matter, md_content) = parse_md(&raw_md)?;
    let category = front_matter.category;
    let tags = front_matter.tags;
    let description = front_matter.description;
    let date = front_matter
        .date
        .unwrap_or_else(|| chrono::Utc::now().format("%Y-%m-%d").to_string());

    // 3. 连接 OSS 和数据库
    let bucket = init_bucket().await?;
    let pool = create_pool().await;

    // 4. 上传 Markdown 到 OSS (路径: article/{filename}.md)
    //    文件名同时作为 title、file_url、slug，保证三者一致
    let file_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("untitled");
    let title = file_name.to_string();
    let oss_key = format!("article/{}.md", file_name);
    let file_url = format!("/{}", file_name);

    println!("📝 标题: {}", title);
    println!("📅 日期: {}", date);

    let response = bucket
        .put_object_with_content_type(&oss_key, md_content.as_bytes(), "text/markdown")
        .await?;

    if response.status_code() != 200 {
        eprintln!("❌ OSS 上传失败，状态码: {}", response.status_code());
        return Ok(());
    }
    println!("☁️  已上传到 OSS: {}", oss_key);

    // 5. 生成 slug
    let slug = generate_slug(&title);

    // 6. 写入数据库（file_url 不带 article/ 前缀）
    let inserted = insert_post(
        &pool,
        &title,
        &slug,
        &file_url,
        category.as_deref(),
        description.as_deref(),
        tags.as_ref(),
    )
    .await?;

    println!("✅ 发布成功! id={}, slug={}", inserted.id, inserted.slug);
    Ok(())
}

/// 根据标题生成 URL 友好的 slug
fn generate_slug(title: &str) -> String {
    let slug = title
        .to_lowercase()
        .chars()
        .map(|c| match c {
            'a'..='z' | '0'..='9' => c,
            ' ' | '-' | '_' => '-',
            _ => '-',
        })
        .collect::<String>();

    let slug = Regex::new("-+")
        .unwrap()
        .replace_all(&slug, "-")
        .to_string();

    slug.trim_matches('-').to_string()
}

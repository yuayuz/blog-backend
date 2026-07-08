//! # Blog Backend
//!
//! 一个基于 axum 的博客后端服务，提供文章管理、图库、图片代理和时间线等 API。
//!
//! ## 模块概览
//!
//! - `db` — PostgreSQL 连接池管理
//! - `s3_client` — 阿里云 OSS 对象存储客户端
//! - `models` — 数据模型与 Markdown 解析
//! - `repository` — 数据库查询层
//! - `service` — 业务逻辑层
//! - `handlers` — HTTP 请求处理器
//! - `routes` — 路由聚合
//!
//! ## API 路由前缀
//!
//! 所有接口统一挂载在 `/rust` 路径下：
//!
//! | 路径 | 说明 |
//! |------|------|
//! | `GET  /rust/` | 服务根路径，返回 "Hello,Rust!" |
//! | `GET  /rust/blog/allTypes` | 获取所有文章分类 |
//! | `GET  /rust/blog/primaryTypes` | 获取一级分类 |
//! | `GET  /rust/blog/childTypes/{parent}` | 获取某分类的子分类 |
//! | `GET  /rust/blog/allPosts` | 获取全部文章列表 |
//! | `GET  /rust/blog/posts/type/{type_key}` | 按分类获取文章 |
//! | `GET  /rust/blog/posts/tag/{tag}` | 按标签获取文章 |
//! | `GET  /rust/blog/article?article_name=xxx` | 获取文章详情（Markdown） |
//! | `POST /rust/blog/upload` | 上传文章（multipart/form-data） |
//! | `GET  /rust/gallery/` | 列出所有图库 |
//! | `GET  /rust/gallery/{name}/images` | 获取图库中的图片 |
//! | `GET  /rust/image/{*path}` | 图片代理/转发 |
//! | `GET  /rust/timeline/` | 获取时间线 |
//! | `GET  /rust/photo-wall/?count=20` | 照片墙（随机选取） |
//!
//! ## 环境变量
//!
//! 通过 `.env` 文件配置：
//!
//! ```text
//! SERVER_ADDR=127.0.0.1:8000          # 监听地址
//! DATABASE_URL=postgres://...          # PostgreSQL 连接串
//! ALIYUN_OSS_ACCESS_KEY=...            # 阿里云 OSS AccessKey
//! ALIYUN_OSS_SECRET_KEY=...            # 阿里云 OSS SecretKey
//! AWS_REGION=oss-cn-beijing            # OSS 区域
//! AWS_BUCKET=blog-media-yu             # Bucket 名称
//! AWS_ENDPOINT=https://...aliyuncs.com # OSS Endpoint
//! ```
//!
//! ## 启动
//!
//! ```sh
//! cargo run
//! ```

pub mod db;
pub mod models;
pub mod repository;
pub mod s3_client;

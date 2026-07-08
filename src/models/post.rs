use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;

/// 文章分类类型（与 `blog_post_types` 表对应）。
///
/// 分类可以有层级关系：`parent_type` 为 `None` 的是顶级分类，
/// 否则是某个分类的子分类。`type_key` 作为唯一标识。
#[derive(Debug, Serialize, FromRow)]
pub struct BlogPostType {
    /// 分类唯一键，例如 `"rust"`、`"web"`、`"database"`
    pub type_key: String,
    /// 分类显示名，例如 `"Rust 笔记"`
    pub name: String,
    /// 父分类的 type_key，为空表示一级分类
    pub parent_type: Option<String>,
}

/// 博客文章（与 `blog_posts` 表对应）。
#[derive(Debug, Serialize, FromRow)]
pub struct BlogPost {
    pub id: i64,
    /// 文章标题，同时用作文件名标识
    pub title: String,
    /// 文章分类 type_key，关联 `blog_post_types.type`
    pub r#type: Option<String>,
    /// 文章摘要，可选
    pub description: Option<String>,
    /// OSS 上的文件路径
    pub file_url: String,
    /// 封面图 OSS 路径，可选
    pub cover_image_url: Option<String>,
    /// 文章状态，通常为 `"published"`
    pub status: String,
    /// 创建时间
    pub created_at: chrono::NaiveDateTime,
    /// 最后修改时间
    pub updated_at: chrono::NaiveDateTime,
    /// 发布时间，用于排序和展示
    pub published_at: Option<chrono::NaiveDateTime>,
    /// URL 友好的唯一标识，由标题自动生成
    pub slug: String,
    /// SEO 标题
    pub meta_title: Option<String>,
    /// SEO 描述
    pub meta_description: Option<String>,
    /// SEO 关键字（逗号分隔）
    pub keywords: Option<String>,
    /// 标签列表
    pub tags: Option<Vec<String>>,
}

/// 获取文章内容的查询参数。
///
/// 示例: `GET /blog/article?article_name=my-post`
#[derive(Deserialize, Debug)]
pub struct ArticleContentParams {
    /// OSS 上的文件名（不含扩展名和目录前缀），如 `"my-post"`
    pub article_name: String,
}

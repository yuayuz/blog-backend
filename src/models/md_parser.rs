use serde::{Deserialize, Serialize};

/// Markdown 文件的 YAML 前置元数据（Front Matter）。
///
/// 写在 `.md` 文件最上方，用 `---` 包裹的 YAML 块，
/// 用于声明文章的标题、分类、标签等信息。
///
/// 示例：
/// ```yaml
/// ---
/// title: "Rust 异步编程入门"
/// description: "一篇关于 async/await 的教程"
/// date: "2025-01-15"
/// category: "rust"
/// tags: ["rust", "async", "tokio"]
/// ---
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub description: Option<String>,
    pub date: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// 解析 Markdown 文本，分离 YAML 前置元数据和正文内容。
///
/// 处理逻辑：
/// 1. 如果文本不以 `---` 开头，视为无 front matter，用默认值填充元数据。
/// 2. 否则按 `---` 分割，第一部分解析为 YAML，第二部分作为 Markdown 正文。
///
/// 返回 `(FrontMatter, Markdown正文)`。
pub fn parse_md(raw: &str) -> Result<(FrontMatter, String), Box<dyn std::error::Error>> {
    if !raw.starts_with("---") {
        // 如果没有 front matter，返回默认值
        return Ok((
            FrontMatter {
                title: "未命名文章".to_string(),
                description: None,
                date: Some(chrono::Utc::now().format("%Y-%m-%d").to_string()),
                category: None,
                tags: None,
            },
            raw.to_string(),
        ));
    }

    let mut parts = raw.splitn(3, "---");
    parts.next(); // 跳过第一个空部分（`---` 之前的内容为空）

    let yaml = parts.next().unwrap_or("");
    let content = parts.next().unwrap_or("").trim().to_string();

    let meta: FrontMatter = serde_yaml::from_str(yaml)?;
    Ok((meta, content))
}

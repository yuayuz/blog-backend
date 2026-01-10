use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub description: Option<String>,
    pub date: String,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
}

pub(crate) fn parse_md(raw: &str) -> Result<(FrontMatter, String), Box<dyn std::error::Error>> {
    if !raw.starts_with("---") {
        // 如果没有 front matter，返回默认值
        return Ok((
            FrontMatter {
                title: "未命名文章".to_string(),
                description: None,
                date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
                category: None,
                tags: None,
            },
            raw.to_string(),
        ));
    }

    let mut parts = raw.splitn(3, "---");
    parts.next(); // 跳过第一个空部分

    let yaml = parts.next().unwrap_or("");
    let content = parts.next().unwrap_or("").trim().to_string();

    let meta: FrontMatter = serde_yaml::from_str(yaml)?;
    Ok((meta, content))
}

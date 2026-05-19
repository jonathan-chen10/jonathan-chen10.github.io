use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Represents data read from frontmatter block.
#[derive(Debug, Clone, Deserialize)]
pub struct Frontmatter {
    title: String,
    date_created: chrono::NaiveDate,
    date_modified: Option<chrono::NaiveDate>,
    tags: Vec<String>,
    #[serde(default)]
    draft: bool
}

#[derive(Debug, Clone, Serialize)]
pub struct TocEntry {
    level: u32,   // h1 to h6
    text: String,
    id: String,
}

#[derive(Debug, Clone)]
pub struct Post {
    frontmatter: Frontmatter,
    html: String,
    slug: String,
    toc: Vec<TocEntry>,
}

pub struct SiteIndex {
    posts: Vec<Post>,
    tags: HashMap<String, Vec<Post>>,
}

pub struct OutputFile {
    path_relative: PathBuf,
    html: String,
}

use std::collections::HashMap;
use std::path::PathBuf;

use chrono;
use serde::Deserialize;

/// Represents data read from frontmatter block.
#[derive(Debug, Clone, Deserialize)]
pub struct Frontmatter {
    pub title: String,
    pub date_created: chrono::NaiveDate,
    pub date_modified: Option<chrono::NaiveDate>,
    pub tags: Vec<String>,
    #[serde(default)]
    pub draft: bool
}

#[derive(Debug, Clone)]
pub struct Post {
    pub meta: Frontmatter,
    pub html: String,
    pub slug: String,
}

#[derive(Debug)]
pub struct SiteIndex {
    pub posts: Vec<Post>,
    pub tags: HashMap<String, Vec<Post>>,
}

pub struct OutputFile {
    pub path_relative: PathBuf,
    pub html: String,
}

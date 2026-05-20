use std::collections::HashMap;
use std::path::PathBuf;

use chrono;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize)]
pub struct TocEntry {
    pub level: u8,   // h1 to h6
    pub text: String,
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct TocNode {
    pub level: u8,
    pub data: Option<TocEntry>,
    pub children: Vec<TocNode>
}

#[derive(Debug, Clone)]
pub struct Post {
    pub meta: Frontmatter,
    pub html: String,
    pub slug: String,
    pub toc: Vec<TocEntry>,
}

pub struct SiteIndex {
    pub posts: Vec<Post>,
    pub tags: HashMap<String, Vec<Post>>,
}

pub struct OutputFile {
    pub path_relative: PathBuf,
    pub html: String,
}

use std::collections::HashMap;
use std::path::PathBuf;

use chrono;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
    pub templates_dir: PathBuf,
    #[serde(default)]
    pub tag_categories: Vec<TagCategory>,
}

/// Represents data read from frontmatter block.
#[derive(Debug, Clone, Deserialize)]
pub struct Frontmatter {
    pub title: String,
    pub date_created: chrono::NaiveDate,
    pub date_modified: Option<chrono::NaiveDate>,
    pub tags: Vec<String>,
    pub blurb: Option<String>,
    #[serde(default)]
    pub draft: bool
}

#[derive(Debug, Clone, Serialize)]
pub struct TocEntry {
    pub level: u8,
    pub text: String,
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct Post {
    pub meta: Frontmatter,
    pub html: String,
    pub slug: String,
    pub toc: Vec<TocEntry>,
}

#[derive(Debug)]
pub struct SiteIndex {
    pub posts: Vec<Post>,
    pub tags: HashMap<String, Vec<Post>>,
}

pub struct OutputFile {
    pub path_relative: PathBuf,
    pub path_assets: Option<PathBuf>,
    pub html: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Color(pub String);

impl Color {
    pub fn new(css_color: impl Into<String>) -> Self {
        Self(css_color.into())
    }
}

impl AsRef<str> for Color {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<Color> for String {
    fn from(color: Color) -> Self {
        color.0
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TagCategory {
    pub name: String,
    pub color: Color,
    pub tags: Vec<String>,
}

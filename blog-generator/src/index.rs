use std::collections::HashMap;

use crate::types::{Post, SiteIndex};

pub fn index(mut posts: Vec<Post>) -> SiteIndex {
    posts.sort_by_key(|p| p.meta.date_created);
    let mut tags: HashMap<String, Vec<Post>> = HashMap::new();

    for p in &posts {
        for tag in &p.meta.tags {
            tags.entry(tag.to_string()).or_default().push(p.clone());
        }
    }

    SiteIndex { posts, tags }
}

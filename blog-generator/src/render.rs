use std::{cmp::Ordering, collections::HashMap, path::{Path, PathBuf}};

use anyhow::Result;
use serde::Serialize;
use tera::{Context, Tera};

use crate::types::{OutputFile, Post, SiteIndex, TagCategory, TocEntry};

#[derive(Serialize)]
struct TeraPost {
    title: String,
    date_created: String, // formatted, e.g. "January 15, 2025"
    date_modified: Option<String>,
    tags: Vec<TeraTagBasic>,
    blurb: Option<String>,
    html: String,
    slug: String,
    toc: Vec<TocEntry>,   // empty vec if post has no headings
}

fn build_tera_post(post: &Post, tag_map: &HashMap<&str, &TagCategory>) -> TeraPost {
    TeraPost {
        title: post.meta.title.clone(),
        date_created: post.meta.date_created.format("%B %-d, %Y").to_string(),
        date_modified: post.meta.date_modified.map(|d| d.format("%B %-d, %Y").to_string()),
        tags: post.meta.tags.clone().into_iter().map(|tag| {
            TeraTagBasic {
                name: tag.clone(),
                color: tag_map.get(tag.as_str()).and_then(|t| Some(t.color.clone().into()))
            }
        }).collect(),
        blurb: post.meta.blurb.clone(),
        html: post.html.clone(),
        slug: post.slug.clone(),
        toc: post.toc.clone(),
    }
}

// Enough info for structs
#[derive(Clone, Serialize)]
struct TeraTagBasic {
    name: String,
    color: Option<String>
}

#[derive(Clone, Serialize)]
struct TeraTag {
    name: String,
    count: usize,
    color: Option<String>,
    category: Option<String>
}

#[derive(Serialize)]
struct PostContext {
    post: TeraPost,
}

fn build_post_context(post: &Post, tag_map: &HashMap<&str, &TagCategory>) -> PostContext {
    PostContext {
        post: build_tera_post(post, tag_map)
    }
}

fn render_post(post: &Post, tera: &Tera, tag_map: &HashMap<&str, &TagCategory>, src_dir: &Path) -> Result<OutputFile> {
    let ctx = build_post_context(post, tag_map);
    Ok(OutputFile {
        path_relative: PathBuf::from("posts").join(&post.slug).join("index.html"),
        path_assets: Some(src_dir.join(&post.slug)),
        html: tera.render("post.html", &Context::from_serialize(ctx)?)?,
    })
}

#[derive(Serialize)]
struct PostIndexContext {
    posts: Vec<TeraPost>,
}

fn build_post_index_context(posts: &[Post], tag_map: &HashMap<&str, &TagCategory>) -> PostIndexContext {
    PostIndexContext {
        posts: posts.into_iter().map(|p| build_tera_post(p, tag_map)).collect()
    }
}

fn render_post_index(posts: &[Post], tera: &Tera, tag_map: &HashMap<&str, &TagCategory>) -> Result<OutputFile> {
    let ctx = build_post_index_context(posts, tag_map);
    Ok(OutputFile {
        path_relative: "index.html".into(),
        path_assets: None,
        html: tera.render("post_index.html", &Context::from_serialize(ctx)?)?,
    })
}

#[derive(Serialize)]
struct TagPostsContext {
    tag: TeraTag,
    posts: Vec<TeraPost>, 
    // posts containing this tag
}

fn render_posts_redirect() -> OutputFile {
    OutputFile {
        path_relative: PathBuf::from("posts").join("index.html"),
        path_assets: None,
        html: r#"<!DOCTYPE html>
<html><head><meta http-equiv="refresh" content="0; url=../" /></head>
<body><a href="../">Redirecting…</a></body></html>"#.to_string(),
    }
}

fn render_tag_posts(
    tag: &str, filtered_posts: &[Post], tera: &Tera, tag_map: &HashMap<&str, &TagCategory>
) -> Result<OutputFile> {
    let ctx = TagPostsContext {
        tag: TeraTag {
            name: tag.to_string(),
            count: filtered_posts.len(),
            color: tag_map.get(tag)
                .map(|cat| Some(cat.color.clone().into()))
                .unwrap_or_else(|| None),
            category: tag_map.get(tag)
                .map(|cat| cat.name.clone()),
        },
        posts: filtered_posts.into_iter().map(|p| build_tera_post(p, tag_map)).collect(),
    };
    Ok(OutputFile {
        path_relative: PathBuf::from("tags").join(tag).join("index.html"),
        path_assets: None,
        html: tera.render("tag.html", &Context::from_serialize(ctx)?)?,
    })
}

#[derive(Serialize)]
struct TagIndexContext {
    tags: Vec<TeraTag>
}

// First order: category, in the order 
// Second order: number of articles with this tag, descending
// Third order: alphabetical
fn make_tag_ordering(categories: &[TagCategory]) -> impl Fn(&TeraTag, &TeraTag) -> Ordering {
    let mut category_idx_map: HashMap<String, usize> = HashMap::new();
    for (i, cat) in categories.iter().enumerate() {
        category_idx_map.insert(cat.name.clone(), i);
    }
    move |a: &TeraTag, b: &TeraTag| {
        let a_cat_score = a.category.as_deref()
            .and_then(|c| category_idx_map.get(c)).copied().unwrap_or(usize::MAX);

        let b_cat_score = b.category.as_deref()
            .and_then(|c| category_idx_map.get(c)).copied().unwrap_or(usize::MAX);

        if a_cat_score > b_cat_score {
            return Ordering::Greater;
        }
        if a_cat_score < b_cat_score {
            return Ordering::Less;
        }
        
        if a.count > b.count {
            return Ordering::Less;
        }
        if a.count < b.count {
            return Ordering::Greater;
        }

        a.name.cmp(&b.name)
    }
}

fn render_tag_index(
    tags: &HashMap<std::string::String, Vec<Post>>, tera: &Tera, categories: &[TagCategory]
) -> Result<OutputFile> {
    let tag_map = make_tag_map(categories)?;
    let mut ctx = TagIndexContext {
        tags: tags.into_iter().map(
            |(t, tposts)| TeraTag { 
                name: t.to_string(), 
                count: tposts.len(),
                color: tag_map.get(t.as_str())
                    .map(|cat| Some(cat.color.clone().into()))
                    .unwrap_or_else(|| None),
                category: tag_map.get(t.as_str())
                    .map(|cat| cat.name.clone()),
            }
        ).collect()
    };
    ctx.tags.sort_by(make_tag_ordering(categories));
    Ok(OutputFile {
        path_relative: PathBuf::from("tags").join("index.html"),
        path_assets: None,
        html: tera.render("tag_index.html", &Context::from_serialize(ctx)?)?,
    })
}

fn make_tag_map(categories: &[TagCategory]) -> Result<HashMap<&str, &TagCategory>> {
    let mut tag_map: HashMap<&str, &TagCategory> = HashMap::new();
    for cat in categories {
        for tag in &cat.tags {
            if let Some(prev) = tag_map.insert(tag.as_str(), cat) {
                anyhow::bail!(
                    "tag {:?} appears in both {:?} and {:?}",
                    tag, prev.name, cat.name
                );
            }
        }
    }
    Ok(tag_map)
}

pub fn render(
    site: &SiteIndex, tera: &Tera, categories: &[TagCategory], input_dir: &Path
) -> Result<Vec<OutputFile>> {
    let tag_map: HashMap<&str, &TagCategory> = make_tag_map(categories)?;

    let mut out: Vec<OutputFile> = Vec::new();

    let main_index = render_post_index(&site.posts, tera, &tag_map)?;
    out.push(main_index);
    out.push(render_posts_redirect());
    let tag_index = render_tag_index(&site.tags, tera, &categories)?;
    out.push(tag_index);

    for p in &site.posts {
        out.push(render_post(p, tera, &tag_map, input_dir)?);
    }

    for (tag, filtered_posts) in &site.tags {
        out.push(render_tag_posts(tag, filtered_posts, tera, &tag_map)?);
    }  

    Ok(out)
}

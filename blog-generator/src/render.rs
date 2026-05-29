use std::{collections::HashMap, path::{Path, PathBuf}};

use anyhow::Result;
use serde::Serialize;
use tera::{Context, Tera};

use crate::types::{OutputFile, Post, SiteIndex, TocEntry};

#[derive(Serialize)]
struct TeraPost {
    title: String,
    date_created: String, // formatted, e.g. "January 15, 2025"
    date_modified: Option<String>,
    tags: Vec<String>,
    blurb: Option<String>,
    html: String,
    slug: String,
    toc: Vec<TocEntry>,   // empty vec if post has no headings
}

impl From<&Post> for TeraPost {
    fn from(post: &Post) -> Self {
        Self {
            title: post.meta.title.clone(),
            date_created: post.meta.date_created.format("%B %-d, %Y").to_string(),
            date_modified: post.meta.date_modified.map(|d| d.format("%B %-d, %Y").to_string()),
            tags: post.meta.tags.clone(),
            blurb: post.meta.blurb.clone(),
            html: post.html.clone(),
            slug: post.slug.clone(),
            toc: post.toc.clone(),
        }
    }
}

#[derive(Clone, Serialize)]
struct TeraTag {
    name: String,
    count: usize,
}

#[derive(Serialize)]
struct PostContext {
    post: TeraPost,
}

impl From<&Post> for PostContext {
    fn from(p: &Post) -> Self {
        Self {
            post: TeraPost::from(p)
        }
    }
}

fn render_post(post: &Post, tera: &Tera, src_dir: &Path) -> Result<OutputFile> {
    let ctx = PostContext::from(post);
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

impl From<&[Post]> for PostIndexContext {
    fn from(posts: &[Post]) -> Self {
        Self {
            posts: posts.into_iter().map(TeraPost::from).collect()
        }
    }
}

fn render_post_index(posts: &[Post], tera: &Tera) -> Result<OutputFile> {
    let ctx = PostIndexContext::from(posts);
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

fn render_tag_posts(tag: &str, filtered_posts: &[Post], tera: &Tera) -> Result<OutputFile> {
    let ctx = TagPostsContext {
        tag: TeraTag {
            name: tag.to_string(),
            count: filtered_posts.len(),
        },
        posts: filtered_posts.into_iter().map(TeraPost::from).collect(),
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

fn render_tag_index(tags: &HashMap<std::string::String, Vec<Post>>, tera: &Tera) -> Result<OutputFile> {
    let mut ctx = TagIndexContext {
        tags: tags.into_iter().map(
            |(t, tposts)| TeraTag { 
                name: t.to_string(), 
                count: tposts.len(),
            }
        ).collect()
    };
    ctx.tags.sort_by(|a, b| b.count.cmp(&a.count));
    Ok(OutputFile {
        path_relative: PathBuf::from("tags").join("index.html"),
        path_assets: None,
        html: tera.render("tag_index.html", &Context::from_serialize(ctx)?)?,
    })
}

pub fn render(site: &SiteIndex, tera: &Tera, input_dir: &Path) -> Result<Vec<OutputFile>> {
    let mut out: Vec<OutputFile> = Vec::new();

    let main_index = render_post_index(&site.posts, tera)?;
    out.push(main_index);
    out.push(render_posts_redirect());
    let tag_index = render_tag_index(&site.tags, tera)?;
    out.push(tag_index);

    for p in &site.posts {
        out.push(render_post(p, tera, input_dir)?);
    }

    for (tag, filtered_posts) in &site.tags {
        out.push(render_tag_posts(tag, filtered_posts, tera)?);
    }  

    Ok(out)
}

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::{Context, Result};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use pulldown_cmark::html::push_html;

use crate::types::{Frontmatter, Post, TocEntry, TocNode};

pub fn parse(paths: Vec<PathBuf>) -> Result<Vec<Post>> {
    paths.iter().filter_map(|p| parse_one(p).transpose())
    .collect()
}

fn parse_one(path: &PathBuf) -> Result<Option<Post>> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("reading {}", path.display()))?;
    let (frontmatter_str, body) = split_frontmatter(&raw);

    match frontmatter_str {
        None => Ok(None),
        Some(s) => {
            let frontmatter: Frontmatter = serde_yaml::from_str(s)
                .with_context(|| format!("parsing frontmatter in {}", path.display()))?;
            if frontmatter.draft {
                return Ok(None);
            }

            let parser = Parser::new_ext(body, Options::all());
            let events: Vec<Event> = parser.map(process_special_cases).collect();

            let mut html = String::new();
            html.push_str(&render_toc(&build_toc(&events)));
            push_html(&mut html, events.clone().into_iter());
            html.push_str(&render_footnotes(&build_footnotes(&events)));

            let slug = get_slug(path)?;

            Ok(Some(Post {
                meta: frontmatter,
                html,
                slug,
                toc: vec![],
            }))
        }
    }
}

fn split_frontmatter(raw: &str) -> (Option<&str>, &str) {
    let Some(rest) = raw.strip_prefix("---\n") else {
        return (None, raw);
    };
    let Some(end) = rest.find("\n---\n") else {
        return (None, raw);
    };
    (Some(&rest[..end]), &rest[end + 5..])
}

fn process_special_cases(event: Event<'_>) -> Event<'_> {
    match event {
        Event::InlineMath(math) => {
            Event::Html(format!("<span class=\"math-inline\">{math}</span>").into())
        }
        Event::DisplayMath(math) => {
            Event::Html(format!("<span class=\"math-display\">{math}</span>").into())
        }
        Event::FootnoteReference(n) => {
            Event::Html(format!("<sup><a id=\"fnref-{n}\" href=\"#fn-{n}\">{n}</a></sup>").into())
        }
        event => event 
    }
}

fn get_slug(path: &PathBuf) -> Result<String> {
    let Some(parent) = path.parent() else {
        return Err(anyhow!("Could not parse path"));
    };
    parent.file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .with_context(|| format!("invalid filename: {}", path.display()))
}

fn build_toc(events: &[Event<'_>]) -> Vec<TocEntry> {
    let mut out: Vec<TocEntry> = vec![];
    let mut this_header: TocEntry = TocEntry {
        level: 0, 
        text: String::from(""), 
        id: String::from("")
    };
    let mut in_header = false;

    for e in events {
        match e {
            Event::Start(tag) => {
                match tag {
                    Tag::Heading { level, .. } => {
                        in_header = true;
                        this_header = TocEntry {
                            level: *level as u8, 
                            text: String::from(""), 
                            id: String::from("")
                        };
                    },
                    _ => {}
                }
            }
            Event::End(tag) => {
                match tag {
                    TagEnd::Heading { .. } => {
                        in_header = false;
                        this_header.id = slugify(&this_header.text);
                        out.push(this_header);
                        this_header = TocEntry { level: 0, text: String::new(), id: String::new() };
                    },
                    _ => {}
                }
            }
            Event::Text(text) => {
                if in_header {
                    this_header.text = this_header.text + text;
                }
            }
            Event::Code(text) => {
                if in_header {
                    this_header.text = this_header.text + "<pre>" + text + "</pre>";
                }
            }
            _ => {}
        }
    }
    out
}

fn render_toc(toc: &[TocEntry]) -> String {
    toc_tree_to_html(&toc_tree(toc))
}

fn toc_tree(toc: &[TocEntry]) -> Vec<TocNode> {
    let mut roots: Vec<TocNode> = Vec::new();
    let mut stack: Vec<TocNode> = Vec::new();

    let normalized = normalize_levels(&toc);

    for entry in normalized {
        let level = entry.level;
        let node = TocNode { level: entry.level, data: Some(entry), children: vec![] };

        // Pop stack until we find a node shallower than current
        while stack.last().map_or(false, |top| top.level >= level) {
            let top = stack.pop().unwrap();
            // Attach to new parent, or to roots if stack is now empty
            if let Some(parent) = stack.last_mut() {
                parent.children.push(top);
            } else {
                roots.push(top);
            }
        }

        stack.push(node);
    }

    // Drain remaining stack
    while let Some(top) = stack.pop() {
        if let Some(parent) = stack.last_mut() {
            parent.children.push(top);
        } else {
            roots.push(top);
        }
    }

    roots
}

fn normalize_levels(toc: &[TocEntry]) -> Vec<TocEntry> {
    let levels: Vec<u8> = toc.into_iter().map(|e| e.level)
        .collect::<BTreeSet<_>>().into_iter().collect();
    toc.into_iter().map(|e| TocEntry {
        level: levels.iter().position(|&l| l == e.level).unwrap() as u8,
        ..e.clone()
    }).collect()
}

fn toc_tree_to_html(tree: &[TocNode]) -> String {
    let items: String = tree.iter().map(|node| tree_to_li(node)).collect();
    format!("<div id=\"toc\"><h2>Table of Contents</h2><ol>{items}</ol></div>")
}

fn tree_to_li(tree: &TocNode) -> String {
   let (data, items) = (
        tree.data.as_ref(),
        tree.children.iter().map(tree_to_li).collect::<String>(),
    );

    match data {
        None => format!("<li><ol>{items}</ol></li>"),
        Some(data) => {
            if tree.children.is_empty() {
                format!("<li><a href=\"#{}\">{}</a></li>", data.id, data.text)
            } else {
                format!("<li><a href=\"#{}\">{}</a><ol>{items}</ol></li>", data.id, data.text)
            }
        }
    }
}

fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

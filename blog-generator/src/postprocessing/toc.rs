use std::collections::BTreeSet;

use anyhow::Result;
use pulldown_cmark::{Event, Tag, TagEnd};

use crate::utils::slugify;

#[derive(Debug, Clone)]
struct TocEntry {
    level: u8,
    text: String,
    id: String,
}

#[derive(Debug, Clone)]
struct TocNode {
    level: u8,
    data: Option<TocEntry>,
    children: Vec<TocNode>,
}

pub fn apply<'a>(events: &[Event<'a>]) -> Result<Vec<Event<'a>>> {
    let toc_event = Event::Html(toc_tree_to_html(&toc_tree(&build_toc(events))).into());
    Ok(std::iter::once(toc_event).chain(events.iter().cloned()).collect())
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

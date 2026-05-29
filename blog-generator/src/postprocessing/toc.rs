use std::collections::BTreeSet;

use anyhow::Result;
use pulldown_cmark::{Event, Tag, TagEnd};

use crate::types::TocEntry;
use crate::utils::slugify;

pub fn apply<'a>(events: &[Event<'a>]) -> Result<(Vec<Event<'a>>, Vec<TocEntry>)> {
    let toc = normalize_levels(&build_toc(events));

    let mut ids = toc.iter().map(|e| e.id.clone());

    let events_with_ids = events
        .iter()
        .map(|event| match event {
            Event::Start(Tag::Heading { level, classes, attrs, .. }) => {
                Event::Start(Tag::Heading {
                    level: *level,
                    id: ids.next().map(Into::into),
                    classes: classes.clone(),
                    attrs: attrs.clone(),
                })
            }
            other => other.clone(),
        })
        .collect();

    Ok((events_with_ids, toc))
}

fn build_toc(events: &[Event<'_>]) -> Vec<TocEntry> {
    let mut out: Vec<TocEntry> = vec![];
    let mut current = TocEntry { level: 0, text: String::new(), id: String::new() };
    let mut plain_text = String::new();
    let mut in_header = false;

    for e in events {
        match e {
            Event::Start(Tag::Heading { level, .. }) => {
                in_header = true;
                current = TocEntry { level: *level as u8, text: String::new(), id: String::new() };
                plain_text.clear();
            }
            Event::End(TagEnd::Heading { .. }) => {
                in_header = false;
                current.id = slugify(&plain_text);
                out.push(current);
                current = TocEntry { level: 0, text: String::new(), id: String::new() };
            }
            Event::Text(text) if in_header => {
                current.text.push_str(text);
                plain_text.push_str(text);
            }
            Event::Code(text) if in_header => {
                current.text.push_str(&format!("<code>{text}</code>"));
                plain_text.push_str(text);
            }
            _ => {}
        }
    }
    out
}

/// Remaps heading levels to 0-indexed ranks so the template can use
/// `entry.level` directly as a nesting depth (0 = top level).
fn normalize_levels(toc: &[TocEntry]) -> Vec<TocEntry> {
    let levels: Vec<u8> = toc
        .iter()
        .map(|e| e.level)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    toc.iter()
        .map(|e| TocEntry {
            level: levels.iter().position(|&l| l == e.level).unwrap() as u8, // guaranteed to be there by construction
            ..e.clone()
        })
        .collect()
}

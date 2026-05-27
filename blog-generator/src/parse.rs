use std::fs;
use std::path::{Path, PathBuf};

use anyhow::anyhow;
use anyhow::{Context, Result};
use pulldown_cmark::{Event, Options, Parser};
use pulldown_cmark::html::push_html;

use crate::postprocessing;
use crate::types::{Frontmatter, Post};

pub fn parse(paths: &[PathBuf]) -> Result<Vec<Post>> {
    paths.iter().filter_map(|p| parse_one(p).transpose())
    .collect()
}

fn parse_one(path: &Path) -> Result<Option<Post>> {
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
            let mut events: Vec<Event> = parser.map(process_special_cases).collect();

            let mut html = String::new();
            events = postprocessing::toc::apply(&events)?;
            events = postprocessing::footnote_links::apply(&events)?;
            push_html(&mut html, events.into_iter());

            let slug = get_slug(path)?;

            Ok(Some(Post {
                meta: frontmatter,
                html,
                slug,
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
        event => event 
    }
}

fn get_slug(path: &Path) -> Result<String> {
    let Some(parent) = path.parent() else {
        return Err(anyhow!("Could not parse path"));
    };
    parent.file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .with_context(|| format!("invalid filename: {}", path.display()))
}

use std::collections::HashMap;

use anyhow::Result;
use pulldown_cmark::{Event, Tag, TagEnd, html::push_html};

pub fn apply<'a>(events: &[Event<'a>]) -> Result<Vec<Event<'a>>> {
    let (references, definitions) = extract_footnote_data(events);
    
    let mut output: Vec<Event<'a>> = Vec::new();
    let mut in_definition = false;

    for event in events {
        match event {
            Event::FootnoteReference(label) => {
                let n = references[label.as_ref()];
                output.push(Event::Html(
                    format!("<sup><a id=\"fnref-{label}\" href=\"#fn-{label}\">{n}</a></sup>").into()
                ));
            }
            Event::Start(Tag::FootnoteDefinition(_)) => {
                in_definition = true;
            }
            Event::End(TagEnd::FootnoteDefinition) => {
                in_definition = false;
            }
            other => {
                if !in_definition {
                    output.push(other.clone());
                }
            }
        }
    }

    if !references.is_empty() {
        output.push(Event::Html(render_footnotes_html(&references, &definitions)?.into()));
    }
    Ok(output)
}

fn extract_footnote_data(events: &[Event<'_>]) -> (HashMap<String, u32>, HashMap<String, String>) {
    let mut references: HashMap<String, u32> = HashMap::new();
    let mut definitions: HashMap<String, String> = HashMap::new();
    let mut counter: u32 = 1;

    let mut current_label: Option<String> = None;
    let mut current_inner: Vec<Event> = Vec::new();

    for event in events {
        match event {
            Event::FootnoteReference(label) => {
                references.entry(label.to_string()).or_insert_with(|| {
                    let n = counter;
                    counter += 1;
                    n
                });
            }
            Event::Start(Tag::FootnoteDefinition(label)) => {
                current_label = Some(label.to_string());
            }
            Event::End(TagEnd::FootnoteDefinition) => {
                if let Some(label) = current_label.take() {
                    let mut body = String::new();
                    push_html(&mut body, current_inner.drain(..));
                    definitions.insert(label, body);
                }
            }
            other => {
                if current_label.is_some() {
                    current_inner.push(other.clone());
                }
            }
        }
    }

    (references, definitions)
}

fn render_footnotes_html(
    references: &HashMap<String, u32>,
    definitions: &HashMap<String, String>,
) -> Result<String> {
    for label in references.keys() {
        if !definitions.contains_key(label) {
            return Err(anyhow::anyhow!(
                "footnote [^{}] referenced but never defined", label
            ));
        }
    }

    let mut ordered: Vec<(&String, &u32)> = references.iter().collect();
    ordered.sort_by_key(|(_, n)| *n);

    let mut html = String::from(
        "<section class=\"footnotes\">\n\
         <h2 id=\"footnote-label\">Footnotes</h2>\n\
         <ol>\n"
    );
    for (label, _) in ordered {
        let body = &definitions[label];
        html.push_str(&format!(
            "<li id=\"fn-{label}\">{body}\
             <a href=\"#fnref-{label}\" aria-label=\"Back to content\">↩</a>\
             </li>\n"
        ));
    }
    html.push_str("</ol>\n</section>");
    Ok(html)
}

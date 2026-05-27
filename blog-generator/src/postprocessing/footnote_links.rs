use anyhow::Result;
use pulldown_cmark::Event;

pub fn apply<'a>(events: &[Event<'a>]) -> Result<Vec<Event<'a>>> {
    Ok(events.into())
}

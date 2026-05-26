use anyhow::Result;
use pulldown_cmark::{Parser, Event, Tag, TagEnd};
use tracing::info;

pub struct ParsedDoc {
    pub title: String,
    pub body: String,
    pub path: String,
}

pub fn parse_markdown(path: &str) -> Result<ParsedDoc> {
    let contents = std::fs::read_to_string(path)?;
    let parser = Parser::new(&contents);

    let mut title = String::new();
    let mut body = String::new();
    let mut in_heading = false;

    for event in parser {
        match event {
            Event::Start(Tag::Heading { .. }) => {
                in_heading = true;
            }
            Event::End(TagEnd::Heading(_)) => {
                in_heading = false;
            }
            Event::Text(text) => {
                if in_heading && title.is_empty() {
                    title = text.to_string();
                } else {
                    body.push_str(&text);
                    body.push(' ');
                }
            }
            _ => {}
        }
    }

    if title.is_empty() {
        title = std::path::Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("untitled")
            .to_string();
    }

    info!("parsed: {}", path);

    Ok(ParsedDoc {
        title,
        body: body.trim().to_string(),
        path: path.to_string(),
    })
}
    

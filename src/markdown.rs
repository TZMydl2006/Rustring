use crate::page::TocItem;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd, html};
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct RenderedMarkdown {
    pub html: String,
    pub title: Option<String>,
    pub toc: Vec<TocItem>,
}

#[derive(Clone, Debug)]
struct FlatHeading {
    level: u8,
    title: String,
    id: String,
}

pub fn render_markdown(markdown: &str) -> RenderedMarkdown {
    let headings = extract_headings(markdown);

    let mut html_output = String::new();
    html::push_html(&mut html_output, Parser::new_ext(markdown, options()));
    let html = inject_heading_ids(&html_output, &headings);

    let title = headings
        .iter()
        .find(|heading| heading.level == 1)
        .map(|heading| heading.title.clone());
    let toc = headings
        .into_iter()
        .map(|heading| TocItem {
            title: heading.title,
            href: format!("#{}", heading.id),
            level: heading.level,
        })
        .collect();

    RenderedMarkdown { html, title, toc }
}

fn extract_headings(markdown: &str) -> Vec<FlatHeading> {
    let mut headings = Vec::new();
    let mut slug_counts = BTreeMap::new();
    let mut current_level = None;
    let mut current_text = String::new();

    for event in Parser::new_ext(markdown, options()) {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                current_level = Some(heading_level(level));
                current_text.clear();
            }
            Event::Text(text) | Event::Code(text) if current_level.is_some() => {
                current_text.push_str(&text);
            }
            Event::SoftBreak | Event::HardBreak if current_level.is_some() => {
                current_text.push(' ');
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some(level) = current_level.take() {
                    let title = collapse_whitespace(&current_text);
                    let id = unique_slug(&title, &mut slug_counts);
                    headings.push(FlatHeading { level, title, id });
                    current_text.clear();
                }
            }
            _ => {}
        }
    }

    headings
}

fn inject_heading_ids(html: &str, headings: &[FlatHeading]) -> String {
    let mut output = String::with_capacity(html.len() + headings.len() * 16);
    let mut remaining = html;

    for heading in headings {
        let needle = format!("<h{}>", heading.level);
        if let Some(index) = remaining.find(&needle) {
            output.push_str(&remaining[..index]);
            output.push_str(&format!("<h{} id=\"{}\">", heading.level, heading.id));
            remaining = &remaining[index + needle.len()..];
        }
    }

    output.push_str(remaining);
    output
}

fn unique_slug(title: &str, counts: &mut BTreeMap<String, usize>) -> String {
    let base = slugify(title);
    let counter = counts.entry(base.clone()).or_insert(0);
    let slug = if *counter == 0 {
        base.clone()
    } else {
        format!("{base}-{}", *counter + 1)
    };
    *counter += 1;
    slug
}

fn slugify(title: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = false;

    for character in title.chars().flat_map(|character| character.to_lowercase()) {
        if character.is_ascii_alphanumeric() {
            slug.push(character);
            last_was_dash = false;
        } else if !last_was_dash {
            slug.push('-');
            last_was_dash = true;
        }
    }

    let slug = slug.trim_matches('-').to_string();
    if slug.is_empty() {
        String::from("section")
    } else {
        slug
    }
}

fn collapse_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn heading_level(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

fn options() -> Options {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options
}

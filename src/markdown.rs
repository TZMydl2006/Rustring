use crate::error::{MiniZensicalError, Result};
use crate::page::{PageMetadata, TocItem};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd, html};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct RenderedMarkdown {
    pub html: String,
    pub title: Option<String>,
    pub toc: Vec<TocItem>,
    pub plain_text: String,
    pub headings: Vec<Heading>,
    pub metadata: PageMetadata,
}

#[derive(Clone, Debug)]
pub struct Heading {
    pub level: u8,
    pub title: String,
    pub id: String,
}

pub fn render_markdown(markdown: &str, source_path: &Path) -> Result<RenderedMarkdown> {
    let (metadata, body) = split_front_matter(markdown, source_path)?;
    let headings = extract_headings(body);

    let mut html_output = String::new();
    html::push_html(&mut html_output, Parser::new_ext(body, options()));
    let html = inject_heading_ids(&html_output, &headings);
    let plain_text = extract_plain_text(body);

    let title = headings
        .iter()
        .find(|heading| heading.level == 1)
        .map(|heading| heading.title.clone());
    let toc = headings
        .iter()
        .map(|heading| TocItem {
            title: heading.title.clone(),
            href: format!("#{}", heading.id),
            level: heading.level,
        })
        .collect();

    Ok(RenderedMarkdown {
        html,
        title,
        toc,
        plain_text,
        headings,
        metadata,
    })
}

fn split_front_matter<'a>(
    markdown: &'a str,
    source_path: &Path,
) -> Result<(PageMetadata, &'a str)> {
    let mut lines = markdown.split_inclusive('\n');
    let Some(first_line) = lines.next() else {
        return Ok((PageMetadata::default(), markdown));
    };

    if trim_line(first_line) != "---" {
        return Ok((PageMetadata::default(), markdown));
    }

    let mut consumed = first_line.len();
    let mut front_matter = String::new();

    for line in lines {
        consumed += line.len();
        if trim_line(line) == "---" {
            if front_matter.trim().is_empty() {
                return Ok((PageMetadata::default(), &markdown[consumed..]));
            }

            let metadata: PageMetadata = serde_yaml::from_str(&front_matter).map_err(|error| {
                MiniZensicalError::FrontMatter {
                    path: source_path.to_path_buf(),
                    message: error.to_string(),
                }
            })?;
            return Ok((metadata.normalized(), &markdown[consumed..]));
        }
        front_matter.push_str(line);
    }

    Err(MiniZensicalError::FrontMatter {
        path: source_path.to_path_buf(),
        message: String::from("missing closing '---' line"),
    })
}

fn extract_headings(markdown: &str) -> Vec<Heading> {
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
                    if title.is_empty() {
                        current_text.clear();
                        continue;
                    }
                    let id = unique_slug(&title, &mut slug_counts);
                    headings.push(Heading { level, title, id });
                    current_text.clear();
                }
            }
            _ => {}
        }
    }

    headings
}

fn extract_plain_text(markdown: &str) -> String {
    let mut text = String::new();

    for event in Parser::new_ext(markdown, options()) {
        match event {
            Event::Text(fragment) | Event::Code(fragment) => push_fragment(&mut text, &fragment),
            Event::SoftBreak | Event::HardBreak => text.push(' '),
            Event::End(TagEnd::Paragraph)
            | Event::End(TagEnd::Heading(_))
            | Event::End(TagEnd::Item)
            | Event::End(TagEnd::CodeBlock)
            | Event::End(TagEnd::List(_))
            | Event::End(TagEnd::Table)
            | Event::End(TagEnd::TableRow)
            | Event::End(TagEnd::TableHead)
            | Event::End(TagEnd::TableCell) => text.push(' '),
            _ => {}
        }
    }

    collapse_whitespace(&text)
}

fn push_fragment(text: &mut String, fragment: &str) {
    if !text.is_empty() && !text.ends_with([' ', '\n', '\t']) {
        text.push(' ');
    }
    text.push_str(fragment);
}

fn inject_heading_ids(html: &str, headings: &[Heading]) -> String {
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

fn trim_line(line: &str) -> &str {
    line.trim_end_matches(['\r', '\n'])
}

#[cfg(test)]
mod tests {
    use super::render_markdown;
    use std::path::Path;

    #[test]
    fn parses_yaml_front_matter_and_body() {
        let rendered = render_markdown(
            r#"---
title: Custom Title
summary: Short summary
tags:
  - rust
  - docs
order: 2
---
# Ignored H1

Body text.
"#,
            Path::new("docs/example.md"),
        )
        .unwrap();

        assert_eq!(rendered.metadata.title.as_deref(), Some("Custom Title"));
        assert_eq!(rendered.metadata.summary.as_deref(), Some("Short summary"));
        assert_eq!(rendered.metadata.tags, vec!["rust", "docs"]);
        assert_eq!(rendered.metadata.order, Some(2));
        assert!(rendered.html.contains("Ignored H1"));
        assert_eq!(rendered.title.as_deref(), Some("Ignored H1"));
    }

    #[test]
    fn keeps_plain_markdown_compatible() {
        let rendered = render_markdown(
            "# Hello\n\n## Intro\n\nWelcome to docs.\n",
            Path::new("docs/index.md"),
        )
        .unwrap();
        assert_eq!(rendered.metadata.title, None);
        assert_eq!(rendered.title.as_deref(), Some("Hello"));
        assert_eq!(rendered.toc.len(), 2);
        assert!(rendered.plain_text.contains("Welcome to docs."));
    }

    #[test]
    fn reports_invalid_front_matter() {
        let error = render_markdown(
            "---\ntitle: [oops\n---\n# Hello\n",
            Path::new("docs/broken.md"),
        )
        .unwrap_err();

        assert!(error.to_string().contains("front matter"));
    }
}

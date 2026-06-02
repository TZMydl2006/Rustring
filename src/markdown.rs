use crate::error::{MiniZensicalError, Result};
use crate::page::{PageMetadata, TocItem};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd, html};
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};

#[derive(Clone, Debug)]
pub struct RenderedMarkdown {
    pub html: String,
    pub title: Option<String>,
    pub toc: Vec<TocItem>,
    pub plain_text: String,
    pub headings: Vec<Heading>,
    pub search_blocks: Vec<SearchBlock>,
    pub metadata: PageMetadata,
}

#[derive(Clone, Debug)]
pub struct Heading {
    pub level: u8,
    pub title: String,
    pub id: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct SearchBlock {
    pub id: String,
    pub kind: String,
    pub text: String,
}

pub fn render_markdown(
    markdown: &str,
    source_path: &Path,
    relative_source: &Path,
    output_path: &Path,
) -> Result<RenderedMarkdown> {
    let (metadata, body) = split_front_matter(markdown, source_path)?;
    let normalized_body = normalize_display_math_blocks(body);
    let body = normalized_body.as_str();
    let headings = extract_headings(body);
    let (body_search_blocks, search_targets) = extract_body_search_blocks(body);

    let mut html_output = String::new();
    let events = Parser::new_ext(body, options())
        .map(|event| relocate_image_event(event, relative_source, output_path));
    let events = normalize_math_events(events.collect());
    html::push_html(&mut html_output, events.into_iter());
    let html = inject_search_target_ids(
        &inject_heading_ids(&html_output, &headings),
        &search_targets,
    );
    let plain_text = extract_plain_text(body);
    let mut search_blocks = headings
        .iter()
        .map(|heading| SearchBlock {
            id: heading.id.clone(),
            kind: String::from("heading"),
            text: heading.title.clone(),
        })
        .collect::<Vec<_>>();
    search_blocks.extend(body_search_blocks);

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
        search_blocks,
        metadata,
    })
}

fn normalize_math_events<'a>(events: Vec<Event<'a>>) -> Vec<Event<'a>> {
    let mut events = events
        .into_iter()
        .map(|event| match event {
            Event::InlineMath(text) => Event::InlineMath(text.trim().to_string().into()),
            Event::DisplayMath(text) => Event::DisplayMath(text.trim().to_string().into()),
            _ => event,
        })
        .collect::<Vec<_>>();

    for index in 0..events.len() {
        if !matches!(events[index], Event::InlineMath(_)) {
            continue;
        }
        trim_space_before_inline_math(&mut events[..index]);
        trim_space_after_inline_math(&mut events[index + 1..]);
    }

    events
}

fn trim_space_before_inline_math(events: &mut [Event<'_>]) {
    let Some(Event::Text(text)) = events.last_mut() else {
        return;
    };
    let trimmed = text.trim_end();
    if trimmed.ends_with(is_cjk_or_opening_punctuation) {
        *text = trimmed.to_string().into();
    }
}

fn trim_space_after_inline_math(events: &mut [Event<'_>]) {
    let Some(Event::Text(text)) = events.first_mut() else {
        return;
    };
    let trimmed = text.trim_start();
    if trimmed.starts_with(is_cjk_or_closing_punctuation) {
        *text = trimmed.to_string().into();
    }
}

fn is_cjk_or_opening_punctuation(character: char) -> bool {
    is_cjk(character) || matches!(character, '(' | '[' | '{' | '<' | '（' | '【' | '《' | '“')
}

fn is_cjk_or_closing_punctuation(character: char) -> bool {
    is_cjk(character)
        || matches!(
            character,
            ')' | ']'
                | '}'
                | '>'
                | ','
                | '.'
                | ';'
                | ':'
                | '!'
                | '?'
                | '）'
                | '】'
                | '》'
                | '，'
                | '。'
                | '；'
                | '：'
                | '！'
                | '？'
                | '”'
        )
}

fn is_cjk(character: char) -> bool {
    matches!(
        character,
        '\u{2e80}'..='\u{2eff}'
            | '\u{3000}'..='\u{303f}'
            | '\u{3400}'..='\u{4dbf}'
            | '\u{4e00}'..='\u{9fff}'
            | '\u{f900}'..='\u{faff}'
    )
}

fn normalize_display_math_blocks(markdown: &str) -> String {
    let mut output = String::with_capacity(markdown.len());
    let mut pending_math_lines: Option<Vec<&str>> = None;
    let mut fence = None;

    for line in markdown.split_inclusive('\n') {
        let trimmed = trim_line(line).trim();

        if let Some(lines) = &mut pending_math_lines {
            if trimmed == "$$" {
                let formula = normalize_display_math_contents(lines);
                output.push_str("$$");
                output.push_str(&formula);
                output.push_str("$$\n");
                pending_math_lines = None;
            } else {
                lines.push(line);
            }
            continue;
        }

        if let Some((marker, length)) = fence {
            output.push_str(line);
            if is_closing_fence(trimmed, marker, length) {
                fence = None;
            }
            continue;
        }

        if let Some(opening_fence) = opening_fence(trimmed) {
            fence = Some(opening_fence);
            output.push_str(line);
        } else if trimmed == "$$" {
            pending_math_lines = Some(Vec::new());
        } else {
            output.push_str(line);
        }
    }

    if let Some(lines) = pending_math_lines {
        output.push_str("$$\n");
        for line in lines {
            output.push_str(line);
        }
    }

    output
}

fn normalize_display_math_contents(lines: &[&str]) -> String {
    let lines = lines
        .iter()
        .map(|line| trim_line(line).trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let formula = lines.join(" ");

    if lines.len() <= 1 || formula.contains(r"\begin{") || formula.contains(r"\\") {
        formula
    } else {
        format!(
            r"\begin{{gathered}} {} \end{{gathered}}",
            lines.join(r" \\ ")
        )
    }
}

fn opening_fence(line: &str) -> Option<(char, usize)> {
    let mut chars = line.chars();
    let marker = chars.next()?;
    if !matches!(marker, '`' | '~') {
        return None;
    }

    let length = 1 + chars.take_while(|character| *character == marker).count();
    (length >= 3).then_some((marker, length))
}

fn is_closing_fence(line: &str, marker: char, opening_length: usize) -> bool {
    let length = line
        .chars()
        .take_while(|character| *character == marker)
        .count();
    length >= opening_length && line[length..].trim().is_empty()
}

fn relocate_image_event<'a>(
    event: Event<'a>,
    relative_source: &Path,
    output_path: &Path,
) -> Event<'a> {
    match event {
        Event::Start(Tag::Image {
            link_type,
            dest_url,
            title,
            id,
        }) => {
            let dest_url = relocate_image_url(&dest_url, relative_source, output_path)
                .map(Into::into)
                .unwrap_or(dest_url);
            Event::Start(Tag::Image {
                link_type,
                dest_url,
                title,
                id,
            })
        }
        _ => event,
    }
}

fn relocate_image_url(
    destination: &str,
    relative_source: &Path,
    output_path: &Path,
) -> Option<String> {
    let (path, suffix) = split_url_suffix(destination);
    if !is_local_relative_url(path) {
        return None;
    }

    let source_dir = relative_source.parent().unwrap_or_else(|| Path::new(""));
    let asset_path = normalize_docs_relative_path(&source_dir.join(path))?;
    Some(format!(
        "{}{}",
        relative_url(output_path, &asset_path),
        suffix
    ))
}

fn split_url_suffix(destination: &str) -> (&str, &str) {
    let suffix_start = destination
        .char_indices()
        .find_map(|(index, character)| matches!(character, '?' | '#').then_some(index))
        .unwrap_or(destination.len());
    destination.split_at(suffix_start)
}

fn is_local_relative_url(url: &str) -> bool {
    if url.is_empty() || url.starts_with(['/', '\\']) {
        return false;
    }

    !url.char_indices().any(|(index, character)| {
        character == ':'
            && url[..index].chars().all(|character| {
                character.is_ascii_alphanumeric() || matches!(character, '+' | '-' | '.')
            })
    })
}

fn normalize_docs_relative_path(path: &Path) -> Option<PathBuf> {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => normalized.push(part),
            Component::CurDir => {}
            Component::ParentDir => {
                if !normalized.pop() {
                    return None;
                }
            }
            Component::RootDir | Component::Prefix(_) => return None,
        }
    }
    Some(normalized)
}

fn relative_url(from_file: &Path, to_file: &Path) -> String {
    let from_dir = from_file.parent().unwrap_or_else(|| Path::new(""));
    let from_parts = normal_components(from_dir);
    let to_parts = normal_components(to_file);
    let mut shared = 0;
    while shared < from_parts.len()
        && shared < to_parts.len()
        && from_parts[shared] == to_parts[shared]
    {
        shared += 1;
    }

    let mut parts = vec!["..".to_string(); from_parts.len().saturating_sub(shared)];
    parts.extend(to_parts.into_iter().skip(shared));
    if parts.is_empty() {
        String::from(".")
    } else {
        parts.join("/")
    }
}

fn normal_components(path: &Path) -> Vec<String> {
    path.components()
        .filter_map(|component| match component {
            Component::Normal(part) => Some(part.to_string_lossy().into_owned()),
            _ => None,
        })
        .collect()
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
            Event::Text(text)
            | Event::Code(text)
            | Event::InlineMath(text)
            | Event::DisplayMath(text)
                if current_level.is_some() =>
            {
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
            Event::Text(fragment)
            | Event::Code(fragment)
            | Event::InlineMath(fragment)
            | Event::DisplayMath(fragment) => push_fragment(&mut text, &fragment),
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

#[derive(Clone, Debug)]
struct SearchHtmlTarget {
    tag: &'static str,
    id: String,
}

#[derive(Clone, Debug)]
struct SearchBlockAccumulator {
    id: String,
    text: String,
}

impl SearchBlockAccumulator {
    fn new(id: String) -> Self {
        Self {
            id,
            text: String::new(),
        }
    }

    fn push_fragment(&mut self, fragment: &str) {
        push_fragment(&mut self.text, fragment);
    }

    fn push_space(&mut self) {
        if !self.text.ends_with(' ') {
            self.text.push(' ');
        }
    }

    fn into_search_block(self) -> Option<SearchBlock> {
        let text = collapse_whitespace(&self.text);
        if text.is_empty() {
            None
        } else {
            Some(SearchBlock {
                id: self.id,
                kind: String::from("body"),
                text,
            })
        }
    }
}

fn extract_body_search_blocks(markdown: &str) -> (Vec<SearchBlock>, Vec<SearchHtmlTarget>) {
    let mut blocks = Vec::new();
    let mut targets = Vec::new();
    let mut counter = 0usize;
    let mut current_paragraph: Option<SearchBlockAccumulator> = None;
    let mut current_code_block: Option<SearchBlockAccumulator> = None;
    let mut item_stack: Vec<SearchBlockAccumulator> = Vec::new();

    for event in Parser::new_ext(markdown, options()) {
        match event {
            Event::Start(Tag::Paragraph) if item_stack.is_empty() => {
                let id = next_search_block_id(&mut counter);
                targets.push(SearchHtmlTarget {
                    tag: "p",
                    id: id.clone(),
                });
                current_paragraph = Some(SearchBlockAccumulator::new(id));
            }
            Event::End(TagEnd::Paragraph) => {
                if let Some(paragraph) = current_paragraph.take() {
                    if let Some(block) = paragraph.into_search_block() {
                        blocks.push(block);
                    }
                }
            }
            Event::Start(Tag::Item) => {
                let id = next_search_block_id(&mut counter);
                targets.push(SearchHtmlTarget {
                    tag: "li",
                    id: id.clone(),
                });
                item_stack.push(SearchBlockAccumulator::new(id));
            }
            Event::End(TagEnd::Item) => {
                if let Some(item) = item_stack.pop() {
                    if let Some(block) = item.into_search_block() {
                        blocks.push(block);
                    }
                }
            }
            Event::Start(Tag::CodeBlock(_)) => {
                let id = next_search_block_id(&mut counter);
                targets.push(SearchHtmlTarget {
                    tag: "pre",
                    id: id.clone(),
                });
                current_code_block = Some(SearchBlockAccumulator::new(id));
            }
            Event::End(TagEnd::CodeBlock) => {
                if let Some(code_block) = current_code_block.take() {
                    if let Some(block) = code_block.into_search_block() {
                        blocks.push(block);
                    }
                }
            }
            Event::Text(fragment)
            | Event::Code(fragment)
            | Event::InlineMath(fragment)
            | Event::DisplayMath(fragment) => {
                push_search_fragment(
                    &mut current_paragraph,
                    &mut current_code_block,
                    &mut item_stack,
                    &fragment,
                );
            }
            Event::SoftBreak | Event::HardBreak => {
                push_search_space(
                    &mut current_paragraph,
                    &mut current_code_block,
                    &mut item_stack,
                );
            }
            _ => {}
        }
    }

    (blocks, targets)
}

fn next_search_block_id(counter: &mut usize) -> String {
    *counter += 1;
    format!("mz-search-block-{counter}")
}

fn push_search_fragment(
    current_paragraph: &mut Option<SearchBlockAccumulator>,
    current_code_block: &mut Option<SearchBlockAccumulator>,
    item_stack: &mut [SearchBlockAccumulator],
    fragment: &str,
) {
    if let Some(code_block) = current_code_block {
        code_block.push_fragment(fragment);
        return;
    }

    if let Some(paragraph) = current_paragraph {
        paragraph.push_fragment(fragment);
    }

    if let Some(item) = item_stack.last_mut() {
        item.push_fragment(fragment);
    }
}

fn push_search_space(
    current_paragraph: &mut Option<SearchBlockAccumulator>,
    current_code_block: &mut Option<SearchBlockAccumulator>,
    item_stack: &mut [SearchBlockAccumulator],
) {
    if let Some(code_block) = current_code_block {
        code_block.push_space();
        return;
    }

    if let Some(paragraph) = current_paragraph {
        paragraph.push_space();
    }

    if let Some(item) = item_stack.last_mut() {
        item.push_space();
    }
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

fn inject_search_target_ids(html: &str, targets: &[SearchHtmlTarget]) -> String {
    let mut output = String::with_capacity(html.len() + targets.len() * 24);
    let mut remaining = html;

    for target in targets {
        let needle = format!("<{}>", target.tag);
        if let Some(index) = remaining.find(&needle) {
            output.push_str(&remaining[..index]);
            output.push_str(&format!("<{} id=\"{}\">", target.tag, target.id));
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
    options.insert(Options::ENABLE_MATH);
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
            Path::new("example.md"),
            Path::new("example/index.html"),
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
            Path::new("index.md"),
            Path::new("index.html"),
        )
        .unwrap();
        assert_eq!(rendered.metadata.title, None);
        assert_eq!(rendered.title.as_deref(), Some("Hello"));
        assert_eq!(rendered.toc.len(), 2);
        assert!(rendered.plain_text.contains("Welcome to docs."));
        assert!(rendered.html.contains("id=\"mz-search-block-1\""));
        assert!(rendered.search_blocks.iter().any(|block| {
            block.kind == "heading" && block.id == "hello" && block.text == "Hello"
        }));
        assert!(rendered.search_blocks.iter().any(|block| {
            block.kind == "body"
                && block.id.starts_with("mz-search-block-")
                && block.text.contains("Welcome to docs.")
        }));
    }

    #[test]
    fn reports_invalid_front_matter() {
        let error = render_markdown(
            "---\ntitle: [oops\n---\n# Hello\n",
            Path::new("docs/broken.md"),
            Path::new("broken.md"),
            Path::new("broken/index.html"),
        )
        .unwrap_err();

        assert!(error.to_string().contains("front matter"));
    }

    #[test]
    fn renders_inline_and_display_math_without_touching_code() {
        let rendered = render_markdown(
            "# Math\n\nInline: $V_{GS} > V_T$.\n\n中文 $Y$ 等于输出。\n\nParentheses: ( $V_{GS}$ )\n\nEnglish $x$ value.\n\n$$\nY_0 = \\overline{AB}\n\nY_1 = A + B\n$$\n\n`$not_math$`\n\n```text\n$$\n$also_not_math$\n$$\n```\n",
            Path::new("docs/math.md"),
            Path::new("math.md"),
            Path::new("math/index.html"),
        )
        .unwrap();

        assert!(
            rendered
                .html
                .contains(r#"<span class="math math-inline">V_{GS} &gt; V_T</span>"#)
        );
        assert!(
            rendered
                .html
                .contains(r#"中文<span class="math math-inline">Y</span>等于输出。"#)
        );
        assert!(
            rendered
                .html
                .contains(r#"Parentheses: (<span class="math math-inline">V_{GS}</span>)"#)
        );
        assert!(
            rendered
                .html
                .contains(r#"English <span class="math math-inline">x</span> value."#)
        );
        assert!(
            rendered
                .html
                .contains(r#"<span class="math math-display">"#)
        );
        assert!(rendered.html.contains(
            r#"\begin{gathered} Y_0 = \overline{AB} \\ Y_1 = A + B \end{gathered}</span>"#
        ));
        assert!(rendered.html.contains("<code>$not_math$</code>"));
        assert!(rendered.html.contains("$also_not_math$"));
        assert!(rendered.plain_text.contains("V_{GS} > V_T"));
    }
}

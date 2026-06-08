use crate::config::Config;
use crate::error::{MiniZensicalError, Result};
use crate::markdown::{MarkdownLink, SearchBlock, render_markdown};
use crate::scanner::{SourceFile, is_index_markdown, normalize_path, titleize};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Serialize)]
pub struct TocItem {
    pub title: String,
    pub href: String,
    pub level: u8,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PageMetadata {
    pub title: Option<String>,
    pub summary: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub order: Option<i32>,
    pub date: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Page {
    pub source_path: PathBuf,
    pub relative_source: PathBuf,
    pub source_key: String,
    pub output_path: PathBuf,
    pub title: String,
    pub html: String,
    pub toc: Vec<TocItem>,
    pub metadata: PageMetadata,
    pub description: Option<String>,
    pub search_excerpt: String,
    pub plain_text: String,
    pub search_headings: Vec<String>,
    pub search_blocks: Vec<SearchBlock>,
    pub links: Vec<MarkdownLink>,
    pub canonical_url: Option<String>,
    pub is_home: bool,
}

pub const PAGE_TITLE_SEARCH_ID: &str = "page-title";

impl Page {
    pub fn from_source(config: &Config, source: &SourceFile) -> Result<Self> {
        let markdown = fs::read_to_string(&source.source_path)
            .map_err(|error| MiniZensicalError::io("read", &source.source_path, error))?;
        let output_path = output_path_for(&source.relative_path, config.project.use_directory_urls);
        let rendered = render_markdown(
            &markdown,
            &source.source_path,
            &source.relative_path,
            &output_path,
        )?;
        let title = rendered
            .metadata
            .title
            .clone()
            .or(rendered.title)
            .unwrap_or_else(|| fallback_title(&source.relative_path));
        let search_excerpt = rendered
            .metadata
            .summary
            .clone()
            .unwrap_or_else(|| excerpt(&rendered.plain_text, 180));
        let description = if search_excerpt.is_empty() {
            None
        } else {
            Some(search_excerpt.clone())
        };
        let canonical_url = config.project.site_url.as_ref().map(|site_url| {
            canonical_url(site_url, &output_path, config.project.use_directory_urls)
        });
        let mut search_blocks = vec![SearchBlock {
            id: PAGE_TITLE_SEARCH_ID.to_string(),
            kind: String::from("title"),
            text: title.clone(),
        }];
        search_blocks.extend(rendered.search_blocks.clone());

        Ok(Self {
            source_path: source.source_path.clone(),
            relative_source: source.relative_path.clone(),
            source_key: normalize_path(&source.relative_path),
            output_path,
            title,
            html: rendered.html,
            toc: rendered.toc,
            metadata: rendered.metadata,
            description,
            search_excerpt,
            plain_text: rendered.plain_text,
            search_headings: rendered
                .headings
                .into_iter()
                .filter(|heading| heading.level <= 2)
                .map(|heading| heading.title)
                .collect(),
            search_blocks,
            links: rendered.links,
            canonical_url,
            is_home: is_root_index_page(&source.relative_path),
        })
    }

    pub fn target_path(&self, config: &Config) -> PathBuf {
        config.site_path_for(&self.output_path)
    }

    pub fn public_url(&self, use_directory_urls: bool) -> String {
        public_url_for(&self.output_path, use_directory_urls)
    }
}

impl PageMetadata {
    pub fn normalized(mut self) -> Self {
        self.title = normalize_optional_text(self.title);
        self.summary = normalize_optional_text(self.summary);
        self.tags = self
            .tags
            .into_iter()
            .map(|tag| tag.trim().to_string())
            .filter(|tag| !tag.is_empty())
            .collect();
        self
    }
}

pub fn output_path_for(relative_source: &Path, use_directory_urls: bool) -> PathBuf {
    let mut path = relative_source.to_path_buf();
    let is_index = is_index_markdown(relative_source);

    if relative_source
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.eq_ignore_ascii_case("readme.md"))
    {
        path.pop();
        path.push("index.md");
    }

    if is_index || !use_directory_urls {
        path.set_extension("html");
    } else {
        path.set_extension("");
        path.push("index.html");
    }

    path
}

pub fn public_url_for(output_path: &Path, use_directory_urls: bool) -> String {
    let mut path = normalize_path(output_path);
    if use_directory_urls && path.ends_with("index.html") {
        path.truncate(path.len() - "index.html".len());
    }
    path
}

fn fallback_title(relative_source: &Path) -> String {
    if is_index_markdown(relative_source) {
        if let Some(parent) = relative_source.parent().and_then(|path| path.file_name()) {
            return titleize(&parent.to_string_lossy());
        }
    }

    let stem = relative_source
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("page");
    titleize(stem)
}

fn canonical_url(site_url: &str, output_path: &Path, use_directory_urls: bool) -> String {
    let base = site_url.trim_end_matches('/');
    let path = public_url_for(output_path, use_directory_urls);

    if path.is_empty() {
        format!("{base}/")
    } else {
        format!("{base}/{path}")
    }
}

fn excerpt(text: &str, max_chars: usize) -> String {
    let collapsed = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.chars().count() <= max_chars {
        return collapsed;
    }

    let mut excerpt = collapsed.chars().take(max_chars).collect::<String>();
    if let Some(index) = excerpt.rfind(' ') {
        excerpt.truncate(index);
    }
    excerpt.push_str("...");
    excerpt
}

fn is_root_index_page(relative_source: &Path) -> bool {
    is_index_markdown(relative_source)
        && relative_source
            .parent()
            .is_none_or(|parent| parent.as_os_str().is_empty())
}

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|text| {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

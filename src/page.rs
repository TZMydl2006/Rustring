use crate::config::Config;
use crate::error::{MiniZensicalError, Result};
use crate::markdown::render_markdown;
use crate::scanner::{SourceFile, is_index_markdown, normalize_path, titleize};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Serialize)]
pub struct TocItem {
    pub title: String,
    pub href: String,
    pub level: u8,
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
    pub canonical_url: Option<String>,
}

impl Page {
    pub fn from_source(config: &Config, source: &SourceFile) -> Result<Self> {
        let markdown = fs::read_to_string(&source.source_path)
            .map_err(|error| MiniZensicalError::io("read", &source.source_path, error))?;
        let rendered = render_markdown(&markdown);
        let output_path = output_path_for(&source.relative_path, config.project.use_directory_urls);
        let title = rendered
            .title
            .unwrap_or_else(|| fallback_title(&source.relative_path));
        let canonical_url = config.project.site_url.as_ref().map(|site_url| {
            canonical_url(site_url, &output_path, config.project.use_directory_urls)
        });

        Ok(Self {
            source_path: source.source_path.clone(),
            relative_source: source.relative_path.clone(),
            source_key: normalize_path(&source.relative_path),
            output_path,
            title,
            html: rendered.html,
            toc: rendered.toc,
            canonical_url,
        })
    }

    pub fn target_path(&self, config: &Config) -> PathBuf {
        config.site_path_for(&self.output_path)
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
    let mut path = normalize_path(output_path);
    if use_directory_urls && path.ends_with("index.html") {
        path.truncate(path.len() - "index.html".len());
    }

    if path.is_empty() {
        format!("{base}/")
    } else {
        format!("{base}/{path}")
    }
}

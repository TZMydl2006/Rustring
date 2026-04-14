use crate::config::Config;
use crate::error::{MiniZensicalError, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct SourceFile {
    pub source_path: PathBuf,
    pub relative_path: PathBuf,
}

#[derive(Clone, Debug, Default)]
pub struct SiteSources {
    pub markdown_files: Vec<SourceFile>,
    pub asset_files: Vec<SourceFile>,
}

pub fn scan_site(config: &Config) -> Result<SiteSources> {
    let docs_dir = config.docs_dir();
    if !docs_dir.exists() {
        return Err(MiniZensicalError::InvalidConfig(format!(
            "docs directory does not exist: {}",
            docs_dir.display()
        )));
    }

    let mut sources = SiteSources::default();
    for entry in WalkDir::new(&docs_dir).sort_by_file_name() {
        let entry = entry.map_err(|error| MiniZensicalError::walk(&docs_dir, error))?;
        if !entry.file_type().is_file() {
            continue;
        }

        let source_path = entry.into_path();
        let relative_path = source_path
            .strip_prefix(&docs_dir)
            .expect("scanned file should stay under docs_dir")
            .to_path_buf();
        let file = SourceFile {
            source_path,
            relative_path,
        };

        if is_markdown_path(&file.relative_path) {
            sources.markdown_files.push(file);
        } else {
            sources.asset_files.push(file);
        }
    }

    sources
        .markdown_files
        .sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    sources
        .asset_files
        .sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    Ok(sources)
}

pub fn is_markdown_path(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
}

pub fn is_index_markdown(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| {
            name.eq_ignore_ascii_case("index.md") || name.eq_ignore_ascii_case("readme.md")
        })
}

pub fn normalize_path(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/")
}

pub fn titleize(input: &str) -> String {
    input
        .split(['-', '_', ' '])
        .filter(|part| !part.is_empty())
        .map(capitalize)
        .collect::<Vec<_>>()
        .join(" ")
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => {
            let mut output = String::new();
            output.extend(first.to_uppercase());
            output.push_str(chars.as_str());
            output
        }
        None => String::new(),
    }
}

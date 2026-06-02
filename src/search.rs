use crate::markdown::SearchBlock;
use crate::page::Page;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize)]
pub struct SearchIndexEntry {
    pub title: String,
    pub url: String,
    pub blocks: Vec<SearchBlock>,
}

pub fn search_index_path() -> PathBuf {
    PathBuf::from("search.json")
}

pub fn build_search_index(pages: &[Page], use_directory_urls: bool) -> Vec<SearchIndexEntry> {
    pages
        .iter()
        .map(|page| SearchIndexEntry {
            title: page.title.clone(),
            url: page.public_url(use_directory_urls),
            blocks: page.search_blocks.clone(),
        })
        .collect()
}

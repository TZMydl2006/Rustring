use crate::page::Page;
use crate::scanner::normalize_path;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};

#[derive(Clone, Debug, Serialize)]
pub struct Graph {
    pub version: u8,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

#[derive(Clone, Debug, Serialize)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    #[serde(rename = "type")]
    pub edge_type: String,
    pub weight: f64,
}

pub fn graph_json_path() -> PathBuf {
    PathBuf::from("graph.json")
}

pub fn build_knowledge_graph(pages: &[Page], use_directory_urls: bool) -> Graph {
    let mut nodes = BTreeMap::<String, GraphNode>::new();
    let mut edges = BTreeMap::<(String, String, String), GraphEdge>::new();
    let page_lookup = pages
        .iter()
        .map(|page| (page.source_key.clone(), document_id(page)))
        .collect::<BTreeMap<_, _>>();

    for page in pages {
        let doc_id = document_id(page);
        nodes.insert(
            doc_id.clone(),
            GraphNode {
                id: doc_id.clone(),
                label: page.title.clone(),
                node_type: String::from("document"),
                url: page.public_url(use_directory_urls),
                summary: page.metadata.summary.clone(),
                tags: page.metadata.tags.clone(),
                date: page.metadata.date.clone(),
                source: Some(page.source_key.clone()),
            },
        );

        for tag in unique_tags(&page.metadata.tags) {
            let tag_id = tag_id(&tag);
            nodes.entry(tag_id.clone()).or_insert_with(|| {
                let slug = slugify(&tag);
                GraphNode {
                    id: tag_id.clone(),
                    label: tag.clone(),
                    node_type: String::from("tag"),
                    url: format!("knowledge-graph/?tag={slug}"),
                    summary: None,
                    tags: Vec::new(),
                    date: None,
                    source: None,
                }
            });
            insert_edge(&mut edges, &doc_id, &tag_id, "has_tag", 2.0);
        }

        for heading in &page.search_headings {
            let topic_id = topic_id(heading);
            nodes.entry(topic_id.clone()).or_insert_with(|| {
                let slug = slugify(heading);
                GraphNode {
                    id: topic_id.clone(),
                    label: heading.clone(),
                    node_type: String::from("topic"),
                    url: format!("knowledge-graph/?topic={slug}"),
                    summary: None,
                    tags: Vec::new(),
                    date: None,
                    source: None,
                }
            });
            insert_edge(&mut edges, &doc_id, &topic_id, "about_topic", 1.5);
        }

        for link in &page.links {
            let Some(target_key) = resolve_markdown_link(page, &link.destination) else {
                continue;
            };
            let Some(target_id) = page_lookup.get(&target_key) else {
                continue;
            };
            if target_id != &doc_id {
                insert_edge(&mut edges, &doc_id, target_id, "links_to", 3.0);
            }
        }
    }

    for (index, left) in pages.iter().enumerate() {
        for right in pages.iter().skip(index + 1) {
            let left_id = document_id(left);
            let right_id = document_id(right);
            let shared_tags = shared_tag_count(left, right);
            if shared_tags > 0 {
                insert_edge(
                    &mut edges,
                    &left_id,
                    &right_id,
                    "shared_tag",
                    shared_tags as f64 * 0.8,
                );
            }

            if same_section(left, right) {
                insert_edge(&mut edges, &left_id, &right_id, "same_section", 0.5);
            }
        }
    }

    Graph {
        version: 1,
        nodes: nodes.into_values().collect(),
        edges: edges.into_values().collect(),
    }
}

fn document_id(page: &Page) -> String {
    format!("doc:{}", page.source_key)
}

fn tag_id(tag: &str) -> String {
    format!("tag:{}", slugify(tag))
}

fn topic_id(topic: &str) -> String {
    format!("topic:{}", slugify(topic))
}

fn unique_tags(tags: &[String]) -> Vec<String> {
    let mut seen = BTreeSet::new();
    tags.iter()
        .map(|tag| tag.trim())
        .filter(|tag| !tag.is_empty())
        .filter_map(|tag| {
            let key = tag.to_lowercase();
            seen.insert(key).then(|| tag.to_string())
        })
        .collect()
}

fn insert_edge(
    edges: &mut BTreeMap<(String, String, String), GraphEdge>,
    source: &str,
    target: &str,
    edge_type: &str,
    weight: f64,
) {
    edges
        .entry((
            source.to_string(),
            target.to_string(),
            edge_type.to_string(),
        ))
        .or_insert_with(|| GraphEdge {
            source: source.to_string(),
            target: target.to_string(),
            edge_type: edge_type.to_string(),
            weight,
        });
}

fn shared_tag_count(left: &Page, right: &Page) -> usize {
    let left_tags = left
        .metadata
        .tags
        .iter()
        .map(|tag| tag.to_lowercase())
        .collect::<BTreeSet<_>>();
    right
        .metadata
        .tags
        .iter()
        .map(|tag| tag.to_lowercase())
        .filter(|tag| left_tags.contains(tag))
        .collect::<BTreeSet<_>>()
        .len()
}

fn same_section(left: &Page, right: &Page) -> bool {
    left.relative_source.parent() == right.relative_source.parent()
}

fn resolve_markdown_link(page: &Page, destination: &str) -> Option<String> {
    let (path, _) = split_url_suffix(destination);
    if !is_local_relative_url(path) || path.is_empty() {
        return None;
    }

    let source_dir = page
        .relative_source
        .parent()
        .unwrap_or_else(|| Path::new(""));
    let target = normalize_docs_relative_path(&source_dir.join(path))?;
    if !target
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
    {
        return None;
    }

    Some(normalize_path(&target))
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

fn slugify(value: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = false;

    for character in value.chars().flat_map(|character| character.to_lowercase()) {
        if character.is_ascii_alphanumeric() {
            slug.push(character);
            last_was_dash = false;
        } else if is_cjk(character) {
            slug.push(character);
            last_was_dash = false;
        } else if !last_was_dash {
            slug.push('-');
            last_was_dash = true;
        }
    }

    let slug = slug.trim_matches('-').to_string();
    if slug.is_empty() {
        String::from("topic")
    } else {
        slug
    }
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

use crate::config::NavItemConfig;
use crate::error::{MiniZensicalError, Result};
use crate::page::Page;
use crate::scanner::{is_index_markdown, normalize_path, titleize};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Navigation {
    pub items: Vec<NavItem>,
    page_order: Vec<OrderedPage>,
}

#[derive(Clone, Debug)]
pub struct NavItem {
    pub title: String,
    pub target: Option<NavTarget>,
    pub children: Vec<NavItem>,
}

#[derive(Clone, Debug)]
pub struct NavTarget {
    pub source_key: String,
    pub output_path: PathBuf,
}

#[derive(Clone, Debug)]
struct OrderedPage {
    title: String,
    source_key: String,
    output_path: PathBuf,
}

#[derive(Clone, Debug, Serialize)]
pub struct RenderNavItem {
    pub title: String,
    pub href: Option<String>,
    pub children: Vec<RenderNavItem>,
    pub active: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct PageLink {
    pub title: String,
    pub href: String,
}

impl Navigation {
    pub fn build(config_nav: &[NavItemConfig], pages: &mut [Page]) -> Result<Self> {
        if config_nav.is_empty() {
            Ok(Self::build_auto(pages))
        } else {
            Self::build_explicit(config_nav, pages)
        }
    }

    pub fn render_for_page(
        &self,
        page: &Page,
    ) -> (Vec<RenderNavItem>, Option<PageLink>, Option<PageLink>) {
        let items = render_items(&self.items, page);
        let Some(position) = self
            .page_order
            .iter()
            .position(|entry| entry.source_key == page.source_key)
        else {
            return (items, None, None);
        };

        let previous = position
            .checked_sub(1)
            .map(|index| page_link(&self.page_order[index], page));
        let next = self
            .page_order
            .get(position + 1)
            .map(|entry| page_link(entry, page));

        (items, previous, next)
    }

    fn build_auto(pages: &mut [Page]) -> Self {
        let mut indices = (0..pages.len()).collect::<Vec<_>>();
        indices.sort_by_key(|index| auto_sort_key(&pages[*index].relative_source));

        let mut items = Vec::new();
        for index in indices {
            let page = &pages[index];
            let mut components = path_components(&page.relative_source);
            let file = components.pop().unwrap_or_default();

            let mut section = &mut items;
            for component in components {
                let title = titleize(&component);
                let position = section
                    .iter()
                    .position(|item: &NavItem| item.target.is_none() && item.title == title);
                if let Some(position) = position {
                    section = &mut section[position].children;
                } else {
                    section.push(NavItem::section(title));
                    let last = section.len() - 1;
                    section = &mut section[last].children;
                }
            }

            let _ = file;
            section.push(NavItem::page(
                page.title.clone(),
                page.source_key.clone(),
                page.output_path.clone(),
            ));
        }

        let mut page_order = Vec::new();
        collect_order(&items, &mut page_order);
        Self { items, page_order }
    }

    fn build_explicit(config_nav: &[NavItemConfig], pages: &mut [Page]) -> Result<Self> {
        let lookup = pages
            .iter()
            .enumerate()
            .map(|(index, page)| (page.source_key.clone(), index))
            .collect::<HashMap<_, _>>();

        let mut seen = HashSet::new();
        let mut page_order = Vec::new();
        let mut items = Vec::new();
        for item in config_nav {
            items.push(build_explicit_item(
                item,
                pages,
                &lookup,
                &mut seen,
                &mut page_order,
            )?);
        }

        Ok(Self { items, page_order })
    }
}

impl NavItem {
    fn section(title: String) -> Self {
        Self {
            title,
            target: None,
            children: Vec::new(),
        }
    }

    fn page(title: String, source_key: String, output_path: PathBuf) -> Self {
        Self {
            title,
            target: Some(NavTarget {
                source_key,
                output_path,
            }),
            children: Vec::new(),
        }
    }
}

fn build_explicit_item(
    item: &NavItemConfig,
    pages: &mut [Page],
    lookup: &HashMap<String, usize>,
    seen: &mut HashSet<String>,
    page_order: &mut Vec<OrderedPage>,
) -> Result<NavItem> {
    if let Some(path) = &item.path {
        let key = normalize_path(Path::new(path));
        let Some(index) = lookup.get(&key).copied() else {
            return Err(MiniZensicalError::InvalidConfig(format!(
                "nav item '{}' references missing page '{}'",
                item.title, path
            )));
        };

        if !seen.insert(key.clone()) {
            return Err(MiniZensicalError::InvalidConfig(format!(
                "page '{}' is listed multiple times in nav",
                path
            )));
        }

        pages[index].title = item.title.clone();
        let page = &pages[index];
        page_order.push(OrderedPage {
            title: item.title.clone(),
            source_key: page.source_key.clone(),
            output_path: page.output_path.clone(),
        });

        Ok(NavItem::page(
            item.title.clone(),
            page.source_key.clone(),
            page.output_path.clone(),
        ))
    } else {
        let mut children = Vec::new();
        for child in &item.children {
            children.push(build_explicit_item(child, pages, lookup, seen, page_order)?);
        }

        Ok(NavItem {
            title: item.title.clone(),
            target: None,
            children,
        })
    }
}

fn collect_order(items: &[NavItem], order: &mut Vec<OrderedPage>) {
    for item in items {
        if let Some(target) = &item.target {
            order.push(OrderedPage {
                title: item.title.clone(),
                source_key: target.source_key.clone(),
                output_path: target.output_path.clone(),
            });
        }
        collect_order(&item.children, order);
    }
}

fn render_items(items: &[NavItem], current_page: &Page) -> Vec<RenderNavItem> {
    items
        .iter()
        .map(|item| render_item(item, current_page))
        .collect()
}

fn render_item(item: &NavItem, current_page: &Page) -> RenderNavItem {
    let children = render_items(&item.children, current_page);
    let self_active = item
        .target
        .as_ref()
        .is_some_and(|target| target.source_key == current_page.source_key);
    let child_active = children.iter().any(|child| child.active);
    let href = item
        .target
        .as_ref()
        .map(|target| relative_href(&current_page.output_path, &target.output_path));

    RenderNavItem {
        title: item.title.clone(),
        href,
        children,
        active: self_active || child_active,
    }
}

fn page_link(entry: &OrderedPage, current_page: &Page) -> PageLink {
    PageLink {
        title: entry.title.clone(),
        href: relative_href(&current_page.output_path, &entry.output_path),
    }
}

pub fn relative_href(from_file: &Path, to_file: &Path) -> String {
    let from_dir = from_file.parent().unwrap_or_else(|| Path::new(""));
    let from_parts = path_components(from_dir);
    let to_parts = path_components(to_file);

    let mut shared = 0;
    while shared < from_parts.len()
        && shared < to_parts.len()
        && from_parts[shared] == to_parts[shared]
    {
        shared += 1;
    }

    let mut parts = Vec::new();
    parts.extend(std::iter::repeat_n(
        String::from(".."),
        from_parts.len().saturating_sub(shared),
    ));
    parts.extend(to_parts.into_iter().skip(shared));

    if parts.is_empty() {
        String::from(".")
    } else {
        parts.join("/")
    }
}

fn auto_sort_key(path: &Path) -> (Vec<String>, bool, String) {
    let mut components = path_components(path);
    let file = components.pop().unwrap_or_default();
    (components, !is_index_markdown(path), file)
}

fn path_components(path: &Path) -> Vec<String> {
    path.components()
        .map(|component| component.as_os_str())
        .filter(|component| *component != OsStr::new("."))
        .map(|component| component.to_string_lossy().into_owned())
        .collect()
}

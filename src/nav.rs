use crate::config::NavItemConfig;
use crate::error::{MiniZensicalError, Result};
use crate::page::Page;
use crate::scanner::{is_index_markdown, normalize_path, titleize};
use serde::Serialize;
use std::cmp::Ordering;
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
        indices.sort_by(|left, right| pages[*left].source_key.cmp(&pages[*right].source_key));

        let mut root = AutoSection::root();
        for index in indices {
            root.insert_page(&pages[index]);
        }
        root.sort_recursive();

        let items = root.into_nav_items();
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

#[derive(Clone, Debug)]
struct AutoSection {
    title: String,
    path_key: String,
    section_order: Option<i32>,
    children: Vec<AutoEntry>,
}

#[derive(Clone, Debug)]
enum AutoEntry {
    Section(AutoSection),
    Page(AutoPage),
}

#[derive(Clone, Debug)]
struct AutoPage {
    title: String,
    source_key: String,
    path_key: String,
    output_path: PathBuf,
    order: Option<i32>,
    is_section_index: bool,
}

impl AutoSection {
    fn root() -> Self {
        Self {
            title: String::new(),
            path_key: String::new(),
            section_order: None,
            children: Vec::new(),
        }
    }

    fn new(title: String, path_key: String) -> Self {
        Self {
            title,
            path_key,
            section_order: None,
            children: Vec::new(),
        }
    }

    fn insert_page(&mut self, page: &Page) {
        let mut components = path_components(&page.relative_source);
        components.pop();
        let is_index_page = is_index_markdown(&page.relative_source);
        let is_section_index = !components.is_empty() && is_index_page;
        let path_key = if components.is_empty() && is_index_page {
            String::new()
        } else {
            page.source_key.clone()
        };

        let section = self.section_for_path(&components);
        if is_section_index {
            section.section_order = page.metadata.order;
        }

        section.children.push(AutoEntry::Page(AutoPage {
            title: page.title.clone(),
            source_key: page.source_key.clone(),
            path_key,
            output_path: page.output_path.clone(),
            order: page.metadata.order,
            is_section_index,
        }));
    }

    fn section_for_path(&mut self, components: &[String]) -> &mut AutoSection {
        let Some((component, remaining)) = components.split_first() else {
            return self;
        };

        let path_key = if self.path_key.is_empty() {
            component.clone()
        } else {
            format!("{}/{}", self.path_key, component)
        };

        let position = self
            .children
            .iter()
            .position(|entry| matches!(entry, AutoEntry::Section(section) if section.path_key == path_key))
            .unwrap_or_else(|| {
                self.children
                    .push(AutoEntry::Section(AutoSection::new(titleize(component), path_key)));
                self.children.len() - 1
            });

        let AutoEntry::Section(section) = &mut self.children[position] else {
            unreachable!("auto navigation section lookup only returns section entries");
        };
        section.section_for_path(remaining)
    }

    fn sort_recursive(&mut self) {
        for child in &mut self.children {
            if let AutoEntry::Section(section) = child {
                section.sort_recursive();
            }
        }
        self.children.sort_by(compare_auto_entries);
    }

    fn into_nav_items(self) -> Vec<NavItem> {
        let AutoSection { children, .. } = self;
        children.into_iter().map(AutoEntry::into_nav_item).collect()
    }

    fn into_nav_item(self) -> NavItem {
        let AutoSection {
            title, children, ..
        } = self;
        NavItem {
            title,
            target: None,
            children: children.into_iter().map(AutoEntry::into_nav_item).collect(),
        }
    }
}

impl AutoEntry {
    fn into_nav_item(self) -> NavItem {
        match self {
            AutoEntry::Section(section) => section.into_nav_item(),
            AutoEntry::Page(page) => NavItem::page(page.title, page.source_key, page.output_path),
        }
    }
}

fn compare_auto_entries(left: &AutoEntry, right: &AutoEntry) -> Ordering {
    auto_index_rank(left)
        .cmp(&auto_index_rank(right))
        .then_with(|| compare_auto_order(auto_order(left), auto_order(right)))
        .then_with(|| auto_path_key(left).cmp(auto_path_key(right)))
}

fn auto_index_rank(entry: &AutoEntry) -> u8 {
    match entry {
        AutoEntry::Page(page) if page.is_section_index => 0,
        _ => 1,
    }
}

fn auto_order(entry: &AutoEntry) -> Option<i32> {
    match entry {
        AutoEntry::Section(section) => section.section_order,
        AutoEntry::Page(page) => page.order,
    }
}

fn compare_auto_order(left: Option<i32>, right: Option<i32>) -> Ordering {
    match (left, right) {
        (Some(left), Some(right)) => left.cmp(&right),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

fn auto_path_key(entry: &AutoEntry) -> &str {
    match entry {
        AutoEntry::Section(section) => &section.path_key,
        AutoEntry::Page(page) => &page.path_key,
    }
}

impl NavItem {
    pub fn section(title: String) -> Self {
        Self {
            title,
            target: None,
            children: Vec::new(),
        }
    }

    pub fn page(title: String, source_key: String, output_path: PathBuf) -> Self {
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

fn path_components(path: &Path) -> Vec<String> {
    path.components()
        .map(|component| component.as_os_str())
        .filter(|component| *component != OsStr::new("."))
        .map(|component| component.to_string_lossy().into_owned())
        .collect()
}

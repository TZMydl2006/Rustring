use crate::config::Config;
use crate::error::{MiniZensicalError, Result};
use crate::nav::{NavItem, Navigation, PageLink, RenderNavItem, relative_href};
use crate::page::Page;
use crate::render::{
    ArchiveGroup, ArchiveSection, FontOption, code_script_contents, code_script_path,
    default_font_options, render_archive_index, render_page, render_tag_archive,
    search_script_contents, search_script_path, stylesheet_contents, stylesheet_path,
    theme_script_contents, theme_script_path,
};
use crate::scanner::{SourceFile, scan_site, titleize};
use crate::search::{build_search_index, search_index_path};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn build_site(config: &Config) -> Result<()> {
    let final_site_dir = config.site_dir();
    let staging_name = unique_dir_name(".minizensical-site-staging");
    let backup_name = unique_dir_name(".minizensical-site-backup");

    let mut staging_config = config.clone();
    staging_config.project.site_dir = staging_name.clone();
    let staging_site_dir = staging_config.site_dir();
    let backup_site_dir = config.root_dir.join(backup_name);

    if staging_site_dir.exists() {
        fs::remove_dir_all(&staging_site_dir)
            .map_err(|error| MiniZensicalError::io("remove", &staging_site_dir, error))?;
    }

    let build_result = build_site_contents(&staging_config);
    if let Err(error) = build_result {
        cleanup_dir(&staging_site_dir);
        return Err(error);
    }

    if backup_site_dir.exists() {
        fs::remove_dir_all(&backup_site_dir)
            .map_err(|error| MiniZensicalError::io("remove", &backup_site_dir, error))?;
    }

    let had_existing_site = final_site_dir.exists();
    if had_existing_site {
        fs::rename(&final_site_dir, &backup_site_dir)
            .map_err(|error| MiniZensicalError::io("rename", &final_site_dir, error))?;
    }

    if let Err(error) = fs::rename(&staging_site_dir, &final_site_dir) {
        if had_existing_site && backup_site_dir.exists() {
            let _ = fs::rename(&backup_site_dir, &final_site_dir);
        }
        cleanup_dir(&staging_site_dir);
        return Err(MiniZensicalError::io("rename", &final_site_dir, error));
    }

    cleanup_dir(&backup_site_dir);
    Ok(())
}

fn build_site_contents(config: &Config) -> Result<()> {
    let site_dir = config.site_dir();
    if site_dir.exists() {
        fs::remove_dir_all(&site_dir)
            .map_err(|error| MiniZensicalError::io("remove", &site_dir, error))?;
    }
    fs::create_dir_all(&site_dir)
        .map_err(|error| MiniZensicalError::io("create", &site_dir, error))?;

    let sources = scan_site(config)?;
    let font_options = build_font_options(&sources.asset_files);
    let mut pages = sources
        .markdown_files
        .iter()
        .map(|source| Page::from_source(config, source))
        .collect::<Result<Vec<_>>>()?;
    pages.sort_by(|left, right| left.output_path.cmp(&right.output_path));

    let mut navigation = Navigation::build(&config.project.nav, &mut pages)?;
    navigation.items.push(NavItem {
        title: "Archive".to_string(),
        target: None,
        children: vec![
            NavItem::page(
                "By Date".to_string(),
                String::new(),
                Path::new("archive/index.html").to_path_buf(),
            ),
            NavItem::page(
                "By Tags".to_string(),
                String::new(),
                Path::new("archive/tags/index.html").to_path_buf(),
            ),
        ],
    });
    write_theme_assets(config, &font_options)?;

    let archive_page_path = Path::new("archive/index.html");
    let tag_page_path = Path::new("archive/tags/index.html");
    let archive_sections = build_date_archive(&pages, archive_page_path);
    let tag_sections = build_tag_archive(&pages, tag_page_path);

    // Build nav items for archive pages.
    // Archive pages live at archive/index.html and archive/tags/index.html.
    let archive_nav = vec![RenderNavItem {
        title: "Archive".to_string(),
        href: None,
        children: vec![
            RenderNavItem {
                title: "By Date".to_string(),
                href: Some("index.html".to_string()),
                active: true,
                children: vec![],
            },
            RenderNavItem {
                title: "By Tags".to_string(),
                href: Some("tags/index.html".to_string()),
                active: false,
                children: vec![],
            },
        ],
        active: true,
    }];

    let tag_nav = vec![RenderNavItem {
        title: "Archive".to_string(),
        href: None,
        children: vec![
            RenderNavItem {
                title: "By Date".to_string(),
                href: Some("../index.html".to_string()),
                active: false,
                children: vec![],
            },
            RenderNavItem {
                title: "By Tags".to_string(),
                href: Some("index.html".to_string()),
                active: true,
                children: vec![],
            },
        ],
        active: true,
    }];

    // Render archive pages
    let archive_html =
        render_archive_index(config, &archive_sections, &archive_nav, &font_options)?;
    write_archive_page(config, archive_page_path, &archive_html)?;

    let tag_html = render_tag_archive(config, &tag_sections, &tag_nav, &font_options)?;
    write_archive_page(config, tag_page_path, &tag_html)?;

    write_search_index(config, &pages)?;
    render_pages(config, &pages, &navigation, &font_options)?;
    copy_assets(config, &sources.asset_files)?;
    Ok(())
}

fn build_date_archive(pages: &[Page], archive_page_path: &Path) -> Vec<ArchiveSection> {
    let mut dated: BTreeMap<String, BTreeMap<String, Vec<PageLink>>> = BTreeMap::new();

    for page in pages {
        let date_str = match &page.metadata.date {
            Some(d) => d.clone(),
            None => continue,
        };
        let (year, month) = if date_str.len() >= 7 {
            (date_str[..4].to_string(), Some(date_str[5..7].to_string()))
        } else {
            (date_str.clone(), None)
        };

        let href = relative_href(archive_page_path, &page.output_path);
        let link = PageLink {
            title: page.title.clone(),
            href,
        };

        dated.entry(year.clone()).or_default();
        let month_label = month
            .as_ref()
            .and_then(|m| match m.as_str() {
                "01" => Some("January".to_string()),
                "02" => Some("February".to_string()),
                "03" => Some("March".to_string()),
                "04" => Some("April".to_string()),
                "05" => Some("May".to_string()),
                "06" => Some("June".to_string()),
                "07" => Some("July".to_string()),
                "08" => Some("August".to_string()),
                "09" => Some("September".to_string()),
                "10" => Some("October".to_string()),
                "11" => Some("November".to_string()),
                "12" => Some("December".to_string()),
                _ => None,
            })
            .unwrap_or_else(|| month.unwrap_or_default());

        dated
            .get_mut(&year)
            .unwrap()
            .entry(month_label)
            .or_default()
            .push(link);
    }

    dated
        .into_iter()
        .rev()
        .map(|(year, months)| ArchiveSection {
            title: year,
            groups: months
                .into_iter()
                .map(|(month, pages)| ArchiveGroup {
                    title: month,
                    pages,
                })
                .collect(),
        })
        .collect()
}

fn build_tag_archive(pages: &[Page], archive_page_path: &Path) -> Vec<ArchiveSection> {
    let mut by_tag: BTreeMap<String, Vec<PageLink>> = BTreeMap::new();

    for page in pages {
        let href = relative_href(archive_page_path, &page.output_path);
        let link = PageLink {
            title: page.title.clone(),
            href,
        };

        if page.metadata.tags.is_empty() {
            by_tag
                .entry("(untagged)".to_string())
                .or_default()
                .push(link.clone());
        } else {
            for tag in &page.metadata.tags {
                by_tag.entry(tag.clone()).or_default().push(link.clone());
            }
        }
    }

    by_tag
        .into_iter()
        .map(|(tag, pages)| ArchiveSection {
            title: tag,
            groups: vec![ArchiveGroup {
                title: String::new(),
                pages,
            }],
        })
        .collect()
}

fn write_archive_page(config: &Config, relative_path: &Path, html: &str) -> Result<()> {
    let path = config.site_path_for(relative_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| MiniZensicalError::io("create", parent, error))?;
    }
    fs::write(&path, html).map_err(|error| MiniZensicalError::io("write", &path, error))
}

fn build_font_options(assets: &[SourceFile]) -> Vec<FontOption> {
    let mut options = default_font_options();
    for asset in assets {
        if !is_provided_font_asset(&asset.relative_path) {
            continue;
        }

        let label = asset
            .relative_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .map(titleize)
            .filter(|label| !label.is_empty())
            .unwrap_or_else(|| String::from("Provided Font"));
        let family = format!("MiniZensical {label}");
        let source_url = relative_href(&stylesheet_path(), &asset.relative_path);
        options.push(FontOption::provided(label, family, source_url));
    }
    options
}

fn is_provided_font_asset(path: &Path) -> bool {
    let components = path
        .components()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    if components.len() < 3
        || !components[0].eq_ignore_ascii_case("assets")
        || !components[1].eq_ignore_ascii_case("fonts")
    {
        return false;
    }

    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            ["woff2", "woff", "ttf", "otf"]
                .iter()
                .any(|allowed| extension.eq_ignore_ascii_case(allowed))
        })
}

fn cleanup_dir(path: &Path) {
    if path.exists() {
        let _ = fs::remove_dir_all(path);
    }
}

fn unique_dir_name(prefix: &str) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    format!("{prefix}-{}-{timestamp}", process::id())
}

fn write_theme_assets(config: &Config, font_options: &[FontOption]) -> Result<()> {
    write_asset(
        config,
        stylesheet_path(),
        stylesheet_contents(font_options).as_bytes(),
    )?;
    write_asset(
        config,
        theme_script_path(),
        theme_script_contents().as_bytes(),
    )?;
    write_asset(
        config,
        search_script_path(),
        search_script_contents().as_bytes(),
    )?;
    write_asset(
        config,
        code_script_path(),
        code_script_contents().as_bytes(),
    )?;
    Ok(())
}

fn write_asset(config: &Config, relative_path: std::path::PathBuf, contents: &[u8]) -> Result<()> {
    let path = config.site_path_for(relative_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| MiniZensicalError::io("create", parent, error))?;
    }
    fs::write(&path, contents).map_err(|error| MiniZensicalError::io("write", &path, error))
}

fn write_search_index(config: &Config, pages: &[Page]) -> Result<()> {
    let path = config.site_path_for(search_index_path());
    let index = build_search_index(pages, config.project.use_directory_urls);
    let contents =
        serde_json::to_vec_pretty(&index).map_err(|source| MiniZensicalError::SerializeSearch {
            path: path.clone(),
            source,
        })?;
    fs::write(&path, contents).map_err(|error| MiniZensicalError::io("write", &path, error))
}

fn render_pages(
    config: &Config,
    pages: &[Page],
    navigation: &Navigation,
    font_options: &[FontOption],
) -> Result<()> {
    for page in pages {
        let (nav_items, previous_page, next_page) = navigation.render_for_page(page);
        let home_href = relative_href(&page.output_path, Path::new("index.html"));
        let stylesheet_href =
            relative_href(&page.output_path, Path::new("assets/minizensical.css"));
        let theme_script_href =
            relative_href(&page.output_path, Path::new("assets/minizensical-theme.js"));
        let search_script_href = relative_href(
            &page.output_path,
            Path::new("assets/minizensical-search.js"),
        );
        let code_script_href =
            relative_href(&page.output_path, Path::new("assets/minizensical-code.js"));
        let search_index_href = relative_href(&page.output_path, Path::new("search.json"));
        let html = render_page(
            config,
            page,
            &nav_items,
            previous_page,
            next_page,
            &home_href,
            &stylesheet_href,
            &theme_script_href,
            &search_script_href,
            &code_script_href,
            &search_index_href,
            font_options,
        )?;
        let output_path = page.target_path(config);
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| MiniZensicalError::io("create", parent, error))?;
        }
        fs::write(&output_path, html)
            .map_err(|error| MiniZensicalError::io("write", &output_path, error))?;
    }
    Ok(())
}

fn copy_assets(config: &Config, assets: &[crate::scanner::SourceFile]) -> Result<()> {
    for asset in assets {
        let output_path = config.site_path_for(&asset.relative_path);
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| MiniZensicalError::io("create", parent, error))?;
        }
        fs::copy(&asset.source_path, &output_path)
            .map_err(|error| MiniZensicalError::io("copy", output_path.clone(), error))?;
    }
    Ok(())
}

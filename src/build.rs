use crate::config::Config;
use crate::error::{MiniZensicalError, Result};
use crate::nav::{Navigation, relative_href};
use crate::page::Page;
use crate::render::{render_page, stylesheet_contents, stylesheet_path};
use crate::scanner::scan_site;
use std::fs;
use std::path::Path;

pub fn build_site(config: &Config) -> Result<()> {
    let site_dir = config.site_dir();
    if site_dir.exists() {
        fs::remove_dir_all(&site_dir)
            .map_err(|error| MiniZensicalError::io("remove", &site_dir, error))?;
    }
    fs::create_dir_all(&site_dir)
        .map_err(|error| MiniZensicalError::io("create", &site_dir, error))?;

    let sources = scan_site(config)?;
    let mut pages = sources
        .markdown_files
        .iter()
        .map(|source| Page::from_source(config, source))
        .collect::<Result<Vec<_>>>()?;
    pages.sort_by(|left, right| left.output_path.cmp(&right.output_path));

    let navigation = Navigation::build(&config.project.nav, &mut pages)?;
    write_theme_assets(config)?;
    render_pages(config, &pages, &navigation)?;
    copy_assets(config, &sources.asset_files)?;
    Ok(())
}

fn write_theme_assets(config: &Config) -> Result<()> {
    let path = config.site_path_for(stylesheet_path());
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| MiniZensicalError::io("create", parent, error))?;
    }
    fs::write(&path, stylesheet_contents())
        .map_err(|error| MiniZensicalError::io("write", &path, error))
}

fn render_pages(config: &Config, pages: &[Page], navigation: &Navigation) -> Result<()> {
    for page in pages {
        let (nav_items, previous_page, next_page) = navigation.render_for_page(page);
        let home_href = relative_href(&page.output_path, Path::new("index.html"));
        let stylesheet_href =
            relative_href(&page.output_path, Path::new("assets/minizensical.css"));
        let html = render_page(
            config,
            page,
            &nav_items,
            previous_page,
            next_page,
            &home_href,
            &stylesheet_href,
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

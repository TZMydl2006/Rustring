use crate::config::Config;
use crate::error::{MiniZensicalError, Result};
use crate::nav::{Navigation, relative_href};
use crate::page::Page;
use crate::render::{render_page, stylesheet_contents, stylesheet_path};
use crate::scanner::scan_site;
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

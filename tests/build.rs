use minizensical::{Config, build_site};
use serde_json::Value;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn builds_a_minimal_site_with_search_ui() {
    let temp_dir = TempDir::new().expect("temp dir");
    write_file(
        temp_dir.path().join("zensical.toml"),
        r#"
[project]
site_name = "Test Docs"
"#,
    );
    write_file(
        temp_dir.path().join("docs/index.md"),
        "# Hello\n\n## Intro\n\nWelcome to the site.\n\n```rust\nfn main() {\n    println!(\"hi\");\n}\n```\n",
    );
    write_file(
        temp_dir.path().join("docs/assets/fonts/demo-sans.woff2"),
        "placeholder font bytes",
    );

    let config = Config::load(temp_dir.path().join("zensical.toml")).unwrap();
    build_site(&config).unwrap();

    let html = fs::read_to_string(temp_dir.path().join("site/index.html")).unwrap();
    assert!(html.contains("Test Docs"));
    assert!(html.contains("doc-search"));
    assert!(html.contains("id=\"page-title\""));
    assert!(!html.contains("search-mode"));
    assert!(!html.contains("highlight-color"));
    assert!(!html.contains("color-swatch"));
    assert!(html.contains("data-theme-choice=\"dark\""));
    assert!(html.contains("minizensical-theme.js"));
    assert!(html.contains("minizensical-search.js"));
    assert!(html.contains("minizensical-code.js"));
    assert!(html.contains("data-font-switcher"));
    assert!(html.contains("Demo Sans"));
    assert!(temp_dir.path().join("site/search.json").exists());
    assert!(
        temp_dir
            .path()
            .join("site/assets/minizensical-theme.js")
            .exists()
    );
    assert!(
        temp_dir
            .path()
            .join("site/assets/minizensical-code.js")
            .exists()
    );
    let search_js =
        fs::read_to_string(temp_dir.path().join("site/assets/minizensical-search.js")).unwrap();
    assert!(search_js.contains("mz-search"));
    assert!(search_js.contains("search-target-active"));
    assert!(search_js.contains("pageBody.addEventListener(\"click\""));
    let css = fs::read_to_string(temp_dir.path().join("site/assets/minizensical.css")).unwrap();
    assert!(css.contains("@font-face"));
    assert!(css.contains("fonts/demo-sans.woff2"));
}

#[test]
fn builds_directory_urls_and_copies_assets() {
    let temp_dir = TempDir::new().expect("temp dir");
    write_file(
        temp_dir.path().join("zensical.toml"),
        r#"
[project]
site_name = "Nested Docs"
"#,
    );
    write_file(temp_dir.path().join("docs/index.md"), "# Home\n");
    write_file(temp_dir.path().join("docs/guide/index.md"), "# Guide\n");
    write_file(temp_dir.path().join("docs/guide/setup.md"), "# Setup\n");
    write_file(temp_dir.path().join("docs/assets/logo.txt"), "logo");

    let config = Config::load(temp_dir.path().join("zensical.toml")).unwrap();
    build_site(&config).unwrap();

    assert!(temp_dir.path().join("site/guide/index.html").exists());
    assert!(temp_dir.path().join("site/guide/setup/index.html").exists());
    assert_eq!(
        fs::read_to_string(temp_dir.path().join("site/assets/logo.txt")).unwrap(),
        "logo"
    );

    let guide_html = fs::read_to_string(temp_dir.path().join("site/guide/index.html")).unwrap();
    assert!(guide_html.contains("setup&#x2f;index.html") || guide_html.contains("guide/setup/"));
}

#[test]
fn explicit_nav_overrides_titles_and_order() {
    let temp_dir = TempDir::new().expect("temp dir");
    write_file(
        temp_dir.path().join("zensical.toml"),
        r#"
[project]
site_name = "Ordered Docs"
nav = [
  { title = "Landing", path = "index.md" },
  { title = "Guide", children = [
    { title = "Install", path = "guide/setup.md" },
    { title = "Overview", path = "guide/index.md" }
  ] }
]
"#,
    );
    write_file(temp_dir.path().join("docs/index.md"), "# Home Heading\n");
    write_file(
        temp_dir.path().join("docs/guide/index.md"),
        "# Guide Heading\n",
    );
    write_file(
        temp_dir.path().join("docs/guide/setup.md"),
        "# Setup Heading\n",
    );

    let config = Config::load(temp_dir.path().join("zensical.toml")).unwrap();
    build_site(&config).unwrap();

    let home_html = fs::read_to_string(temp_dir.path().join("site/index.html")).unwrap();
    assert!(home_html.contains("<title>Landing - Ordered Docs</title>"));
    assert!(home_html.contains("Previous") || home_html.contains("Next"));
    assert!(home_html.contains(">Install<"));

    let setup_html =
        fs::read_to_string(temp_dir.path().join("site/guide/setup/index.html")).unwrap();
    assert!(setup_html.contains("<h1 id=\"page-title\">Install</h1>"));
    assert!(setup_html.contains("Landing"));
    assert!(setup_html.contains("Overview"));
}

#[test]
fn front_matter_drives_rendering_and_search_index() {
    let temp_dir = TempDir::new().expect("temp dir");
    write_file(
        temp_dir.path().join("zensical.toml"),
        r#"
[project]
site_name = "Meta Docs"
"#,
    );
    write_file(
        temp_dir.path().join("docs/index.md"),
        r#"---
title: Custom Home
summary: Searchable summary for the landing page.
tags:
  - rust
  - search
---
# Hidden H1

Body keyword lives here.

## Search Panel

The heading should also be searchable.
"#,
    );

    let config = Config::load(temp_dir.path().join("zensical.toml")).unwrap();
    build_site(&config).unwrap();

    let html = fs::read_to_string(temp_dir.path().join("site/index.html")).unwrap();
    assert!(html.contains("<title>Custom Home - Meta Docs</title>"));
    assert!(html.contains("Searchable summary for the landing page."));
    assert!(html.contains("tag-chip\">rust"));
    assert!(html.contains("tag-chip\">search"));

    let search_json = fs::read_to_string(temp_dir.path().join("site/search.json")).unwrap();
    let entries: Value = serde_json::from_str(&search_json).unwrap();
    let entry = entries
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["title"] == "Custom Home")
        .unwrap();

    assert!(entry.get("summary").is_none());
    assert!(entry.get("tags").is_none());
    assert!(entry.get("headings").is_none());
    assert!(entry.get("body").is_none());

    let blocks = entry["blocks"].as_array().unwrap();
    assert!(blocks.iter().any(|block| {
        block["id"] == "page-title" && block["kind"] == "title" && block["text"] == "Custom Home"
    }));
    assert!(blocks.iter().any(|block| {
        block["kind"] == "heading"
            && (block["text"] == "Hidden H1" || block["text"] == "Search Panel")
    }));
    assert!(blocks.iter().any(|block| {
        block["kind"] == "body"
            && block["id"]
                .as_str()
                .is_some_and(|id| id.starts_with("mz-search-block-"))
            && block["text"]
                .as_str()
                .is_some_and(|text| text.contains("Body keyword"))
    }));
    assert!(!blocks.iter().any(|block| block["text"] == "rust"));
}

#[test]
fn auto_navigation_orders_pages_and_sections_as_siblings() {
    let temp_dir = TempDir::new().expect("temp dir");
    write_file(
        temp_dir.path().join("zensical.toml"),
        r#"
[project]
site_name = "Auto Ordered Docs"
	"#,
    );
    write_file(
        temp_dir.path().join("docs/index.md"),
        r#"---
title: Home
order: 0
---
# Home
"#,
    );
    write_file(
        temp_dir.path().join("docs/helloworld.md"),
        r#"---
title: Hello, World!
order: 1
---
# Hello
"#,
    );
    write_file(
        temp_dir.path().join("docs/project-showcase.md"),
        r#"---
title: Project Showcase
order: 2
---
# Project
"#,
    );
    write_file(
        temp_dir.path().join("docs/guide/index.md"),
        r#"---
title: Guide Overview
order: 3
---
# Guide
"#,
    );
    write_file(
        temp_dir.path().join("docs/guide/alpha.md"),
        r#"---
title: Alpha
order: 2
---
# Alpha
"#,
    );
    write_file(
        temp_dir.path().join("docs/guide/beta.md"),
        r#"---
title: Beta
order: 1
---
# Beta
	"#,
    );
    write_file(
        temp_dir.path().join("docs/guide/front-matter.md"),
        r#"---
title: Front Matter
order: 3
---
# Front Matter
"#,
    );
    write_file(
        temp_dir.path().join("docs/draft.md"),
        r#"---
title: Draft
order: 99
---
# Draft
"#,
    );

    let config = Config::load(temp_dir.path().join("zensical.toml")).unwrap();
    build_site(&config).unwrap();

    let home_html = fs::read_to_string(temp_dir.path().join("site/index.html")).unwrap();
    let home_position = nav_position(&home_html, ">Home<");
    let hello_position = nav_position(&home_html, ">Hello, World!<");
    let project_position = nav_position(&home_html, ">Project Showcase<");
    let guide_position = nav_position(&home_html, "<span class=\"nav-section\">Guide</span>");
    let draft_position = nav_position(&home_html, ">Draft<");

    assert!(home_position < hello_position);
    assert!(hello_position < project_position);
    assert!(project_position < guide_position);
    assert!(guide_position < draft_position);

    let guide_html = fs::read_to_string(temp_dir.path().join("site/guide/index.html")).unwrap();
    let overview_position = nav_position(&guide_html, ">Guide Overview<");
    let beta_position = guide_html.find(">Beta<").unwrap();
    let alpha_position = guide_html.find(">Alpha<").unwrap();
    let front_matter_position = nav_position(&guide_html, ">Front Matter<");

    assert!(overview_position < beta_position);
    assert!(beta_position < alpha_position);
    assert!(alpha_position < front_matter_position);
}

#[test]
fn auto_navigation_uses_stable_path_fallbacks() {
    let temp_dir = TempDir::new().expect("temp dir");
    write_file(
        temp_dir.path().join("zensical.toml"),
        r#"
[project]
site_name = "Fallback Ordered Docs"
"#,
    );
    write_file(temp_dir.path().join("docs/index.md"), "# Home\n");
    write_file(temp_dir.path().join("docs/draft.md"), "# Draft\n");
    write_file(temp_dir.path().join("docs/guide/index.md"), "# Guide\n");
    write_file(
        temp_dir.path().join("docs/tie-b.md"),
        r#"---
title: Tie B
order: 1
---
# Tie B
"#,
    );
    write_file(
        temp_dir.path().join("docs/tie-a.md"),
        r#"---
title: Tie A
order: 1
---
# Tie A
"#,
    );

    let config = Config::load(temp_dir.path().join("zensical.toml")).unwrap();
    build_site(&config).unwrap();

    let home_html = fs::read_to_string(temp_dir.path().join("site/index.html")).unwrap();
    let tie_a_position = nav_position(&home_html, ">Tie A<");
    let tie_b_position = nav_position(&home_html, ">Tie B<");
    let home_position = nav_position(&home_html, ">Home<");
    let draft_position = nav_position(&home_html, ">Draft<");
    let guide_position = nav_position(&home_html, "<span class=\"nav-section\">Guide</span>");

    assert!(tie_a_position < tie_b_position);
    assert!(tie_b_position < home_position);
    assert!(home_position < draft_position);
    assert!(draft_position < guide_position);
}

#[test]
fn reports_missing_explicit_nav_pages() {
    let temp_dir = TempDir::new().expect("temp dir");
    write_file(
        temp_dir.path().join("zensical.toml"),
        r#"
[project]
site_name = "Broken Docs"
nav = [
  { title = "Home", path = "missing.md" }
]
"#,
    );
    write_file(temp_dir.path().join("docs/index.md"), "# Home\n");

    let config = Config::load(temp_dir.path().join("zensical.toml")).unwrap();
    let error = build_site(&config).unwrap_err();
    assert!(error.to_string().contains("references missing page"));
}

#[test]
fn supports_flat_html_output_when_directory_urls_are_disabled() {
    let temp_dir = TempDir::new().expect("temp dir");
    write_file(
        temp_dir.path().join("zensical.toml"),
        r#"
[project]
site_name = "Flat Docs"
use_directory_urls = false
"#,
    );
    write_file(temp_dir.path().join("docs/index.md"), "# Home\n");
    write_file(temp_dir.path().join("docs/guide/setup.md"), "# Setup\n");

    let config = Config::load(temp_dir.path().join("zensical.toml")).unwrap();
    build_site(&config).unwrap();

    assert!(temp_dir.path().join("site/index.html").exists());
    assert!(temp_dir.path().join("site/guide/setup.html").exists());
}

#[test]
fn archive_pages_use_correct_relative_links_to_content_pages() {
    let temp_dir = TempDir::new().expect("temp dir");
    write_file(
        temp_dir.path().join("zensical.toml"),
        r#"
[project]
site_name = "Archive Docs"
"#,
    );
    write_file(
        temp_dir.path().join("docs/project-case.md"),
        r#"---
title: Project Case
tags:
  - showcase
date: 2025-04-01
---
# Project Case
"#,
    );

    let config = Config::load(temp_dir.path().join("zensical.toml")).unwrap();
    build_site(&config).unwrap();

    let by_date_html = fs::read_to_string(temp_dir.path().join("site/archive/index.html")).unwrap();
    assert!(by_date_html.contains("href=\"../project-case/index.html\""));
    assert!(!by_date_html.contains("href=\"project-case/\""));
    assert!(!by_date_html.contains("href=\"archive/project-case\""));

    let by_tags_html =
        fs::read_to_string(temp_dir.path().join("site/archive/tags/index.html")).unwrap();
    assert!(by_tags_html.contains("href=\"../../project-case/index.html\""));
    assert!(!by_tags_html.contains("href=\"project-case/\""));
    assert!(!by_tags_html.contains("href=\"archive/project-case\""));
}

#[test]
fn keeps_previous_site_when_rebuild_fails() {
    let temp_dir = TempDir::new().expect("temp dir");
    write_file(
        temp_dir.path().join("zensical.toml"),
        r#"
[project]
site_name = "Stable Docs"
"#,
    );
    write_file(temp_dir.path().join("docs/index.md"), "# Stable\n");

    let config = Config::load(temp_dir.path().join("zensical.toml")).unwrap();
    build_site(&config).unwrap();

    let first_html = fs::read_to_string(temp_dir.path().join("site/index.html")).unwrap();
    assert!(first_html.contains("Stable"));

    fs::remove_dir_all(temp_dir.path().join("docs")).unwrap();
    let error = build_site(&config).unwrap_err();
    assert!(error.to_string().contains("docs directory does not exist"));

    let still_served_html = fs::read_to_string(temp_dir.path().join("site/index.html")).unwrap();
    assert!(still_served_html.contains("Stable"));
}

fn write_file(path: impl AsRef<Path>, contents: &str) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, contents).unwrap();
}

fn nav_position(html: &str, needle: &str) -> usize {
    html.find(needle)
        .unwrap_or_else(|| panic!("missing navigation item: {needle}"))
}

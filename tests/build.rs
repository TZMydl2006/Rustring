use minizensical::{Config, build_site};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn builds_a_minimal_site() {
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
        "# Hello\n\n## Intro\n\nWelcome to the site.\n",
    );

    let config = Config::load(temp_dir.path().join("zensical.toml")).unwrap();
    build_site(&config).unwrap();

    let html = fs::read_to_string(temp_dir.path().join("site/index.html")).unwrap();
    assert!(html.contains("Test Docs"));
    assert!(html.contains("Hello"));
    assert!(html.contains("On this page"));
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
    assert!(guide_html.contains("setup&#x2f;index.html"));
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
    assert!(home_html.contains("Next: Install"));

    let setup_html =
        fs::read_to_string(temp_dir.path().join("site/guide/setup/index.html")).unwrap();
    assert!(setup_html.contains("<h1>Install</h1>"));
    assert!(setup_html.contains("Previous: Landing"));
    assert!(setup_html.contains("Next: Overview"));
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

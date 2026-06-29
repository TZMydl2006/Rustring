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
    assert!(html.contains("minizensical-math.js"));
    assert!(html.contains("Knowledge Graph"));
    assert!(html.contains("data-font-switcher"));
    assert!(html.contains("Demo Sans"));
    assert!(temp_dir.path().join("site/search.json").exists());
    assert!(temp_dir.path().join("site/graph.json").exists());
    assert!(
        temp_dir
            .path()
            .join("site/knowledge-graph/index.html")
            .exists()
    );
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
    assert!(
        temp_dir
            .path()
            .join("site/assets/minizensical-math.js")
            .exists()
    );
    assert!(
        temp_dir
            .path()
            .join("site/assets/minizensical-graph.js")
            .exists()
    );
    assert!(temp_dir.path().join("site/assets/d3.min.js").exists());
    let d3_js = fs::read_to_string(temp_dir.path().join("site/assets/d3.min.js")).unwrap();
    assert!(d3_js.starts_with("// https://d3js.org v7.9.0"));
    let search_js =
        fs::read_to_string(temp_dir.path().join("site/assets/minizensical-search.js")).unwrap();
    assert!(search_js.contains("mz-search"));
    assert!(search_js.contains("search-target-active"));
    assert!(search_js.contains("pageBody.addEventListener(\"click\""));
    assert!(search_js.contains("syncTargetHighlight"));
    assert!(search_js.contains("history.pushState"));
    assert!(search_js.contains("popstate"));
    assert!(search_js.contains("hashchange"));
    assert!(search_js.contains(".search-match"));
    assert!(search_js.contains("aria-current"));
    assert!(search_js.contains("is-active"));
    let css = fs::read_to_string(temp_dir.path().join("site/assets/minizensical.css")).unwrap();
    assert!(css.contains("@font-face"));
    assert!(css.contains("fonts/demo-sans.woff2"));
    assert!(css.contains(r#":root[data-theme="sepia"]"#));
    assert!(css.contains(r#":root[data-theme="ocean"]"#));
    assert!(css.contains(r#":root[data-theme="forest"]"#));
    assert!(css.contains(".graph-stage"));
    assert!(css.contains(".search-match.is-active"));
    assert!(css.contains(".math-inline"));
    assert!(css.contains("white-space: nowrap"));
    assert!(css.contains("text-align: center !important"));

    let theme_js =
        fs::read_to_string(temp_dir.path().join("site/assets/minizensical-theme.js")).unwrap();
    assert!(theme_js.contains("sepia"));
    assert!(theme_js.contains("ocean"));
    assert!(theme_js.contains("forest"));

    let graph_js =
        fs::read_to_string(temp_dir.path().join("site/assets/minizensical-graph.js")).unwrap();
    assert!(graph_js.contains("graphJson"));
    assert!(graph_js.contains("knowledge-graph"));
    assert!(graph_js.contains("forceSimulation"));
    assert!(graph_js.contains("forceManyBody"));
    assert!(graph_js.contains("forceLink"));
    assert!(graph_js.contains("forceX"));
    assert!(graph_js.contains("d3.drag"));
    assert!(graph_js.contains("d3.zoom"));
    assert!(graph_js.contains("ResizeObserver"));
}

#[test]
fn builds_knowledge_graph_json_page_and_relationships() {
    let temp_dir = TempDir::new().expect("temp dir");
    write_file(
        temp_dir.path().join("zensical.toml"),
        r#"
[project]
site_name = "Graph Docs"
"#,
    );
    write_file(
        temp_dir.path().join("docs/index.md"),
        r#"---
title: Home
summary: Root summary.
tags:
  - rust
  - graph
---
# Home

## Overview

Read [Setup](guide/setup.md "setup link") for the implementation notes.
"#,
    );
    write_file(
        temp_dir.path().join("docs/guide/setup.md"),
        r#"---
title: Setup
tags:
  - rust
  - guide
---
# Setup

## Parser Notes
"#,
    );
    write_file(
        temp_dir.path().join("docs/guide/extra.md"),
        r#"---
title: Extra
tags:
  - guide
---
# Extra
"#,
    );

    let config = Config::load(temp_dir.path().join("zensical.toml")).unwrap();
    build_site(&config).unwrap();

    let graph_json = fs::read_to_string(temp_dir.path().join("site/graph.json")).unwrap();
    let graph: Value = serde_json::from_str(&graph_json).unwrap();
    let nodes = graph["nodes"].as_array().unwrap();
    let edges = graph["edges"].as_array().unwrap();

    assert_eq!(graph["version"], 1);
    assert!(nodes.iter().any(|node| {
        node["id"] == "doc:index.md"
            && node["type"] == "document"
            && node["label"] == "Home"
            && node["url"] == ""
    }));
    assert!(nodes.iter().any(|node| {
        node["id"] == "doc:guide/setup.md"
            && node["type"] == "document"
            && node["url"] == "guide/setup/"
    }));
    assert!(
        nodes
            .iter()
            .any(|node| { node["type"] == "tag" && node["label"] == "rust" })
    );
    assert!(
        nodes
            .iter()
            .all(|node| matches!(node["type"].as_str(), Some("document" | "tag")))
    );

    assert!(edge_exists(edges, "doc:index.md", "tag:rust", "has_tag"));
    assert!(edge_exists(
        edges,
        "doc:index.md",
        "doc:guide/setup.md",
        "links_to"
    ));
    assert!(edge_exists_unordered(
        edges,
        "doc:index.md",
        "doc:guide/setup.md",
        "shared_tag"
    ));
    assert!(edges.iter().all(|edge| matches!(
        edge["type"].as_str(),
        Some("has_tag" | "links_to" | "shared_tag")
    )));
    for edge in edges {
        let source = edge["source"].as_str().unwrap();
        let target = edge["target"].as_str().unwrap();
        let edge_type = edge["type"].as_str().unwrap();
        let source_type = nodes
            .iter()
            .find(|node| node["id"] == source)
            .and_then(|node| node["type"].as_str())
            .expect("edge source node");
        let target_type = nodes
            .iter()
            .find(|node| node["id"] == target)
            .and_then(|node| node["type"].as_str())
            .expect("edge target node");
        match edge_type {
            "has_tag" => assert_eq!((source_type, target_type), ("document", "tag")),
            "links_to" | "shared_tag" => {
                assert_eq!((source_type, target_type), ("document", "document"));
            }
            _ => unreachable!(),
        }
    }

    let graph_html =
        fs::read_to_string(temp_dir.path().join("site/knowledge-graph/index.html")).unwrap();
    assert!(graph_html.contains("data-graph-json=\"../graph.json\""));
    assert!(graph_html.contains("id=\"knowledge-graph\""));
    assert!(graph_html.contains("id=\"graph-filter\""));
    assert!(graph_html.contains("id=\"graph-show-tags\" type=\"checkbox\""));
    assert!(!graph_html.contains("id=\"graph-show-tags\" type=\"checkbox\" checked"));
    assert!(graph_html.contains("id=\"graph-center-force\""));
    assert!(graph_html.contains("id=\"graph-repulsion-force\""));
    assert!(graph_html.contains("id=\"graph-link-force\""));
    assert!(graph_html.contains("id=\"graph-link-distance\""));
    assert!(graph_html.contains("../assets/d3.min.js"));
    assert!(!graph_html.contains("cdn.jsdelivr.net/npm/d3"));
    assert!(!graph_html.contains("data-graph-type"));
    assert!(!graph_html.contains("graph-detail"));
    assert!(!graph_html.contains("Topics"));
    assert!(graph_html.contains("minizensical-graph.js"));
    assert!(graph_html.contains("href=\"../index.html\">Home</a>"));
    assert!(graph_html.contains("href=\"../guide/setup/index.html\">Setup</a>"));
    assert!(graph_html.contains("href=\"index.html\">Knowledge Graph</a>"));

    let home_html = fs::read_to_string(temp_dir.path().join("site/index.html")).unwrap();
    assert!(home_html.contains("Knowledge Graph"));
    assert!(home_html.contains(r#"href="guide/setup/index.html" title="setup link""#));
    assert!(!home_html.contains(r#"href="guide/setup.md""#));
}

#[test]
fn renders_inline_and_display_math_with_mathjax_support() {
    let temp_dir = TempDir::new().expect("temp dir");
    write_file(
        temp_dir.path().join("zensical.toml"),
        r#"
[project]
site_name = "Math Docs"
"#,
    );
    write_file(
        temp_dir.path().join("docs/index.md"),
        "# Math\n\nInline: ( $V_{GS} > V_T$ ).\n\n中文 $Y$ 等于输出。\n\nEnglish $x$ value.\n\n$$\nY_0 = \\frac{A + B}{2}\n\nY_1 = A - B\n$$\n\n`$not_math$`\n",
    );

    let config = Config::load(temp_dir.path().join("zensical.toml")).unwrap();
    build_site(&config).unwrap();

    let html = fs::read_to_string(temp_dir.path().join("site/index.html")).unwrap();
    assert!(html.contains(r#"(<span class="math math-inline">V_{GS} &gt; V_T</span>)."#));
    assert!(html.contains(r#"中文<span class="math math-inline">Y</span>等于输出。"#));
    assert!(html.contains(r#"English <span class="math math-inline">x</span> value."#));
    assert!(html.contains(r#"<span class="math math-display">"#));
    assert!(html.contains(
        r#"\begin{gathered} Y_0 = \frac{A + B}{2} \\ Y_1 = A - B \end{gathered}</span>"#
    ));
    assert!(html.contains("<code>$not_math$</code>"));
    assert!(html.contains("minizensical-math.js"));

    let math_js =
        fs::read_to_string(temp_dir.path().join("site/assets/minizensical-math.js")).unwrap();
    assert!(math_js.contains("MathJax.tex2svgPromise"));
    assert!(math_js.contains("document.head.appendChild(MathJax.svgStylesheet())"));
    assert!(math_js.contains(r#"fontCache: "local""#));
    assert!(math_js.contains(".trim()"));
    assert!(!math_js.contains("rendered.style.width"));
    assert!(math_js.contains("mathjax@3/es5/tex-svg.js"));
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
fn relocates_local_markdown_images_and_preserves_external_urls() {
    let temp_dir = TempDir::new().expect("temp dir");
    write_file(
        temp_dir.path().join("zensical.toml"),
        r#"
[project]
site_name = "Image Docs"
"#,
    );
    write_file(
        temp_dir.path().join("docs/index.md"),
        "# Home\n\n![root](./root.png)\n",
    );
    write_file(
        temp_dir.path().join("docs/guide/page.md"),
        r#"# Image Paths

![sibling](./sibling.png)
![parent asset](../assets/shared.png)
![nested asset](assets/local.png)
![external](https://example.com/external.png)
![external http](http://example.com/external.png)
"#,
    );
    write_file(temp_dir.path().join("docs/root.png"), "root image");
    write_file(
        temp_dir.path().join("docs/guide/sibling.png"),
        "sibling image",
    );
    write_file(
        temp_dir.path().join("docs/assets/shared.png"),
        "shared image",
    );
    write_file(
        temp_dir.path().join("docs/guide/assets/local.png"),
        "local image",
    );

    let config = Config::load(temp_dir.path().join("zensical.toml")).unwrap();
    build_site(&config).unwrap();

    let home_html = fs::read_to_string(temp_dir.path().join("site/index.html")).unwrap();
    assert!(home_html.contains(r#"src="root.png""#));

    let page_html = fs::read_to_string(temp_dir.path().join("site/guide/page/index.html")).unwrap();
    assert!(page_html.contains(r#"src="../sibling.png""#));
    assert!(page_html.contains(r#"src="../../assets/shared.png""#));
    assert!(page_html.contains(r#"src="../assets/local.png""#));
    assert!(page_html.contains(r#"src="https://example.com/external.png""#));
    assert!(page_html.contains(r#"src="http://example.com/external.png""#));

    assert!(temp_dir.path().join("site/root.png").exists());
    assert!(temp_dir.path().join("site/guide/sibling.png").exists());
    assert!(temp_dir.path().join("site/assets/shared.png").exists());
    assert!(temp_dir.path().join("site/guide/assets/local.png").exists());
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
    write_file(
        temp_dir.path().join("docs/index.md"),
        "# Home\n\nRead [Setup](guide/setup.md).\n",
    );
    write_file(
        temp_dir.path().join("docs/guide/setup.md"),
        "# Setup\n\nBack to [Home](../index.md).\n\n![diagram](./diagram.png)\n",
    );
    write_file(temp_dir.path().join("docs/guide/diagram.png"), "diagram");

    let config = Config::load(temp_dir.path().join("zensical.toml")).unwrap();
    build_site(&config).unwrap();

    assert!(temp_dir.path().join("site/index.html").exists());
    assert!(temp_dir.path().join("site/guide/setup.html").exists());
    assert!(temp_dir.path().join("site/guide/diagram.png").exists());
    let home_html = fs::read_to_string(temp_dir.path().join("site/index.html")).unwrap();
    assert!(home_html.contains(r#"href="guide/setup.html""#));
    assert!(!home_html.contains(r#"href="guide/setup.md""#));

    let setup_html = fs::read_to_string(temp_dir.path().join("site/guide/setup.html")).unwrap();
    assert!(setup_html.contains(r#"href="../index.html""#));
    assert!(setup_html.contains(r#"src="diagram.png""#));
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

fn edge_exists(edges: &[Value], source: &str, target: &str, edge_type: &str) -> bool {
    edges.iter().any(|edge| {
        edge["source"] == source && edge["target"] == target && edge["type"] == edge_type
    })
}

fn edge_exists_unordered(edges: &[Value], left: &str, right: &str, edge_type: &str) -> bool {
    edge_exists(edges, left, right, edge_type) || edge_exists(edges, right, left, edge_type)
}

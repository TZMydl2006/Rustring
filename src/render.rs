use crate::config::Config;
use crate::error::Result;
use crate::nav::{PageLink, RenderNavItem};
use crate::page::Page;
use minijinja::{Environment, context};
use serde::Serialize;
use std::path::PathBuf;

pub fn render_page(
    config: &Config,
    page: &Page,
    navigation: &[RenderNavItem],
    previous_page: Option<PageLink>,
    next_page: Option<PageLink>,
    home_href: &str,
    stylesheet_href: &str,
    theme_script_href: &str,
    search_script_href: &str,
    code_script_href: &str,
    math_script_href: &str,
    search_index_href: &str,
    font_options: &[FontOption],
) -> Result<String> {
    let mut environment = Environment::new();
    environment.add_template("main.html", MAIN_TEMPLATE)?;

    let template = environment.get_template("main.html")?;
    let nav_html = render_navigation_html(navigation);
    let rendered = template.render(context! {
        site_name => config.project.site_name.clone(),
        title => page.title.clone(),
        description => page.description.clone(),
        canonical_url => page.canonical_url.clone(),
        nav_html => nav_html,
        page => context! {
            title => page.title.clone(),
            content => page.html.clone(),
            toc => page.toc.clone(),
            summary => page.metadata.summary.clone(),
            tags => page.metadata.tags.clone(),
            is_home => page.is_home,
        },
        previous_page => previous_page,
        next_page => next_page,
        home_href => home_href.to_string(),
        stylesheet_href => stylesheet_href.to_string(),
        theme_boot_script => theme_boot_script(),
        theme_script_href => theme_script_href.to_string(),
        search_script_href => search_script_href.to_string(),
        code_script_href => code_script_href.to_string(),
        math_script_href => math_script_href.to_string(),
        search_index_href => search_index_href.to_string(),
        font_options => font_options.iter().collect::<Vec<_>>(),
    })?;

    Ok(rendered)
}

#[derive(Clone, Debug, Serialize)]
pub struct ArchiveGroup {
    pub title: String,
    pub pages: Vec<PageLink>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ArchiveSection {
    pub title: String,
    pub groups: Vec<ArchiveGroup>,
}

#[derive(Clone, Debug, Serialize)]
pub struct FontOption {
    pub label: String,
    pub css_value: String,
    pub source_url: Option<String>,
    pub font_family: Option<String>,
}

impl FontOption {
    pub fn builtin(label: &str, css_value: &str) -> Self {
        Self {
            label: label.to_string(),
            css_value: css_value.to_string(),
            source_url: None,
            font_family: None,
        }
    }

    pub fn provided(label: String, family: String, source_url: String) -> Self {
        Self {
            label,
            css_value: format!("\"{family}\", {DEFAULT_SANS_FONT_STACK}"),
            source_url: Some(source_url),
            font_family: Some(family),
        }
    }
}

pub fn default_font_options() -> Vec<FontOption> {
    vec![
        FontOption::builtin("Sans", DEFAULT_SANS_FONT_STACK),
        FontOption::builtin("Serif", "Georgia, \"Times New Roman\", serif"),
        FontOption::builtin("Mono", DEFAULT_MONO_FONT_STACK),
    ]
}

pub fn render_archive_index(
    config: &Config,
    sections: &[ArchiveSection],
    navigation: &[RenderNavItem],
    font_options: &[FontOption],
) -> Result<String> {
    let home_href = "..";
    let stylesheet_href = "../assets/minizensical.css";
    let theme_script_href = "../assets/minizensical-theme.js";
    let search_script_href = "../assets/minizensical-search.js";
    let search_index_href = "../search.json";

    render_simple_page(
        config,
        "Archive",
        "Browse all pages grouped by date.",
        sections,
        navigation,
        home_href,
        stylesheet_href,
        theme_script_href,
        search_script_href,
        search_index_href,
        font_options,
    )
}

pub fn render_tag_archive(
    config: &Config,
    sections: &[ArchiveSection],
    navigation: &[RenderNavItem],
    font_options: &[FontOption],
) -> Result<String> {
    let home_href = "../..";
    let stylesheet_href = "../../assets/minizensical.css";
    let theme_script_href = "../../assets/minizensical-theme.js";
    let search_script_href = "../../assets/minizensical-search.js";
    let search_index_href = "../../search.json";

    render_simple_page(
        config,
        "Tags",
        "Browse all pages grouped by tag.",
        sections,
        navigation,
        home_href,
        stylesheet_href,
        theme_script_href,
        search_script_href,
        search_index_href,
        font_options,
    )
}

pub fn render_knowledge_graph_page(
    config: &Config,
    navigation: &[RenderNavItem],
    font_options: &[FontOption],
) -> Result<String> {
    let mut environment = Environment::new();
    environment.add_template("knowledge-graph.html", KNOWLEDGE_GRAPH_TEMPLATE)?;
    let template = environment.get_template("knowledge-graph.html")?;
    let nav_html = render_navigation_html(navigation);

    let rendered = template.render(context! {
        site_name => config.project.site_name.clone(),
        title => format!("Knowledge Graph - {}", config.project.site_name),
        description => "Explore links and shared tags between Markdown documents.".to_string(),
        nav_html => nav_html,
        home_href => "..".to_string(),
        stylesheet_href => "../assets/minizensical.css".to_string(),
        theme_boot_script => theme_boot_script(),
        theme_script_href => "../assets/minizensical-theme.js".to_string(),
        search_script_href => "../assets/minizensical-search.js".to_string(),
        search_index_href => "../search.json".to_string(),
        code_script_href => "../assets/minizensical-code.js".to_string(),
        d3_script_href => "../assets/d3.min.js".to_string(),
        graph_script_href => "../assets/minizensical-graph.js".to_string(),
        graph_json_href => "../graph.json".to_string(),
        font_options => font_options.iter().collect::<Vec<_>>(),
    })?;

    Ok(rendered)
}

fn render_simple_page(
    config: &Config,
    page_title: &str,
    description: &str,
    sections: &[ArchiveSection],
    navigation: &[RenderNavItem],
    home_href: &str,
    stylesheet_href: &str,
    theme_script_href: &str,
    search_script_href: &str,
    search_index_href: &str,
    font_options: &[FontOption],
) -> Result<String> {
    let mut environment = Environment::new();
    environment.add_template("archive.html", ARCHIVE_TEMPLATE)?;
    let template = environment.get_template("archive.html")?;
    let nav_html = render_navigation_html(navigation);
    let rendered = template.render(context! {
        site_name => config.project.site_name.clone(),
        title => format!("{} - {}", page_title, config.project.site_name),
        description => description.to_string(),
        nav_html => nav_html,
        archive_title => page_title.to_string(),
        sections => sections.iter().collect::<Vec<_>>(),
        home_href => home_href.to_string(),
        stylesheet_href => stylesheet_href.to_string(),
        theme_boot_script => theme_boot_script(),
        theme_script_href => theme_script_href.to_string(),
        search_script_href => search_script_href.to_string(),
        search_index_href => search_index_href.to_string(),
        code_script_href => code_script_href_for(search_script_href),
        font_options => font_options.iter().collect::<Vec<_>>(),
    })?;
    Ok(rendered)
}

pub fn stylesheet_path() -> PathBuf {
    PathBuf::from("assets/minizensical.css")
}

pub fn stylesheet_contents(font_options: &[FontOption]) -> String {
    let mut contents = String::new();
    for option in font_options {
        let Some(source_url) = &option.source_url else {
            continue;
        };
        contents.push_str("@font-face {\n");
        contents.push_str("  font-family: \"");
        contents.push_str(&css_string(
            option
                .font_family
                .as_deref()
                .unwrap_or(option.label.as_str()),
        ));
        contents.push_str("\";\n");
        contents.push_str("  src: url(\"");
        contents.push_str(&css_string(source_url));
        contents.push_str("\") format(\"");
        contents.push_str(font_format(source_url));
        contents.push_str("\");\n");
        contents.push_str("  font-display: swap;\n");
        contents.push_str("}\n\n");
    }
    contents.push_str(STYLE_SHEET);
    contents
}

pub fn search_script_path() -> PathBuf {
    PathBuf::from("assets/minizensical-search.js")
}

pub fn search_script_contents() -> &'static str {
    SEARCH_SCRIPT
}

pub fn code_script_path() -> PathBuf {
    PathBuf::from("assets/minizensical-code.js")
}

pub fn code_script_contents() -> &'static str {
    CODE_SCRIPT
}

pub fn math_script_path() -> PathBuf {
    PathBuf::from("assets/minizensical-math.js")
}

pub fn math_script_contents() -> &'static str {
    MATH_SCRIPT
}

pub fn graph_script_path() -> PathBuf {
    PathBuf::from("assets/minizensical-graph.js")
}

pub fn graph_script_contents() -> &'static str {
    GRAPH_SCRIPT
}

pub fn d3_script_path() -> PathBuf {
    PathBuf::from("assets/d3.min.js")
}

pub fn d3_script_contents() -> &'static [u8] {
    include_bytes!("../vendor/d3/d3.min.js")
}

pub fn theme_script_path() -> PathBuf {
    PathBuf::from("assets/minizensical-theme.js")
}

pub fn theme_script_contents() -> &'static str {
    THEME_SCRIPT
}

const DEFAULT_SANS_FONT_STACK: &str =
    "\"Avenir Next\", \"IBM Plex Sans\", \"Segoe UI\", sans-serif";
const DEFAULT_MONO_FONT_STACK: &str =
    "\"IBM Plex Mono\", \"Cascadia Code\", \"SFMono-Regular\", monospace";

fn code_script_href_for(search_script_href: &str) -> String {
    search_script_href.replace("minizensical-search.js", "minizensical-code.js")
}

fn css_string(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\A ")
}

fn font_format(source_url: &str) -> &'static str {
    let lower = source_url.to_lowercase();
    if lower.ends_with(".woff2") {
        "woff2"
    } else if lower.ends_with(".woff") {
        "woff"
    } else if lower.ends_with(".ttf") {
        "truetype"
    } else if lower.ends_with(".otf") {
        "opentype"
    } else {
        "woff2"
    }
}

const MAIN_TEMPLATE: &str = r##"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{{ title }} - {{ site_name }}</title>
  {% if description %}
  <meta name="description" content="{{ description }}">
  {% endif %}
  {% if canonical_url %}
  <link rel="canonical" href="{{ canonical_url }}">
  {% endif %}
  <script>{{ theme_boot_script | safe }}</script>
  <link rel="stylesheet" href="{{ stylesheet_href }}">
</head>
<body data-search-index="{{ search_index_href }}" data-site-home="{{ home_href }}">
  <div class="ambient ambient-a"></div>
  <div class="ambient ambient-b"></div>
  <div class="shell">
    <aside class="sidebar">
      <a class="brand" href="{{ home_href }}">
        <span class="brand-mark">MZ</span>
        <span class="brand-copy">
          <strong>{{ site_name }}</strong>
          <span>Rust static docs with course-ready polish</span>
        </span>
      </a>

      <section class="search-panel">
  <label class="search-label" for="doc-search">Search docs</label>
  <input id="doc-search" class="search-input" type="search" placeholder="Search titles, headings, and content">
  <p class="search-hint">Try keywords like <code>front matter</code>, <code>search</code>, or <code>architecture</code>.</p>
  <div id="search-status" class="search-status">Search is ready as soon as the page loads.</div>
  <div id="search-results" class="search-results" hidden></div>
</section>

      <section class="theme-panel">
        <p class="theme-label">Theme</p>
        <div class="theme-toggle" data-theme-switcher>
          <button type="button" class="theme-option" data-theme-choice="light">Light</button>
          <button type="button" class="theme-option" data-theme-choice="dark">Dark</button>
          <button type="button" class="theme-option" data-theme-choice="system">System</button>
          <button type="button" class="theme-option" data-theme-choice="sepia">Sepia</button>
          <button type="button" class="theme-option" data-theme-choice="ocean">Ocean</button>
          <button type="button" class="theme-option" data-theme-choice="forest">Forest</button>
        </div>
        <p class="theme-hint">The theme choice is saved in your browser and follows system preference in <code>System</code> mode.</p>
      </section>

      <section class="font-panel">
        <p class="font-label">Font</p>
        <div class="font-toggle" data-font-switcher>
          {% for font in font_options %}
          <button type="button" class="font-option" data-font-value="{{ font.css_value }}">{{ font.label }}</button>
          {% endfor %}
        </div>
        <p class="font-hint">Fonts placed in <code>docs/assets/fonts/</code> are added to this switcher after build.</p>
      </section>

      <div class="nav-shell">
        {{ nav_html | safe }}
      </div>
    </aside>

    <main class="content">
      <article class="page" id="page-start">
        <header class="page-header">
          <p class="eyebrow">MiniZensical</p>
          {% if page.tags | length > 0 %}
          <div class="tag-list">
            {% for tag in page.tags %}
            <span class="tag-chip">{{ tag }}</span>
            {% endfor %}
          </div>
          {% endif %}
          <h1 id="page-title">{{ page.title }}</h1>
          {% if page.summary %}
          <p class="page-summary">{{ page.summary }}</p>
          {% endif %}
        </header>

        <div class="page-body">
          {{ page.content | safe }}
        </div>

        <nav class="pager">
          {% if previous_page %}
          <a class="pager-link" href="{{ previous_page.href }}">
            <span class="pager-eyebrow">Previous</span>
            <strong>{{ previous_page.title }}</strong>
          </a>
          {% else %}
          <span class="pager-placeholder"></span>
          {% endif %}

          {% if next_page %}
          <a class="pager-link align-right" href="{{ next_page.href }}">
            <span class="pager-eyebrow">Next</span>
            <strong>{{ next_page.title }}</strong>
          </a>
          {% endif %}
        </nav>
      </article>
    </main>

    <aside class="toc">
      <div class="toc-card">
        <p class="toc-label">On this page</p>
        {% if page.toc | length > 0 %}
        <ul class="toc-list">
          {% for item in page.toc %}
          <li class="toc-level-{{ item.level }}">
            <a href="{{ item.href }}">{{ item.title }}</a>
          </li>
          {% endfor %}
        </ul>
        {% else %}
        <p class="toc-empty">Add headings to generate a table of contents automatically.</p>
        {% endif %}
      </div>
    </aside>
  </div>

  <script src="{{ theme_script_href }}"></script>
  <script src="{{ code_script_href }}"></script>
  <script src="{{ math_script_href }}"></script>
  <script src="{{ search_script_href }}"></script>
</body>
</html>
"##;

const STYLE_SHEET: &str = r#"
:root {
  color-scheme: light;
  --bg: #f3f4ef;
  --panel: rgba(255, 255, 251, 0.84);
  --panel-strong: rgba(255, 255, 252, 0.92);
  --ink: #1a2a2b;
  --muted: #5b6d70;
  --accent: #0d6d68;
  --accent-soft: rgba(13, 109, 104, 0.1);
  --accent-strong: #094946;
  --warm: #c46c3b;
  --line: rgba(16, 34, 35, 0.1);
  --shadow: 0 22px 60px rgba(12, 37, 39, 0.1);
  --bg-top: #f7f7f1;
  --panel-solid: rgba(255, 255, 255, 0.92);
  --search-surface: rgba(255, 255, 255, 0.92);
  --hero-surface: linear-gradient(145deg, rgba(13, 109, 104, 0.96), rgba(8, 72, 69, 0.94)),
    linear-gradient(135deg, rgba(255, 255, 255, 0.05), transparent);
  --hero-text: #ffffff;
  --hero-subtle: rgba(255, 255, 255, 0.86);
  --ambient-a: rgba(196, 108, 59, 0.14);
  --ambient-b: rgba(13, 109, 104, 0.12);
  --code-bg: #102123;
  --code-ink: #eef5f3;
  --code-border: rgba(205, 232, 232, 0.12);
  --code-toolbar-bg: rgba(255, 255, 255, 0.05);
  --control-border: rgba(13, 109, 104, 0.16);
  --control-border-hover: rgba(13, 109, 104, 0.35);
  --focus-ring: rgba(13, 109, 104, 0.2);
  --item-border: rgba(13, 109, 104, 0.08);
  --item-border-hover: rgba(13, 109, 104, 0.24);
  --active-border: rgba(13, 109, 104, 0.38);
  --inline-code-bg: rgba(16, 33, 35, 0.07);
  --quote-bg: rgba(13, 109, 104, 0.04);
  --quote-border: rgba(13, 109, 104, 0.25);
  --token-comment: #8ea6a5;
  --token-keyword: #86d7ff;
  --token-string: #b8e986;
  --token-number: #ffd479;
  --token-function: #f5c2ff;
  --token-operator: #ffb38a;
  --token-tag: #8fdcff;
  --token-attr: #ffd479;
  --content-font-default: "Avenir Next", "IBM Plex Sans", "Segoe UI", sans-serif;
  --content-font: var(--content-font-default);
  --code-font: "IBM Plex Mono", "Cascadia Code", "SFMono-Regular", monospace;
  font-family: var(--content-font);
}

:root[data-theme="dark"] {
  color-scheme: dark;
  --bg: #0d1417;
  --panel: rgba(17, 25, 29, 0.86);
  --panel-strong: rgba(20, 29, 34, 0.94);
  --ink: #edf4f2;
  --muted: #9fb4b5;
  --accent: #79d6d0;
  --accent-soft: rgba(121, 214, 208, 0.12);
  --accent-strong: #d8fffb;
  --warm: #f2ad7d;
  --line: rgba(205, 232, 232, 0.1);
  --shadow: 0 24px 70px rgba(2, 8, 10, 0.45);
  --bg-top: #131d21;
  --panel-solid: rgba(16, 23, 27, 0.92);
  --search-surface: rgba(13, 20, 24, 0.92);
  --hero-surface: linear-gradient(145deg, rgba(8, 32, 35, 0.96), rgba(16, 76, 82, 0.92)),
    linear-gradient(135deg, rgba(255, 255, 255, 0.02), transparent);
  --hero-text: #f4fcfb;
  --hero-subtle: rgba(244, 252, 251, 0.78);
  --ambient-a: rgba(242, 173, 125, 0.12);
  --ambient-b: rgba(121, 214, 208, 0.12);
  --code-bg: #081114;
  --code-ink: #d9f7f4;
  --code-border: rgba(205, 232, 232, 0.14);
  --code-toolbar-bg: rgba(255, 255, 255, 0.04);
  --control-border: rgba(121, 214, 208, 0.22);
  --control-border-hover: rgba(121, 214, 208, 0.44);
  --focus-ring: rgba(121, 214, 208, 0.22);
  --item-border: rgba(121, 214, 208, 0.11);
  --item-border-hover: rgba(121, 214, 208, 0.28);
  --active-border: rgba(121, 214, 208, 0.42);
  --inline-code-bg: rgba(121, 214, 208, 0.1);
  --quote-bg: rgba(121, 214, 208, 0.07);
  --quote-border: rgba(121, 214, 208, 0.28);
  --token-comment: #89aaa8;
  --token-keyword: #7fd4ff;
  --token-string: #b8eb91;
  --token-number: #ffdc8a;
  --token-function: #e7bdff;
  --token-operator: #ffad82;
  --token-tag: #91dfff;
  --token-attr: #ffd67a;
}

:root[data-theme="sepia"] {
  color-scheme: light;
  --bg: #efe3cc;
  --panel: rgba(255, 249, 237, 0.88);
  --panel-strong: rgba(255, 250, 240, 0.94);
  --ink: #2d2418;
  --muted: #6f5b42;
  --accent: #8f4f24;
  --accent-soft: rgba(143, 79, 36, 0.13);
  --accent-strong: #59300f;
  --warm: #b76e2d;
  --line: rgba(73, 48, 20, 0.16);
  --shadow: 0 22px 54px rgba(80, 52, 23, 0.14);
  --bg-top: #f8ecd6;
  --panel-solid: rgba(255, 251, 242, 0.96);
  --search-surface: rgba(255, 250, 240, 0.96);
  --hero-surface: linear-gradient(145deg, rgba(111, 70, 31, 0.96), rgba(153, 87, 40, 0.9));
  --hero-text: #fff8eb;
  --hero-subtle: rgba(255, 248, 235, 0.84);
  --ambient-a: rgba(183, 110, 45, 0.18);
  --ambient-b: rgba(94, 135, 103, 0.12);
  --code-bg: #2a2118;
  --code-ink: #fff3dd;
  --code-border: rgba(255, 239, 206, 0.16);
  --code-toolbar-bg: rgba(255, 248, 235, 0.06);
  --control-border: rgba(143, 79, 36, 0.2);
  --control-border-hover: rgba(143, 79, 36, 0.42);
  --focus-ring: rgba(143, 79, 36, 0.24);
  --item-border: rgba(143, 79, 36, 0.12);
  --item-border-hover: rgba(143, 79, 36, 0.28);
  --active-border: rgba(143, 79, 36, 0.44);
  --inline-code-bg: rgba(143, 79, 36, 0.11);
  --quote-bg: rgba(143, 79, 36, 0.07);
  --quote-border: rgba(143, 79, 36, 0.3);
  --token-comment: #b59b77;
  --token-keyword: #7dc7e6;
  --token-string: #c6e889;
  --token-number: #ffd27a;
  --token-function: #eec7ff;
  --token-operator: #f1a06e;
  --token-tag: #88d6ed;
  --token-attr: #ffd27a;
}

:root[data-theme="ocean"] {
  color-scheme: dark;
  --bg: #071820;
  --panel: rgba(9, 31, 42, 0.9);
  --panel-strong: rgba(10, 39, 52, 0.96);
  --ink: #e5f7fb;
  --muted: #9ac1ca;
  --accent: #63d7ff;
  --accent-soft: rgba(99, 215, 255, 0.13);
  --accent-strong: #cef5ff;
  --warm: #8de0c4;
  --line: rgba(178, 231, 244, 0.14);
  --shadow: 0 26px 70px rgba(0, 8, 14, 0.5);
  --bg-top: #0c2430;
  --panel-solid: rgba(8, 27, 37, 0.96);
  --search-surface: rgba(8, 31, 43, 0.96);
  --hero-surface: linear-gradient(145deg, rgba(5, 61, 83, 0.96), rgba(6, 95, 120, 0.9));
  --hero-text: #effcff;
  --hero-subtle: rgba(239, 252, 255, 0.8);
  --ambient-a: rgba(99, 215, 255, 0.13);
  --ambient-b: rgba(141, 224, 196, 0.13);
  --code-bg: #041016;
  --code-ink: #e3fbff;
  --code-border: rgba(178, 231, 244, 0.16);
  --code-toolbar-bg: rgba(255, 255, 255, 0.04);
  --control-border: rgba(99, 215, 255, 0.24);
  --control-border-hover: rgba(99, 215, 255, 0.48);
  --focus-ring: rgba(99, 215, 255, 0.24);
  --item-border: rgba(99, 215, 255, 0.12);
  --item-border-hover: rgba(99, 215, 255, 0.3);
  --active-border: rgba(99, 215, 255, 0.46);
  --inline-code-bg: rgba(99, 215, 255, 0.1);
  --quote-bg: rgba(99, 215, 255, 0.07);
  --quote-border: rgba(99, 215, 255, 0.3);
}

:root[data-theme="forest"] {
  color-scheme: dark;
  --bg: #0f180f;
  --panel: rgba(20, 32, 20, 0.9);
  --panel-strong: rgba(26, 39, 24, 0.96);
  --ink: #eef7e8;
  --muted: #adc2a4;
  --accent: #8fd16a;
  --accent-soft: rgba(143, 209, 106, 0.14);
  --accent-strong: #ddffd0;
  --warm: #d9b56f;
  --line: rgba(222, 246, 210, 0.13);
  --shadow: 0 26px 70px rgba(3, 10, 4, 0.52);
  --bg-top: #182317;
  --panel-solid: rgba(17, 28, 17, 0.96);
  --search-surface: rgba(21, 35, 21, 0.96);
  --hero-surface: linear-gradient(145deg, rgba(37, 77, 39, 0.96), rgba(75, 104, 41, 0.9));
  --hero-text: #fbfff6;
  --hero-subtle: rgba(251, 255, 246, 0.8);
  --ambient-a: rgba(143, 209, 106, 0.13);
  --ambient-b: rgba(217, 181, 111, 0.12);
  --code-bg: #081108;
  --code-ink: #effbe8;
  --code-border: rgba(222, 246, 210, 0.16);
  --code-toolbar-bg: rgba(255, 255, 255, 0.04);
  --control-border: rgba(143, 209, 106, 0.24);
  --control-border-hover: rgba(143, 209, 106, 0.48);
  --focus-ring: rgba(143, 209, 106, 0.24);
  --item-border: rgba(143, 209, 106, 0.12);
  --item-border-hover: rgba(143, 209, 106, 0.3);
  --active-border: rgba(143, 209, 106, 0.46);
  --inline-code-bg: rgba(143, 209, 106, 0.1);
  --quote-bg: rgba(143, 209, 106, 0.07);
  --quote-border: rgba(143, 209, 106, 0.3);
}

* {
  box-sizing: border-box;
}

html {
  scroll-behavior: smooth;
}

body {
  margin: 0;
  min-height: 100vh;
  color: var(--ink);
  background:
    radial-gradient(circle at 0% 0%, var(--ambient-a), transparent 28%),
    radial-gradient(circle at 100% 20%, var(--ambient-b), transparent 30%),
    linear-gradient(180deg, var(--bg-top) 0%, var(--bg) 100%);
  position: relative;
}

a {
  color: inherit;
}

code,
pre {
  font-family: var(--code-font);
}

.ambient {
  position: fixed;
  inset: auto;
  pointer-events: none;
  z-index: 0;
  filter: blur(18px);
}

.ambient-a {
  top: 80px;
  left: 2vw;
  width: 220px;
  height: 220px;
  border-radius: 999px;
  background: var(--ambient-a);
}

.ambient-b {
  right: 4vw;
  bottom: 60px;
  width: 260px;
  height: 260px;
  border-radius: 999px;
  background: var(--ambient-b);
}

.shell {
  position: relative;
  z-index: 1;
  display: grid;
  grid-template-columns: minmax(260px, 320px) minmax(0, 1fr) minmax(210px, 260px);
  gap: 24px;
  min-height: 100vh;
  padding: 24px;
}

.graph-view .shell {
  grid-template-columns: minmax(260px, 320px) minmax(0, 1fr);
}

.sidebar,
.page,
.toc-card,
.hero-card,
.search-results,
.search-panel {
  border: 1px solid var(--line);
  border-radius: 24px;
  background: var(--panel);
  backdrop-filter: blur(16px);
  box-shadow: var(--shadow);
}

.sidebar {
  position: sticky;
  top: 24px;
  align-self: start;
  padding: 20px;
  display: grid;
  gap: 18px;
}

.brand {
  display: grid;
  grid-template-columns: auto 1fr;
  gap: 14px;
  align-items: center;
  text-decoration: none;
}

.brand-mark {
  width: 50px;
  height: 50px;
  border-radius: 16px;
  display: inline-grid;
  place-items: center;
  background: linear-gradient(135deg, var(--accent), var(--warm));
  color: white;
  font-weight: 800;
  letter-spacing: 0.08em;
}

.brand-copy {
  display: grid;
  gap: 2px;
}

.brand-copy strong {
  font-size: 1rem;
}

.brand-copy span {
  color: var(--muted);
  font-size: 0.88rem;
  line-height: 1.4;
}

.search-panel {
  padding: 18px;
}

.theme-panel,
.font-panel {
  padding: 18px;
}

.search-label {
  display: block;
  margin-bottom: 10px;
  font-size: 0.86rem;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--accent-strong);
}

.theme-label,
.font-label {
  margin: 0 0 10px;
  font-size: 0.86rem;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--accent-strong);
}

.search-input {
  width: 100%;
  border: 1px solid var(--control-border);
  border-radius: 16px;
  padding: 13px 14px;
  font: inherit;
  background: var(--panel-solid);
  color: var(--ink);
}

.search-input:focus {
  outline: 2px solid var(--focus-ring);
  border-color: var(--control-border-hover);
}

.search-hint,
.search-status {
  margin: 10px 0 0;
  color: var(--muted);
  font-size: 0.9rem;
  line-height: 1.5;
}

.search-results {
  margin-top: 14px;
  padding: 10px;
  display: grid;
  gap: 10px;
  max-height: 380px;
  overflow-y: auto;
  background: var(--panel-strong);
}

.search-result-group {
  display: grid;
  gap: 8px;
  padding: 14px;
  border-radius: 18px;
  border: 1px solid var(--item-border);
  background: var(--search-surface);
}

.search-result-title {
  margin: 0;
  font-size: 1rem;
  line-height: 1.35;
}

.search-match-list {
  list-style: none;
  margin: 0;
  padding: 0;
  display: grid;
  gap: 8px;
}

.search-match {
  display: block;
  padding: 10px 12px;
  text-decoration: none;
  border-radius: 14px;
  border: 1px solid var(--item-border);
  background: var(--panel-solid);
  color: var(--muted);
  line-height: 1.55;
  transition: transform 160ms ease, border-color 160ms ease, box-shadow 160ms ease, color 160ms ease;
}

.search-match:hover {
  transform: translateY(-1px);
  border-color: var(--item-border-hover);
  box-shadow: 0 12px 25px rgba(12, 37, 39, 0.08);
  color: var(--ink);
}

.search-match.is-active {
  border-color: var(--active-border);
  background: var(--accent-soft);
  color: var(--accent-strong);
  box-shadow: inset 3px 0 0 var(--accent);
}

.search-empty {
  margin: 0;
  color: var(--muted);
  line-height: 1.5;
}

.search-target-active {
  outline: 3px solid rgba(255, 212, 80, 0.72);
  outline-offset: 5px;
  border-radius: 12px;
  background: rgba(255, 238, 153, 0.24);
  transition: background 180ms ease, outline-color 180ms ease;
}

mark {
  background: #ffe66d;
  color: inherit;
  border-radius: 4px;
  padding: 0 3px;
}

.theme-toggle,
.font-toggle {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.theme-option,
.font-option {
  border: 1px solid var(--control-border);
  background: var(--panel-solid);
  color: var(--ink);
  border-radius: 999px;
  padding: 9px 14px;
  font: inherit;
  font-size: 0.92rem;
  cursor: pointer;
  transition: transform 160ms ease, border-color 160ms ease, background 160ms ease, color 160ms ease;
}

.theme-option:hover,
.font-option:hover {
  transform: translateY(-1px);
  border-color: var(--control-border-hover);
}

.theme-option.is-active,
.font-option.is-active {
  background: var(--accent);
  color: var(--hero-text);
  border-color: transparent;
}

.theme-hint,
.font-hint {
  margin: 10px 0 0;
  color: var(--muted);
  font-size: 0.9rem;
  line-height: 1.55;
}

.nav-shell {
  min-width: 0;
}

.nav-list {
  list-style: none;
  margin: 0;
  padding: 0;
}

.nav-item {
  margin: 8px 0;
}

.nav-item > a,
.nav-section {
  display: block;
  padding: 9px 12px;
  text-decoration: none;
  border-radius: 14px;
  color: var(--muted);
  transition: background 160ms ease, color 160ms ease, transform 160ms ease;
}

.nav-item > a:hover {
  background: rgba(13, 109, 104, 0.06);
  color: var(--accent-strong);
  transform: translateX(2px);
}

.nav-item.active > a,
.nav-item.active > .nav-section {
  background: var(--accent-soft);
  color: var(--accent-strong);
  font-weight: 700;
}

.nav-item .nav-list {
  margin-left: 14px;
  padding-left: 10px;
  border-left: 1px solid var(--line);
}

.content {
  display: grid;
  gap: 24px;
}

.tag-chip {
  display: inline-flex;
  align-items: center;
  padding: 6px 10px;
  border-radius: 999px;
  font-size: 0.75rem;
  font-weight: 700;
  letter-spacing: 0.04em;
  background: var(--accent-soft);
  color: var(--accent-strong);
}

.page {
  padding: 34px;
}

.page-header {
  padding-bottom: 24px;
  border-bottom: 1px solid var(--line);
}

.eyebrow {
  margin: 0 0 10px;
  font-size: 0.76rem;
  letter-spacing: 0.16em;
  text-transform: uppercase;
  color: var(--warm);
  font-weight: 800;
}

.tag-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-bottom: 14px;
}

.page-header h1 {
  margin: 0;
  font-size: clamp(2rem, 3.2vw, 3.2rem);
  line-height: 1.05;
}

.page-summary {
  margin: 16px 0 0;
  max-width: 60ch;
  font-size: 1.04rem;
  line-height: 1.75;
  color: var(--muted);
}

.page-body {
  margin-top: 28px;
  line-height: 1.82;
  font-size: 1rem;
}

.page-body h1,
.page-body h2,
.page-body h3,
.page-body h4 {
  line-height: 1.18;
}

.page-body [id] {
  scroll-margin-top: 28px;
}

.page-body h2,
.page-body h3 {
  margin-top: 1.8em;
}

.page-body p,
.page-body ul,
.page-body ol,
.page-body table,
.page-body blockquote,
.page-body pre {
  margin: 1.1em 0;
}

.page-body pre {
  padding: 18px;
  overflow-x: auto;
  border-radius: 20px;
  background: var(--code-bg);
  color: var(--code-ink);
}

.page-body .code-block {
  margin: 1.1em 0;
  overflow: hidden;
  border: 1px solid var(--code-border);
  border-radius: 18px;
  background: var(--code-bg);
  color: var(--code-ink);
  box-shadow: 0 16px 34px rgba(7, 20, 22, 0.16);
}

.page-body .code-block pre {
  margin: 0;
  padding: 18px;
  border-radius: 0;
  background: transparent;
}

.code-toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 10px 12px;
  border-bottom: 1px solid var(--code-border);
  background: var(--code-toolbar-bg);
}

.code-language {
  color: rgba(238, 245, 243, 0.72);
  font-size: 0.78rem;
  font-weight: 700;
  text-transform: uppercase;
}

.copy-code-button {
  border: 1px solid rgba(238, 245, 243, 0.18);
  border-radius: 999px;
  padding: 6px 11px;
  background: rgba(255, 255, 255, 0.08);
  color: var(--code-ink);
  font: inherit;
  font-size: 0.82rem;
  cursor: pointer;
  transition: background 160ms ease, border-color 160ms ease, transform 160ms ease;
}

.copy-code-button:hover {
  transform: translateY(-1px);
  border-color: rgba(238, 245, 243, 0.34);
  background: rgba(255, 255, 255, 0.13);
}

.copy-code-button.is-copied {
  border-color: rgba(121, 214, 208, 0.5);
  background: rgba(121, 214, 208, 0.16);
}

.token-comment {
  color: var(--token-comment);
  font-style: italic;
}

.token-keyword {
  color: var(--token-keyword);
}

.token-string {
  color: var(--token-string);
}

.token-number {
  color: var(--token-number);
}

.token-function {
  color: var(--token-function);
}

.token-operator {
  color: var(--token-operator);
}

.token-tag {
  color: var(--token-tag);
}

.token-attr {
  color: var(--token-attr);
}

.page-body :not(pre) > code {
  padding: 0.18em 0.48em;
  border-radius: 8px;
  background: var(--inline-code-bg);
}

.page-body blockquote {
  margin-left: 0;
  padding: 14px 18px;
  border-left: 4px solid var(--quote-border);
  border-radius: 0 16px 16px 0;
  background: var(--quote-bg);
  color: var(--muted);
}

.page-body table {
  width: 100%;
  border-collapse: collapse;
  overflow: hidden;
  border-radius: 18px;
}

.page-body th,
.page-body td {
  border: 1px solid var(--line);
  padding: 11px 12px;
  text-align: left;
}

.page-body th {
  background: rgba(13, 109, 104, 0.06);
}

.page-body img {
  display: block;
  max-width: 100%;
  border-radius: 22px;
  box-shadow: 0 18px 35px rgba(12, 37, 39, 0.12);
}

.math {
  font-family: var(--content-font);
}

.math-inline {
  white-space: nowrap;
}

.math-display {
  display: block;
  max-width: 100%;
  margin: 1.1em 0;
  overflow-x: auto;
  overflow-y: hidden;
  text-align: center;
}

.math mjx-container {
  margin: 0 !important;
}

.math-inline mjx-container {
  text-align: center !important;
}

.pager {
  display: flex;
  justify-content: space-between;
  gap: 14px;
  margin-top: 36px;
  padding-top: 24px;
  border-top: 1px solid var(--line);
}

.pager-placeholder {
  flex: 1;
}

.pager-link {
  flex: 1;
  display: grid;
  gap: 6px;
  padding: 16px 18px;
  border-radius: 20px;
  text-decoration: none;
  background: rgba(13, 109, 104, 0.04);
  border: 1px solid rgba(13, 109, 104, 0.08);
}

.pager-link.align-right {
  text-align: right;
}

.pager-eyebrow {
  font-size: 0.78rem;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--muted);
}

.toc {
  position: sticky;
  top: 24px;
  align-self: start;
}

.toc-card {
  padding: 20px;
}

.toc-label {
  margin: 0 0 14px;
  font-size: 0.78rem;
  text-transform: uppercase;
  letter-spacing: 0.12em;
  font-weight: 800;
  color: var(--accent-strong);
}

.toc-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.toc-list li {
  margin: 8px 0;
}

.toc-list a {
  text-decoration: none;
  color: var(--muted);
}

.toc-level-2 {
  padding-left: 10px;
}

.toc-level-3,
.toc-level-4,
.toc-level-5,
.toc-level-6 {
  padding-left: 18px;
}

.toc-empty {
  color: var(--muted);
  line-height: 1.6;
  margin-bottom: 0;
}

.graph-page .page-body {
  min-width: 0;
}

.graph-controls {
  position: absolute;
  z-index: 4;
  top: 14px;
  right: 14px;
  width: min(300px, calc(100% - 28px));
  border: 1px solid var(--control-border);
  border-radius: 8px;
  background: color-mix(in srgb, var(--panel-solid) 94%, transparent);
  box-shadow: 0 12px 30px rgba(18, 41, 39, 0.14);
  backdrop-filter: blur(12px);
  overflow: hidden;
}

.graph-controls-summary {
  display: flex;
  align-items: center;
  justify-content: space-between;
  min-height: 44px;
  padding: 0 14px;
  color: var(--accent-strong);
  font-size: 0.82rem;
  font-weight: 800;
  cursor: pointer;
  list-style: none;
}

.graph-controls-summary::-webkit-details-marker {
  display: none;
}

.graph-controls-summary::after {
  content: "+";
  font-size: 1.15rem;
  font-weight: 500;
}

.graph-controls[open] .graph-controls-summary::after {
  content: "-";
}

.graph-controls-body {
  display: grid;
  gap: 13px;
  padding: 2px 14px 14px;
  border-top: 1px solid var(--line);
}

.graph-control {
  display: grid;
  gap: 6px;
  color: var(--muted);
  font-size: 0.78rem;
  font-weight: 700;
}

.graph-control-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.graph-input {
  width: 100%;
  border: 1px solid var(--control-border);
  border-radius: 7px;
  padding: 9px 10px;
  font: inherit;
  background: var(--panel-solid);
  color: var(--ink);
}

.graph-input:focus {
  outline: 2px solid var(--focus-ring);
  border-color: var(--control-border-hover);
}

.graph-toggle {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  color: var(--ink);
  cursor: pointer;
}

.graph-toggle input {
  width: 34px;
  height: 18px;
  accent-color: var(--accent);
}

.graph-control input[type="range"] {
  width: 100%;
  accent-color: var(--accent);
}

.graph-control output {
  color: var(--accent-strong);
  font-variant-numeric: tabular-nums;
}

.graph-reset {
  min-height: 36px;
  border: 1px solid var(--control-border);
  border-radius: 7px;
  background: var(--panel-solid);
  color: var(--accent-strong);
  font: inherit;
  font-weight: 800;
  cursor: pointer;
}

.graph-reset:hover {
  border-color: var(--control-border-hover);
  background: var(--accent-soft);
}

.graph-stage {
  position: relative;
  width: 100%;
  min-height: 620px;
  border: 1px solid var(--line);
  border-radius: 8px;
  background: var(--panel-strong);
  overflow: hidden;
  touch-action: none;
}

.graph-svg {
  display: block;
  width: 100%;
  height: 620px;
  cursor: grab;
  user-select: none;
}

.graph-svg:active {
  cursor: grabbing;
}

.graph-edge {
  stroke: var(--line);
  stroke-width: 1.2;
  opacity: 0.5;
  transition: opacity 140ms ease, stroke-width 140ms ease;
}

.graph-edge.links_to {
  stroke: var(--accent);
  marker-end: url(#graph-arrow);
}

.graph-edge.has_tag {
  stroke: var(--warm);
}

.graph-edge.shared_tag {
  stroke-dasharray: 5 5;
}

.graph-arrow {
  fill: var(--accent);
}

.graph-node {
  cursor: pointer;
  outline: none;
}

.graph-node circle {
  stroke: color-mix(in srgb, var(--panel-solid) 75%, var(--ink));
  stroke-width: 1.5;
  transition: opacity 140ms ease, stroke 140ms ease, stroke-width 140ms ease;
}

.graph-node.is-focused circle,
.graph-node:focus circle,
.graph-node.is-search-match circle {
  stroke: var(--accent);
  stroke-width: 3;
}

.graph-node.document circle {
  fill: var(--accent);
}

.graph-node.tag circle {
  fill: var(--warm);
}

.graph-label {
  fill: var(--ink);
  font-size: 11px;
  font-weight: 700;
  paint-order: stroke;
  stroke: var(--panel-solid);
  stroke-width: 3.5px;
  stroke-linejoin: round;
  pointer-events: none;
  opacity: 0;
  transition: opacity 140ms ease;
}

.graph-node.is-focused .graph-label,
.graph-node.is-neighbor .graph-label,
.graph-node.is-prominent .graph-label,
.graph-node.is-search-match .graph-label,
.graph-svg.show-all-labels .graph-label {
  opacity: 1;
}

.graph-node.is-dimmed,
.graph-edge.is-dimmed {
  opacity: 0.12;
}

.graph-edge.is-active {
  opacity: 0.95;
  stroke-width: 2.2;
}

.graph-node.is-search-dimmed,
.graph-edge.is-search-dimmed {
  opacity: 0.22;
}

.graph-empty {
  fill: var(--muted);
  font-size: 14px;
}

.graph-status {
  position: absolute;
  z-index: 3;
  left: 14px;
  bottom: 12px;
  margin: 0;
  padding: 5px 8px;
  border-radius: 6px;
  background: color-mix(in srgb, var(--panel-solid) 88%, transparent);
  color: var(--muted);
  font-size: 0.78rem;
  pointer-events: none;
}

@media (min-width: 1181px) {
  html,
  body {
    height: 100%;
    overflow: hidden;
  }

  .shell {
    height: 100dvh;
    min-height: 0;
    overflow: hidden;
  }

  .sidebar,
  .content,
  .toc {
    min-height: 0;
    max-height: calc(100dvh - 48px);
    overflow-y: auto;
    overscroll-behavior: contain;
    scrollbar-gutter: stable;
    scrollbar-width: thin;
    scrollbar-color: rgba(13, 109, 104, 0.34) transparent;
  }

  .sidebar,
  .toc {
    position: static;
    align-self: stretch;
  }

  .sidebar,
  .content {
    padding-right: 18px;
  }

  .toc {
    padding-right: 10px;
  }

  .sidebar::-webkit-scrollbar,
  .content::-webkit-scrollbar,
  .toc::-webkit-scrollbar {
    width: 8px;
  }

  .sidebar::-webkit-scrollbar-track,
  .content::-webkit-scrollbar-track,
  .toc::-webkit-scrollbar-track {
    background: transparent;
  }

  .sidebar::-webkit-scrollbar-thumb,
  .content::-webkit-scrollbar-thumb,
  .toc::-webkit-scrollbar-thumb {
    background: rgba(13, 109, 104, 0.28);
    border-radius: 999px;
  }

  .sidebar::-webkit-scrollbar-thumb:hover,
  .content::-webkit-scrollbar-thumb:hover,
  .toc::-webkit-scrollbar-thumb:hover {
    background: rgba(13, 109, 104, 0.42);
  }

  .content {
    align-self: stretch;
  }
}

@media (max-width: 1180px) {
  .shell {
    grid-template-columns: minmax(240px, 300px) minmax(0, 1fr);
  }

  .toc {
    display: none;
  }
}

@media (max-width: 820px) {
  .shell,
  .graph-view .shell {
    grid-template-columns: 1fr;
    padding: 18px;
  }

  .sidebar,
  .toc {
    position: static;
  }

  .page {
    padding: 24px;
  }

  .pager {
    flex-direction: column;
  }

  .pager-link.align-right {
    text-align: left;
  }

  .graph-stage {
    min-height: 520px;
  }

  .graph-svg {
    height: 520px;
  }

  .graph-controls {
    top: 10px;
    right: 10px;
    width: min(280px, calc(100% - 20px));
  }
}

	.archive-section {
	  margin-bottom: 2.8em;
	}

	.archive-section h2 {
	  font-size: 1.6rem;
	  margin: 0 0 1em;
	  color: var(--accent-strong);
	  border-bottom: 2px solid var(--line);
	  padding-bottom: 0.4em;
	}

	.archive-group {
	  margin: 1.2em 0 1.6em;
	}

	.archive-group-title {
	  font-size: 1.1rem;
	  margin: 0 0 0.6em;
	  color: var(--warm);
	}

	.archive-list {
	  list-style: none;
	  padding: 0;
	  margin: 0;
	}

	.archive-list li {
	  margin: 0.5em 0;
	  padding: 0;
	}

	.archive-list a {
	  text-decoration: none;
	  color: var(--accent);
	  font-size: 1rem;
	  transition: color 160ms ease, padding-left 160ms ease;
	}

	.archive-list a:hover {
	  color: var(--accent-strong);
	  padding-left: 4px;
	}

"#;

const THEME_SCRIPT: &str = r#"
(() => {
  const storageKey = "minizensical-theme-choice";
  const root = document.documentElement;
  const buttons = Array.from(document.querySelectorAll("button[data-theme-choice]"));
  const media = window.matchMedia("(prefers-color-scheme: dark)");
  const themeChoices = new Set(["light", "dark", "system", "sepia", "ocean", "forest"]);

  const normalizeChoice = (value) => {
    if (themeChoices.has(value)) {
      return value;
    }
    return "system";
  };

  const resolveTheme = (choice) => {
    if (choice === "system") {
      return media.matches ? "dark" : "light";
    }
    return choice;
  };

  const applyChoice = (rawChoice) => {
    const choice = normalizeChoice(rawChoice);
    const theme = resolveTheme(choice);
    root.dataset.themeChoice = choice;
    root.dataset.theme = theme;

    buttons.forEach((button) => {
      const active = button.dataset.themeChoice === choice;
      button.classList.toggle("is-active", active);
      button.setAttribute("aria-pressed", active ? "true" : "false");
    });
  };

  const readStoredChoice = () => {
    try {
      return normalizeChoice(window.localStorage.getItem(storageKey));
    } catch (_error) {
      return normalizeChoice(root.dataset.themeChoice);
    }
  };

  const persistChoice = (choice) => {
    try {
      if (choice === "system") {
        window.localStorage.removeItem(storageKey);
      } else {
        window.localStorage.setItem(storageKey, choice);
      }
    } catch (_error) {
      // Ignore persistence errors in restricted browsers.
    }
  };

  buttons.forEach((button) => {
    button.addEventListener("click", () => {
      const choice = normalizeChoice(button.dataset.themeChoice);
      persistChoice(choice);
      applyChoice(choice);
    });
  });

  const initialChoice = readStoredChoice();
  applyChoice(initialChoice);

  if (typeof media.addEventListener === "function") {
    media.addEventListener("change", () => {
      if (readStoredChoice() === "system") {
        applyChoice("system");
      }
    });
  }
})();
"#;

fn theme_boot_script() -> &'static str {
    r#"
(() => {
  const storageKey = "minizensical-theme-choice";
  const root = document.documentElement;
  const media = window.matchMedia("(prefers-color-scheme: dark)");
  const themeChoices = new Set(["light", "dark", "system", "sepia", "ocean", "forest"]);

  let choice = "system";
  try {
    const stored = window.localStorage.getItem(storageKey);
    if (themeChoices.has(stored)) {
      choice = stored;
    }
  } catch (_error) {
    // Ignore storage access failures and fall back to system mode.
  }

  const theme = choice === "system" ? (media.matches ? "dark" : "light") : choice;
  root.dataset.themeChoice = choice;
  root.dataset.theme = theme;
})();
"#
}

const SEARCH_SCRIPT: &str = r##"
(() => {
  const input = document.getElementById("doc-search");
  const results = document.getElementById("search-results");
  const status = document.getElementById("search-status");

  if (!input || !results || !status) {
    return;
  }

  const { searchIndex: searchIndexHref = "", siteHome: siteHomeHref = "" } = document.body.dataset;
  const indexUrl = new URL(searchIndexHref, window.location.href);
  const siteRootUrl = new URL(siteHomeHref || ".", window.location.href);
  let entries = [];
  let bodyClearBound = false;

  const scrollTargetIntoView = (target, block = "start") => {
    if (!target) {
      return;
    }
    target.scrollIntoView({ block, behavior: "smooth" });
  };

  const decodeHashValue = (value) => {
    if (!value) {
      return "";
    }
    try {
      return decodeURIComponent(value);
    } catch (_error) {
      return value;
    }
  };

  const decodeHashId = () => decodeHashValue(window.location.hash.replace(/^#/, ""));

  const normalizeContentPath = (pathname) => {
    let path = pathname || "/";
    if (path.endsWith("/index.html")) {
      path = path.slice(0, -"/index.html".length);
    }
    if (path.length > 1 && path.endsWith("/")) {
      path = path.slice(0, -1);
    }
    return path || "/";
  };

  const sameDocumentUrl = (url) =>
    url.origin === window.location.origin
    && normalizeContentPath(url.pathname) === normalizeContentPath(window.location.pathname);

  const searchLocationKey = (url) => {
    const query = url.searchParams.get("mz-search") || "";
    const targetId = decodeHashValue(url.hash.replace(/^#/, ""));
    if (!query || !targetId) {
      return "";
    }
    return [
      url.origin,
      normalizeContentPath(url.pathname),
      query,
      targetId
    ].join("|");
  };

  const syncActiveSearchMatch = () => {
    const currentKey = searchLocationKey(new URL(window.location.href));
    document.querySelectorAll(".search-match").forEach((anchor) => {
      const matchKey = searchLocationKey(new URL(anchor.href, window.location.href));
      const active = Boolean(currentKey && matchKey === currentKey);
      anchor.classList.toggle("is-active", active);
      if (active) {
        anchor.setAttribute("aria-current", "true");
      } else {
        anchor.removeAttribute("aria-current");
      }
    });
  };

  const targetFromHash = () => {
    const targetId = decodeHashId();
    return targetId ? document.getElementById(targetId) : null;
  };

  const setupInPageAnchorScroll = () => {
    document.querySelectorAll('a[href^="#"]').forEach((anchor) => {
      anchor.addEventListener("click", (event) => {
        const href = anchor.getAttribute("href") || "";
        if (href === "#") {
          return;
        }

        let targetId = "";
        try {
          targetId = decodeURIComponent(href.slice(1));
        } catch (_error) {
          targetId = href.slice(1);
        }

        const target = document.getElementById(targetId);
        if (!target) {
          return;
        }

        event.preventDefault();
        window.history.pushState(null, "", "#" + encodeURIComponent(targetId));
        scrollTargetIntoView(target);
      });
    });
  };

  const revealInitialHashTarget = () => {
    const url = new URL(window.location.href);
    if (url.searchParams.has("mz-search")) {
      return;
    }

    const target = targetFromHash();
    if (target) {
      window.setTimeout(() => scrollTargetIntoView(target), 30);
    }
  };

  const escapeHtml = (value) =>
    String(value || "").replace(/[&<>\"']/g, (character) => ({
      "&": "&amp;",
      "<": "&lt;",
      ">": "&gt;",
      "\"": "&quot;",
      "'": "&#39;"
    })[character]);

  const escapeRegExp = (value) => value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");

  const termsForQuery = (query) =>
    Array.from(new Set(query.trim().toLowerCase().split(/\s+/).filter(Boolean)));

  const highlightText = (text, terms) => {
    const source = String(text || "");
    if (!terms.length) {
      return escapeHtml(source);
    }
    const pattern = new RegExp("(" + terms.map(escapeRegExp).join("|") + ")", "gi");
    let html = "";
    let lastIndex = 0;

    source.replace(pattern, (match, _term, offset) => {
      html += escapeHtml(source.slice(lastIndex, offset));
      html += "<mark>" + escapeHtml(match) + "</mark>";
      lastIndex = offset + match.length;
      return match;
    });

    html += escapeHtml(source.slice(lastIndex));
    return html;
  };

  const firstMatchIndex = (text, terms) => {
    const lowerText = String(text || "").toLowerCase();
    let firstIndex = -1;

    for (const term of terms) {
      const index = lowerText.indexOf(term);
      if (index >= 0 && (firstIndex < 0 || index < firstIndex)) {
        firstIndex = index;
      }
    }
    return firstIndex;
  };

  const makeSnippet = (text, terms) => {
    const source = String(text || "");
    const maxLength = 180;
    const matchIndex = firstMatchIndex(source, terms);

    if (source.length <= maxLength) {
      return source;
    }

    if (matchIndex < 0) {
      return source.slice(0, maxLength).trimEnd() + "...";
    }

    const start = Math.max(0, matchIndex - 70);
    const end = Math.min(source.length, start + maxLength);
    return (start > 0 ? "..." : "") + source.slice(start, end).trim() + (end < source.length ? "..." : "");
  };

  const blockMatches = (block, terms) => {
    const text = String(block.text || "").toLowerCase();
    return terms.every((term) => text.includes(term));
  };

  const blockScore = (block, terms) => {
    const text = String(block.text || "").toLowerCase();
    const weight = block.kind === "title" ? 8 : block.kind === "heading" ? 5 : 1;
    let occurrences = 0;

    for (const term of terms) {
      let index = text.indexOf(term);
      while (index >= 0) {
        occurrences += 1;
        index = text.indexOf(term, index + term.length);
      }
    }

    return weight + occurrences;
  };

  const buildMatchUrl = (entryUrl, blockId, query) => {
    const url = new URL(entryUrl || "", siteRootUrl);
    url.searchParams.set("mz-search", query);
    url.hash = blockId ? encodeURIComponent(blockId) : "";
    return url.toString();
  };

  const renderResults = (query, groups) => {
    if (!query) {
      results.hidden = true;
      results.innerHTML = "";
      status.textContent = "Search titles, headings, and body text from every page.";
      return;
    }

    results.hidden = false;

    if (groups.length === 0) {
      results.innerHTML = "<div class=\"search-result-group\"><p class=\"search-empty\">No matching sections. Try a shorter keyword.</p></div>";
      status.textContent = "No pages matched " + query + ".";
      return;
    }

    const terms = termsForQuery(query);
    const totalMatches = groups.reduce((total, group) => total + group.matches.length, 0);
    const html = groups.map(({ entry, matches }) => {
      const matchHtml = matches.slice(0, 6).map((block) => {
        const snippet = makeSnippet(block.text, terms);
        const href = buildMatchUrl(entry.url, block.id, query);
        return (
          '<li><a class="search-match" href="' + escapeHtml(href) + '">' +
            highlightText(snippet, terms) +
          '</a></li>'
        );
      }).join("");

      return (
        '<section class="search-result-group">' +
          '<h3 class="search-result-title">' + escapeHtml(entry.title) + '</h3>' +
          '<ul class="search-match-list">' + matchHtml + '</ul>' +
        '</section>'
      );
    }).join("");

    results.innerHTML = html;
    syncActiveSearchMatch();
    status.textContent = totalMatches + " match(es) in " + groups.length + " page(s) for " + query + ".";
  };

  const search = (query) => {
    const normalized = query.trim().toLowerCase();
    if (!normalized) {
      renderResults("", []);
      return;
    }

    const terms = termsForQuery(normalized);
    const groups = entries
      .map((entry) => {
        const matches = (entry.blocks || [])
          .filter((block) => block.id && block.text && blockMatches(block, terms))
          .map((block) => ({ block, score: blockScore(block, terms) }));
        const score = matches.reduce((total, match) => total + match.score, 0);
        return { entry, matches, score };
      })
      .filter((group) => group.matches.length > 0)
      .sort((a, b) => b.score - a.score || a.entry.title.localeCompare(b.entry.title))
      .slice(0, 8)
      .map((group) => ({
        entry: group.entry,
        matches: group.matches.sort((a, b) => b.score - a.score).map((match) => match.block)
      }));

    renderResults(normalized, groups);
  };

  const removeTargetMarks = () => {
    document.querySelectorAll("mark[data-search-target-mark]").forEach((mark) => {
      const parent = mark.parentNode;
      if (!parent) {
        return;
      }
      parent.replaceChild(document.createTextNode(mark.textContent || ""), mark);
      parent.normalize();
    });
  };

  const clearRenderedTargetHighlight = () => {
    document.querySelectorAll(".search-target-active").forEach((element) => {
      element.classList.remove("search-target-active");
    });
    removeTargetMarks();
  };

  const clearTargetHighlight = () => {
    clearRenderedTargetHighlight();

    const url = new URL(window.location.href);
    if (url.searchParams.has("mz-search")) {
      url.searchParams.delete("mz-search");
      window.history.replaceState(null, "", url.pathname + url.search + url.hash);
    }
    syncActiveSearchMatch();
  };

  const highlightTargetText = (target, terms) => {
    if (!terms.length) {
      return;
    }

    const pattern = new RegExp("(" + terms.map(escapeRegExp).join("|") + ")", "gi");
    const walker = document.createTreeWalker(target, NodeFilter.SHOW_TEXT, {
      acceptNode(node) {
        const parent = node.parentElement;
        if (!parent || parent.closest("mark, script, style, textarea")) {
          return NodeFilter.FILTER_REJECT;
        }
        const text = node.nodeValue || "";
        return terms.some((term) => text.toLowerCase().includes(term))
          ? NodeFilter.FILTER_ACCEPT
          : NodeFilter.FILTER_REJECT;
      }
    });
    const textNodes = [];

    while (walker.nextNode()) {
      textNodes.push(walker.currentNode);
    }

    textNodes.forEach((node) => {
      const source = node.nodeValue || "";
      const fragment = document.createDocumentFragment();
      let lastIndex = 0;

      source.replace(pattern, (match, _term, offset) => {
        fragment.appendChild(document.createTextNode(source.slice(lastIndex, offset)));
        const mark = document.createElement("mark");
        mark.dataset.searchTargetMark = "true";
        mark.textContent = match;
        fragment.appendChild(mark);
        lastIndex = offset + match.length;
        return match;
      });

      fragment.appendChild(document.createTextNode(source.slice(lastIndex)));
      node.parentNode.replaceChild(fragment, node);
    });
  };

  const bindBodyClear = () => {
    if (bodyClearBound) {
      return;
    }

    const pageBody = document.querySelector(".page-body");
    if (!pageBody) {
      return;
    }

    pageBody.addEventListener("click", clearTargetHighlight);
    bodyClearBound = true;
  };

  const syncTargetHighlight = ({ scroll = false } = {}) => {
    const url = new URL(window.location.href);
    const query = url.searchParams.get("mz-search") || "";
    const targetId = decodeHashId();

    if (query && !input.value) {
      input.value = query;
    }
    clearRenderedTargetHighlight();

    if (!query || !targetId) {
      syncActiveSearchMatch();
      return;
    }

    const target = document.getElementById(targetId);
    if (!target) {
      syncActiveSearchMatch();
      return;
    }

    const terms = termsForQuery(query);
    target.classList.add("search-target-active");
    highlightTargetText(target, terms);
    bindBodyClear();
    syncActiveSearchMatch();

    if (scroll) {
      window.setTimeout(() => scrollTargetIntoView(target, "center"), 30);
    }
  };

  const handleSearchResultClick = (event) => {
    const eventTarget = event.target instanceof Element ? event.target : event.target.parentElement;
    const anchor = eventTarget ? eventTarget.closest(".search-match") : null;
    if (!anchor) {
      return;
    }

    const targetUrl = new URL(anchor.href, window.location.href);
    if (!sameDocumentUrl(targetUrl)) {
      return;
    }

    event.preventDefault();
    window.history.pushState(null, "", targetUrl.pathname + targetUrl.search + targetUrl.hash);
    syncTargetHighlight({ scroll: true });
  };

  setupInPageAnchorScroll();
  revealInitialHashTarget();
  syncTargetHighlight({ scroll: true });
  status.textContent = "Loading the search index...";

  fetch(indexUrl, { cache: "no-store" })
    .then((res) => {
      if (!res.ok) throw new Error("Failed to load search index: " + res.status);
      return res.json();
    })
    .then((data) => {
      entries = Array.isArray(data) ? data : [];
      status.textContent = "Search is ready across " + entries.length + " page(s).";
      search(input.value);
    })
    .catch(() => {
      status.textContent = "Search index could not be loaded. Use 'cargo run -- serve' or deploy the built site through HTTP.";
    });

  input.addEventListener("input", (e) => search(e.target.value));
  results.addEventListener("click", handleSearchResultClick);
  window.addEventListener("popstate", () => syncTargetHighlight({ scroll: true }));
  window.addEventListener("hashchange", () => syncTargetHighlight({ scroll: true }));
})();
"##;

const GRAPH_SCRIPT: &str = r##"
(() => {
  const svg = document.getElementById("knowledge-graph");
  const stage = svg && svg.closest(".graph-stage");
  const status = document.getElementById("graph-status");
  const queryInput = document.getElementById("graph-filter");
  const showTagsInput = document.getElementById("graph-show-tags");
  const controls = document.getElementById("graph-controls");
  const resetButton = document.getElementById("graph-reset");
  const forceInputs = {
    center: document.getElementById("graph-center-force"),
    repulsion: document.getElementById("graph-repulsion-force"),
    attraction: document.getElementById("graph-link-force"),
    distance: document.getElementById("graph-link-distance")
  };

  if (!svg || !stage || !status || !queryInput || !showTagsInput || !resetButton) {
    return;
  }

  if (!window.d3) {
    status.textContent = "Knowledge graph runtime could not load. Rebuild the site so assets/d3.min.js is published.";
    return;
  }

  const d3 = window.d3;
  const { graphJson: graphJsonHref = "", siteHome: siteHomeHref = "" } = document.body.dataset;
  const graphUrl = new URL(graphJsonHref, window.location.href);
  const siteRootUrl = new URL(siteHomeHref || ".", window.location.href);
  let graph = { nodes: [], edges: [] };
  let simulation = null;
  let currentNodes = [];
  let currentEdges = [];
  let nodeSelection = null;
  let edgeSelection = null;
  let canvas = null;
  let hoveredNodeId = "";
  let selectedTagId = "";
  let zoomScale = 1;
  let width = 900;
  let height = 620;
  const positions = new Map();
  const defaults = { center: 10, repulsion: 90, attraction: 35, distance: 95 };
  const svgSelection = d3.select(svg);

  const trimLabel = (value, length = 24) => {
    const text = String(value || "");
    return text.length > length ? text.slice(0, length - 1) + "..." : text;
  };

  const nodeSearchText = (node) =>
    [
      node.label,
      node.summary,
      node.source,
      node.type,
      ...(node.tags || [])
    ].filter(Boolean).join(" ").toLowerCase();

  const nodeRadius = (node) => {
    const degree = Number(node.degree) || 0;
    return node.type === "tag"
      ? 6 + Math.min(3, Math.sqrt(degree))
      : 7 + Math.min(5, Math.sqrt(degree) * 1.3);
  };

  const endpointId = (endpoint) => typeof endpoint === "object" ? endpoint.id : endpoint;
  const documentUrl = (node) => new URL(node.url || "", siteRootUrl).toString();
  const forceValue = (name) => {
    const value = Number(forceInputs[name] && forceInputs[name].value);
    return Number.isFinite(value) ? value : defaults[name];
  };

  const savePositions = () => {
    currentNodes.forEach((node) => {
      if (Number.isFinite(node.x) && Number.isFinite(node.y)) {
        positions.set(node.id, { x: node.x, y: node.y, vx: node.vx || 0, vy: node.vy || 0 });
      }
    });
  };

  const updateOutputs = () => {
    Object.entries(forceInputs).forEach(([name, input]) => {
      const output = input && document.querySelector('[data-force-output="' + name + '"]');
      if (output) output.value = input.value;
    });
  };

  const updateStatus = () => {
    const query = queryInput.value.trim().toLowerCase();
    if (query) {
      const matches = currentNodes.filter((node) => nodeSearchText(node).includes(query)).length;
      status.textContent = matches + " match(es) among " + currentNodes.length + " node(s).";
    } else {
      status.textContent = currentNodes.length + " node(s), " + currentEdges.length + " edge(s) visible.";
    }
  };

  const updateVisualState = () => {
    if (!nodeSelection || !edgeSelection) return;
    const focusId = hoveredNodeId || selectedTagId;
    const neighbors = new Set(focusId ? [focusId] : []);
    if (focusId) {
      currentEdges.forEach((edge) => {
        const sourceId = endpointId(edge.source);
        const targetId = endpointId(edge.target);
        if (sourceId === focusId) neighbors.add(targetId);
        if (targetId === focusId) neighbors.add(sourceId);
      });
    }

    const query = queryInput.value.trim().toLowerCase();
    nodeSelection
      .classed("is-focused", (node) => node.id === focusId)
      .classed("is-neighbor", (node) => Boolean(focusId) && neighbors.has(node.id) && node.id !== focusId)
      .classed("is-dimmed", (node) => Boolean(focusId) && !neighbors.has(node.id))
      .classed("is-search-match", (node) => Boolean(query) && nodeSearchText(node).includes(query))
      .classed("is-search-dimmed", (node) => Boolean(query) && !nodeSearchText(node).includes(query));

    edgeSelection
      .classed("is-active", (edge) => {
        const sourceId = endpointId(edge.source);
        const targetId = endpointId(edge.target);
        return Boolean(focusId) && (sourceId === focusId || targetId === focusId);
      })
      .classed("is-dimmed", (edge) => Boolean(focusId)
        && endpointId(edge.source) !== focusId
        && endpointId(edge.target) !== focusId)
      .classed("is-search-dimmed", (edge) => Boolean(query)
        && !nodeSearchText(edge.source).includes(query)
        && !nodeSearchText(edge.target).includes(query));

    svgSelection.classed("show-all-labels", zoomScale >= 1.45);
    updateStatus();
  };

  const updateForces = (restart = true) => {
    if (!simulation) return;
    const centerStrength = forceValue("center") / 100;
    const attractionStrength = forceValue("attraction") / 100;
    simulation.force("charge").strength(-forceValue("repulsion"));
    simulation.force("link")
      .distance(forceValue("distance"))
      .strength((edge) => Math.min(1, attractionStrength * (0.75 + (Number(edge.weight) || 1) * 0.12)));
    simulation.force("x").x(width / 2).strength(centerStrength);
    simulation.force("y").y(height / 2).strength(centerStrength);
    if (restart) simulation.alpha(0.55).restart();
  };

  const activateNode = (node) => {
    if (node.__dragged) {
      node.__dragged = false;
      return;
    }
    if (node.type === "document" && node.url !== undefined) {
      window.location.href = documentUrl(node);
      return;
    }
    if (node.type === "tag") {
      selectedTagId = selectedTagId === node.id ? "" : node.id;
      updateVisualState();
    }
  };

  const dragBehavior = () => d3.drag()
    .clickDistance(4)
    .on("start", (event, node) => {
      event.sourceEvent.stopPropagation();
      node.__dragged = false;
      node.__dragStartX = event.x;
      node.__dragStartY = event.y;
      if (!event.active) simulation.alphaTarget(0.22).restart();
      node.fx = node.x;
      node.fy = node.y;
    })
    .on("drag", (event, node) => {
      if (Math.hypot(event.x - node.__dragStartX, event.y - node.__dragStartY) > 4) {
        node.__dragged = true;
      }
      node.fx = event.x;
      node.fy = event.y;
    })
    .on("end", (event, node) => {
      if (!event.active) simulation.alphaTarget(0);
      node.fx = null;
      node.fy = null;
    });

  const rebuildGraph = () => {
    savePositions();
    if (simulation) simulation.stop();

    const allowedTypes = showTagsInput.checked ? new Set(["document", "tag"]) : new Set(["document"]);
    currentNodes = graph.nodes
      .filter((node) => allowedTypes.has(node.type))
      .map((node) => ({ ...node, ...(positions.get(node.id) || {}) }));
    const nodeIds = new Set(currentNodes.map((node) => node.id));
    currentEdges = graph.edges
      .filter((edge) => ["has_tag", "links_to", "shared_tag"].includes(edge.type))
      .filter((edge) => nodeIds.has(edge.source) && nodeIds.has(edge.target))
      .map((edge) => ({ ...edge }));

    const degree = new Map(currentNodes.map((node) => [node.id, 0]));
    currentEdges.forEach((edge) => {
      degree.set(edge.source, (degree.get(edge.source) || 0) + 1);
      degree.set(edge.target, (degree.get(edge.target) || 0) + 1);
    });
    currentNodes.forEach((node) => { node.degree = degree.get(node.id) || 0; });

    selectedTagId = currentNodes.some((node) => node.id === selectedTagId) ? selectedTagId : "";
    svgSelection.selectAll("*").remove();
    const defs = svgSelection.append("defs");
    defs.append("marker")
      .attr("id", "graph-arrow")
      .attr("viewBox", "0 -5 10 10")
      .attr("refX", 17)
      .attr("markerWidth", 5)
      .attr("markerHeight", 5)
      .attr("orient", "auto")
      .append("path")
      .attr("d", "M0,-5L10,0L0,5")
      .attr("class", "graph-arrow");

    canvas = svgSelection.append("g").attr("class", "graph-canvas");
    edgeSelection = canvas.append("g")
      .attr("class", "graph-edges")
      .selectAll("line")
      .data(currentEdges)
      .join("line")
      .attr("class", (edge) => "graph-edge " + edge.type)
      .attr("stroke-width", (edge) => Math.max(1, Math.min(3, Number(edge.weight) || 1)));

    nodeSelection = canvas.append("g")
      .attr("class", "graph-nodes")
      .selectAll("g")
      .data(currentNodes, (node) => node.id)
      .join("g")
      .attr("class", (node) => "graph-node " + node.type)
      .classed("is-prominent", (node) => node.type === "document" && node.degree >= 2)
      .attr("tabindex", 0)
      .attr("role", "button")
      .attr("aria-label", (node) => node.label || node.id)
      .on("mouseenter", (_, node) => { hoveredNodeId = node.id; updateVisualState(); })
      .on("mouseleave", () => { hoveredNodeId = ""; updateVisualState(); })
      .on("click", (event, node) => { event.stopPropagation(); activateNode(node); })
      .on("keydown", (event, node) => {
        if (event.key === "Enter" || event.key === " ") {
          event.preventDefault();
          activateNode(node);
        }
      })
      .call(dragBehavior());

    nodeSelection.append("circle").attr("r", nodeRadius);
    nodeSelection.append("text")
      .attr("class", "graph-label")
      .attr("x", 0)
      .attr("y", (node) => nodeRadius(node) + 15)
      .attr("text-anchor", "middle")
      .text((node) => trimLabel(node.label || node.id));
    nodeSelection.append("title").text((node) => node.label || node.id);

    if (!currentNodes.length) {
      canvas.append("text")
        .attr("x", width / 2)
        .attr("y", height / 2)
        .attr("text-anchor", "middle")
        .attr("class", "graph-empty")
        .text("No documents are available.");
    }

    simulation = d3.forceSimulation(currentNodes)
      .force("link", d3.forceLink(currentEdges).id((node) => node.id))
      .force("charge", d3.forceManyBody())
      .force("x", d3.forceX(width / 2))
      .force("y", d3.forceY(height / 2))
      .force("collision", d3.forceCollide().radius((node) => nodeRadius(node) + 7).strength(0.9))
      .on("tick", () => {
        edgeSelection
          .attr("x1", (edge) => edge.source.x)
          .attr("y1", (edge) => edge.source.y)
          .attr("x2", (edge) => edge.target.x)
          .attr("y2", (edge) => edge.target.y);
        nodeSelection.attr("transform", (node) => "translate(" + node.x + "," + node.y + ")");
      });
    updateForces(false);
    simulation.alpha(0.9).restart();
    updateVisualState();
  };

  const zoomBehavior = d3.zoom()
    .scaleExtent([0.35, 4])
    .on("zoom", (event) => {
      zoomScale = event.transform.k;
      if (canvas) canvas.attr("transform", event.transform);
      updateVisualState();
    });
  svgSelection.call(zoomBehavior).on("dblclick.zoom", null);
  svgSelection.on("click", () => {
    selectedTagId = "";
    updateVisualState();
  });

  const resizeGraph = () => {
    width = Math.max(320, stage.clientWidth);
    height = Math.max(420, stage.clientHeight);
    svgSelection.attr("viewBox", "0 0 " + width + " " + height);
    if (simulation) updateForces(true);
  };

  queryInput.addEventListener("input", updateVisualState);
  showTagsInput.addEventListener("change", rebuildGraph);
  Object.values(forceInputs).forEach((input) => {
    if (!input) return;
    input.addEventListener("input", () => {
      updateOutputs();
      updateForces(true);
    });
  });
  resetButton.addEventListener("click", () => {
    Object.entries(defaults).forEach(([name, value]) => {
      if (forceInputs[name]) forceInputs[name].value = value;
    });
    updateOutputs();
    currentNodes.forEach((node, index) => {
      const angle = index * 2.399963229728653;
      const radius = 18 * Math.sqrt(index);
      node.x = width / 2 + Math.cos(angle) * radius;
      node.y = height / 2 + Math.sin(angle) * radius;
      node.vx = 0;
      node.vy = 0;
      node.fx = null;
      node.fy = null;
    });
    updateForces(true);
    svgSelection.transition().duration(250).call(zoomBehavior.transform, d3.zoomIdentity);
  });

  if (controls && window.matchMedia("(max-width: 820px)").matches) {
    controls.removeAttribute("open");
  }
  updateOutputs();
  resizeGraph();
  if (window.ResizeObserver) {
    new ResizeObserver(resizeGraph).observe(stage);
  } else {
    window.addEventListener("resize", resizeGraph);
  }

  status.textContent = "Loading the knowledge graph...";
  fetch(graphUrl, { cache: "no-store" })
    .then((response) => {
      if (!response.ok) {
        throw new Error("Failed to load graph: " + response.status);
      }
      return response.json();
    })
    .then((data) => {
      graph = {
        nodes: Array.isArray(data.nodes)
          ? data.nodes.filter((node) => node.type === "document" || node.type === "tag")
          : [],
        edges: Array.isArray(data.edges)
          ? data.edges.filter((edge) => ["has_tag", "links_to", "shared_tag"].includes(edge.type))
          : []
      };
      rebuildGraph();
    })
    .catch(() => {
      status.textContent = "Knowledge graph could not be loaded. Use 'cargo run -- serve' or deploy the built site through HTTP.";
    });
})();
"##;

const CODE_SCRIPT: &str = r##"
(() => {
  const root = document.documentElement;
  const fontButtons = Array.from(document.querySelectorAll("[data-font-value]"));
  const fontStorageKey = "minizensical-font-choice";

  const applyFont = (value, persist = true) => {
    if (!value) {
      return;
    }
    root.style.setProperty("--content-font", value);
    fontButtons.forEach((button) => {
      const active = button.dataset.fontValue === value;
      button.classList.toggle("is-active", active);
      button.setAttribute("aria-pressed", active ? "true" : "false");
    });
    if (persist) {
      try {
        window.localStorage.setItem(fontStorageKey, value);
      } catch (_error) {
        // Ignore persistence errors in restricted browsers.
      }
    }
  };

  if (fontButtons.length > 0) {
    const availableFonts = new Set(fontButtons.map((button) => button.dataset.fontValue));
    let initialFont = fontButtons[0].dataset.fontValue;
    try {
      const storedFont = window.localStorage.getItem(fontStorageKey);
      initialFont = availableFonts.has(storedFont) ? storedFont : initialFont;
    } catch (_error) {
      // Keep the first configured option.
    }
    applyFont(initialFont, false);
    fontButtons.forEach((button) => {
      button.addEventListener("click", () => applyFont(button.dataset.fontValue));
    });
  }

  const escapeHtml = (value) =>
    value.replace(/[&<>\"']/g, (character) => ({
      "&": "&amp;",
      "<": "&lt;",
      ">": "&gt;",
      "\"": "&quot;",
      "'": "&#39;"
    })[character]);

  const aliases = new Map([
    ["rs", "rust"],
    ["js", "javascript"],
    ["ts", "typescript"],
    ["sh", "bash"],
    ["shell", "bash"],
    ["yml", "yaml"],
    ["md", "markdown"],
    ["html", "markup"],
    ["xml", "markup"]
  ]);

  const keywordSets = {
    rust: "as async await break const continue crate dyn else enum extern false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true type unsafe use where while".split(" "),
    javascript: "async await break case catch class const continue default delete do else export extends false finally for from function if import in instanceof let new null return static super switch this throw true try typeof undefined var void while yield".split(" "),
    typescript: "abstract any as async await boolean break case catch class const constructor continue declare default delete do else enum export extends false finally for from function if implements import in infer instanceof interface keyof let module namespace never new null number object private protected public readonly return static string super switch this throw true try type typeof undefined unknown var void while yield".split(" "),
    css: "align-items background border color display flex font grid height justify-content margin padding position width".split(" "),
    toml: "true false".split(" "),
    yaml: "true false null".split(" "),
    bash: "case do done elif else esac export fi for function if in local read return set then while".split(" ")
  };

  const detectLanguage = (code) => {
    const classes = Array.from(code.classList);
    for (const className of classes) {
      const match = className.match(/(?:language-|lang-)?([a-z0-9_+-]+)/i);
      if (match) {
        const language = match[1].toLowerCase();
        return aliases.get(language) || language;
      }
    }
    return "text";
  };

  const matchSticky = (pattern, source, offset) => {
    pattern.lastIndex = offset;
    const match = pattern.exec(source);
    return match && match.index === offset ? match[0] : null;
  };

  const scannerPatterns = (language) => {
    const keywords = keywordSets[language] || keywordSets.javascript;
    const keywordPattern = new RegExp("\\b(?:" + keywords.map((word) => word.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")).join("|") + ")\\b", "y");
    const commentPattern = language === "bash" || language === "yaml" || language === "toml"
      ? /#[^\n]*/y
      : /\/\/[^\n]*|\/\*[\s\S]*?\*\//y;
    return [
      ["comment", commentPattern],
      ["string", /"(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*'|`(?:\\.|[^`\\])*`/y],
      ["number", /\b\d+(?:\.\d+)?(?:[a-zA-Z_]\w*)?\b/y],
      ["keyword", keywordPattern],
      ["function", /\b[A-Za-z_][\w-]*(?=\s*\()/y],
      ["operator", /=>|->|==|!=|<=|>=|&&|\|\||[+\-*\/%=!<>:]/y]
    ];
  };

  const highlightGeneric = (source, language) => {
    const patterns = scannerPatterns(language);
    let html = "";
    let index = 0;
    while (index < source.length) {
      let matched = false;
      for (const [type, pattern] of patterns) {
        const token = matchSticky(pattern, source, index);
        if (token) {
          html += '<span class="token-' + type + '">' + escapeHtml(token) + '</span>';
          index += token.length;
          matched = true;
          break;
        }
      }
      if (!matched) {
        html += escapeHtml(source[index]);
        index += 1;
      }
    }
    return html;
  };

  const highlightMarkup = (source) => {
    let html = "";
    let index = 0;
    const comment = /<!--[\s\S]*?-->/y;
    const tag = /<\/?[A-Za-z][^>\n]*?>/y;
    while (index < source.length) {
      const commentToken = matchSticky(comment, source, index);
      if (commentToken) {
        html += '<span class="token-comment">' + escapeHtml(commentToken) + '</span>';
        index += commentToken.length;
        continue;
      }
      const tagToken = matchSticky(tag, source, index);
      if (tagToken) {
        const tagHtml = escapeHtml(tagToken)
          .replace(/^(&lt;\/?)([A-Za-z][\w:-]*)/, '$1<span class="token-tag">$2</span>')
          .replace(/([\w:-]+)(=)/g, '<span class="token-attr">$1</span>$2');
        html += tagHtml;
        index += tagToken.length;
        continue;
      }
      html += escapeHtml(source[index]);
      index += 1;
    }
    return html;
  };

  const highlightMarkdown = (source) =>
    escapeHtml(source)
      .replace(/^(\s{0,3}#{1,6}\s.+)$/gm, '<span class="token-keyword">$1</span>')
      .replace(/^(\s*[-*+]\s)/gm, '<span class="token-operator">$1</span>')
      .replace(/(`[^`\n]+`)/g, '<span class="token-string">$1</span>');

  const highlightCode = (source, language) => {
    if (language === "text" || language === "plain") {
      return escapeHtml(source);
    }
    if (language === "markup") {
      return highlightMarkup(source);
    }
    if (language === "markdown") {
      return highlightMarkdown(source);
    }
    return highlightGeneric(source, language);
  };

  const copyText = async (text) => {
    if (navigator.clipboard && window.isSecureContext) {
      await navigator.clipboard.writeText(text);
      return;
    }
    const textarea = document.createElement("textarea");
    textarea.value = text;
    textarea.setAttribute("readonly", "");
    textarea.style.position = "fixed";
    textarea.style.opacity = "0";
    document.body.appendChild(textarea);
    textarea.select();
    document.execCommand("copy");
    textarea.remove();
  };

  document.querySelectorAll(".page-body pre > code").forEach((code, index) => {
    const pre = code.parentElement;
    if (!pre || pre.dataset.codeEnhanced === "true") {
      return;
    }
    pre.dataset.codeEnhanced = "true";
    const rawCode = code.textContent || "";
    const language = detectLanguage(code);
    code.innerHTML = highlightCode(rawCode, language);

    const wrapper = document.createElement("div");
    wrapper.className = "code-block";
    const toolbar = document.createElement("div");
    toolbar.className = "code-toolbar";

    const label = document.createElement("span");
    label.className = "code-language";
    label.textContent = language === "text" ? "code" : language;

    const button = document.createElement("button");
    button.type = "button";
    button.className = "copy-code-button";
    button.textContent = "Copy";
    button.setAttribute("aria-label", "Copy code block " + (index + 1));

    button.addEventListener("click", async () => {
      try {
        await copyText(rawCode);
        button.textContent = "Copied";
        button.classList.add("is-copied");
        window.setTimeout(() => {
          button.textContent = "Copy";
          button.classList.remove("is-copied");
        }, 1400);
      } catch (_error) {
        button.textContent = "Failed";
        window.setTimeout(() => {
          button.textContent = "Copy";
        }, 1400);
      }
    });

    toolbar.append(label, button);
    pre.before(wrapper);
    wrapper.append(toolbar, pre);
  });
})();
"##;

const MATH_SCRIPT: &str = r#"
(() => {
  const formulas = Array.from(document.querySelectorAll(".math"));
  if (!formulas.length) {
    return;
  }

  window.MathJax = {
    startup: {
      typeset: false,
      ready() {
        MathJax.startup.defaultReady();
        Promise.all(formulas.map(async (element) => {
          const source = (element.textContent || "").trim();
          if (!source) {
            return;
          }
          const display = element.classList.contains("math-display");
          const rendered = await MathJax.tex2svgPromise(source, { display });
          element.replaceChildren(rendered);
          element.classList.add("math-rendered");
        })).then(() => {
          document.head.appendChild(MathJax.svgStylesheet());
        }).catch((error) => {
          console.error("MathJax rendering failed:", error);
        });
      }
    },
    svg: {
      fontCache: "local"
    }
  };

  const script = document.createElement("script");
  script.src = "https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-svg.js";
  script.async = true;
  script.dataset.minizensicalMathjax = "true";
  document.head.appendChild(script);
})();
"#;

const KNOWLEDGE_GRAPH_TEMPLATE: &str = r##"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{{ title }}</title>
  <meta name="description" content="{{ description }}">
  <script>{{ theme_boot_script | safe }}</script>
  <link rel="stylesheet" href="{{ stylesheet_href }}">
</head>
<body class="graph-view" data-search-index="{{ search_index_href }}" data-site-home="{{ home_href }}" data-graph-json="{{ graph_json_href | safe }}">
  <div class="ambient ambient-a"></div>
  <div class="ambient ambient-b"></div>
  <div class="shell">
    <aside class="sidebar">
      <a class="brand" href="{{ home_href }}">
        <span class="brand-mark">MZ</span>
        <span class="brand-copy">
          <strong>{{ site_name }}</strong>
          <span>Rust static docs with course-ready polish</span>
        </span>
      </a>

      <section class="search-panel">
  <label class="search-label" for="doc-search">Search docs</label>
  <input id="doc-search" class="search-input" type="search" placeholder="Search titles, headings, and content">
  <p class="search-hint">Try keywords like <code>front matter</code>, <code>search</code>, or <code>architecture</code>.</p>
  <div id="search-status" class="search-status">Search is ready as soon as the page loads.</div>
  <div id="search-results" class="search-results" hidden></div>
</section>

      <section class="theme-panel">
        <p class="theme-label">Theme</p>
        <div class="theme-toggle" data-theme-switcher>
          <button type="button" class="theme-option" data-theme-choice="light">Light</button>
          <button type="button" class="theme-option" data-theme-choice="dark">Dark</button>
          <button type="button" class="theme-option" data-theme-choice="system">System</button>
          <button type="button" class="theme-option" data-theme-choice="sepia">Sepia</button>
          <button type="button" class="theme-option" data-theme-choice="ocean">Ocean</button>
          <button type="button" class="theme-option" data-theme-choice="forest">Forest</button>
        </div>
        <p class="theme-hint">The theme choice is saved in your browser and follows system preference in <code>System</code> mode.</p>
      </section>

      <section class="font-panel">
        <p class="font-label">Font</p>
        <div class="font-toggle" data-font-switcher>
          {% for font in font_options %}
          <button type="button" class="font-option" data-font-value="{{ font.css_value }}">{{ font.label }}</button>
          {% endfor %}
        </div>
        <p class="font-hint">Fonts placed in <code>docs/assets/fonts/</code> are added to this switcher after build.</p>
      </section>

      <div class="nav-shell">
        {{ nav_html | safe }}
      </div>
    </aside>

    <main class="content">
      <article class="page graph-page">
        <header class="page-header">
          <p class="eyebrow">Knowledge Graph</p>
          <h1>Knowledge Graph</h1>
          <p class="page-summary">{{ description }}</p>
        </header>

        <div class="page-body">
          <section class="graph-stage" aria-label="Interactive knowledge graph">
            <details id="graph-controls" class="graph-controls" open>
              <summary class="graph-controls-summary">Graph controls</summary>
              <div class="graph-controls-body">
                <label class="graph-control" for="graph-filter">
                  <span>Search documents</span>
                  <input id="graph-filter" class="graph-input" type="search" placeholder="Search titles, tags, and sources">
                </label>
                <label class="graph-toggle" for="graph-show-tags">
                  <input id="graph-show-tags" type="checkbox">
                  <span>Show tags</span>
                </label>
                <label class="graph-control" for="graph-center-force">
                  <span class="graph-control-row"><span>Center force</span><output data-force-output="center">10</output></span>
                  <input id="graph-center-force" type="range" min="0" max="40" value="10">
                </label>
                <label class="graph-control" for="graph-repulsion-force">
                  <span class="graph-control-row"><span>Node repulsion</span><output data-force-output="repulsion">90</output></span>
                  <input id="graph-repulsion-force" type="range" min="20" max="300" value="90">
                </label>
                <label class="graph-control" for="graph-link-force">
                  <span class="graph-control-row"><span>Link attraction</span><output data-force-output="attraction">35</output></span>
                  <input id="graph-link-force" type="range" min="0" max="100" value="35">
                </label>
                <label class="graph-control" for="graph-link-distance">
                  <span class="graph-control-row"><span>Link distance</span><output data-force-output="distance">95</output></span>
                  <input id="graph-link-distance" type="range" min="30" max="220" value="95">
                </label>
                <button id="graph-reset" class="graph-reset" type="button">Reset layout</button>
              </div>
            </details>
            <svg id="knowledge-graph" class="graph-svg" role="img" aria-label="Draggable document knowledge graph"></svg>
            <p id="graph-status" class="graph-status">Loading the knowledge graph...</p>
          </section>
        </div>
      </article>
    </main>
  </div>

  <script src="{{ theme_script_href }}"></script>
  <script src="{{ code_script_href }}"></script>
  <script src="{{ search_script_href }}"></script>
  <script src="{{ d3_script_href | safe }}"></script>
  <script src="{{ graph_script_href }}"></script>
</body>
</html>
"##;

const ARCHIVE_TEMPLATE: &str = r##"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{{ title }}</title>
  <meta name="description" content="{{ description }}">
  <script>{{ theme_boot_script | safe }}</script>
  <link rel="stylesheet" href="{{ stylesheet_href }}">
</head>
<body data-search-index="{{ search_index_href }}" data-site-home="{{ home_href }}">
  <div class="ambient ambient-a"></div>
  <div class="ambient ambient-b"></div>
  <div class="shell">
    <aside class="sidebar">
      <a class="brand" href="{{ home_href }}">
        <span class="brand-mark">MZ</span>
        <span class="brand-copy">
          <strong>{{ site_name }}</strong>
          <span>Rust static docs with course-ready polish</span>
        </span>
      </a>

      <section class="search-panel">
  <label class="search-label" for="doc-search">Search docs</label>
  <input id="doc-search" class="search-input" type="search" placeholder="Search titles, headings, and content">
  <p class="search-hint">Try keywords like <code>front matter</code>, <code>search</code>, or <code>architecture</code>.</p>
  <div id="search-status" class="search-status">Search is ready as soon as the page loads.</div>
  <div id="search-results" class="search-results" hidden></div>
</section>

      <section class="theme-panel">
        <p class="theme-label">Theme</p>
        <div class="theme-toggle" data-theme-switcher>
          <button type="button" class="theme-option" data-theme-choice="light">Light</button>
          <button type="button" class="theme-option" data-theme-choice="dark">Dark</button>
          <button type="button" class="theme-option" data-theme-choice="system">System</button>
          <button type="button" class="theme-option" data-theme-choice="sepia">Sepia</button>
          <button type="button" class="theme-option" data-theme-choice="ocean">Ocean</button>
          <button type="button" class="theme-option" data-theme-choice="forest">Forest</button>
        </div>
        <p class="theme-hint">The theme choice is saved in your browser and follows system preference in <code>System</code> mode.</p>
      </section>

      <section class="font-panel">
        <p class="font-label">Font</p>
        <div class="font-toggle" data-font-switcher>
          {% for font in font_options %}
          <button type="button" class="font-option" data-font-value="{{ font.css_value }}">{{ font.label }}</button>
          {% endfor %}
        </div>
        <p class="font-hint">Fonts placed in <code>docs/assets/fonts/</code> are added to this switcher after build.</p>
      </section>

      <div class="nav-shell">
        {{ nav_html | safe }}
      </div>
    </aside>

    <main class="content">
      <article class="page archive-page">
        <header class="page-header">
          <p class="eyebrow">Archive</p>
          <h1>{{ archive_title }}</h1>
          <p class="page-summary">{{ description }}</p>
        </header>

        <div class="page-body">
          {% for section in sections %}
          <section class="archive-section">
            <h2>{{ section.title }}</h2>
            {% for group in section.groups %}
            <div class="archive-group">
              <h3 class="archive-group-title">{{ group.title }}</h3>
              <ul class="archive-list">
                {% for page in group.pages %}
                <li>
                  <a href="{{ page.href | safe }}">{{ page.title }}</a>
                </li>
                {% endfor %}
              </ul>
            </div>
            {% endfor %}
          </section>
          {% endfor %}
        </div>
      </article>
    </main>
  </div>

  <script src="{{ theme_script_href }}"></script>
  <script src="{{ code_script_href }}"></script>
  <script src="{{ search_script_href }}"></script>
</body>
</html>
"##;

fn render_navigation_html(items: &[RenderNavItem]) -> String {
    let mut html = String::from("<ul class=\"nav-list\">");
    for item in items {
        html.push_str("<li class=\"nav-item");
        if item.active {
            html.push_str(" active");
        }
        html.push_str("\">");

        if let Some(href) = &item.href {
            html.push_str("<a href=\"");
            html.push_str(&escape_html(href));
            html.push_str("\">");
            html.push_str(&escape_html(&item.title));
            html.push_str("</a>");
        } else {
            html.push_str("<span class=\"nav-section\">");
            html.push_str(&escape_html(&item.title));
            html.push_str("</span>");
        }

        if !item.children.is_empty() {
            html.push_str(&render_navigation_html(&item.children));
        }

        html.push_str("</li>");
    }
    html.push_str("</ul>");
    html
}

fn escape_html(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    for character in input.chars() {
        match character {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&#39;"),
            _ => output.push(character),
        }
    }
    output
}

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
          <button type="button" class="theme-option" data-theme-choice="light">Day</button>
          <button type="button" class="theme-option" data-theme-choice="dark">Night</button>
          <button type="button" class="theme-option" data-theme-choice="system">System</button>
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
  border: 1px solid rgba(13, 109, 104, 0.15);
  border-radius: 16px;
  padding: 13px 14px;
  font: inherit;
  background: var(--panel-solid);
  color: var(--ink);
}

.search-input:focus {
  outline: 2px solid rgba(13, 109, 104, 0.2);
  border-color: rgba(13, 109, 104, 0.35);
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
  border: 1px solid rgba(13, 109, 104, 0.08);
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
  border: 1px solid rgba(13, 109, 104, 0.08);
  background: var(--panel-solid);
  color: var(--muted);
  line-height: 1.55;
  transition: transform 160ms ease, border-color 160ms ease, box-shadow 160ms ease, color 160ms ease;
}

.search-match:hover {
  transform: translateY(-1px);
  border-color: rgba(13, 109, 104, 0.24);
  box-shadow: 0 12px 25px rgba(12, 37, 39, 0.08);
  color: var(--ink);
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
  border: 1px solid rgba(13, 109, 104, 0.16);
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
  border-color: rgba(13, 109, 104, 0.35);
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
  border: 1px solid rgba(205, 232, 232, 0.12);
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
  border-bottom: 1px solid rgba(205, 232, 232, 0.1);
  background: rgba(255, 255, 255, 0.05);
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
  color: #8ea6a5;
  font-style: italic;
}

.token-keyword {
  color: #86d7ff;
}

.token-string {
  color: #b8e986;
}

.token-number {
  color: #ffd479;
}

.token-function {
  color: #f5c2ff;
}

.token-operator {
  color: #ffb38a;
}

.token-tag {
  color: #8fdcff;
}

.token-attr {
  color: #ffd479;
}

.page-body :not(pre) > code {
  padding: 0.18em 0.48em;
  border-radius: 8px;
  background: rgba(16, 33, 35, 0.07);
}

.page-body blockquote {
  margin-left: 0;
  padding: 14px 18px;
  border-left: 4px solid rgba(13, 109, 104, 0.25);
  border-radius: 0 16px 16px 0;
  background: rgba(13, 109, 104, 0.04);
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

@media (max-width: 1180px) {
  .shell {
    grid-template-columns: minmax(240px, 300px) minmax(0, 1fr);
  }

  .toc {
    display: none;
  }
}

@media (max-width: 820px) {
  .shell {
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
  const buttons = Array.from(document.querySelectorAll("[data-theme-choice]"));
  const media = window.matchMedia("(prefers-color-scheme: dark)");

  const normalizeChoice = (value) => {
    if (value === "light" || value === "dark") {
      return value;
    }
    return "system";
  };

  const resolveTheme = (choice) => {
    if (choice === "dark") {
      return "dark";
    }
    if (choice === "light") {
      return "light";
    }
    return media.matches ? "dark" : "light";
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

  let choice = "system";
  try {
    const stored = window.localStorage.getItem(storageKey);
    if (stored === "light" || stored === "dark") {
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

  const decodeHashId = () => {
    const raw = window.location.hash.replace(/^#/, "");
    if (!raw) {
      return "";
    }
    try {
      return decodeURIComponent(raw);
    } catch (_error) {
      return raw;
    }
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

  const clearTargetHighlight = () => {
    document.querySelectorAll(".search-target-active").forEach((element) => {
      element.classList.remove("search-target-active");
    });
    removeTargetMarks();

    const url = new URL(window.location.href);
    if (url.searchParams.has("mz-search")) {
      url.searchParams.delete("mz-search");
      window.history.replaceState(null, "", url.pathname + url.search + url.hash);
    }
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

  const applyTargetHighlight = () => {
    const url = new URL(window.location.href);
    const query = url.searchParams.get("mz-search") || "";
    const targetId = decodeHashId();

    if (query && !input.value) {
      input.value = query;
    }
    if (!query || !targetId) {
      return;
    }

    const target = document.getElementById(targetId);
    if (!target) {
      return;
    }

    const terms = termsForQuery(query);
    target.classList.add("search-target-active");
    highlightTargetText(target, terms);
    window.setTimeout(() => target.scrollIntoView({ block: "center", behavior: "smooth" }), 30);

    const pageBody = document.querySelector(".page-body");
    if (pageBody) {
      pageBody.addEventListener("click", clearTargetHighlight, { once: true });
    }
  };

  applyTargetHighlight();
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
          <button type="button" class="theme-option" data-theme-choice="light">Day</button>
          <button type="button" class="theme-option" data-theme-choice="dark">Night</button>
          <button type="button" class="theme-option" data-theme-choice="system">System</button>
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

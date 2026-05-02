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
    search_index_href: &str,
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
        search_index_href => search_index_href.to_string(),
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

pub fn render_archive_index(
    config: &Config,
    sections: &[ArchiveSection],
    navigation: &[RenderNavItem],
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
    )
}

pub fn render_tag_archive(
    config: &Config,
    sections: &[ArchiveSection],
    navigation: &[RenderNavItem],
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
    })?;
    Ok(rendered)
}

pub fn stylesheet_path() -> PathBuf {
    PathBuf::from("assets/minizensical.css")
}

pub fn stylesheet_contents() -> &'static str {
    STYLE_SHEET
}

pub fn search_script_path() -> PathBuf {
    PathBuf::from("assets/minizensical-search.js")
}

pub fn search_script_contents() -> &'static str {
    SEARCH_SCRIPT
}

pub fn theme_script_path() -> PathBuf {
    PathBuf::from("assets/minizensical-theme.js")
}

pub fn theme_script_contents() -> &'static str {
    THEME_SCRIPT
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
  <input id="doc-search" class="search-input" type="search" placeholder="Search titles, headings, tags, and content">
  <div class="search-mode">
    <label class="mode-option"><input type="radio" name="search-mode" value="all" checked> All</label>
    <label class="mode-option"><input type="radio" name="search-mode" value="title"> Title</label>
    <label class="mode-option"><input type="radio" name="search-mode" value="tag"> Tags</label>
  </div>
  <div class="highlight-color">
    <span class="color-label">Highlight:</span>
    <button type="button" class="color-swatch" data-color="#ffeb3b" style="background:#ffeb3b" aria-label="Yellow"></button>
    <button type="button" class="color-swatch" data-color="#a8e6cf" style="background:#a8e6cf" aria-label="Green"></button>
    <button type="button" class="color-swatch" data-color="#90caf9" style="background:#90caf9" aria-label="Blue"></button>
    <button type="button" class="color-swatch" data-color="#f8bbd0" style="background:#f8bbd0" aria-label="Pink"></button>
    <button type="button" class="color-swatch" data-color="#ffcc80" style="background:#ffcc80" aria-label="Orange"></button>
    <button type="button" class="color-swatch" data-color="inherit" style="background:var(--accent-soft);border-color:var(--accent)" aria-label="Theme default">Default</button>
  </div>
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

      <div class="nav-shell">
        {{ nav_html | safe }}
      </div>
    </aside>

    <main class="content">
      {% if page.is_home %}
      <section class="hero">
        <div class="hero-copy">
          <p class="eyebrow">Rust Course Project</p>
          <h1>MiniZensical turns Markdown into a searchable course showcase.</h1>
          <p class="hero-summary">We keep the zensical core workflow, then add front matter, instant search, live preview, and a more expressive reading experience.</p>
          <div class="hero-actions">
            <a class="hero-button primary" href="#search-showcase">See the new workflow</a>
            <a class="hero-button secondary" href="#page-start">Read the homepage notes</a>
          </div>
        </div>
        <div class="hero-grid">
          <article class="hero-card" id="search-showcase">
            <span class="hero-card-tag">Search</span>
            <h2>Search the whole site in one box</h2>
            <p>Titles, summaries, tags, headings, and body text all enter the same index.</p>
          </article>
          <article class="hero-card">
            <span class="hero-card-tag">Writing</span>
            <h2>Front matter controls pages</h2>
            <p>Use <code>title</code>, <code>summary</code>, <code>tags</code>, and <code>order</code> to organize content without touching Rust code.</p>
          </article>
          <article class="hero-card">
            <span class="hero-card-tag">Demo</span>
            <h2>Local preview keeps up live</h2>
            <p>The preview server rebuilds and refreshes automatically, which makes classroom demos much smoother.</p>
          </article>
        </div>
      </section>
      {% endif %}

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
          <h1>{{ page.title }}</h1>
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
  font-family: "Avenir Next", "IBM Plex Sans", "Segoe UI", sans-serif;
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
  font-family: "IBM Plex Mono", "SFMono-Regular", monospace;
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

.theme-panel {
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

.theme-label {
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

.search-result {
  display: grid;
  gap: 8px;
  padding: 14px;
  text-decoration: none;
  border-radius: 18px;
  border: 1px solid rgba(13, 109, 104, 0.08);
  background: var(--search-surface);
  transition: transform 160ms ease, border-color 160ms ease, box-shadow 160ms ease;
}

.search-result:hover {
  transform: translateY(-1px);
  border-color: rgba(13, 109, 104, 0.24);
  box-shadow: 0 12px 25px rgba(12, 37, 39, 0.08);
}

.search-result strong {
  font-size: 1rem;
}

.search-result p {
  margin: 0;
  color: var(--muted);
  line-height: 1.5;
}

.search-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.search-meta span {
  font-size: 0.78rem;
  padding: 4px 8px;
  border-radius: 999px;
  background: var(--accent-soft);
  color: var(--accent-strong);
}

.theme-toggle {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.theme-option {
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

.theme-option:hover {
  transform: translateY(-1px);
  border-color: rgba(13, 109, 104, 0.35);
}

.theme-option.is-active {
  background: var(--accent);
  color: var(--hero-text);
  border-color: transparent;
}

.theme-hint {
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

.hero {
  display: grid;
  grid-template-columns: minmax(0, 1.1fr) minmax(280px, 0.9fr);
  gap: 22px;
}

.hero-copy,
.hero-card {
  padding: 28px;
}

.hero-copy {
  border-radius: 28px;
  background: var(--hero-surface);
  color: var(--hero-text);
  box-shadow: 0 24px 60px rgba(8, 72, 69, 0.24);
}

.hero-copy h1 {
  margin: 0 0 16px;
  font-size: clamp(2.2rem, 4vw, 4rem);
  line-height: 1;
  max-width: 14ch;
}

.hero-summary {
  max-width: 60ch;
  font-size: 1.02rem;
  line-height: 1.75;
  color: var(--hero-subtle);
}

.hero-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  margin-top: 24px;
}

.hero-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 12px 16px;
  border-radius: 999px;
  text-decoration: none;
  font-weight: 700;
}

.hero-button.primary {
  background: white;
  color: var(--accent-strong);
}

.hero-button.secondary {
  background: rgba(255, 255, 255, 0.12);
  color: var(--hero-text);
  border: 1px solid rgba(255, 255, 255, 0.16);
}

.hero-grid {
  display: grid;
  gap: 18px;
}

.hero-card {
  background: linear-gradient(180deg, var(--panel-solid), var(--panel));
}

.hero-card h2 {
  margin: 10px 0 10px;
  font-size: 1.18rem;
}

.hero-card p {
  margin: 0;
  color: var(--muted);
  line-height: 1.65;
}

.hero-card-tag,
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

@media (max-width: 920px) {
  .hero {
    grid-template-columns: 1fr;
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

  .page,
  .hero-copy,
  .hero-card {
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

  .search-mode {
  display: flex;
  gap: 8px;
  margin: 8px 0;
  }

  .mode-option {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-size: 0.9rem;
  color: var(--muted);
  cursor: pointer;
  }

  .mode-option input[type="radio"] {
  accent-color: var(--accent);
  margin: 0;
}

  .highlight-color {
  display: flex;
  align-items: center;
  gap: 6px;
  margin: 8px 0;
  flex-wrap: wrap;
}

.color-label {
  font-size: 0.86rem;
  font-weight: 700;
  letter-spacing: 0.04em;
  color: var(--accent-strong);
}

.color-swatch {
  width: 28px;
  height: 28px;
  border-radius: 50%;
  border: 2px solid transparent;
  cursor: pointer;
  padding: 0;
  transition: transform 160ms ease, border-color 160ms ease;
}

.color-swatch:hover {
  transform: scale(1.15);
}

.color-swatch.is-active {
  border-color: var(--accent);
  box-shadow: 0 0 0 2px var(--accent-soft);
}

.color-swatch[data-color="inherit"] {
  font-size: 0.72rem;
  font-weight: 700;
  border-radius: 14px;
  width: auto;
  padding: 2px 8px;
  height: auto;
  color: var(--accent-strong);
}

  mark {
  background: var(--search-highlight, #ffeb3b);
  color: inherit;
  border-radius: 4px;
  padding: 0 4px;
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
  const modeRadios = document.querySelectorAll('input[name="search-mode"]');
  const colorSwatches = document.querySelectorAll('.color-swatch');

  if (!input || !results || !status || !modeRadios.length || !colorSwatches.length) {
    return;
  }

  const { searchIndex: searchIndexHref = "", siteHome: siteHomeHref = "" } = document.body.dataset;
  const indexUrl = new URL(searchIndexHref, window.location.href);
  const siteRootUrl = new URL(siteHomeHref || ".", window.location.href);
  let entries = [];

  const escapeHtml = (value) =>
    value.replace(/[&<>\"']/g, (c) => ({
      "&": "&amp;",
      "<": "&lt;",
      ">": "&gt;",
      "\"": "&quot;",
      "'": "&#39;"
    })[c]);

  const resolveEntryUrl = (entryUrl) => new URL(entryUrl || "", siteRootUrl).toString();

  const highlightText = (text, terms) => {
    let result = text;
    for (const term of terms) {
      const escaped = escapeHtml(term);
      const regex = new RegExp(escaped.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'gi');
      result = result.replace(regex, (match) => '<mark>' + match + '</mark>');
    }
    return result;
  };

  const getBestMatchType = (entry, terms, mode) => {
    const title = (entry.title || "").toLowerCase();
    const tags = (entry.tags || []).join(" ").toLowerCase();
    const summary = (entry.summary || "").toLowerCase();
    const body = (entry.body || "").toLowerCase();
    const headings = (entry.headings || []).join(" ").toLowerCase();

    if (mode === "title") {
      for (const t of terms) { if (title.includes(t)) return "标题匹配"; }
      return null;
    }
    if (mode === "tag") {
      for (const t of terms) { if (tags.includes(t)) return "标签匹配"; }
      return null;
    }
    for (const t of terms) {
      if (title.includes(t)) return "标题匹配";
      if (headings.includes(t)) return "标题匹配";
      if (tags.includes(t)) return "标签匹配";
      if (summary.includes(t)) return "摘要匹配";
      if (body.includes(t)) return "正文匹配";
    }
    return null;
  };

  const renderResults = (query, matchedEntries, mode) => {
    if (!query) {
      results.hidden = true;
      results.innerHTML = "";
      status.textContent = "Search titles, tags, headings, and body text from every page.";
      return;
    }

    results.hidden = false;

    if (matchedEntries.length === 0) {
      results.innerHTML = "<div class=\"search-result\"><strong>No results</strong><p>Try a shorter keyword, a different mode, or a tag name.</p></div>";
      status.textContent = "No pages matched " + query + ".";
      return;
    }

    const terms = query.trim().toLowerCase().split(/\s+/).filter(Boolean);

    const html = matchedEntries.map((entry) => {
      const bestType = getBestMatchType(entry, terms, mode);
      const metaTags = [];
      if (bestType) metaTags.push(bestType);
      for (const t of entry.tags.slice(0, 3)) if (!metaTags.includes(t)) metaTags.push(t);
      for (const h of entry.headings.slice(0, 2)) if (!metaTags.includes(h)) metaTags.push(h);
      const meta = metaTags.slice(0, 4).map((label) =>
        '<span>' + escapeHtml(label) + '</span>'
      ).join("");

      const rawSummary = entry.summary || "Open this page to explore the matching section.";
      const highlightedSummary = highlightText(escapeHtml(rawSummary), terms);

      return (
        '<a class="search-result" href="' + escapeHtml(resolveEntryUrl(entry.url)) + '">' +
          '<strong>' + escapeHtml(entry.title) + '</strong>' +
          '<p>' + highlightedSummary + '</p>' +
          (meta ? '<div class="search-meta">' + meta + '</div>' : "") +
        '</a>'
      );
    }).join("");

    results.innerHTML = html;
    status.textContent = matchedEntries.length + " result(s) for " + query + ".";
  };

  const scoreEntry = (entry, terms, mode) => {
    const title = (entry.title || "").toLowerCase();
    const summary = (entry.summary || "").toLowerCase();
    const body = (entry.body || "").toLowerCase();
    const headings = (entry.headings || []).join(" ").toLowerCase();
    const tags = (entry.tags || []).join(" ").toLowerCase();
    const combined = title + " " + summary + " " + body + " " + headings + " " + tags;

    if (mode === "title") {
      for (const t of terms) if (!title.includes(t)) return -1;
      let score = 0;
      for (const t of terms) if (title.includes(t)) score += 6;
      return score;
    }
    if (mode === "tag") {
      for (const t of terms) if (!tags.includes(t)) return -1;
      let score = 0;
      for (const t of terms) if (tags.includes(t)) score += 4;
      return score;
    }
    // mode === "all"
    let score = 0;
    for (const t of terms) {
      if (!combined.includes(t)) return -1;
      if (title.includes(t)) score += 6;
      if (headings.includes(t)) score += 5;
      if (tags.includes(t)) score += 4;
      if (summary.includes(t)) score += 3;
      if (body.includes(t)) score += 1;
    }
    return score;
  };

  const search = (query) => {
    const normalized = query.trim().toLowerCase();
    if (!normalized) {
      renderResults("", [], getCurrentMode());
      return;
    }

    const terms = normalized.split(/\s+/).filter(Boolean);
    const mode = getCurrentMode();
    const matchedEntries = entries
      .map((e) => ({ entry: e, score: scoreEntry(e, terms, mode) }))
      .filter(({ score }) => score >= 0)
      .sort((a, b) => b.score - a.score || a.entry.title.localeCompare(b.entry.title))
      .slice(0, 8)
      .map(({ entry }) => entry);

    renderResults(normalized, matchedEntries, mode);
  };

  const getCurrentMode = () => {
    for (const radio of modeRadios) {
      if (radio.checked) return radio.value;
    }
    return "all";
  };

  // ---- 颜色选择器逻辑 ----
  const highlightStorageKey = "minizensical-highlight-color";
  const root = document.documentElement;

  const applyHighlightColor = (color) => {
    if (color === "inherit") {
      root.style.removeProperty("--search-highlight");
    } else {
      root.style.setProperty("--search-highlight", color);
    }
    colorSwatches.forEach((swatch) => {
      const isActive = swatch.dataset.color === color;
      swatch.classList.toggle("is-active", isActive);
    });
    try {
      window.localStorage.setItem(highlightStorageKey, color);
    } catch (_) {}
  };

  const loadSavedHighlightColor = () => {
    try {
      const saved = window.localStorage.getItem(highlightStorageKey);
      if (saved) applyHighlightColor(saved);
    } catch (_) {}
  };

  colorSwatches.forEach((swatch) => {
    swatch.addEventListener("click", () => {
      applyHighlightColor(swatch.dataset.color);
    });
  });

  loadSavedHighlightColor();
  if (!window.localStorage.getItem(highlightStorageKey)) {
    applyHighlightColor("#ffeb3b");
  }

  // ---- 模式切换 ----
  modeRadios.forEach((radio) => {
    radio.addEventListener("change", () => search(input.value));
  });

  // ---- 加载搜索索引 ----
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
  <input id="doc-search" class="search-input" type="search" placeholder="Search titles, headings, tags, and content">
  <div class="search-mode">
    <label class="mode-option"><input type="radio" name="search-mode" value="all" checked> All</label>
    <label class="mode-option"><input type="radio" name="search-mode" value="title"> Title</label>
    <label class="mode-option"><input type="radio" name="search-mode" value="tag"> Tags</label>
  </div>
  <div class="highlight-color">
    <span class="color-label">Highlight:</span>
    <button type="button" class="color-swatch" data-color="#ffeb3b" style="background:#ffeb3b" aria-label="Yellow"></button>
    <button type="button" class="color-swatch" data-color="#a8e6cf" style="background:#a8e6cf" aria-label="Green"></button>
    <button type="button" class="color-swatch" data-color="#90caf9" style="background:#90caf9" aria-label="Blue"></button>
    <button type="button" class="color-swatch" data-color="#f8bbd0" style="background:#f8bbd0" aria-label="Pink"></button>
    <button type="button" class="color-swatch" data-color="#ffcc80" style="background:#ffcc80" aria-label="Orange"></button>
    <button type="button" class="color-swatch" data-color="inherit" style="background:var(--accent-soft);border-color:var(--accent)" aria-label="Theme default">Default</button>
  </div>
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

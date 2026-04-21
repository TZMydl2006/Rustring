use crate::config::Config;
use crate::error::Result;
use crate::nav::{PageLink, RenderNavItem};
use crate::page::Page;
use minijinja::{Environment, context};
use std::path::PathBuf;

pub fn render_page(
    config: &Config,
    page: &Page,
    navigation: &[RenderNavItem],
    previous_page: Option<PageLink>,
    next_page: Option<PageLink>,
    home_href: &str,
    stylesheet_href: &str,
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
        <p class="search-hint">Try keywords like <code>front matter</code>, <code>search</code>, or <code>architecture</code>.</p>
        <div id="search-status" class="search-status">Search is ready as soon as the page loads.</div>
        <div id="search-results" class="search-results" hidden></div>
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
  font-family: "Avenir Next", "IBM Plex Sans", "Segoe UI", sans-serif;
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
    radial-gradient(circle at 0% 0%, rgba(196, 108, 59, 0.18), transparent 28%),
    radial-gradient(circle at 100% 20%, rgba(13, 109, 104, 0.18), transparent 30%),
    linear-gradient(180deg, #f7f7f1 0%, #edf1ee 100%);
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
  background: rgba(196, 108, 59, 0.14);
}

.ambient-b {
  right: 4vw;
  bottom: 60px;
  width: 260px;
  height: 260px;
  border-radius: 999px;
  background: rgba(13, 109, 104, 0.12);
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

.search-label {
  display: block;
  margin-bottom: 10px;
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
  background: rgba(255, 255, 255, 0.86);
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
  background: rgba(255, 255, 255, 0.92);
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
  background:
    linear-gradient(145deg, rgba(13, 109, 104, 0.96), rgba(8, 72, 69, 0.94)),
    linear-gradient(135deg, rgba(255, 255, 255, 0.05), transparent);
  color: white;
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
  color: rgba(255, 255, 255, 0.86);
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
  color: white;
  border: 1px solid rgba(255, 255, 255, 0.16);
}

.hero-grid {
  display: grid;
  gap: 18px;
}

.hero-card {
  background: linear-gradient(180deg, rgba(255, 255, 253, 0.96), rgba(246, 249, 246, 0.92));
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
  background: #102123;
  color: #eef5f3;
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
"#;

const SEARCH_SCRIPT: &str = r#"
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
    value.replace(/[&<>\"']/g, (character) => ({
      "&": "&amp;",
      "<": "&lt;",
      ">": "&gt;",
      "\"": "&quot;",
      "'": "&#39;"
    })[character]);

  const resolveEntryUrl = (entryUrl) => new URL(entryUrl || "", siteRootUrl).toString();

  const renderResults = (query, matchedEntries) => {
    if (!query) {
      results.hidden = true;
      results.innerHTML = "";
      status.textContent = "Search titles, tags, headings, and body text from every page.";
      return;
    }

    results.hidden = false;

    if (matchedEntries.length === 0) {
      results.innerHTML = "<div class=\"search-result\"><strong>No results</strong><p>Try a shorter keyword, a tag, or a heading name.</p></div>";
      status.textContent = `No pages matched “${query}”.`;
      return;
    }

    const html = matchedEntries.map((entry) => {
      const meta = [...entry.tags.slice(0, 3), ...entry.headings.slice(0, 2)].map((label) =>
        `<span>${escapeHtml(label)}</span>`
      ).join("");

      return `
        <a class="search-result" href="${escapeHtml(resolveEntryUrl(entry.url))}">
          <strong>${escapeHtml(entry.title)}</strong>
          <p>${escapeHtml(entry.summary || "Open this page to explore the matching section.")}</p>
          ${meta ? `<div class="search-meta">${meta}</div>` : ""}
        </a>
      `;
    }).join("");

    results.innerHTML = html;
    status.textContent = `${matchedEntries.length} result(s) for “${query}”.`;
  };

  const scoreEntry = (entry, terms) => {
    const title = (entry.title || "").toLowerCase();
    const summary = (entry.summary || "").toLowerCase();
    const body = (entry.body || "").toLowerCase();
    const headings = (entry.headings || []).join(" ").toLowerCase();
    const tags = (entry.tags || []).join(" ").toLowerCase();
    const combined = `${title} ${summary} ${body} ${headings} ${tags}`;

    let score = 0;
    for (const term of terms) {
      if (!combined.includes(term)) {
        return -1;
      }
      if (title.includes(term)) score += 6;
      if (headings.includes(term)) score += 5;
      if (tags.includes(term)) score += 4;
      if (summary.includes(term)) score += 3;
      if (body.includes(term)) score += 1;
    }

    return score;
  };

  const search = (query) => {
    const normalized = query.trim().toLowerCase();
    if (!normalized) {
      renderResults("", []);
      return;
    }

    const terms = normalized.split(/\s+/).filter(Boolean);
    const matchedEntries = entries
      .map((entry) => ({ entry, score: scoreEntry(entry, terms) }))
      .filter(({ score }) => score >= 0)
      .sort((left, right) => right.score - left.score || left.entry.title.localeCompare(right.entry.title))
      .slice(0, 8)
      .map(({ entry }) => entry);

    renderResults(normalized, matchedEntries);
  };

  status.textContent = "Loading the search index...";

  fetch(indexUrl, { cache: "no-store" })
    .then((response) => {
      if (!response.ok) {
        throw new Error(`Failed to load search index: ${response.status}`);
      }
      return response.json();
    })
    .then((data) => {
      entries = Array.isArray(data) ? data : [];
      status.textContent = `Search is ready across ${entries.length} page(s).`;
      search(input.value);
    })
    .catch(() => {
      status.textContent = "Search index could not be loaded. Use `cargo run -- serve` or deploy the built site through HTTP.";
    });

  input.addEventListener("input", (event) => {
    search(event.target.value);
  });
})();
"#;

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

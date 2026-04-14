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
) -> Result<String> {
    let mut environment = Environment::new();
    environment.add_template("main.html", MAIN_TEMPLATE)?;

    let template = environment.get_template("main.html")?;
    let nav_html = render_navigation_html(navigation);
    let rendered = template.render(context! {
        site_name => config.project.site_name.clone(),
        title => page.title.clone(),
        canonical_url => page.canonical_url.clone(),
        nav_html => nav_html,
        page => context! {
            title => page.title.clone(),
            content => page.html.clone(),
            toc => page.toc.clone(),
        },
        previous_page => previous_page,
        next_page => next_page,
        home_href => home_href.to_string(),
        stylesheet_href => stylesheet_href.to_string(),
    })?;

    Ok(rendered)
}

pub fn stylesheet_path() -> PathBuf {
    PathBuf::from("assets/minizensical.css")
}

pub fn stylesheet_contents() -> &'static str {
    STYLE_SHEET
}

const MAIN_TEMPLATE: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{{ title }} - {{ site_name }}</title>
  {% if canonical_url %}
  <link rel="canonical" href="{{ canonical_url }}">
  {% endif %}
  <link rel="stylesheet" href="{{ stylesheet_href }}">
</head>
<body>
  <div class="shell">
    <aside class="sidebar">
      <a class="brand" href="{{ home_href }}">{{ site_name }}</a>
      {{ nav_html | safe }}
    </aside>
    <main class="content">
      <article class="page">
        <header class="page-header">
          <p class="eyebrow">MiniZensical</p>
          <h1>{{ page.title }}</h1>
        </header>
        <div class="page-body">
          {{ page.content | safe }}
        </div>
        <nav class="pager">
          {% if previous_page %}
          <a class="pager-link" href="{{ previous_page.href }}">Previous: {{ previous_page.title }}</a>
          {% else %}
          <span></span>
          {% endif %}
          {% if next_page %}
          <a class="pager-link" href="{{ next_page.href }}">Next: {{ next_page.title }}</a>
          {% endif %}
        </nav>
      </article>
    </main>
    <aside class="toc">
      <div class="toc-card">
        <h2>On this page</h2>
        {% if page.toc | length > 0 %}
        <ul class="toc-list">
          {% for item in page.toc %}
          <li class="toc-level-{{ item.level }}">
            <a href="{{ item.href }}">{{ item.title }}</a>
          </li>
          {% endfor %}
        </ul>
        {% else %}
        <p class="toc-empty">Add headings to generate a table of contents.</p>
        {% endif %}
      </div>
    </aside>
  </div>
</body>
</html>
"#;

const STYLE_SHEET: &str = r#"
:root {
  color-scheme: light;
  --bg: #f4efe6;
  --panel: rgba(255, 252, 246, 0.9);
  --panel-strong: #fffdf8;
  --text: #1e1c18;
  --muted: #6a6255;
  --accent: #b55d32;
  --accent-soft: rgba(181, 93, 50, 0.12);
  --line: rgba(44, 33, 18, 0.12);
  --shadow: 0 18px 50px rgba(62, 45, 27, 0.08);
  font-family: "IBM Plex Sans", "Segoe UI", sans-serif;
}

* {
  box-sizing: border-box;
}

body {
  margin: 0;
  background:
    radial-gradient(circle at top left, rgba(244, 202, 163, 0.45), transparent 30%),
    linear-gradient(180deg, #f8f1e7 0%, #f0e7da 100%);
  color: var(--text);
}

a {
  color: inherit;
}

.shell {
  display: grid;
  grid-template-columns: minmax(220px, 280px) minmax(0, 1fr) minmax(200px, 260px);
  gap: 24px;
  min-height: 100vh;
  padding: 24px;
}

.sidebar,
.toc-card,
.page {
  background: var(--panel);
  backdrop-filter: blur(12px);
  border: 1px solid var(--line);
  border-radius: 24px;
  box-shadow: var(--shadow);
}

.sidebar {
  padding: 24px 20px;
  position: sticky;
  top: 24px;
  align-self: start;
}

.brand {
  display: inline-block;
  margin-bottom: 18px;
  font-size: 1.25rem;
  font-weight: 700;
  color: var(--accent);
  text-decoration: none;
}

.nav-list {
  list-style: none;
  padding-left: 0;
  margin: 0;
}

.nav-item {
  margin: 8px 0;
}

.nav-item > a,
.nav-section {
  display: inline-block;
  padding: 6px 10px;
  border-radius: 12px;
  text-decoration: none;
  color: var(--muted);
}

.nav-item.active > a,
.nav-item.active > .nav-section {
  background: var(--accent-soft);
  color: var(--accent);
  font-weight: 600;
}

.nav-item .nav-list {
  margin-left: 14px;
  padding-left: 12px;
  border-left: 1px solid var(--line);
}

.page {
  padding: 32px;
}

.eyebrow {
  margin: 0 0 10px;
  color: var(--accent);
  font-size: 0.8rem;
  letter-spacing: 0.16em;
  text-transform: uppercase;
}

.page-header h1 {
  margin: 0;
  font-size: clamp(2rem, 3vw, 3rem);
}

.page-body {
  margin-top: 28px;
  line-height: 1.75;
}

.page-body h1,
.page-body h2,
.page-body h3,
.page-body h4 {
  line-height: 1.2;
  scroll-margin-top: 24px;
}

.page-body pre,
.page-body code {
  font-family: "IBM Plex Mono", "SFMono-Regular", monospace;
}

.page-body pre {
  padding: 16px;
  overflow-x: auto;
  border-radius: 16px;
  background: #1f1a16;
  color: #f7f2eb;
}

.page-body :not(pre) > code {
  padding: 0.15em 0.45em;
  border-radius: 8px;
  background: rgba(32, 23, 11, 0.08);
}

.page-body table {
  width: 100%;
  border-collapse: collapse;
}

.page-body th,
.page-body td {
  border: 1px solid var(--line);
  padding: 10px 12px;
  text-align: left;
}

.page-body img {
  max-width: 100%;
  border-radius: 18px;
}

.pager {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  margin-top: 32px;
  padding-top: 20px;
  border-top: 1px solid var(--line);
}

.pager-link {
  text-decoration: none;
  color: var(--accent);
  font-weight: 600;
}

.toc {
  position: sticky;
  top: 24px;
  align-self: start;
}

.toc-card {
  padding: 20px;
}

.toc-card h2 {
  margin-top: 0;
  font-size: 1rem;
}

.toc-list {
  list-style: none;
  margin: 0;
  padding: 0;
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
  padding-left: 20px;
}

.toc-empty {
  color: var(--muted);
  margin-bottom: 0;
}

@media (max-width: 1100px) {
  .shell {
    grid-template-columns: 240px minmax(0, 1fr);
  }

  .toc {
    display: none;
  }
}

@media (max-width: 780px) {
  .shell {
    grid-template-columns: 1fr;
  }

  .sidebar {
    position: static;
  }

  .page {
    padding: 24px;
  }
}
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

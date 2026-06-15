# AGENTS.md

## Project Overview

Rustring is a Rust static documentation site generator. The Cargo package is currently named `minizensical`, while the installed CLI binary is named `rustring`.

The primary workflow is:

```text
zensical.toml + docs/ -> rustring build/serve -> site/
```

Users install the CLI with Cargo and run it from their own documentation project. Do not assume user documents live inside this repository.

## Start Here

Before changing code:

1. Run `git status --short --branch` and preserve unrelated user changes.
2. Read `Cargo.toml`, `src/main.rs`, and the module relevant to the task.
3. Treat `README.md` as the user manual and this file as developer context.
4. Never edit generated files under `site/`; change `docs/`, `src/`, or `zensical.toml` instead.

The default/global Python installation is managed by `uv`. If an auxiliary Python command needs a missing package outside a project environment, prefer `uv run --with <package> ...` instead of `pip` or `conda`.

## Architecture

- `src/main.rs`: Clap CLI entry point for `init`, `build`, and `serve`.
- `src/config.rs`: loads and validates `zensical.toml`; project paths are rooted at the config file directory.
- `src/init.rs`: creates a default config and `docs/index.md` without overwriting existing files.
- `src/scanner.rs`: recursively separates Markdown sources from copied static assets.
- `src/markdown.rs`: parses YAML front matter, Markdown, math, headings, search blocks, and document links.
- `src/page.rs`: creates the page model, output paths, metadata, search data, and canonical URLs.
- `src/nav.rs`: builds automatic or explicit navigation and previous/next ordering.
- `src/search.rs`: creates the static search index.
- `src/graph.rs`: creates document/tag nodes and graph relationships.
- `src/render.rs`: owns HTML templates, CSS, and embedded browser scripts.
- `src/build.rs`: orchestrates atomic site generation, archives, graph output, rendering, and asset copying.
- `src/server.rs`: preview HTTP server, file watching, rebuilds, and live reload.
- `tests/build.rs`: end-to-end build behavior and generated-output assertions.
- `vendor/d3/`: vendored D3 runtime and its license; keep graph rendering offline-capable.

## Behavioral Invariants

- `Config::root_dir` is the directory containing the selected `zensical.toml`.
- `docs_dir` and `site_dir` are safe relative paths under that root. Absolute paths and parent traversal are rejected.
- `docs_dir` and `site_dir` must differ.
- `index.md` and `README.md` map to directory index pages.
- With directory URLs enabled, a normal Markdown file maps to `<name>/index.html`; otherwise it maps to `<name>.html`.
- A failed build must preserve the last successful `site_dir`; keep the staging/backup replacement flow intact.
- `serve` must reload the config and source tree, rebuild on changes, and keep serving the last successful build after failures.
- Search, archive, theme, font, code, math, and knowledge-graph behavior are public user features. Avoid regressions when changing shared rendering or page models.
- D3 is vendored locally. Do not introduce a runtime CDN dependency for the knowledge graph.

## Content Contracts

Supported front matter fields are:

```yaml
title: Optional page title
summary: Optional description and search excerpt
tags: [optional, list]
date: 2026-06-16
order: 1
```

`title` overrides the first Markdown H1. `order` affects automatic navigation only; an index page's order controls its directory group. `date` feeds date archives, and `tags` feed tag archives and the graph.

Explicit navigation entries define either `path` or `children`, never both. Paths are relative to `docs_dir`, and missing or duplicated pages are errors.

Non-Markdown files under `docs_dir` are copied to the same relative output path. Font files under `docs/assets/fonts/` are additionally exposed through the generated font selector.

## Development Guidelines

- Prefer the existing module boundaries and direct, readable Rust over new abstraction layers.
- Use structured TOML/YAML/Markdown parsing instead of string matching where practical.
- Keep changes scoped. This repository is developed collaboratively, so avoid unrelated formatting and broad refactors.
- Add or update focused tests for public behavior, path handling, generated files, and failure recovery.
- Preserve existing user changes, including uncommitted files, unless explicitly asked to modify them.
- When public commands, configuration, front matter, output layout, or user-visible behavior changes, update `README.md` in the same change.
- Keep stable developer guidance here; do not add chronological implementation diaries or temporary task notes.

## Verification

For normal Rust changes, run:

```bash
cargo test
cargo build --release
```

For CLI, configuration, build, server, path, or packaging changes, also test outside the repository:

```bash
tmpdir=$(mktemp -d)
cd "$tmpdir"
/absolute/path/to/repo/target/release/rustring init
/absolute/path/to/repo/target/release/rustring build
/absolute/path/to/repo/target/release/rustring serve
```

Verify that `zensical.toml`, `docs/index.md`, `site/index.html`, `site/search.json`, `site/graph.json`, and `site/knowledge-graph/index.html` are created. For server changes, request the home page and knowledge-graph page over HTTP.

Documentation-only changes should still be checked against the current CLI help, config validation, page metadata types, and relevant tests. Run `git diff --check` before finishing.

## Current Distribution

The supported installation command is:

```bash
cargo install --git https://github.com/TZMydl2006/Rustring.git --bin rustring
```

There is currently no PyPI package, Homebrew formula, prebuilt binary release, or automated cross-platform release pipeline. Do not document those as available until they exist and have been tested.

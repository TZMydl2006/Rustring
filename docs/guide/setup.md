---
title: Setup & Preview
summary: The shortest path from editing Markdown to showing a refreshed page in the browser.
tags:
  - guide
  - preview
  - setup
date: 2024-12-19
order: 1
---
# Setup

You only need a Rust toolchain to work with MiniZensical.

## Daily workflow

1. Edit Markdown or assets inside `docs/`
2. Run `cargo run -- serve`
3. Keep the browser open while you work

## Commands

Generate the static site:

```bash
cargo run -- build
```

Generate the site and start the local preview server:

```bash
cargo run -- serve
```

When `serve` is running, editing Markdown files, static assets, or `zensical.toml` triggers an automatic rebuild, and the browser page refreshes automatically after a successful rebuild.

## Why this matters for demos

During a classroom presentation, this workflow makes it easy to:

- edit a page
- save the file
- show the new page state immediately

That makes the project feel much more alive than a one-time static export.

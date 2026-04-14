# MiniZensical

MiniZensical is a tiny documentation site generator written in Rust.

## Why this project

We keep the core zensical workflow:

- read `zensical.toml`
- scan `docs/`
- turn Markdown into HTML pages
- build navigation and a page table of contents
- emit a static site into `site/`

## What is included

This first-stage MVP focuses on build-time generation instead of live preview.

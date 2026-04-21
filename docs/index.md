---
title: MiniZensical
summary: A Rust course project that keeps the zensical workflow and adds searchable docs, front matter, tags, and a more presentation-ready reading experience.
tags:
  - home
  - rust
  - search
order: 0
---
# MiniZensical

MiniZensical is our course-friendly version of zensical. It keeps the core pipeline:

- read `zensical.toml`
- scan Markdown and static assets from `docs/`
- build HTML pages into `site/`
- preview locally with `cargo run -- serve`

## What is new in phase 2

The second stage adds features that make the project easier to show in class:

- instant client-side search
- YAML front matter for page metadata
- tags and summaries at the page level
- a more polished reading layout for demos

## Search ideas for the demo

Try these queries in the search box:

- `front matter`
- `architecture`
- `preview`
- `school badge`

## Static resource example

This image is copied from `docs/assets/交大校徽-蓝色.png` to `site/assets/交大校徽-蓝色.png`.

![西安交大校徽](assets/交大校徽-蓝色.png)

[查看原图](assets/交大校徽-蓝色.png)

## Suggested presentation flow

1. Open the homepage and explain that this is a minimal zensical-inspired generator.
2. Search for `front matter` or `architecture`.
3. Open the guide pages to show metadata and automatic ordering.
4. Open the project showcase page to explain your innovation story.

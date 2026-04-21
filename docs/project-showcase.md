---
title: Project Showcase
summary: "A short story for the course presentation: what we kept from zensical, what we added, and how the team can explain the design."
tags:
  - course
  - showcase
  - architecture
order: 1
---
# Project Showcase

This page is meant for the final presentation.

## What we kept from zensical

We preserved the core static-site workflow:

- configuration in `zensical.toml`
- Markdown content in `docs/`
- generated output in `site/`

## What we added

Our version adds three visible upgrades:

1. `search.json` plus a live search box
2. YAML front matter for summaries, tags, and ordering
3. A more expressive UI for course demos

## Architecture

The Rust side still follows a clear pipeline:

1. parse configuration
2. scan source files
3. build page models
4. generate navigation and search data
5. render the final static site

## Team talking points

- one member explains configuration and scanning
- one member explains Markdown parsing and front matter
- one member explains navigation and UI rendering
- one member explains serving, testing, and integration

## Demo keywords

If you want to showcase the search feature, try these words:

- `architecture`
- `metadata`
- `preview`
- `search`

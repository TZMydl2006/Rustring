---
title: Guide Overview
summary: The guide section explains the everyday workflow for building, previewing, and extending MiniZensical.
tags:
  - guide
  - workflow
order: 0
---
# Guide Overview

This section is for teammates who are new to the repo.

## Build command

Run:

```bash
cargo run -- build
```

## Local preview

Run:

```bash
cargo run -- serve
```

Then open:

```text
http://127.0.0.1:3000
```

The preview server rebuilds automatically when files in `docs/` or `zensical.toml` change, and the open browser page reloads after a successful rebuild.

## What to open next

- `Setup & Preview` for the daily workflow
- `Front Matter` for metadata fields like `summary`, `tags`, and `order`
- `Resources` for images and static files

## Theme switching

Use the sidebar buttons to switch between `Day`, `Night`, and `System` themes. `System` follows the browser or operating system preference, and the last manual choice is saved in the browser.

---
title: Front Matter
summary: Use YAML front matter to control page titles, summaries, tags, and ordering without editing Rust code.
tags:
  - guide
  - metadata
  - front matter
date: 2025-01-20
order: 2
---
# Front Matter

MiniZensical now supports a small YAML front matter block at the top of a Markdown file.

## Supported fields

- `title`
- `summary`
- `tags`
- `order`

## Example

```md
---
title: Project Showcase
summary: Explain the architecture and innovation highlights.
tags:
  - course
  - architecture
order: 1
---
# This H1 can still exist
```

## Rules

- `title` overrides the first `# H1`
- `summary` is shown near the top of the page and reused as the page description
- `tags` appear as chips on the page and feed the tag archive
- `order` changes how sibling pages are arranged in automatic navigation

## When to use it

Use front matter when you want clearer navigation, better archive pages, and a more polished document presentation without changing Rust code.

---
title: Markdown Link Graph Test
summary: A dedicated page for verifying directed Markdown reference edges in the knowledge graph.
tags:
  - graph-link-test
date: 2026-06-11
order: 90
---
# Markdown Link Graph Test

This page is dedicated to testing Markdown document references in the knowledge graph.
Its unique `graph-link-test` tag avoids creating shared-tag edges with the existing
documents, so the outgoing reference edges are easy to identify.

## Same-directory references

- [Open Hello, World!](helloworld.md)
- [Open Project Showcase](project-showcase.md "Course presentation page")

## Nested-document references

- [Open Setup & Preview](guide/setup.md)
- [Open Front Matter metadata fields](guide/front-matter.md#supported-fields)

## Expected graph result

After running `cargo run -- build`, open the Knowledge Graph page and locate
`Markdown Link Graph Test`. It should have directed solid-line references to:

1. `Hello, World!`
2. `Project Showcase`
3. `Setup & Preview`
4. `Front Matter`

The arrow direction is from this test document to each referenced document. A dashed
line represents a shared front matter tag instead, so this test page intentionally
does not reuse the tags of those four target documents.

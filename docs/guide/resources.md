---
title: Resources
summary: Images, PDFs, and other files placed under docs/ are copied into the generated site at the same relative path.
tags:
  - guide
  - assets
date: 2024-12-15
order: 3
---

# Resources

This page shows how to use images and other static files in MiniZensical.

## Rule of thumb

Any non-Markdown file inside `docs/` is copied to the same relative location inside `site/`.

Examples:

- `docs/assets/交大校徽-蓝色.png -> site/assets/交大校徽-蓝色.png`
- `docs/files/report.pdf -> site/files/report.pdf`

## Image syntax

Write local image paths relative to the Markdown source file, just as you would when previewing the Markdown file in an editor.

During the build, MiniZensical resolves the image from the Markdown file location and rewrites the generated HTML path so that it points to the copied asset inside `site/`.

Supported local styles:

Examples:

```md
![same folder](./image.png)
![parent assets folder](../assets/aaa.png)
![nested assets folder](assets/aaa.png)
```

External image URLs are preserved without rewriting:

```md
![external image](https://example.com/image.png)
```

Actual example on this page:

![badge](../assets/%E4%BA%A4%E5%A4%A7%E6%A0%A1%E5%BE%BD-%E8%93%9D%E8%89%B2.png)

## Link syntax

If you only want a clickable link instead of an embedded image:

```md
[view original](../../assets/aaa.png)
```

Only embedded image paths are relocated automatically. Keep ordinary file links relative to the generated page URL.

Actual example:

[查看校徽原图](../../assets/%E4%BA%A4%E5%A4%A7%E6%A0%A1%E5%BE%BD-%E8%93%9D%E8%89%B2.png)

## Practical advice

- Put reusable images in `docs/assets/`
- Write embedded image paths relative to the Markdown source file
- Run `cargo run -- build` for a one-time export
- If `cargo run -- serve` is already running, saving the file is enough to trigger a rebuild and refresh

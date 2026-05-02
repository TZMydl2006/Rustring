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

With `use_directory_urls = true` (default), image paths should be based on URL depth.

Practical rule:

- `docs/index.md` -> use `assets/aaa.png` (no `../`)
- Other Markdown files directly under `docs/` (for example `docs/project-showcase.md`) -> use `../assets/aaa.png`
- Files under `docs/xxx/` (for example `docs/guide/resources.md`) -> use `../../assets/aaa.png`
- Each extra folder level adds one more `../`

Examples:

```md
![example in docs/index.md](assets/aaa.png)
![example in docs/project-showcase.md](../assets/aaa.png)
![example in docs/guide/resources.md](../../assets/aaa.png)
```

Actual example on this page:

![badge](../../assets/%E4%BA%A4%E5%A4%A7%E6%A0%A1%E5%BE%BD-%E8%93%9D%E8%89%B2.png)

## Link syntax

If you only want a clickable link instead of an embedded image:

```md
[view original](../../assets/aaa.png)
```

Actual example:

[查看校徽原图](../../assets/%E4%BA%A4%E5%A4%A7%E6%A0%A1%E5%BE%BD-%E8%93%9D%E8%89%B2.png)

## Practical advice

- Put reusable images in `docs/assets/`
- Use paths relative to the output URL depth when `use_directory_urls = true`
- Run `cargo run -- build` for a one-time export
- If `cargo run -- serve` is already running, saving the file is enough to trigger a rebuild and refresh

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

The current version already supports two core workflows:

- `cargo run -- build` to generate the static site
- `cargo run -- serve` to build once and start a local preview server

## Add images and assets

Any non-Markdown file placed under `docs/` is copied to the same relative path inside `site/`.

For example, this image lives at:

```text
docs/assets/交大校徽-蓝色.png
```

On the homepage, the relative Markdown path is:

```md
![西安交大校徽](assets/交大校徽-蓝色.png)
```

The result looks like this:

![西安交大校徽](assets/交大校徽-蓝色.png)

You can also create a normal download/view link:

[查看原图](assets/交大校徽-蓝色.png)

More examples are collected on the `Guide / Resources` page.

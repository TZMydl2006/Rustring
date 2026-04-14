# Setup

You only need a Rust toolchain to build MiniZensical.

## Steps

1. Prepare `zensical.toml`
2. Place Markdown files inside `docs/`
3. Run the build or serve command

## Result

Open `site/index.html` in a browser to view the generated site.

## Commands

Generate the static site:

```bash
cargo run -- build
```

Generate the site and start the local preview server:

```bash
cargo run -- serve
```

## Adding images and resources

Place images, PDFs, or other static files anywhere under `docs/`.

For example:

```text
docs/assets/交大校徽-蓝色.png
```

Then reference it from Markdown using a relative path:

```md
![校徽](../assets/交大校徽-蓝色.png)
```

If you want a fuller example, open the `Resources` page in the guide.

# Resources

This page shows how to use images and other static files in MiniZensical.

## Rule of thumb

Any non-Markdown file inside `docs/` is copied to the same relative location inside `site/`.

Examples:

- `docs/assets/交大校徽-蓝色.png -> site/assets/交大校徽-蓝色.png`
- `docs/files/report.pdf -> site/files/report.pdf`

## Image syntax

If a page is in the root `docs/` directory, you can write:

```md
![校徽](assets/交大校徽-蓝色.png)
```

If a page is inside `docs/guide/`, you need to go up one directory:

```md
![校徽](../assets/交大校徽-蓝色.png)
```

Actual example on this page:

![校徽](../assets/交大校徽-蓝色.png)

## Link syntax

If you only want a clickable link instead of an embedded image:

```md
[查看校徽原图](../assets/交大校徽-蓝色.png)
```

Actual example:

[查看校徽原图](../assets/交大校徽-蓝色.png)

## Practical advice

- Put reusable images in `docs/assets/`
- Use paths relative to the current Markdown file
- After adding a new image or resource, run `cargo run -- build` or `cargo run -- serve` again

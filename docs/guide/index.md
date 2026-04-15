# Guide Overview

This section demonstrates nested navigation.

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

The preview server now rebuilds automatically when files in `docs/` or `zensical.toml` change, and the open browser page will reload automatically after a successful rebuild.

## Output

The generated site is written into the `site/` directory.

## Next pages

- `Setup` explains the basic project workflow
- `Resources` shows how to reference images and other static files

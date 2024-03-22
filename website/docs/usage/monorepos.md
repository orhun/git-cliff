---
sidebar_position: 5
---

# Monorepos

You can generate a changelog scoped to a specific directory via `--include-path` and `--exclude-path`.

This requires changing the current working directory to the target folder. The included/excluded paths must be relative to the repository's root.

```bash
cd packages/some_library
git cliff --include-path "packages/some_library/**/*" --repository "../../"
```

```bash
cd packages/some_library
git cliff --include-path "packages/some_library/**/*" --repository "../../" --exclude-path ".github/*"
```

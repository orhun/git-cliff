---
sidebar_position: 5
---

# Monorepos

You can generate a changelog scoped to a specific directory by just switching to that directory:

```bash
cd packages/some_library
git cliff
```

To include/exclude specific paths, use the `--include-path` and `--exclude-path` arguments:

```bash
cd packages/some_library
git cliff --include-path "packages/some_library/**/*" --exclude-path ".github/*"
```

These paths must be relative to the repository's root and should be a valid glob pattern.

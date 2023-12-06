---
sidebar_position: 5
---

# Monorepos

You can generate a changelog scoped to a specific directory:

```bash
git cliff --include-path "**/*.toml" --include-path "*.md"
git cliff --exclude-path ".github/*"
```

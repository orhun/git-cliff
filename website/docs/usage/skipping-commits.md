---
sidebar_position: 7
---

# Skipping commits

You can use `--skip-commit` argument to skip specific commits by their SHA1 value:

```bash
git cliff --skip-commit a78bc368e9ee382a3016c0c4bab41f7de4503bcd
```

If you have multiple commits to skip, you can either use this argument multiple times or create `.cliffignore` at the root of your repository.

For example:

```bash
# contents of .cliffignore

4f88dda8c746173ea59f920b7579b7f6c74bd6c8
10c3194381f2cc4f93eb97404369568882ed8677
```

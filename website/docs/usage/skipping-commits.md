---
sidebar_position: 8
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

## Git blame ignore revisions

If your repository contains a [`.git-blame-ignore-revs`](https://git-scm.com/docs/git-blame#Documentation/git-blame.txt---ignore-revs-fileltfilegt) file at its root, git-cliff automatically:

- skips the commit hashes listed in that file
- skips commits that only modify `.git-blame-ignore-revs`

Comments and blank lines in the file are ignored, similar to `.cliffignore`.

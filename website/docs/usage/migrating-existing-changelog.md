---
sidebar_position: 14
---

# Migrating an Existing Changelog

If you have an existing `CHANGELOG.md` and want to start using `git-cliff`
without losing your legacy history, you can do so by aligning your header and
using the `--prepend` flag.

## Prerequisites

`git-cliff` finds the injection point by matching the rendered `header` value
from your config. It strips that header, prepends the newly generated entries,
and re-inserts it. For this to work your existing `CHANGELOG.md` **must** start
with the exact same text that the `header` template in your `cliff.toml`
renders to.

For example, if your config contains:

```toml
[changelog]
header = """
# Changelog\n
All notable changes to this project will be documented in this file.\n
"""
```

then the top of your `CHANGELOG.md` must be exactly those lines.

:::tip
Keep your `header` value as **static text** (no Tera template variables).
If the header changes between runs, `--prepend` cannot find the insertion point.
:::

## Prepending New Releases

Once the header matches, use `--prepend` together with `--unreleased` so
`git-cliff` only generates entries for commits that have not been released yet,
avoiding duplicate sections:

```bash
# append unreleased commits to the top of an existing changelog
git cliff --unreleased --prepend CHANGELOG.md

# same, but also set the tag for the upcoming release
git cliff --unreleased --tag 1.2.0 --prepend CHANGELOG.md
```

## Limitations

| Issue | Cause | Workaround |
|-------|-------|------------|
| `--prepend` inserts nothing / fails | `header` in config does not match the file's opening text exactly | Copy the rendered header into the file verbatim |
| Duplicate release sections | Running without `--unreleased` causes git-cliff to process the full history | Always pair `--prepend` with `--unreleased` |
| Full overwrite of legacy history | Using `-o` / `--output` instead of `--prepend` | Use `--prepend` for existing changelogs; reserve `-o` for fresh ones |

## Marking the Legacy Section (Optional)

To make the boundary between generated and legacy content visible, add an HTML
comment just above the old entries:

```markdown
<!-- changelog managed by git-cliff below this line -->
```

This comment is not processed by `git-cliff` — it exists purely as a
human-readable marker.
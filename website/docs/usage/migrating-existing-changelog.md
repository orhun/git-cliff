---
sidebar_position: 14
---

# Migrating an existing changelog

You can start using **git-cliff** without replacing the release history that is
already in `CHANGELOG.md`. The usual migration flow is to keep the existing
entries in place and use [`--prepend`](/docs/usage/args) for new releases.

Before the first migration, commit or otherwise back up `CHANGELOG.md`. This
makes it easy to inspect the first generated diff and restore the original file
if the configured header does not match the existing one.

## Choose how the existing header is recognized

When `--prepend` writes a new release, **git-cliff** removes the existing header
before writing the newly rendered header. How that header is found depends on
the configuration.

### Static headers

For a static header, configure [`changelog.header`](/docs/configuration/changelog#header)
to match the header that is already in the file.

For example, if the existing changelog starts with:

```md
# Changelog

## 1.0.0
```

use a matching header:

```toml title="cliff.toml"
[changelog]
header = "# Changelog\n"
```

On prepend, **git-cliff** removes the first matching configured header and then
writes the newly generated content before the old release entries. If the
configured text does not match the existing header, the old header is preserved
and you can end up with two headers. Normalize the old header or adjust the
configuration before the first prepend.

### Headers that use template variables

Headers containing template variables can change between runs, so matching the
rendered header text is unreliable. Use
[`header_marker`](/docs/configuration/changelog#header_marker) instead:

```toml title="cliff.toml"
[changelog]
header = """
# Changelog

Tracked releases: {{ releases | length }}
"""
header_marker = "<!-- git-cliff: end of header -->"
```

For an existing changelog that predates **git-cliff**, add the marker once,
immediately after the old header:

```md
# Changelog
<!-- git-cliff: end of header -->

## 1.0.0
```

The first prepend removes everything through that marker and writes the current
rendered header plus a fresh marker. Future prepends can then find the header
boundary even when the rendered header changes.

Static headers do not emit `header_marker`; they continue to be matched using
their configured text.

## Prepend the first git-cliff release

Once the header is ready, generate only the release that should be added and
prepend it to the existing file. For example:

```bash
git cliff --unreleased --tag 1.1.0 --prepend CHANGELOG.md
```

`--prepend` must be used with `--unreleased`, `--latest`, or an explicit commit
range. The generated section does not include the configured footer, so the
footer already present at the bottom of the existing changelog stays in place.

After the first run, inspect the diff:

```bash
git diff -- CHANGELOG.md
```

Check that:

- the old releases are still present;
- there is only one changelog header;
- the new release appears above the old releases;
- any existing footer is still at the bottom.

## Limitations

- `--prepend` does not detect whether the same release is already present. If
  you run the same release command twice, you can create duplicate release
  entries.
- `--output CHANGELOG.md` and `--prepend CHANGELOG.md` cannot target the same
  file in one command. Use `--prepend` by itself when updating that file.
- The existing changelog is read as UTF-8 text.
- If a static configured header differs from the existing header, **git-cliff**
  cannot know which old text is the header. Fix that mismatch before the first
  prepend rather than relying on automatic detection.

Once the first migration diff looks correct, later releases can use the same
`--prepend` workflow without rewriting the older history.

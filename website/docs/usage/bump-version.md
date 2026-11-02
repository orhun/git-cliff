---
sidebar_position: 4
---

# Bump version

To calculate and set the next semantic version (i.e. _bump the version_) for the unreleased changes:

```bash
git cliff --bump
```

- Basic:
  - For example, if you have `1.0.0` and committed "feat: xyz", `git-cliff --bump --unreleased` will create a changelog for `1.1.0`.

- Tag prefixes:
    - Tag prefixes are also supported, for example `testing/v1.0.0-beta.1` can be updated to `testing/v1.0.0-beta.2`

How it works is that for a semantic versioning such as `<MAJOR>.<MINOR>.<PATCH>`:

- "fix:" -> increments `PATCH`
- "feat:" -> increments `MINOR`
- "scope!" (breaking changes) -> increments `MAJOR`

:::note

For unconventional commits, only the subject line (the first line) is used while calculating the next version.

:::

:::note

The next version is checked against the regex value set by [tag_pattern](/docs/configuration/git#tag_pattern).

:::

## Get version

You can also calculate and print the next semantic version to `stdout`:

```bash
git cliff --bumped-version
```

:::tip

You can also get the bumped version [from the context](/docs/usage/print-context) as follows:

```bash
git cliff --unreleased --bump --context | jq -r .[0].version
```

:::

## Bump to a specific version type

Optionally, you can specify a bump type in `--bump`:

```bash
git cliff --bump [major|minor|patch|prerelease]
```

## Prerelease versions

You can initialize a prerelease for a base version bump:

```bash
git cliff --bump --prerelease beta
```

This produces a version such as `1.3.0-beta.0`.

Use `--prerelease <identifier>` with `--bump` whenever you want to create or
continue a prerelease:

```bash
git cliff --bump --prerelease beta --bumped-version
# 1.2.0-beta.2 -> 1.2.0-beta.3
```

To increment an existing prerelease suffix without changing the base version:

```bash
git cliff --bump prerelease
```

Examples:

- `1.2.0-beta.0` -> `1.2.0-beta.1`
- `1.2.0-beta` -> `1.2.0-beta.1`
- `1.2.0-beta.2` -> `1.2.0-beta.3`

To promote an existing prerelease to its stable version, omit `--prerelease`:

```bash
git cliff --bump
```

This removes only the suffix, preserving the base version, tag prefix, and build
metadata. It does not recalculate from the last stable tag, create a Git tag, or
publish a release. To print only the stable version:

```bash
git cliff --bumped-version
```

Examples:

- `1.2.0-beta.2` -> `1.2.0`
- `1.2.0-rc.1+build.45` -> `1.2.0+build.45`

For an existing prerelease, promotion also takes precedence over
`--bump major|minor|patch` when no prerelease identifier is requested. For example,
`1.2.0-rc.1 --bump minor` produces `1.2.0`, not `1.3.0`.
Stable versions continue to bump normally.

The explicit `--bump prerelease` operation is an exception: it increments the
existing suffix without requiring `--prerelease`. It errors for a stable version
and cannot be combined with `--prerelease`. The configured
`bump_type = "prerelease"` has the same behavior.

## Zero-based versioning scheme

When working with a zero-based versioning scheme (i.e., `0.x.y` or `0.0.x`),
it is often desirable to preserve the leading zero even when introducing a breaking change.
A switch from `0` to `1` should indicate a higher API stability level.

You can modify the bumping rules to preserve the zero-based versioning scheme in the
[configuration file](/docs/configuration/bump).

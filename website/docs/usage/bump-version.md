---
sidebar_position: 4
---

# Bump version

To calculate and set the next semantic version (i.e. _bump the version_) for the unreleased changes:

```bash
git cliff --bump
```

For example, if you have `1.0.0` and committed "feat: xyz", `git-cliff --bump --unreleased` will create a changelog for `1.1.0`.

How it works is that for a semantic versioning such as `<MAJOR>.<MINOR>.<PATCH>`:

- "fix:" -> increments `PATCH`
- "feat:" -> increments `MINOR`
- "scope!" (breaking changes) -> increments `MAJOR`

You can also calculate and print the next semantic version to `stdout`:

```bash
git cliff --bumped-version
```

Tip: you can also get the bumped version [from the context](/docs/usage/print-context) as follows:

```bash
git cliff --unreleased --bump --context | jq -r .[0].version
```

## Zero-based versioning scheme

When working with a zero-based versioning scheme (i.e., `0.x.y` or `0.0.x`),
it is often desirable to preserve the leading zero even when introducing a breaking change.
A switch from `0` to `1` should indicate a higher API stability level.

You can modify the bumping rules to preserve the zero-based versioning scheme in the
[configuration file](/docs/configuration/bump).

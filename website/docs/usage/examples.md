---
sidebar_position: 3
---

# Examples

To simply create a changelog at your projects git root directory:

```bash
# same as running `git-cliff --config cliff.toml --repository .`
# same as running `git-cliff --workdir .`
git cliff
```

Set a tag for the unreleased changes:

```bash
# generates a changelog for the tag '1.0.0' without creating the tag itself
git cliff --tag 1.0.0
```

Generate a changelog for a certain part of git history:

```bash
# only takes the latest tag into account
git cliff --latest

# only takes the current tag into account
# useful if you checkout a specific tag (e.g. `git checkout v0.0.1`)
# (requires a tag to be present for the current commit (i.e. HEAD))
git cliff --current

# generate changelog for unreleased commits
git cliff --unreleased
git cliff --unreleased --tag 1.0.0
```

Generate a changelog for a specific commit range (based on [git ranges](https://git-scm.com/docs/git-range-diff)):

```bash
git cliff 4c7b043..a440c6e
git cliff 4c7b043..HEAD
git cliff HEAD~2..
git cliff v2.2.1..
git cliff v0.1.0..HEAD
```

Only include the tags from the current branch:

```bash
git cliff --use-branch-tags
```

Sort the commits inside sections:

```bash
# The oldest commit will be on top.
# (default)
git cliff --sort oldest

# The newest commit will be on top.
git cliff --sort newest
```

Sort the tags in topological order:

```bash
# Process in topological order instead of chronological.
git cliff --topo-order
```

Save the changelog file to the specified file:

```bash
# Set output path
git cliff --output CHANGELOG.md

# Without path, the default is `CHANGELOG.md`
git cliff -o
```

Prepend new changes to an existing changelog file:

```bash
# 1- changelog header is removed from CHANGELOG.md
# 2- new entries are prepended to CHANGELOG.md without footer part
# the --prepend option is incompatible with -o (output) if the file paths are equal
git cliff --unreleased --tag 1.0.0 --prepend CHANGELOG.md
```

Set/remove the changelog parts:

```bash
git cliff --body $template --strip footer
```

Skip running the commands defined in [pre](/docs/configuration/git#commit_preprocessors)/[postprocessors](/docs/configuration/changelog#postprocessors).

```bash
# No external command execution
git cliff --no-exec
```

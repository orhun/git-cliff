# Examples

See [command-line arguments](/docs/usage).

The default [configuration file](/docs/configuration) (`cliff.toml`) can be generated using the `--init` flag:

```bash
# create cliff.toml
git cliff --init
```

Then simply create a changelog at your projects git root directory:

```bash
# same as running `git-cliff --config cliff.toml --repository .`
# same as running `git-cliff --workdir .`
git cliff
```

Set a tag for the unreleased changes:

```bash
# it doesn't have to be an existing tag
git cliff --tag 1.0.0
```

Calculate and set the next semantic version (i.e. _bump the version_) for the unreleased changes:

```bash
# Semver: {MAJOR}.{MINOR}.{PATCH}
# "fix:" increments PATCH, "feat:" increments MINOR and "scope!" (breaking changes) increments MAJOR
git cliff --bump
# Simply calculate and return next semantic version
git cliff --bumped-version
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

# generate changelog for a specific commit range
git cliff 4c7b043..a440c6e
git cliff 4c7b043..HEAD
git cliff HEAD~2..
```

Generate a changelog scoped to a specific directory (useful for monorepos):

```bash
git cliff --include-path "**/*.toml" --include-path "*.md"
git cliff --exclude-path ".github/*"
```

Generate a changelog for multiple git repositories:

```bash
# merges the commit history
git cliff --repository path1 path2
```

Generate a changelog that includes yet unexisting commit messages:

```bash
commit_msg="chore(release): update CHANGELOG.md for 1.0.0"

# You might need to include the commit messages that do not exist
# for testing purposes or solving the chicken-egg problem.
git cliff --with-commit "$commit_msg" -o CHANGELOG.md

git add CHANGELOG.md && git commit -m "$commit_msg"
```

> The commit SHA will be empty as default when `--with-commit` is used. Specify the hash with a message separated by single whitespace for setting the commit SHA. e.g. `--with-commit "8f55e69eba6e6ce811ace32bd84cc82215673cb6 feat: add X"`

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

Print the changelog [context](/docs/templating/context) as JSON:

```bash
# print context to stdout
git cliff --context

# save context to a file
git cliff --context --output context.json
```

Prepend new changes to an existing changelog file:

```bash
# 1- changelog header is removed from CHANGELOG.md
# 2- new entries are prepended to CHANGELOG.md without footer part
git cliff --unreleased --tag 1.0.0 --prepend CHANGELOG.md
```

Set/remove the changelog parts:

```bash
git cliff --body $template --strip footer
```

Also, see the [release script](https://github.com/orhun/git-cliff/blob/main/release.sh) of this project which sets the changelog as a message of an annotated tag.

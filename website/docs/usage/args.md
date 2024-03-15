---
sidebar_position: 1
---

# Command-line Arguments

```
git-cliff [FLAGS] [OPTIONS] [--] [RANGE]
```

## Flags

```
-h, --help            Prints help information
-V, --version         Prints version information
-v, --verbose...      Increases the logging verbosity
    --bump            Bumps the version for unreleased changes
    --bumped-version  Prints bumped version for unreleased changes
-l, --latest          Processes the commits starting from the latest tag
    --current         Processes the commits that belong to the current tag
-u, --unreleased      Processes the commits that do not belong to a tag
    --topo-order      Sorts the tags topologically
-x, --context         Prints changelog context as JSON
    --no-exec         Disables the external command execution
```

## Options

```
-i, --init [<CONFIG>]                 Writes the default configuration file to cliff.toml
-c, --config <PATH>                   Sets the configuration file [env: GIT_CLIFF_CONFIG=] [default: cliff.toml]
-w, --workdir <PATH>                  Sets the working directory [env: GIT_CLIFF_WORKDIR=]
-r, --repository <PATH>...            Sets the git repository [env: GIT_CLIFF_REPOSITORY=]
    --include-path <PATTERN>...       Sets the path to include related commits [env: GIT_CLIFF_INCLUDE_PATH=]
    --exclude-path <PATTERN>...       Sets the path to exclude related commits [env: GIT_CLIFF_EXCLUDE_PATH=]
    --release-tags-pattern <PATTERN>  Sets the regex for matching git tags [env: GIT_CLIFF_RELEASE_TAGS_PATTERN=]
    --with-commit <MSG>...            Sets custom commit messages to include in the changelog [env: GIT_CLIFF_WITH_COMMIT=]
    --skip-commit <SHA1>...           Sets commits that will be skipped in the changelog [env: GIT_CLIFF_SKIP_COMMIT=]
-p, --prepend <PATH>                  Prepends entries to the given changelog file [env: GIT_CLIFF_PREPEND=]
-o, --output [<PATH>]                 Writes output to the given file [env: GIT_CLIFF_OUTPUT=]
-t, --tag <TAG>                       Sets the tag for the latest version [env: GIT_CLIFF_TAG=]
-b, --body-template <TEMPLATE>        Sets the Tera template to be rendered for each release in the changelog. [env: GIT_CLIFF_BODY_TEMPLATE=]
-s, --strip <PART>                    Strips the given parts from the changelog [possible values: header, footer, all]
    --commit-sort-order <SORT>        Sets sorting of the commits inside groups [default: oldest] [possible values: oldest, newest]
    --github-token <TOKEN>            Sets the GitHub API token [env: GITHUB_TOKEN]
    --github-repo <OWNER/REPO>        Sets the GitHub repository [env: GITHUB_REPO=]
```

## Args

```
[RANGE]  Sets the commit range to process
```

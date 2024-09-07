---
sidebar_position: 1
---

# Command-line Arguments

```
git-cliff [FLAGS] [OPTIONS] [--] [RANGE]
```

## Flags

```
-h, --help             Prints help information
-V, --version          Prints version information
-v, --verbose...       Increases the logging verbosity
    --bumped-version   Prints bumped version for unreleased changes
-l, --latest           Processes the commits starting from the latest tag
    --current          Processes the commits that belong to the current tag
-u, --unreleased       Processes the commits that do not belong to a tag
    --topo-order       Sorts the tags topologically
    --use-branch-tags  Include only the tags that belong to the current branch
    --no-exec          Disables the external command execution
-x, --context          Prints changelog context as JSON
```

## Options

```
-i, --init [<CONFIG>]              Writes the default configuration file to cliff.toml
    --bump                         Bumps the version for unreleased changes [default: auto] [possible values: auto, major, minor, patch]
-c, --config <PATH>                Sets the configuration file [env: GIT_CLIFF_CONFIG=] [default: cliff.toml]
-w, --workdir <PATH>               Sets the working directory [env: GIT_CLIFF_WORKDIR=]
-r, --repository <PATH>...         Sets the git repository [env: GIT_CLIFF_REPOSITORY=]
    --include-path <PATTERN>...    Sets the path to include related commits [env: GIT_CLIFF_INCLUDE_PATH=]
    --exclude-path <PATTERN>...    Sets the path to exclude related commits [env: GIT_CLIFF_EXCLUDE_PATH=]
    --tag-pattern <PATTERN>        Sets the regex for matching git tags [env: GIT_CLIFF_TAG_PATTERN=]
    --with-commit <MSG>...         Sets custom commit messages to include in the changelog [env: GIT_CLIFF_WITH_COMMIT=]
    --with-tag-message [<MSG>]     Sets custom message for the latest release [env: GIT_CLIFF_WITH_TAG_MESSAGE=]
    --ignore-tags <PATTERN>        Sets the tags to ignore in the changelog [env: GIT_CLIFF_IGNORE_TAGS=]
    --count-tags <PATTERN>         Sets the tags to count in the changelog [env: GIT_CLIFF_COUNT_TAGS=]
    --skip-commit <SHA1>...        Sets commits that will be skipped in the changelog [env: GIT_CLIFF_SKIP_COMMIT=]
-p, --prepend <PATH>               Prepends entries to the given changelog file [env: GIT_CLIFF_PREPEND=]
-o, --output [<PATH>]              Writes output to the given file [env: GIT_CLIFF_OUTPUT=]
-t, --tag <TAG>                    Sets the tag for the latest version [env: GIT_CLIFF_TAG=]
-b, --body <TEMPLATE>              Sets the template for the changelog body [env: GIT_CLIFF_TEMPLATE=]
    --from-context <PATH>          Generates changelog from a JSON context [env: GIT_CLIFF_CONTEXT=]
-s, --strip <PART>                 Strips the given parts from the changelog [possible values: header, footer, all]
    --sort <SORT>                  Sets sorting of the commits inside sections [default: oldest] [possible values: oldest, newest]
    --github-token <TOKEN>         Sets the GitHub API token [env: GITHUB_TOKEN]
    --github-repo <OWNER/REPO>     Sets the GitHub repository [env: GITHUB_REPO=]
    --gitlab-token <TOKEN>         Sets the GitLab API token [env: GITLAB_TOKEN]
    --gitlab-repo <OWNER/REPO>     Sets the GitLab repository [env: GITLAB_REPO=]
    --gitea-token <TOKEN>          Sets the Gitea API token [env: GITEA_TOKEN]
    --gitea-repo <OWNER/REPO>      Sets the Gitea repository [env: GITEA_REPO=]
    --bitbucket-token <TOKEN>      Sets the Bitbucket API token [env: BITBUCKET_TOKEN]
    --bitbucket-repo <OWNER/REPO>  Sets the Bitbucket repository [env: BITBUCKET_REPO=]
```

## Args

```
[RANGE]  Sets the commit range to process
```

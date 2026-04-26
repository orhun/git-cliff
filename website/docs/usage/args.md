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
    --dry-run          Prints the computed commit range and exits without rendering
-x, --context          Prints changelog context as JSON
    --use-native-tls   Load TLS certificates from the native certificate store
```

## Options

```
-i, --init [<CONFIG>]              Writes the default configuration file to cliff.toml
    --bump                         Bumps the version for unreleased changes [default: auto] [possible values: auto, major, minor, patch]
-c, --config <PATH>                Sets the configuration file [env: GIT_CLIFF_CONFIG=] [default: cliff.toml]
    --config-url <URL>             Sets the URL for the configuration file [env: GIT_CLIFF_CONFIG_URL=]
-w, --workdir <PATH>               Sets the working directory [env: GIT_CLIFF_WORKDIR=]
-r, --repository <PATH>...         Sets the git repository [env: GIT_CLIFF_REPOSITORY=]
    --include-path <PATTERN>...    Sets the path to include related commits [env: GIT_CLIFF_INCLUDE_PATH=]
    --exclude-path <PATTERN>...    Sets the path to exclude related commits [env: GIT_CLIFF_EXCLUDE_PATH=]
    --tag-pattern <PATTERN>        Sets the regex for matching git tags [env: GIT_CLIFF_TAG_PATTERN=]
    --with-commit <MSG>...         Sets custom commit messages to include in the changelog [env: GIT_CLIFF_WITH_COMMIT=]
    --with-tag-message [<MSG>]     Sets custom message for the latest release [env: GIT_CLIFF_WITH_TAG_MESSAGE=]
    --skip-tags <PATTERN>          Sets the tags to skip in the changelog [env: GIT_CLIFF_SKIP_TAGS=]
    --ignore-tags <PATTERN>        Sets the tags to ignore in the changelog [env: GIT_CLIFF_IGNORE_TAGS=]
    --count-tags <PATTERN>         Sets the tags to count in the changelog [env: GIT_CLIFF_COUNT_TAGS=]
    --skip-commit <SHA1>...        Sets commits that will be skipped in the changelog [env: GIT_CLIFF_SKIP_COMMIT=]
    --start-at <REV>               Include this revision as the lower bound (walk forward from here) [env: GIT_CLIFF_START_AT=]
    --start-after <REV>            Exclude this revision; start walking forward from its successor [env: GIT_CLIFF_START_AFTER=]
    --end-at <REV>                 Include this revision as the upper bound (walk back from here) [env: GIT_CLIFF_END_AT=]
    --end-before <REV>             Exclude this revision; stop walking before reaching it [env: GIT_CLIFF_END_BEFORE=]
-p, --prepend <PATH>               Prepends entries to the given changelog file [env: GIT_CLIFF_PREPEND=]
-o, --output [<PATH>]              Writes output to the given file [env: GIT_CLIFF_OUTPUT=]
-t, --tag <TAG>                    Sets the tag for the latest version [env: GIT_CLIFF_TAG=]
-b, --body <TEMPLATE>              Sets the template for the changelog body [env: GIT_CLIFF_TEMPLATE=]
    --from-context <PATH>          Generates changelog from a JSON context [env: GIT_CLIFF_CONTEXT=]
-s, --strip <PART>                 Strips the given parts from the changelog [possible values: header, footer, all]
    --sort <SORT>                  Sets sorting of the commits inside sections [default: oldest] [possible values: oldest, newest]
```

## Remote Options

```
    --github-token <TOKEN>            Sets the GitHub API token [env: GITHUB_TOKEN]
    --github-repo <OWNER/REPO>        Sets the GitHub repository [env: GITHUB_REPO=]
    --gitlab-token <TOKEN>            Sets the GitLab API token [env: GITLAB_TOKEN]
    --gitlab-repo <OWNER/REPO>        Sets the GitLab repository [env: GITLAB_REPO=]
    --gitea-token <TOKEN>             Sets the Gitea API token [env: GITEA_TOKEN]
    --gitea-repo <OWNER/REPO>         Sets the Gitea repository [env: GITEA_REPO=]
    --bitbucket-token <TOKEN>         Sets the Bitbucket API token [env: BITBUCKET_TOKEN]
    --bitbucket-repo <OWNER/REPO>     Sets the Bitbucket repository [env: BITBUCKET_REPO=]
    --azure-devops-token <TOKEN>      Sets the Azure DevOps API token [env: AZURE_DEVOPS_TOKEN]
    --azure-devops-repo <OWNER/REPO>  Sets the Azure DevOps repository [env: AZURE_DEVOPS_REPO=]
    --offline                         Disable network access for remote repositories [env: GIT_CLIFF_OFFLINE]
```

## Args

```
[RANGE]  Sets the commit range to process
```

## Range selection

Every git-cliff invocation selects a contiguous slice of history. The flags above are named shortcuts for picking the two endpoints of that slice, with inclusivity baked in. The four `--*-at` / `--*-before` / `--*-after` options make the endpoints (and their inclusivity) explicit.

### New endpoint options

| CLI flag          | Config key          | Meaning                                  |
| ----------------- | ------------------- | ---------------------------------------- |
| `--start-at X`    | `start_at = "X"`    | Include `X`; walk forward. `[X, ...`     |
| `--start-after X` | `start_after = "X"` | Exclude `X`; walk forward. `(X, ...`     |
| `--end-at Y`      | `end_at = "Y"`      | Include `Y`; walk back. `..., Y]`        |
| `--end-before Y`  | `end_before = "Y"`  | Exclude `Y`; stop before it. `..., Y)`   |

The naming convention: `*_at` is inclusive, `*_before` / `*_after` is exclusive. Within each pair, at most one may be set. The two pairs are independent, so any inclusivity combination is expressible. Unspecified sides fall back to the existing defaults (left = first commit, right = `HEAD`).

CLI overrides config one side at a time: setting either flag for a side replaces both config keys for that side. So `--start-at v1.0.0` on the CLI cleanly overrides `start_after = "v0.9.0"` from a shared team config (the left side becomes inclusive at `v1.0.0`); config's right side is untouched.

### Conflicts

- `--start-at` and `--start-after` cannot be combined; same for `--end-at` and `--end-before`.
- The new endpoint options cannot be combined with the legacy range flags (`--latest`, `--current`, `--unreleased`, `--bump`, positional `A..B`). Pick one style.

### How legacy flags map to endpoints

| Legacy         | Equivalent endpoint form                              |
| -------------- | ----------------------------------------------------- |
| (no flags)     | `--end-at HEAD` (left defaults to first commit)       |
| `--unreleased` | `--start-after <last_tag> --end-at HEAD`              |
| `--latest`     | `--start-after <prev_tag> --end-at <last_tag>`        |
| `--current`    | `--start-after <prev_tag> --end-at <current_tag>`     |
| `<A>..<B>`     | `--start-after A --end-at B`                          |

### Previewing with `--dry-run`

`--dry-run` prints the computed interval, the number of commits it covers, and the git revision range that will be walked, then exits without rendering a changelog:

```
$ git cliff --start-at v0.1.0 --end-at v0.2.0 --dry-run
range:    [v0.1.0, v0.2.0]
commits:  3
emitted:  02deb7a7...^..9f14f5d1...
```

The `range:` line uses the revisions you specified (math-interval notation; `[` / `]` are inclusive, `(` / `)` are exclusive). The `emitted:` line shows the git revision range actually passed to the walker, with revisions resolved to full commit SHAs.

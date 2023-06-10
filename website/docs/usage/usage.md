---
sidebar_position: 3
---
# Usage

```
git-cliff [FLAGS] [OPTIONS] [--] [RANGE]
```

## Flags

```
-h, --help        Prints help information
-V, --version     Prints version information
-v, --verbose...  Increases the logging verbosity
-i, --init        Writes the default configuration file to cliff.toml
-l, --latest      Processes the commits starting from the latest tag
    --current     Processes the commits that belong to the current tag
-u, --unreleased  Processes the commits that do not belong to a tag
    --topo-order  Sorts the tags topologically
    --context     Prints changelog context as JSON
```

## Options

```
-c, --config <PATH>              Sets the configuration file [env: GIT_CLIFF_CONFIG=] [default: cliff.toml]
-w, --workdir <PATH>             Sets the working directory [env: GIT_CLIFF_WORKDIR=]
-r, --repository <PATH>...       Sets the git repository [env: GIT_CLIFF_REPOSITORY=]
    --include-path <PATTERN>...  Sets the path to include related commits [env: GIT_CLIFF_INCLUDE_PATH=]
    --exclude-path <PATTERN>...  Sets the path to exclude related commits [env: GIT_CLIFF_EXCLUDE_PATH=]
    --with-commit <MSG>...       Sets custom commit messages to include in the changelog [env: GIT_CLIFF_WITH_COMMIT=]
-p, --prepend <PATH>             Prepends entries to the given changelog file [env: GIT_CLIFF_PREPEND=]
-o, --output <PATH>              Writes output to the given file [env: GIT_CLIFF_OUTPUT=]
-t, --tag <TAG>                  Sets the tag for the latest version [env: GIT_CLIFF_TAG=]
-b, --body <TEMPLATE>            Sets the template for the changelog body [env: GIT_CLIFF_TEMPLATE=]
-s, --strip <PART>               Strips the given parts from the changelog [possible values: header, footer, all]
    --sort <SORT>                Sets sorting of the commits inside sections [default: oldest] [possible values: oldest, newest]
```

## Args

```
[RANGE]  Sets the commit range to process
```

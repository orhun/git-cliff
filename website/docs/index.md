---
sidebar_position: 1
---

# Getting Started

**git-cliff** can generate [changelog](https://en.wikipedia.org/wiki/Changelog) files from the [Git](https://git-scm.com/) history by utilizing [conventional commits](/docs/configuration/git#conventional_commits) as well as regex-powered [custom parsers](/docs/configuration/git#commit_parsers).

The [changelog template](category/templating) can be customized with a [configuration file](configuration) to match the desired format.

## Quickstart

1. Install **git-cliff**:

```bash
cargo install git-cliff
```

Also, see the other [installation options](installation).

2. Initialize **git-cliff**:

```bash
git-cliff --init
```

Edit the default [configuration](configuration) (`cliff.toml`) as you like. Check out the [examples](templating/examples) for different templates.

3. Generate a changelog:

```bash
git-cliff -o CHANGELOG.md
```

See the [command-line usage examples](usage/examples).

:::note
The `git cliff` command can also be used interchangeably with `git-cliff` (with a `-`) in most environments (when `git` is installed). However, when using the [NPM](/docs/installation/npm) installation method, the `git-cliff` command should be used.
:::

## Contribute

Contributions are highly appreciated! See the [contribution guidelines](https://github.com/orhun/git-cliff/blob/main/CONTRIBUTING.md) for getting started.

Feel free to submit issues and toss ideas!

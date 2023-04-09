---
sidebar_position: 1
---
# Getting Started

**git-cliff** can generate [changelog](https://en.wikipedia.org/wiki/Changelog) files from the [Git](https://git-scm.com/) history by utilizing [conventional commits](configuration#conventional_commits) as well as regex-powered [custom parsers](configuration#commit_parsers).

The [changelog template](templating) can be customized with a [configuration file](configuration) to match the desired format.

## Quickstart

1. Install **git-cliff**:

```bash
cargo install git-cliff
```

Also, see the other [installation options](installation).

2. Initialize **git-cliff**:

```bash
git cliff --init
```

Edit the default [configuration](configuration) (`cliff.toml`) as you like. Check out the [examples](templating/examples) for different templates.

3. Generate a changelog:

```bash
git cliff -o CHANGELOG.md
```

See the [command-line usage examples](usage/examples).

## Contribute

Contributions are highly appreciated! See the [contribution guidelines](https://github.com/orhun/git-cliff/blob/main/CONTRIBUTING.md) for getting started.

Feel free to submit issues and toss ideas!

---
sidebar_position: 2
---

# Initializing

The default [configuration file](/docs/configuration) (`cliff.toml`) can be generated using the `--init` flag:

```bash
# create cliff.toml
git cliff --init

# create a config file with a custom name
git-cliff --init --config custom.toml
```

There are also other templates under the [examples](https://github.com/orhun/git-cliff/blob/main/examples) directory. See the [template examples](/docs/templating/examples) for previewing the templates.

To initialize `git-cliff` with one of those templates, simply use the name of the template:

```bash
# create cliff.toml with Keep a Changelog format
git cliff --init keepachangelog
```

Also, you can use a template without creating the configuration file. Just give the name of the template to the `--config` option as follows:

```bash
# generate a changelog with using the built-in "detailed" template
git cliff --config detailed
```

Here are the list of available templates:

- [`keepachangelog.toml`](https://github.com/orhun/git-cliff/tree/main/examples/keepachangelog.toml): changelog in [Keep a Changelog format](https://keepachangelog.com/en/1.1.0/).
- [`github.toml`](https://github.com/orhun/git-cliff/tree/main/examples/github.toml): changelog in the [GitHub's format](https://docs.github.com/en/repositories/releasing-projects-on-github/automatically-generated-release-notes).
- [`github-keepachangelog.toml`](https://github.com/orhun/git-cliff/tree/main/examples/github-keepachangelog.toml): combination of the previous two formats.
- [`detailed.toml`](https://github.com/orhun/git-cliff/tree/main/examples/detailed.toml): changelog that contains links to the commits.
- [`minimal.toml`](https://github.com/orhun/git-cliff/tree/main/examples/minimal.toml): minimal changelog.
- [`scoped.toml`](https://github.com/orhun/git-cliff/tree/main/examples/scoped.toml): changelog with commits are grouped by their scopes.
- [`scopesorted.toml`](https://github.com/orhun/git-cliff/tree/main/examples/scopesorted.toml): changelog with commits grouped by their scopes and sorted by group.
- [`cocogitto.toml`](https://github.com/orhun/git-cliff/tree/main/examples/cocogitto.toml): changelog similar to [cocogitto's format](https://github.com/cocogitto/cocogitto/blob/main/CHANGELOG.md).
- [`unconventional.toml`](https://github.com/orhun/git-cliff/tree/main/examples/unconventional.toml): changelog for unconventional commits.

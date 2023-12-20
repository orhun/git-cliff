---
sidebar_position: 2
---

# Initializing

The default [configuration file](/docs/configuration) (`cliff.toml`) can be generated using the `--init` flag:

```bash
# create cliff.toml
git cliff --init
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

- [`keepachangelog.toml`](https://github.com/orhun/git-cliff/tree/main/examples/keepachangelog.toml)
- [`detailed.toml`](https://github.com/orhun/git-cliff/tree/main/examples/detailed.toml)
- [`minimal.toml`](https://github.com/orhun/git-cliff/tree/main/examples/minimal.toml)
- [`scoped.toml`](https://github.com/orhun/git-cliff/tree/main/examples/scoped.toml)
- [`scopesorted.toml`](https://github.com/orhun/git-cliff/tree/main/examples/scopesorted.toml)
- [`cocogitto.toml`](https://github.com/orhun/git-cliff/tree/main/examples/cocogitto.toml)
- [`unconventional.toml`](https://github.com/orhun/git-cliff/tree/main/examples/unconventional.toml)

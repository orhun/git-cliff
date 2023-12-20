---
sidebar_position: 4
---
# Configuration

**git-cliff** configuration file supports [TOML](https://github.com/toml-lang/toml) (preferred) and [YAML](https://yaml.org) formats.

The configuration file is read from `$HOME/git-cliff/cliff.toml` if the file exists. This location depends on the platform, for example:

- on Linux: `/home/<user>/.config/git-cliff/cliff.toml`
- on Windows: `C:\Users\<user>\AppData\Roaming\git-cliff\cliff.toml`
- on macOS: `/Users/<user>/Library/Application Support/git-cliff/cliff.toml`

See [cliff.toml](https://github.com/orhun/git-cliff/blob/main/config/cliff.toml) for the default configuration values.

## Environment Configuration Overrides

It's possible to use environment variables to override configuration elements. If an environment variable matches a configuration element, the variable's value will be used instead of the element's.

Format:

```
[PREFIX]__[CONFIG SECTION]__[FIELD NAME]
```

#### Examples

To override the `footer` element:

```bash
export GIT_CLIFF__CHANGELOG__FOOTER="<!-- footer from env -->"
```

To override the `ignore_tags` element:

```bash
export GIT_CLIFF__GIT__IGNORE_TAGS="v[0-9]+.[0-9]+.[0-9]+-rc[0-9]+"
```

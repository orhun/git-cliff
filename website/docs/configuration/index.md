---
sidebar_position: 4
---

# Configuration

**git-cliff** configuration file supports [TOML](https://github.com/toml-lang/toml) (preferred) and [YAML](https://yaml.org) formats.

## File Path

**git-cliff** will look for a configuration file first in the project directory, then in the global user directory. If no configuration file is found, **git-cliff** will use the default configuration values. See [cliff.toml](https://github.com/orhun/git-cliff/blob/main/config/cliff.toml) for the default configuration values.

It looks for the following configuration files in this order:

- `cliff.toml`
- `.cliff.toml`
- `.config/cliff.toml`
- `$HOME/cliff.toml`
- `$HOME/.cliff.toml`
- `$HOME/.config/cliff.toml`

If no configuration file is found in the current directory, it will search the parent directories.

### Home Directory

The `$HOME` directory is dependent on the platform. For example:

- on Linux: `/home/<user>`
- on Windows: `C:\Users\<user>\AppData\Roaming`
- on macOS: `/Users/<user>/Library/Application Support`

## Environment Configuration Overrides

It's possible to use environment variables to override configuration elements. If an environment variable matches a configuration element, the variable's value will be used instead of the element's.

Format:

```
[PREFIX]__[CONFIG SECTION]__[FIELD NAME]
```

## Examples

To override the `footer` element:

```bash
export GIT_CLIFF__CHANGELOG__FOOTER="<!-- footer from env -->"
```

To override the `ignore_tags` element:

```bash
export GIT_CLIFF__GIT__IGNORE_TAGS="v[0-9]+.[0-9]+.[0-9]+-rc[0-9]+"
```

---
sidebar_position: 4
---

# Configuration

**git-cliff** configuration file supports [TOML](https://github.com/toml-lang/toml) (preferred) and [YAML](https://yaml.org) formats.

## File Path

**git-cliff** will look for a configuration file first in the project directory, then in the global user directory. If no configuration file is found, **git-cliff** will use the default configuration values. See [cliff.toml](https://github.com/orhun/git-cliff/blob/main/config/cliff.toml) for the default configuration values.

:::tip

The configuration schema is published on [SchemaStore](https://www.schemastore.org/git-cliff.json).

:::

It looks for the following project configuration files in this order:

- `cliff.toml`
- `.cliff.toml`
- `.config/cliff.toml`

If no configuration file is found in the current directory, it will search the parent directories.

### Global Configuration

The global configuration file is located at:

```text
<CONFIG_DIR>/git-cliff/cliff.toml
```

`<CONFIG_DIR>` is the `config_dir()` returned by [`etcetera`](https://github.com/lunacookies/etcetera)'s default base strategy and depends on the platform:

- Linux: `$XDG_CONFIG_HOME`, or `~/.config`
- macOS: `$XDG_CONFIG_HOME`, or `~/.config`
- Windows: `%APPDATA%` (typically `~\AppData\Roaming`)

On macOS, the legacy `~/Library/Application Support/git-cliff/cliff.toml` path is also supported for backwards compatibility.

## Environment Configuration Overrides

It's possible to use environment variables to override configuration elements. If an environment variable matches a configuration element, the variable's value will be used instead of the element's.

Format:

```
[PREFIX]__[CONFIG SECTION]__[FIELD NAME]
```

### Examples

To override the `footer` element:

```bash
export GIT_CLIFF__CHANGELOG__FOOTER="<!-- footer from env -->"
```

To override the `ignore_tags` element:

```bash
export GIT_CLIFF__GIT__IGNORE_TAGS="v[0-9]+.[0-9]+.[0-9]+-rc[0-9]+"
```

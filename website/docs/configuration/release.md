# `release`

This section contains options regarding releases.

```toml
[release]
tags_pattern = "v[0-9].*"
skip_tags_pattern = "rc"
order_by = "time"
```

### tags_pattern

Regex to select git tags that represent releases.

Examples:

- `tags_pattern = "v[0-9].*"`

This value can be also overridden with using the `--release-tags-pattern` argument.

### skip_tags_pattern

Regex to select git tags that do not represent proper releases. Takes precedence over [`release.tags_pattern`](#tags_pattern).
Changes belonging to these releases will be included in the next non-skipped release.

Examples:

- `skip_tags_pattern = "rc"`

### order_by

Whether to order releases chronologically or topologically.
Must be either `time` or `topology`.

This value can be also overridden with using the `--release-order-by` argument.

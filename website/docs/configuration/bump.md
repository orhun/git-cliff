# `bump`

This section contains the bump version related configuration options.

```toml
[bump]
features_always_bump_minor = true
breaking_always_bump_major = true
```

### features_always_bump_minor

Configures automatic minor version increments for feature changes.
When `true`, a feature will always trigger a minor version update.
When `false`, a feature will trigger:

- A patch version update if the major version is 0.
- A minor version update otherwise.

### breaking_always_bump_major

Configures 0 -> 1 major version increments for breaking changes.
When `true`, a breaking change commit will always trigger a major version update
(including the transition from version 0 to 1)
When `false`, a breaking change commit will trigger:

- A minor version update if the major version is 0.
- A major version update otherwise.

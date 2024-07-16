# `bump`

This section contains the bump version related configuration options.

```toml
[bump]
features_always_bump_minor = true
breaking_always_bump_major = true
initial_tag = "0.1.0"
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

### initial_tag

Configures the initial version of the project.

When set, the version will be set to this value if no tags are found.

### custom_major_increment_regex & custom_minor_increment_regex

Configures additional commit types that should increment the major or minor accordingly.

They should be used rarely, only in the case you have a spacial case for incrementing versions.

Expects a valid regex pattern.

For example:

```toml
[bump]
features_always_bump_minor = true
breaking_always_bump_major = true
custom_major_increment_regex = "major"
custom_minor_increment_regex = "minor|more"
```

with this history:

```
5189568 (HEAD -> main) major: 1
0b17b48 (tag: 0.1.0) initial commit
```

will result in:

```bash
git-cliff --bumped-version

1.0.0
```

or, with history:

```
47206d0 (HEAD -> main) more: 1
0b17b48 (tag: 0.1.0) initial commit
```

will result in:

```bash
git-cliff --bumped-version

0.2.0
```

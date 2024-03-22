# `bump`

## Migrating
A new subcommand `migrate-config` was added to git-cliff to help updating your configuration. It takes the path of your old config and a destination to write the updated version to.

Example:

- `git-cliff migrate-config --in cliff.toml --out cliff-new.toml`

## Backwards compatability
While configuration version 1 is deprecated, the cli argument `--config-version` can be used to make git-cliff use old configuration files. Keep in mind that new features might not be supported when using the old configuration format.

Example:

- `git-cliff --config-version 1 --config cliff.toml`

## Changes

### Section `[changelog]`
- Renamed option `body` to `body_template`.
- Renamed option `footer` to `footer_template` to indicate that it is a Tera template instead of a static string.
- Renamed option `trim` to `trim_body_whitespace` to indicate that it is a Tera template instead of static string.
- Moved and renamed option `git.filter_commits` to `changelog.exclude_ungrouped_changes` to be more descriptive.

### Section `[release]`
- The section `[release]` was introduced to group options that deal with releases. Its options were extracted from the former `[git]` section. This clearly separates options like `release.tags_pattern` from `commit.exclude_tags_pattern` and thus improves comprehensibility.
- Moved and renamed `git.tag_pattern` to `release.tags_pattern` to emphasize them affecting releases instead of individual commits.
- Moved and renamed `git.ignore_tags` to `release.skip_tags_pattern` to differentiate from `git.skip_tags` and emphasize its relation to `release.tags_pattern`.
- Moved and renamed `git.topo_order` to `release.order_by`.
- Changed type of `release.order_by` to an enum with values `time` and `topology` for clarity.

### Section `[commit]` (formerly `[git]`)
- The section `[git]` has been renamed to `[commit]`, because its options only affect handling of commits and no other git operations. This frees up `git` for possible future additions of options that affect other git structures than commits.
- Renamed option `git.conventional_commits` to `commit.parse_conventional_commits` for clarity.
- Renamed option `git.filter_unconventional` to `commit.exclude_unconventional_commits` to emphasize its effect of actually excluding unconventional commits from processing.
- Renamed option `git.split_commits` to `commit.split_by_newline` to indicate how it splits commits.
- Renamed option `git.commit_preprocessors` to `commit.message_preprocessors` to emphasize that it acts on commit messages.
- Renamed option `git.skip_tags` to `commit.exclude_tags_pattern`.
- Renamed option `git.protect_breaking_changes` to `commit.retain_breaking_changes` to emphasize it retaining commits that would otherwise be skipped.
- Renamed option `git.sort_commits` to `commit.sort_order` to clarify that it sets the ordering instead of enabling/disabling order all together.
- Renamed option `git.limit_commits` to `commit.max_commit_count` to emphasize that it limits the number of commits and not a property of the commits themselves.

### CLI Arguments
CLI arguments were updated to align with the new names of their respective configuration options.
- Renamed cli argument `--tag-pattern` to `--release-tags-pattern`.
- Renamed cli argument `--body` to `--body-template`.
- Renamed cli argument `--sort` to `--commit-sort.order`.

### Miscellaneous Changes
- Updated descriptions.
- Updated documentation.

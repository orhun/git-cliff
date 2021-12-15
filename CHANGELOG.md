# Changelog
All notable changes to this project will be documented in this file.

## [0.5.0] - 2021-12-15

### Bug Fixes

- Update log test about exclude path
- Override the sort related config if args are present (#39)
- Checkout the repository before running fixtures
- Use the defined configuration file for fixtures
- Update the multi line docker command
- Strip the carriage return on fixtures while comparing
- Drop the skipped releases from 'previous' field

### Documentation

- Update `--with-commit` example in README.md

### Features

- Add `--topo-order` flag for sorting tags (#29)
- Support specifying the sorting methods in config (#31)
- Accept glob patterns for `--commit-path` argument
- Support multiple values for `--commit-path` argument
- Add `--exclude-path` argument for excluding related commits
- Add `--current` flag for processing the current tag (#37)
- Add `ignore_tags` option (#40)
- Use more explanatory error messages about templates
- Support having both conventional and unconventional commits in the changelog
- Add `--with-commit` argument for including custom commit messages in changelog

### Miscellaneous Tasks

- Improve the workflow for test fixtures
- Run test fixtures on ubuntu-latest
- Indicate the breaking changes via default config

### Refactor

- Rename the config value for commit order

### Styling

- [**breaking**] Rename `--commit-path` argument to `--include-path`

## [0.4.2] - 2021-10-22

### Bug Fixes

- Install the Rust toolchain explicitly for crates.io releases

## [0.4.1] - 2021-10-22

### Bug Fixes

- Add support for special characters in scopes (#26)

### Documentation

- Add GitLab CI/CD section to README.md (#24)
- Update GitLab CI/CD section

### Miscellaneous Tasks

- Run CI workflows periodically
- Remove unnecessary Cargo.lock entry from .gitignore
- Upgrade dependencies
- Migrate to Rust 2021 edition
- Bump the Rust version in Dockerfile

### Refactor

- Use a better error message for invalid repo path

## [0.4.0] - 2021-10-01

### Bug Fixes

- Update lychee arguments to skip checking files
- Remove tags from the base image names
- Remove only the leading "v" from tags (#18)

### Documentation

- Add scope-sorted example (#16)
- Add raw/rendered output for scoped-sorted example
- Add packaging status badge to installation section
- Mention the signing key for binary releases (#17)
- Add "build from source" section to README.md

### Features

- Add `--sort` argument for sorting commits (#15)

### Miscellaneous Tasks

- Set a version for the checkout action
- Update the runner to ubuntu-20.04
- Use cache for docker builds
- Use docker meta for tagging for GHCR
- Extend the tags for docker meta
- Rename the GHCR package due to legacy reasons
- Specify the latest tag explicitly
- Use explicit image name for docker automated builds
- Use docker.yml workflow for CI/CD
- Upgrade dependencies

### Styling

- Fix the newline issues in scoped-sorted example

## [0.3.0] - 2021-09-10

### Bug Fixes

- Fix default regexes and references in docs (#7)

### Documentation

- Update installation instructions for Arch Linux
- Add badge for joining the Matrix chat
- Update example regexes
- Update the default regex in scoped config example

### Features

- Support parsing the missing scopes with `default_scope` (#8)
- Support generating a changelog scoped to a directory (#11)

### Miscellaneous Tasks

- Upgrade dependencies

## [0.2.6] - 2021-09-04

### Bug Fixes

- Pin the cargo-chef version in Dockerfile

### Documentation

- Update docker commands to only mount the .git directory

### Miscellaneous Tasks

- Bump `git-conventional` to `0.10.1` (fixes #6)
- Bump dependencies
- Bump cargo-chef version in Dockerfile

## [0.2.5] - 2021-08-20

### Documentation

- Mention breaking changes for templating
- Update template examples to mention how to contribute

### Features

- Add `breaking_description` to the template context (#4)

### Miscellaneous Tasks

- Show the committed changes before creating a tag

## [0.2.4] - 2021-08-20

### Bug Fixes

- Change the config file location for crates.io release

## [0.2.3] - 2021-08-18

### Bug Fixes

- Fetch the dependencies before copying the file to embed

## [0.2.2] - 2021-08-18

### Bug Fixes

- Copy the config file into registry to resolve it for embed

## [0.2.1] - 2021-08-18

### Bug Fixes

- Copy the configuration file to embed into package

## [0.2.0] - 2021-08-18

### Bug Fixes

- Use custom error type for UTF-8 errors

### Documentation

- Update the doc comment of `prepend`

### Features

- Embed the default configuration file into the binary
- Add `--init` flag for creating the default config
- Support a global location for configuration file (#2)

### Miscellaneous Tasks

- Move `cliff.toml` to config/

### Refactor

- Create a constant for default configuration file
- Update the log message for unprocessed tags

### Styling

- Update the message of `--init` flag

## [0.1.2] - 2021-08-14

### Bug Fixes

- Use the correct name of completions binary

### Documentation

- Update the example completion command

## [0.1.1] - 2021-08-14

### Bug Fixes

- Set the previous release when using `--latest` (#3)

### Documentation

- Add installation instructions for the AUR

### Miscellaneous Tasks

- Rename the shell completions binary
- Upgrade dependencies

### Performance

- Process only the last 'previous' release
- Optimize the release vector size

## [0.1.0] - 2021-08-12

### Bug Fixes

- Update the environment variable parsing settings
- Use footers field as an array for the context
- Sort the commits in topological order
- Return error if there is not a latest tag to process
- Update symbolic link to the default config
- Remove symbolic link
- Use 7 digits for short SHA

### Documentation

- Update README.md about usage
- Update README.md about template and examples
- Add examples for CLI usage
- Add examples for templating
- Update detailed template example
- Add preview image to README.md

### Miscellaneous Tasks

- Upgrade dependencies
- Remove etc directory from .gitignore
- Bump the rust version
- Upgrade dependencies

### Refactor

- Rename changelog argument to prepend

### Styling

- Center the badges
- Update the comments in template context
- Remove comments from template context
- Wrap table of contents into summary
- Remove quotes from rendered output

### Testing

- Add tests
- Update repository tests about getting the latest tag

## [0.1.0-rc.21] - 2021-07-01

### Bug Fixes

- Wait for core library to update on crates.io before publish

## [0.1.0-rc.20] - 2021-06-30

### Bug Fixes

- Wait between publishing crates

## [0.1.0-rc.19] - 2021-06-30

### Bug Fixes

- Generate changelog on a dedicated/different job

### Miscellaneous Tasks

- Update project details

## [0.1.0-rc.18] - 2021-06-30

### Bug Fixes

- Use a separate step for setting the changelog body
- Fix the syntax of publish step arguments

## [0.1.0-rc.17] - 2021-06-29

### Bug Fixes

- Publish the cargo workspace members seperately

### Miscellaneous Tasks

- Verify the created tag after creation
- Indicate which versions are managed by the script

## [0.1.0-rc.16] - 2021-06-29

### Bug Fixes

- Update lychee arguments to exclude invalid links

### Documentation

- Add CONTRIBUTING.md
- Add link to the signer key of the tag
- Add RELEASE.md

### Miscellaneous Tasks

- Enable crates.io releases
- Set the new version in release script

## [0.1.0-rc.15] - 2021-06-23

### Miscellaneous Tasks

- Use only one step for uploading releases

## [0.1.0-rc.14] - 2021-06-23

### Bug Fixes

- Strip the changelog header before escaping

## [0.1.0-rc.13] - 2021-06-23

### Miscellaneous Tasks

- Use seperate steps for uploading releases

## [0.1.0-rc.12] - 2021-06-21

### Bug Fixes

- Use printf to prevent field splitting the variable
- Fix the character escape in release script
- Return tags by their creation order

### Miscellaneous Tasks

- Update .editorconfig about shell scripts
- Include the commit id in the custom template

## [0.1.0-rc.11] - 2021-06-21

### Miscellaneous Tasks

- Remove the custom changelog template

### Refactor

- Use custom error message for GroupError

## [0.1.0-rc.10] - 2021-06-20

### Styling

- Update the order of entries in config

## [0.1.0-rc.8] - 2021-06-20

### Bug Fixes

- Specify the committer email in release script
- Double quote the environment variable

### Miscellaneous Tasks

- Rename the docker automated builds action

## [0.1.0-rc.7] - 2021-06-20

### Features

- Support setting the body template via args

### Miscellaneous Tasks

- Set a custom changelog for the tag message
- Override the changelog template

## [0.1.0-rc.6] - 2021-06-20

### Miscellaneous Tasks

- Set the release body on linux

## [0.1.0-rc.5] - 2021-06-19

### Bug Fixes

- Update config to skip release commits

## [0.1.0-rc.4] - 2021-06-19

### Revert

- Chore(config): update template to include commit ids

## [0.1.0-rc.3] - 2021-06-19

### Bug Fixes

- Strip the unreleased title from tag message
- Update commit parsers to match the commit type

### Miscellaneous Tasks

- Fix setting the release body
- Update the skip_tags regex
- Update template to include commit ids

## [0.1.0-rc.2] - 2021-06-19

### Bug Fixes

- Use default tag_pattern for tests

### Features

- Add `--output` argument

### Miscellaneous Tasks

- Add release title to the tag message
- Set the release name explicitly
- Remove user directive from Dockerfile
- Set the changelog as release body

### Refactor

- Make tag_pattern optional

## [0.1.0-rc.1] - 2021-06-16

### Documentation

- Update the doc comment for completions script
- Add usage section

### Features

- Add `--workdir` argument
- Show the processsed commit message

### Miscellaneous Tasks

- Add release script
- Update the release script about arguments
- Strip the markdown format from tag message

### Refactor

- Update the value name for `--strip`
- Improve logging
- Update value names and description

## [0.1.0-beta.4] - 2021-06-14

### Bug Fixes

- Use bash while setting the release version

### Miscellaneous Tasks

- Add docker releases

## [0.1.0-beta.3] - 2021-06-14

### Bug Fixes

- Include configuration file in the binary releases
- Specify the bash as shell

### Miscellaneous Tasks

- Set the release body text

## [0.1.0-beta.2] - 2021-06-14

### Bug Fixes

- Install musl-tools for musl builds

### Miscellaneous Tasks

- Update config

<!-- generated by git-cliff -->

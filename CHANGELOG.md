# Changelog

All notable changes to this project will be documented in this file.

## [0.10.0] - 2022-11-20

### Bug Fixes

- Warn against invalid tag range for `--current` flag ([#124](https://github.com/orhun/git-cliff/issues/124))
- Use an alternative method to fetch registry
- Fix syntax error in Dockerfile

### Documentation

- Add MacPorts install info ([#111](https://github.com/orhun/git-cliff/issues/111))
- Update badge URL for Docker builds

### Features

- Do not skip breaking changes if configured ([#114](https://github.com/orhun/git-cliff/issues/114))
- Changelog for the last n commits ([#116](https://github.com/orhun/git-cliff/issues/116))
- Add a short variant `-d` for specifying `--date-order` flag

### Miscellaneous Tasks

- Update versions in Dockerfile
- Upgrade core dependencies

### Refactor

- Improve cargo-chef caching in Dockerfile
- Utilize workspace dependencies

## [0.9.2] - 2022-09-24

### Bug Fixes

- Remove custom user creation from the Dockerfile ([#109](https://github.com/orhun/git-cliff/issues/109))

### Miscellaneous Tasks

- Remove cargo-audit config
- Switch to cargo-tarpaulin for measuring code coverage ([#110](https://github.com/orhun/git-cliff/issues/110))
- Upgrade dependencies

## [0.9.1] - 2022-09-20

### Bug Fixes

- Configure git safe.directory for Docker image ([#108](https://github.com/orhun/git-cliff/issues/108))

### Miscellaneous Tasks

- Remove ansi_term dependency for fixing RUSTSEC-2021-0139
- Upgrade dependencies

### Refactor

- Apply clippy suggestions

### Styling

- Update styling for with-commit example

## [0.9.0] - 2022-08-16

### Documentation

- Add test repository link to README.md

### Features

- Support splitting commits by lines ([#101](https://github.com/orhun/git-cliff/issues/101))
- Support setting commit SHA while using `--with-commit`
- Add commit author and committer to the context ([#100](https://github.com/orhun/git-cliff/issues/100))

### Miscellaneous Tasks

- Use an alternative method to fetch registry
- Enable building arm64 docker images
- Update the description on Docker Hub on push
- Disable updating the description on Docker Hub
- Add GitHub Sponsors option for funding
- Upgrade dependencies
- Update MSRV to 1.60.0
- Upgrade versions in Dockerfile
- Enable strip option for release profile

### Refactor

- Run clippy for tests
- Use a more concise conversion for string

## [0.8.1] - 2022-07-12

### Bug Fixes

- Set fail-fast strategy to false

### Miscellaneous Tasks

- Update windows runners to windows-2022

## [0.8.0] - 2022-07-12

### Bug Fixes

- Update lychee arguments to skip checking protonmail

### Documentation

- Clarify that `--tag` argument can be an unexisting tag
- Switch chronological and topological ([#99](https://github.com/orhun/git-cliff/issues/99))

### Features

- Support external commands for commit preprocessors ([#86](https://github.com/orhun/git-cliff/issues/86))
- Support changing commit scope with `commit_parsers` ([#94](https://github.com/orhun/git-cliff/issues/94))
- [**breaking**] Pass footer token and separator to template ([#97](https://github.com/orhun/git-cliff/issues/97))

### Miscellaneous Tasks

- Set MSRV to 1.58.1 ([#87](https://github.com/orhun/git-cliff/issues/87))
- Update tera to 1.16.0 ([#70](https://github.com/orhun/git-cliff/issues/70))
- Disable building arm64 docker images temporarily
- Upgrade dependencies

### Refactor

- Apply clippy suggestions
- Apply clippy suggestions

## [0.7.0] - 2022-04-24

### Bug Fixes

- Pin the Rust nightly version
- Pin the Rust nightly version
- Allow custom commit range while prepending ([#68](https://github.com/orhun/git-cliff/issues/68))
- Remove redundant logging while using `--context` ([#71](https://github.com/orhun/git-cliff/issues/71))
- Update expected changelog date

### Documentation

- Add more regex examples for commit_preprocessors
- Update GitHub Actions reference link in README.md
- Add `cliff-jumper` to similar projects ([#83](https://github.com/orhun/git-cliff/issues/83))
- Update the title of projects section

### Features

- Show a message if a newer version is available ([#69](https://github.com/orhun/git-cliff/issues/69))
- Add `--context` flag for outputting context ([#71](https://github.com/orhun/git-cliff/issues/71))
- Support placing configuration inside Cargo.toml ([#46](https://github.com/orhun/git-cliff/issues/46))
- [**breaking**] Prefix environment variables with `GIT_CLIFF_` ([#76](https://github.com/orhun/git-cliff/issues/76))
- Print more debug information when `-vv` is used ([#79](https://github.com/orhun/git-cliff/issues/79))
- Support preprocessing commit messages using regex ([#62](https://github.com/orhun/git-cliff/issues/62))
- Add man page generation script ([#35](https://github.com/orhun/git-cliff/issues/35))

### Miscellaneous Tasks

- Return to nightly builds ([#73](https://github.com/orhun/git-cliff/issues/73))
- Include man page in the release assets
- Upgrade git-conventional dependency ([#82](https://github.com/orhun/git-cliff/issues/82))
- Upgrade versions in Dockerfile
- Build Docker images for arm64
- Disable default features for the Docker image
- Strip the binaries in Docker image
- Upgrade dependencies

### Refactor

- Make update-informer opt-out via feature flag ([#69](https://github.com/orhun/git-cliff/issues/69))
- Use implicit Result type in completions script

### Styling

- Update the changelog template for tag message

## [0.6.1] - 2022-03-13

### Bug Fixes

- Do not skip all tags when `skip_tags` is empty ([#63](https://github.com/orhun/git-cliff/issues/63))
- Use root commit when --latest and there is only one tag ([#59](https://github.com/orhun/git-cliff/issues/59))
- Use the correct branch for codecov ([#65](https://github.com/orhun/git-cliff/issues/65))
- Fix `keepachangelog` config example ([#66](https://github.com/orhun/git-cliff/issues/66))

### Documentation

- Add another option of GitHub Actions ([#64](https://github.com/orhun/git-cliff/issues/64))
- Document timestamp format of `Release` struct ([#67](https://github.com/orhun/git-cliff/issues/67))

### Miscellaneous Tasks

- Upgrade regex dependency to fix CVE-2022-24713
- Upgrade dependencies

## [0.6.0] - 2022-02-12

### Bug Fixes

- Only drop previous releases if skipped ([#44](https://github.com/orhun/git-cliff/issues/44))
- Run clippy from nightly toolchain
- Update tests about optional config values
- Set the previous release when using `--unreleased` ([#47](https://github.com/orhun/git-cliff/issues/47))
- Lower the priority of global configuration file ([#51](https://github.com/orhun/git-cliff/issues/51))
- Update the download link of latest grcov release
- Use the correct tar command for extracting grcov archive
- Update grcov download command
- Update custom error tests

### Documentation

- Update template contexts about link_parsers
- Add minimal example
- Update copyright years

### Features

- Add `link_parsers` for parsing/extracting links ([#42](https://github.com/orhun/git-cliff/issues/42))
- Make the `git` section optional ([#45](https://github.com/orhun/git-cliff/issues/45))
- Make the `changelog` section optional ([#45](https://github.com/orhun/git-cliff/issues/45))
- [**breaking**] Use conventional commit body to check against commit parsers
- [**breaking**] Replace --topo-order by --date-order ([#58](https://github.com/orhun/git-cliff/issues/58))

### Miscellaneous Tasks

- Update arg parsing to clap v3 ([#49](https://github.com/orhun/git-cliff/issues/49))
- Upgrade dependencies
- Bump the Rust version in Dockerfile
- Run cargo-audit for checking vulnerabilities
- Update the runner to macos-11

### Refactor

- Apply clippy suggestions
- [**breaking**] Change the default value of `trim` to `true`
- Unify serde and serde_derive using derive feature ([#57](https://github.com/orhun/git-cliff/issues/57))

### Styling

- Update the styling
- Comply with MD022 and fix minor typos ([#61](https://github.com/orhun/git-cliff/issues/61))

## [0.5.0] - 2021-12-15

### Bug Fixes

- Update log test about exclude path
- Override the sort related config if args are present ([#39](https://github.com/orhun/git-cliff/issues/39))
- Checkout the repository before running fixtures
- Use the defined configuration file for fixtures
- Update the multi line docker command
- Strip the carriage return on fixtures while comparing
- Drop the skipped releases from 'previous' field

### Documentation

- Update `--with-commit` example in README.md

### Features

- Add `--topo-order` flag for sorting tags ([#29](https://github.com/orhun/git-cliff/issues/29))
- Support specifying the sorting methods in config ([#31](https://github.com/orhun/git-cliff/issues/31))
- Accept glob patterns for `--commit-path` argument
- Support multiple values for `--commit-path` argument
- Add `--exclude-path` argument for excluding related commits
- Add `--current` flag for processing the current tag ([#37](https://github.com/orhun/git-cliff/issues/37))
- Add `ignore_tags` option ([#40](https://github.com/orhun/git-cliff/issues/40))
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

- Add support for special characters in scopes ([#26](https://github.com/orhun/git-cliff/issues/26))

### Documentation

- Add GitLab CI/CD section to README.md ([#24](https://github.com/orhun/git-cliff/issues/24))
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
- Remove only the leading "v" from tags ([#18](https://github.com/orhun/git-cliff/issues/18))

### Documentation

- Add scope-sorted example ([#16](https://github.com/orhun/git-cliff/issues/16))
- Add raw/rendered output for scoped-sorted example
- Add packaging status badge to installation section
- Mention the signing key for binary releases ([#17](https://github.com/orhun/git-cliff/issues/17))
- Add "build from source" section to README.md

### Features

- Add `--sort` argument for sorting commits ([#15](https://github.com/orhun/git-cliff/issues/15))

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

- Fix default regexes and references in docs ([#7](https://github.com/orhun/git-cliff/issues/7))

### Documentation

- Update installation instructions for Arch Linux
- Add badge for joining the Matrix chat
- Update example regexes
- Update the default regex in scoped config example

### Features

- Support parsing the missing scopes with `default_scope` ([#8](https://github.com/orhun/git-cliff/issues/8))
- Support generating a changelog scoped to a directory ([#11](https://github.com/orhun/git-cliff/issues/11))

### Miscellaneous Tasks

- Upgrade dependencies

## [0.2.6] - 2021-09-04

### Bug Fixes

- Pin the cargo-chef version in Dockerfile

### Documentation

- Update docker commands to only mount the .git directory

### Miscellaneous Tasks

- Bump `git-conventional` to `0.10.1` ([#6](https://github.com/orhun/git-cliff/issues/6))
- Bump dependencies
- Bump cargo-chef version in Dockerfile

## [0.2.5] - 2021-08-20

### Documentation

- Mention breaking changes for templating
- Update template examples to mention how to contribute

### Features

- Add `breaking_description` to the template context ([#4](https://github.com/orhun/git-cliff/issues/4))

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
- Support a global location for configuration file ([#2](https://github.com/orhun/git-cliff/issues/2))

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

- Set the previous release when using `--latest` ([#3](https://github.com/orhun/git-cliff/issues/3))

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

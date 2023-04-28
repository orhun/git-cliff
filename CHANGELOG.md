# Changelog

All notable changes to this project will be documented in this file.

## [1.2.0] - 2023-04-28

### ⛰️  Features

- *(args)* Update clap and clap extras to v4 ([#137](https://github.com/orhun/git-cliff/issues/137))
- *(commit)* Make the fields of `Signature` public
- *(config)* Add a custom configuration file for the repository
- *(config)* Support placing configuration inside pyproject.toml ([#147](https://github.com/orhun/git-cliff/issues/147))
- *(docker)* Generate SBOM/provenance for the Docker image
- *(parser)* Support using regex group values ([#145](https://github.com/orhun/git-cliff/issues/145))

### 🐛 Bug Fixes

- *(ci)* Use MUSL build of cargo-tarpaulin
- *(ci)* Update cargo-tarpaulin installation command for CI
- *(config)* [**breaking**] Nested environment config overrides ([#157](https://github.com/orhun/git-cliff/issues/157))
- *(config)* Set max of `limit_commits` to the number of commits ([#140](https://github.com/orhun/git-cliff/issues/140))
- *(deploy)* Set the node cache dependency path
- *(docker)* Remove target directory from .dockerignore
- *(release)* Use the correct argument in release script
- *(website)* Fix broken links

### 🚜 Refactor

- *(cd)* Remove unnecessary config update
- *(ci)* Test the website deployment with a different job
- *(lib)* [**breaking**] Move changelog module to git-cliff-core
- *(test)* Handle errors for changelog module tests
- *(website)* Update header location

### 📚 Documentation

- *(blog)* Add blog post about what's new in 1.2.0
- *(blog)* Update the blog post style
- *(config)* Update the sections
- *(config)* Add comments to the default configuration file
- *(contributing)* Mention the nightly requirement for rustfmt
- *(contributing)* Update MSRV
- *(examples)* Move examples to separate file
- *(github)* Update the pull request template about GitHub labels
- *(github)* Update pull request template
- *(github)* Update issue templates
- *(github)* Update funding options
- *(github)* Add security policy
- *(readme)* Update README.md about documentation website
- *(readme)* Add tj-actions/git-cliff to the list of actions ([#152](https://github.com/orhun/git-cliff/issues/152))
- *(readme)* Add discord badge
- *(readme)* Add release-plz to related projects ([#151](https://github.com/orhun/git-cliff/issues/151))
- *(readme)* Fix typos in README.md
- *(readme)* Remove unneeded word in README.md ([#141](https://github.com/orhun/git-cliff/issues/141))
- *(readme)* Add link to the Console #141 interview about git-cliff
- *(website)* Add Twitter link to banner
- *(website)* Move documentation to the website ([#153](https://github.com/orhun/git-cliff/issues/153))

### 🎨 Styling

- *(docs)* Fix the grammar for tj-actions
- *(docs)* Update the formatting for python integration example
- *(readme)* Update the style for project name
- *(readme)* Apply formatting
- *(website)* Update the style for environment variable section

### 🧪 Testing

- *(deploy)* Test the website deployment for pull requests

### ⚙️ Miscellaneous Tasks

- *(cargo)* Update MSRV to 1.64.0
- *(cd)* Temporarily switch back to action-rs/toolchain
- *(ci)* Switch to dtolnay/rust-toolchain action
- *(ci)* Update runner images
- *(deps)* Bump actions/configure-pages from 1 to 3
- *(deps)* Bump actions/deploy-pages from 1 to 2
- *(deps)* Upgrade transitive dependencies
- *(deps)* Update clap dependencies
- *(deps)* Upgrade workspace dependencies
- *(deps)* Upgrade core dependencies
- *(docker)* Update versions in Dockerfile
- *(docker)* Bump the action versions in docker workflow
- *(docker)* Bump build-push-action to v4
- *(editorconfig)* Fix editorconfig syntax
- *(editorconfig)* Update editorconfig for better code readability
- *(examples)* Remove EXAMPLES.md
- *(github)* Integrate Dependabot
- *(github)* Integrate bors
- *(github)* Add contact links for issues
- *(website)* Add workflow file for deploying the website
- *(website)* Move website to website folder
- *(website)* Move website to docs for GitHub pages deployment

## [1.1.2] - 2023-01-20

### 🐛 Bug Fixes

- *(changelog)* Allow saving context to a file ([#138](https://github.com/orhun/git-cliff/issues/138))
- *(changelog)* Do not skip all tags when `skip_tags` is empty ([#136](https://github.com/orhun/git-cliff/issues/136))
- *(git)* Derive the tag order from commits instead of timestamp ([#139](https://github.com/orhun/git-cliff/issues/139))

### 🎨 Styling

- *(fmt)* Update the derives in config module

### ⚙️ Miscellaneous Tasks

- *(cargo)* Add metadata for cargo-binstall
- *(deps)* Upgrade core dependencies
- *(docker)* Update versions in Dockerfile

### Revert

- *(git)* Use timestamp for deriving the tag order ([#139](https://github.com/orhun/git-cliff/issues/139))

## [1.1.1] - 2023-01-09

### 📚 Documentation

- *(readme)* Update README.md about the NPM package

## [1.1.1-rc.4] - 2023-01-09

### 🐛 Bug Fixes

- *(npm)* Fix the type casting in base NPM package

## [1.1.1-rc.3] - 2023-01-09

### 🐛 Bug Fixes

- *(npm)* Fix the variable declaration for NPM package OS

### ⚙️ Miscellaneous Tasks

- *(cd)* Parallelize releasing on crates.io

## [1.1.1-rc.2] - 2023-01-09

### 🐛 Bug Fixes

- *(npm)* Rename the NPM binary package for Windows

### ⚙️ Miscellaneous Tasks

- *(cd)* Add README.md to the base NPM package
- *(npm)* Add more keywords to the base NPM package

## [1.1.1-rc.1] - 2023-01-09

### ⚙️ Miscellaneous Tasks

- *(npm)* Package `git-cliff` for npm ([#133](https://github.com/orhun/git-cliff/issues/133))

## [1.1.0] - 2023-01-08

### ⛰️  Features

- *(git)* Support generating changelog for multiple git repositories ([#13](https://github.com/orhun/git-cliff/issues/13))

### 🚜 Refactor

- *(cd)* Use the git-cliff-action output for GitHub release body

### 📚 Documentation

- *(readme)* Update copyright years
- *(readme)* Disable Liquid parsing in README.md by using raw blocks

### ⚙️ Miscellaneous Tasks

- *(cd)* Publish binaries for more platforms/architectures
- *(cd)* Bump git-cliff-action to v2
- *(config)* Update the description in the default config
- *(deps)* Upgrade dependencies
- *(deps)* Bump `git-conventional` dependency ([#130](https://github.com/orhun/git-cliff/issues/130))
- *(docker)* Add Jekyll configuration to .dockerignore
- *(github)* Add Jekyll theme configuration for GitHub pages
- *(release)* Improve the release script with additional messages

## [1.0.0] - 2022-12-25

### ⛰️  Features

- *(cd)* Publish Debian package via release workflow ([#113](https://github.com/orhun/git-cliff/issues/113))
- *(cd)* Include completions and mangen in binary releases ([#115](https://github.com/orhun/git-cliff/issues/115))
- *(changelog)* [**breaking**] Use current time for `--tag` argument ([#107](https://github.com/orhun/git-cliff/issues/107))
- *(changelog)* Allow running with `--prepend` and `--output` ([#120](https://github.com/orhun/git-cliff/issues/120))
- *(changelog, config)* [**breaking**] Replace `--date-order` by `--topo-order`

### 🐛 Bug Fixes

- *(fixtures)* Fix test fixture failures

### 🚜 Refactor

- *(clippy)* Apply clippy suggestions

### 📚 Documentation

- *(readme)* Fix GitHub badges in README.md

### 🎨 Styling

- *(readme)* Update README.md about the styling of footer field

### ⚙️ Miscellaneous Tasks

- *(cd)* Remove deprecated set-output usage
- *(ci)* Update actions/checkout to v3
- *(config)* Comment out custom commit preprocessor ([#112](https://github.com/orhun/git-cliff/issues/112))
- *(fixtures)* Run all test fixtures

## [0.10.0] - 2022-11-20

### ⛰️  Features

- *(args)* Add a short variant `-d` for specifying `--date-order` flag
- *(changelog)* Do not skip breaking changes if configured ([#114](https://github.com/orhun/git-cliff/issues/114))
- *(config)* Changelog for the last n commits ([#116](https://github.com/orhun/git-cliff/issues/116))

### 🐛 Bug Fixes

- *(changelog)* Warn against invalid tag range for `--current` flag ([#124](https://github.com/orhun/git-cliff/issues/124))
- *(docker)* Fix syntax error in Dockerfile
- *(docker)* Use an alternative method to fetch registry

### 🚜 Refactor

- *(deps)* Utilize workspace dependencies
- *(docker)* Improve cargo-chef caching in Dockerfile

### 📚 Documentation

- *(readme)* Update badge URL for Docker builds
- *(readme)* Add MacPorts install info ([#111](https://github.com/orhun/git-cliff/issues/111))

### ⚙️ Miscellaneous Tasks

- *(deps)* Upgrade core dependencies
- *(docker)* Update versions in Dockerfile

## [0.9.2] - 2022-09-24

### 🐛 Bug Fixes

- *(docker)* Remove custom user creation from the Dockerfile ([#109](https://github.com/orhun/git-cliff/issues/109))

### ⚙️ Miscellaneous Tasks

- *(audit)* Remove cargo-audit config
- *(ci)* Switch to cargo-tarpaulin for measuring code coverage ([#110](https://github.com/orhun/git-cliff/issues/110))
- *(deps)* Upgrade dependencies

## [0.9.1] - 2022-09-20

### 🐛 Bug Fixes

- *(docker)* Configure git safe.directory for Docker image ([#108](https://github.com/orhun/git-cliff/issues/108))

### 🚜 Refactor

- *(clippy)* Apply clippy suggestions

### 🎨 Styling

- *(readme)* Update styling for with-commit example

### ⚙️ Miscellaneous Tasks

- *(deps)* Upgrade dependencies
- *(deps)* Remove ansi_term dependency for fixing RUSTSEC-2021-0139

## [0.9.0] - 2022-08-16

### ⛰️  Features

- *(changelog)* Support setting commit SHA while using `--with-commit`
- *(changelog)* Support splitting commits by lines ([#101](https://github.com/orhun/git-cliff/issues/101))
- *(commit)* Add commit author and committer to the context ([#100](https://github.com/orhun/git-cliff/issues/100))

### 🚜 Refactor

- *(clippy)* Run clippy for tests
- *(commit)* Use a more concise conversion for string

### 📚 Documentation

- *(readme)* Add test repository link to README.md

### ⚙️ Miscellaneous Tasks

- *(build)* Enable strip option for release profile
- *(deps)* Upgrade dependencies
- *(docker)* Upgrade versions in Dockerfile
- *(docker)* Disable updating the description on Docker Hub
- *(docker)* Update the description on Docker Hub on push
- *(docker)* Enable building arm64 docker images
- *(docker)* Use an alternative method to fetch registry
- *(funding)* Add GitHub Sponsors option for funding
- *(project)* Update MSRV to 1.60.0

## [0.8.1] - 2022-07-12

### 🐛 Bug Fixes

- *(cd)* Set fail-fast strategy to false

### ⚙️ Miscellaneous Tasks

- *(cd)* Update windows runners to windows-2022

## [0.8.0] - 2022-07-12

### ⛰️  Features

- *(changelog)* Support external commands for commit preprocessors ([#86](https://github.com/orhun/git-cliff/issues/86))
- *(commit)* [**breaking**] Pass footer token and separator to template ([#97](https://github.com/orhun/git-cliff/issues/97))
- *(config)* Support changing commit scope with `commit_parsers` ([#94](https://github.com/orhun/git-cliff/issues/94))

### 🐛 Bug Fixes

- *(ci)* Update lychee arguments to skip checking protonmail

### 🚜 Refactor

- *(clippy)* Apply clippy suggestions
- *(clippy)* Apply clippy suggestions

### 📚 Documentation

- *(readme)* Switch chronological and topological ([#99](https://github.com/orhun/git-cliff/issues/99))
- *(readme)* Clarify that `--tag` argument can be an unexisting tag

### ⚙️ Miscellaneous Tasks

- *(deps)* Upgrade dependencies
- *(deps)* Update tera to 1.16.0 ([#70](https://github.com/orhun/git-cliff/issues/70))
- *(docker)* Disable building arm64 docker images temporarily
- *(project)* Set MSRV to 1.58.1 ([#87](https://github.com/orhun/git-cliff/issues/87))

## [0.7.0] - 2022-04-24

### ⛰️  Features

- *(args)* [**breaking**] Prefix environment variables with `GIT_CLIFF_` ([#76](https://github.com/orhun/git-cliff/issues/76))
- *(args)* Add `--context` flag for outputting context ([#71](https://github.com/orhun/git-cliff/issues/71))
- *(cli)* Show a message if a newer version is available ([#69](https://github.com/orhun/git-cliff/issues/69))
- *(config)* Support placing configuration inside Cargo.toml ([#46](https://github.com/orhun/git-cliff/issues/46))
- *(git)* Support preprocessing commit messages using regex ([#62](https://github.com/orhun/git-cliff/issues/62))
- *(log)* Print more debug information when `-vv` is used ([#79](https://github.com/orhun/git-cliff/issues/79))
- *(man)* Add man page generation script ([#35](https://github.com/orhun/git-cliff/issues/35))

### 🐛 Bug Fixes

- *(build)* Pin the Rust nightly version
- *(changelog)* Allow custom commit range while prepending ([#68](https://github.com/orhun/git-cliff/issues/68))
- *(ci)* Pin the Rust nightly version
- *(fixtures)* Update expected changelog date
- *(log)* Remove redundant logging while using `--context` ([#71](https://github.com/orhun/git-cliff/issues/71))

### 🚜 Refactor

- *(cli)* Make update-informer opt-out via feature flag ([#69](https://github.com/orhun/git-cliff/issues/69))
- *(completions)* Use implicit Result type in completions script

### 📚 Documentation

- *(readme)* Update the title of projects section
- *(readme)* Add `cliff-jumper` to similar projects ([#83](https://github.com/orhun/git-cliff/issues/83))
- *(readme)* Update GitHub Actions reference link in README.md
- *(readme)* Add more regex examples for commit_preprocessors

### 🎨 Styling

- *(release)* Update the changelog template for tag message

### ⚙️ Miscellaneous Tasks

- *(cd)* Include man page in the release assets
- *(ci)* Return to nightly builds ([#73](https://github.com/orhun/git-cliff/issues/73))
- *(deps)* Upgrade dependencies
- *(deps)* Upgrade git-conventional dependency ([#82](https://github.com/orhun/git-cliff/issues/82))
- *(docker)* Strip the binaries in Docker image
- *(docker)* Disable default features for the Docker image
- *(docker)* Build Docker images for arm64
- *(docker)* Upgrade versions in Dockerfile

## [0.6.1] - 2022-03-13

### 🐛 Bug Fixes

- *(changelog)* Use root commit when --latest and there is only one tag ([#59](https://github.com/orhun/git-cliff/issues/59))
- *(changelog)* Do not skip all tags when `skip_tags` is empty ([#63](https://github.com/orhun/git-cliff/issues/63))
- *(example)* Fix `keepachangelog` config example ([#66](https://github.com/orhun/git-cliff/issues/66))
- *(project)* Use the correct branch for codecov ([#65](https://github.com/orhun/git-cliff/issues/65))

### 📚 Documentation

- *(core)* Document timestamp format of `Release` struct ([#67](https://github.com/orhun/git-cliff/issues/67))
- *(readme)* Add another option of GitHub Actions ([#64](https://github.com/orhun/git-cliff/issues/64))

### ⚙️ Miscellaneous Tasks

- *(deps)* Upgrade dependencies
- *(deps)* Upgrade regex dependency to fix CVE-2022-24713

## [0.6.0] - 2022-02-12

### ⛰️  Features

- *(changelog)* [**breaking**] Use conventional commit body to check against commit parsers
- *(changelog)* Add `link_parsers` for parsing/extracting links ([#42](https://github.com/orhun/git-cliff/issues/42))
- *(changelog, config)* [**breaking**] Replace --topo-order by --date-order ([#58](https://github.com/orhun/git-cliff/issues/58))
- *(config)* Make the `changelog` section optional ([#45](https://github.com/orhun/git-cliff/issues/45))
- *(config)* Make the `git` section optional ([#45](https://github.com/orhun/git-cliff/issues/45))

### 🐛 Bug Fixes

- *(changelog)* Set the previous release when using `--unreleased` ([#47](https://github.com/orhun/git-cliff/issues/47))
- *(changelog)* Only drop previous releases if skipped ([#44](https://github.com/orhun/git-cliff/issues/44))
- *(ci)* Update grcov download command
- *(ci)* Use the correct tar command for extracting grcov archive
- *(ci)* Update the download link of latest grcov release
- *(ci)* Run clippy from nightly toolchain
- *(config)* Lower the priority of global configuration file ([#51](https://github.com/orhun/git-cliff/issues/51))
- *(test)* Update tests about optional config values
- *(tests)* Update custom error tests

### 🚜 Refactor

- *(clippy)* Apply clippy suggestions
- *(config)* [**breaking**] Change the default value of `trim` to `true`
- *(lib)* Unify serde and serde_derive using derive feature ([#57](https://github.com/orhun/git-cliff/issues/57))

### 📚 Documentation

- *(config)* Add minimal example
- *(readme)* Update copyright years
- *(readme)* Update template contexts about link_parsers

### 🎨 Styling

- *(changelog)* Comply with MD022 and fix minor typos ([#61](https://github.com/orhun/git-cliff/issues/61))
- *(readme)* Update the styling

### ⚙️ Miscellaneous Tasks

- *(args)* Update arg parsing to clap v3 ([#49](https://github.com/orhun/git-cliff/issues/49))
- *(cd)* Update the runner to macos-11
- *(ci)* Run cargo-audit for checking vulnerabilities
- *(deps)* Upgrade dependencies
- *(docker)* Bump the Rust version in Dockerfile

## [0.5.0] - 2021-12-15

### ⛰️  Features

- *(args)* Add `--with-commit` argument for including custom commit messages in changelog
- *(args)* Add `--current` flag for processing the current tag ([#37](https://github.com/orhun/git-cliff/issues/37))
- *(args)* Add `--exclude-path` argument for excluding related commits
- *(args)* Support multiple values for `--commit-path` argument
- *(args)* Accept glob patterns for `--commit-path` argument
- *(changelog)* Support having both conventional and unconventional commits in the changelog
- *(changelog)* Add `--topo-order` flag for sorting tags ([#29](https://github.com/orhun/git-cliff/issues/29))
- *(config)* Add `ignore_tags` option ([#40](https://github.com/orhun/git-cliff/issues/40))
- *(config)* Support specifying the sorting methods in config ([#31](https://github.com/orhun/git-cliff/issues/31))
- *(template)* Use more explanatory error messages about templates

### 🐛 Bug Fixes

- *(args)* Override the sort related config if args are present ([#39](https://github.com/orhun/git-cliff/issues/39))
- *(changelog)* Drop the skipped releases from 'previous' field
- *(fixtures)* Strip the carriage return on fixtures while comparing
- *(fixtures)* Update the multi line docker command
- *(fixtures)* Use the defined configuration file for fixtures
- *(fixtures)* Checkout the repository before running fixtures
- *(tests)* Update log test about exclude path

### 🚜 Refactor

- *(config)* Rename the config value for commit order

### 📚 Documentation

- *(readme)* Update `--with-commit` example in README.md

### 🎨 Styling

- *(args)* [**breaking**] Rename `--commit-path` argument to `--include-path`

### ⚙️ Miscellaneous Tasks

- *(config)* Indicate the breaking changes via default config
- *(fixtures)* Run test fixtures on ubuntu-latest
- *(fixtures)* Improve the workflow for test fixtures

## [0.4.2] - 2021-10-22

### 🐛 Bug Fixes

- *(cd)* Install the Rust toolchain explicitly for crates.io releases

## [0.4.1] - 2021-10-22

### 🐛 Bug Fixes

- *(changelog)* Add support for special characters in scopes ([#26](https://github.com/orhun/git-cliff/issues/26))

### 🚜 Refactor

- *(git)* Use a better error message for invalid repo path

### 📚 Documentation

- *(readme)* Update GitLab CI/CD section
- *(readme)* Add GitLab CI/CD section to README.md ([#24](https://github.com/orhun/git-cliff/issues/24))

### ⚙️ Miscellaneous Tasks

- *(ci)* Run CI workflows periodically
- *(deps)* Upgrade dependencies
- *(docker)* Bump the Rust version in Dockerfile
- *(project)* Migrate to Rust 2021 edition
- *(project)* Remove unnecessary Cargo.lock entry from .gitignore

## [0.4.0] - 2021-10-01

### ⛰️  Features

- *(changelog)* Add `--sort` argument for sorting commits ([#15](https://github.com/orhun/git-cliff/issues/15))

### 🐛 Bug Fixes

- *(ci)* Update lychee arguments to skip checking files
- *(config)* Remove only the leading "v" from tags ([#18](https://github.com/orhun/git-cliff/issues/18))
- *(docker)* Remove tags from the base image names

### 📚 Documentation

- *(config)* Add scope-sorted example ([#16](https://github.com/orhun/git-cliff/issues/16))
- *(readme)* Add "build from source" section to README.md
- *(readme)* Mention the signing key for binary releases ([#17](https://github.com/orhun/git-cliff/issues/17))
- *(readme)* Add packaging status badge to installation section
- *(readme)* Add raw/rendered output for scoped-sorted example

### 🎨 Styling

- *(config)* Fix the newline issues in scoped-sorted example

### ⚙️ Miscellaneous Tasks

- *(deps)* Upgrade dependencies
- *(docker)* Use docker.yml workflow for CI/CD
- *(docker)* Use explicit image name for docker automated builds
- *(docker)* Specify the latest tag explicitly
- *(docker)* Rename the GHCR package due to legacy reasons
- *(docker)* Extend the tags for docker meta
- *(docker)* Use docker meta for tagging for GHCR
- *(docker)* Use cache for docker builds
- *(workflow)* Update the runner to ubuntu-20.04
- *(workflow)* Set a version for the checkout action

## [0.3.0] - 2021-09-10

### ⛰️  Features

- *(changelog)* Support generating a changelog scoped to a directory ([#11](https://github.com/orhun/git-cliff/issues/11))
- *(changelog)* Support parsing the missing scopes with `default_scope` ([#8](https://github.com/orhun/git-cliff/issues/8))

### 🐛 Bug Fixes

- *(config)* Fix default regexes and references in docs ([#7](https://github.com/orhun/git-cliff/issues/7))

### 📚 Documentation

- *(config)* Update the default regex in scoped config example
- *(readme)* Update example regexes
- *(readme)* Add badge for joining the Matrix chat
- *(readme)* Update installation instructions for Arch Linux

### ⚙️ Miscellaneous Tasks

- *(deps)* Upgrade dependencies

## [0.2.6] - 2021-09-04

### 🐛 Bug Fixes

- *(docker)* Pin the cargo-chef version in Dockerfile

### 📚 Documentation

- *(readme)* Update docker commands to only mount the .git directory

### ⚙️ Miscellaneous Tasks

- *(deps)* Bump dependencies
- *(deps)* Bump `git-conventional` to `0.10.1` ([#6](https://github.com/orhun/git-cliff/issues/6))
- *(docker)* Bump cargo-chef version in Dockerfile

## [0.2.5] - 2021-08-20

### ⛰️  Features

- *(template)* Add `breaking_description` to the template context ([#4](https://github.com/orhun/git-cliff/issues/4))

### 📚 Documentation

- *(readme)* Update template examples to mention how to contribute
- *(readme)* Mention breaking changes for templating

### ⚙️ Miscellaneous Tasks

- *(release)* Show the committed changes before creating a tag

## [0.2.4] - 2021-08-20

### 🐛 Bug Fixes

- *(cd)* Change the config file location for crates.io release

## [0.2.3] - 2021-08-18

### 🐛 Bug Fixes

- *(cd)* Fetch the dependencies before copying the file to embed

## [0.2.2] - 2021-08-18

### 🐛 Bug Fixes

- *(cd)* Copy the config file into registry to resolve it for embed

## [0.2.1] - 2021-08-18

### 🐛 Bug Fixes

- *(cd)* Copy the configuration file to embed into package

## [0.2.0] - 2021-08-18

### ⛰️  Features

- *(config)* Support a global location for configuration file ([#2](https://github.com/orhun/git-cliff/issues/2))
- *(config)* Add `--init` flag for creating the default config
- *(config)* Embed the default configuration file into the binary

### 🐛 Bug Fixes

- *(config)* Use custom error type for UTF-8 errors

### 🚜 Refactor

- *(lib)* Update the log message for unprocessed tags
- *(lib)* Create a constant for default configuration file

### 📚 Documentation

- *(changelog)* Update the doc comment of `prepend`

### 🎨 Styling

- *(args)* Update the message of `--init` flag

### ⚙️ Miscellaneous Tasks

- *(config)* Move `cliff.toml` to config/

## [0.1.2] - 2021-08-14

### 🐛 Bug Fixes

- *(cd)* Use the correct name of completions binary

### 📚 Documentation

- *(completions)* Update the example completion command

## [0.1.1] - 2021-08-14

### 🐛 Bug Fixes

- *(changelog)* Set the previous release when using `--latest` ([#3](https://github.com/orhun/git-cliff/issues/3))

### 📚 Documentation

- *(readme)* Add installation instructions for the AUR

### ⚡ Performance

- *(changelog)* Optimize the release vector size
- *(changelog)* Process only the last 'previous' release

### ⚙️ Miscellaneous Tasks

- *(deps)* Upgrade dependencies
- *(project)* Rename the shell completions binary

## [0.1.0] - 2021-08-12

### 🐛 Bug Fixes

- *(changelog)* Return error if there is not a latest tag to process
- *(changelog)* Use footers field as an array for the context
- *(config)* Update the environment variable parsing settings
- *(example)* Remove symbolic link
- *(example)* Update symbolic link to the default config
- *(git)* Sort the commits in topological order
- *(template)* Use 7 digits for short SHA

### 🚜 Refactor

- *(args)* Rename changelog argument to prepend

### 📚 Documentation

- *(readme)* Add preview image to README.md
- *(readme)* Update detailed template example
- *(readme)* Add examples for templating
- *(readme)* Add examples for CLI usage
- *(readme)* Update README.md about template and examples
- *(readme)* Update README.md about usage

### 🎨 Styling

- *(readme)* Remove quotes from rendered output
- *(readme)* Wrap table of contents into summary
- *(readme)* Remove comments from template context
- *(readme)* Update the comments in template context
- *(readme)* Center the badges

### 🧪 Testing

- *(config)* Add tests
- *(git)* Update repository tests about getting the latest tag

### ⚙️ Miscellaneous Tasks

- *(deps)* Upgrade dependencies
- *(deps)* Upgrade dependencies
- *(docker)* Bump the rust version
- *(git)* Remove etc directory from .gitignore

## [0.1.0-rc.21] - 2021-07-01

### 🐛 Bug Fixes

- *(cd)* Wait for core library to update on crates.io before publish

## [0.1.0-rc.20] - 2021-06-30

### 🐛 Bug Fixes

- *(cd)* Wait between publishing crates

## [0.1.0-rc.19] - 2021-06-30

### 🐛 Bug Fixes

- *(cd)* Generate changelog on a dedicated/different job

### ⚙️ Miscellaneous Tasks

- *(cargo)* Update project details

## [0.1.0-rc.18] - 2021-06-30

### 🐛 Bug Fixes

- *(cd)* Fix the syntax of publish step arguments
- *(cd)* Use a separate step for setting the changelog body

## [0.1.0-rc.17] - 2021-06-29

### 🐛 Bug Fixes

- *(cd)* Publish the cargo workspace members seperately

### ⚙️ Miscellaneous Tasks

- *(release)* Indicate which versions are managed by the script
- *(release)* Verify the created tag after creation

## [0.1.0-rc.16] - 2021-06-29

### 🐛 Bug Fixes

- *(ci)* Update lychee arguments to exclude invalid links

### 📚 Documentation

- *(contributing)* Add CONTRIBUTING.md
- *(release)* Add RELEASE.md
- *(release)* Add link to the signer key of the tag

### ⚙️ Miscellaneous Tasks

- *(cd)* Enable crates.io releases
- *(release)* Set the new version in release script

## [0.1.0-rc.15] - 2021-06-23

### ⚙️ Miscellaneous Tasks

- *(cd)* Use only one step for uploading releases

## [0.1.0-rc.14] - 2021-06-23

### 🐛 Bug Fixes

- *(cd)* Strip the changelog header before escaping

## [0.1.0-rc.13] - 2021-06-23

### ⚙️ Miscellaneous Tasks

- *(cd)* Use seperate steps for uploading releases

## [0.1.0-rc.12] - 2021-06-21

### 🐛 Bug Fixes

- *(cd)* Use printf to prevent field splitting the variable
- *(git)* Return tags by their creation order
- *(release)* Fix the character escape in release script

### ⚙️ Miscellaneous Tasks

- *(project)* Update .editorconfig about shell scripts
- *(release)* Include the commit id in the custom template

## [0.1.0-rc.11] - 2021-06-21

### 🚜 Refactor

- *(error)* Use custom error message for GroupError

### ⚙️ Miscellaneous Tasks

- *(cd)* Remove the custom changelog template

## [0.1.0-rc.10] - 2021-06-20

### 🎨 Styling

- *(config)* Update the order of entries in config

## [0.1.0-rc.8] - 2021-06-20

### 🐛 Bug Fixes

- *(cd)* Double quote the environment variable
- *(release)* Specify the committer email in release script

### ⚙️ Miscellaneous Tasks

- *(docker)* Rename the docker automated builds action

## [0.1.0-rc.7] - 2021-06-20

### ⛰️  Features

- *(changelog)* Support setting the body template via args

### ⚙️ Miscellaneous Tasks

- *(cd)* Override the changelog template
- *(release)* Set a custom changelog for the tag message

## [0.1.0-rc.6] - 2021-06-20

### ⚙️ Miscellaneous Tasks

- *(cd)* Set the release body on linux

## [0.1.0-rc.5] - 2021-06-19

### 🐛 Bug Fixes

- *(config)* Update config to skip release commits

## [0.1.0-rc.4] - 2021-06-19

### Revert

- *(uncategorized)* Chore(config): update template to include commit ids

## [0.1.0-rc.3] - 2021-06-19

### 🐛 Bug Fixes

- *(config)* Update commit parsers to match the commit type
- *(release)* Strip the unreleased title from tag message

### ⚙️ Miscellaneous Tasks

- *(cd)* Fix setting the release body
- *(config)* Update template to include commit ids
- *(config)* Update the skip_tags regex

## [0.1.0-rc.2] - 2021-06-19

### ⛰️  Features

- *(args)* Add `--output` argument

### 🐛 Bug Fixes

- *(test)* Use default tag_pattern for tests

### 🚜 Refactor

- *(config)* Make tag_pattern optional

### ⚙️ Miscellaneous Tasks

- *(cd)* Set the changelog as release body
- *(cd)* Set the release name explicitly
- *(docker)* Remove user directive from Dockerfile
- *(release)* Add release title to the tag message

## [0.1.0-rc.1] - 2021-06-16

### ⛰️  Features

- *(args)* Add `--workdir` argument
- *(logs)* Show the processsed commit message

### 🚜 Refactor

- *(args)* Update value names and description
- *(args)* Update the value name for `--strip`
- *(logs)* Improve logging

### 📚 Documentation

- *(bin)* Update the doc comment for completions script
- *(readme)* Add usage section

### ⚙️ Miscellaneous Tasks

- *(project)* Update the release script about arguments
- *(project)* Add release script
- *(release)* Strip the markdown format from tag message

## [0.1.0-beta.4] - 2021-06-14

### 🐛 Bug Fixes

- *(cd)* Use bash while setting the release version

### ⚙️ Miscellaneous Tasks

- *(cd)* Add docker releases

## [0.1.0-beta.3] - 2021-06-14

### 🐛 Bug Fixes

- *(cd)* Specify the bash as shell
- *(cd)* Include configuration file in the binary releases

### ⚙️ Miscellaneous Tasks

- *(cd)* Set the release body text

## [0.1.0-beta.2] - 2021-06-14

### 🐛 Bug Fixes

- *(cd)* Install musl-tools for musl builds

### ⚙️ Miscellaneous Tasks

- *(config)* Update config

<!-- generated by git-cliff -->

# Changelog

All notable changes to this project will be documented in this file.

## [1.0.27] - 2023-01-09

### Testing

- Last attempt of npm publish

## [1.0.26] - 2023-01-09

### Bug Fixes

- Update npm config

## [1.0.25] - 2023-01-09

### Bug Fixes

- Use alternative approach for npm login

## [1.0.24] - 2023-01-09

### Miscellaneous Tasks

- Update npm config

## [1.0.23] - 2023-01-09

### Bug Fixes

- Remove setup-node action

## [1.0.22] - 2023-01-09

### Bug Fixes

- Use npm publish instead of yarn publish

## [1.0.21] - 2023-01-09

### Miscellaneous Tasks

- Set the token in npmrc

## [1.0.20] - 2023-01-09

### Bug Fixes

- Specify more tokens

## [1.0.19] - 2023-01-09

### Bug Fixes

- Specify registry explicitly

## [1.0.18] - 2023-01-09

### Features

- Publish on GitHub NPM registry

## [1.0.17] - 2023-01-07

### Refactor

- Use changelog content

## [1.0.16] - 2023-01-07

### Bug Fixes

- Use env

## [1.0.15] - 2023-01-07

### Testing

- Print changelog

## [1.0.14] - 2023-01-07

### Testing

- Set release text manually

## [1.0.13] - 2023-01-07

### Bug Fixes

- Set release body

## [1.0.12] - 2023-01-06

### Miscellaneous Tasks

- Apply shellcheck suggestion

## [1.0.10] - 2023-01-06

### Bug Fixes

- Use an alternative way for splitting

## [1.0.9] - 2023-01-06

### Bug Fixes

- Optional dependencies are not added to git-cliff
- Debug env

## [1.0.8] - 2023-01-06

### Bug Fixes

- Process doesnt return status code
- Fix env substition

## [1.0.7] - 2023-01-06

### Miscellaneous Tasks

- Add debug log for generated manifest

## [1.0.6] - 2023-01-06

### Bug Fixes

- Fix splitting os and arch

## [1.0.5] - 2023-01-06

### Miscellaneous Tasks

- Update NPM versions in release script
- Debug NPM publish step

## [1.0.4] - 2023-01-06

### Bug Fixes

- Use cp instead of install

## [1.0.3] - 2023-01-06

### Bug Fixes

- Fix emulating build

## [1.0.2] - 2023-01-06

### Bug Fixes

- Emulate build

## [1.0.1] - 2023-01-06

### Bug Fixes

- Add timestamp field to Release structs
- Update tests about recent changes
- Update fields
- Require -u or -l flag for prepending changelog
- Write to the given file for prepend operation
- Install zlib dependency
- Install musl-tools for musl builds
- Include configuration file in the binary releases
- Specify the bash as shell
- Use bash while setting the release version
- Use default tag_pattern for tests
- Strip the unreleased title from tag message
- Update commit parsers to match the commit type
- Update config to skip release commits
- Specify the committer email in release script
- Double quote the environment variable
- Use printf to prevent field splitting the variable
- Fix the character escape in release script
- Return tags by their creation order
- Strip the changelog header before escaping
- Update lychee arguments to exclude invalid links
- Publish the cargo workspace members seperately
- Use a separate step for setting the changelog body
- Fix the syntax of publish step arguments
- Generate changelog on a dedicated/different job
- Wait between publishing crates
- Wait for core library to update on crates.io before publish
- Update the environment variable parsing settings
- Use footers field as an array for the context
- Sort the commits in topological order
- Return error if there is not a latest tag to process
- Update symbolic link to the default config
- Remove symbolic link
- Use 7 digits for short SHA
- Set the previous release when using `--latest` ([#3](https://github.com/orhun/git-cliff/issues/3))
- Use the correct name of completions binary
- Use custom error type for UTF-8 errors
- Copy the configuration file to embed into package
- Copy the config file into registry to resolve it for embed
- Fetch the dependencies before copying the file to embed
- Change the config file location for crates.io release
- Pin the cargo-chef version in Dockerfile
- Fix default regexes and references in docs ([#7](https://github.com/orhun/git-cliff/issues/7))
- Update lychee arguments to skip checking files
- Remove tags from the base image names
- Remove only the leading "v" from tags ([#18](https://github.com/orhun/git-cliff/issues/18))
- Add support for special characters in scopes ([#26](https://github.com/orhun/git-cliff/issues/26))
- Install the Rust toolchain explicitly for crates.io releases
- Update log test about exclude path
- Override the sort related config if args are present ([#39](https://github.com/orhun/git-cliff/issues/39))
- Checkout the repository before running fixtures
- Use the defined configuration file for fixtures
- Update the multi line docker command
- Strip the carriage return on fixtures while comparing
- Drop the skipped releases from 'previous' field
- Only drop previous releases if skipped ([#44](https://github.com/orhun/git-cliff/issues/44))
- Run clippy from nightly toolchain
- Update tests about optional config values
- Set the previous release when using `--unreleased` ([#47](https://github.com/orhun/git-cliff/issues/47))
- Lower the priority of global configuration file ([#51](https://github.com/orhun/git-cliff/issues/51))
- Update the download link of latest grcov release
- Use the correct tar command for extracting grcov archive
- Update grcov download command
- Update custom error tests
- Do not skip all tags when `skip_tags` is empty ([#63](https://github.com/orhun/git-cliff/issues/63))
- Use root commit when --latest and there is only one tag ([#59](https://github.com/orhun/git-cliff/issues/59))
- Use the correct branch for codecov ([#65](https://github.com/orhun/git-cliff/issues/65))
- Fix `keepachangelog` config example ([#66](https://github.com/orhun/git-cliff/issues/66))
- Pin the Rust nightly version
- Pin the Rust nightly version
- Allow custom commit range while prepending ([#68](https://github.com/orhun/git-cliff/issues/68))
- Remove redundant logging while using `--context` ([#71](https://github.com/orhun/git-cliff/issues/71))
- Update expected changelog date
- Update lychee arguments to skip checking protonmail
- Set fail-fast strategy to false
- Configure git safe.directory for Docker image ([#108](https://github.com/orhun/git-cliff/issues/108))
- Remove custom user creation from the Dockerfile ([#109](https://github.com/orhun/git-cliff/issues/109))
- Warn against invalid tag range for `--current` flag ([#124](https://github.com/orhun/git-cliff/issues/124))
- Use an alternative method to fetch registry
- Fix syntax error in Dockerfile
- Fix test fixture failures

### Documentation

- Add doc comment to GroupParser
- Add comments
- Add FUNDING.yml
- Add comments to struct fields
- Add doc comment to generate method
- Add comments to main
- Mention docker
- Use latest tag for the docker command
- Add CHANGELOG.md
- Update the doc comment for completions script
- Add usage section
- Add CONTRIBUTING.md
- Add link to the signer key of the tag
- Add RELEASE.md
- Update README.md about usage
- Update README.md about template and examples
- Add examples for CLI usage
- Add examples for templating
- Update detailed template example
- Add preview image to README.md
- Add installation instructions for the AUR
- Update the example completion command
- Update the doc comment of `prepend`
- Mention breaking changes for templating
- Update template examples to mention how to contribute
- Update docker commands to only mount the .git directory
- Update installation instructions for Arch Linux
- Add badge for joining the Matrix chat
- Update example regexes
- Update the default regex in scoped config example
- Add scope-sorted example ([#16](https://github.com/orhun/git-cliff/issues/16))
- Add raw/rendered output for scoped-sorted example
- Add packaging status badge to installation section
- Mention the signing key for binary releases ([#17](https://github.com/orhun/git-cliff/issues/17))
- Add "build from source" section to README.md
- Add GitLab CI/CD section to README.md ([#24](https://github.com/orhun/git-cliff/issues/24))
- Update GitLab CI/CD section
- Update `--with-commit` example in README.md
- Update template contexts about link_parsers
- Add minimal example
- Update copyright years
- Add another option of GitHub Actions ([#64](https://github.com/orhun/git-cliff/issues/64))
- Document timestamp format of `Release` struct ([#67](https://github.com/orhun/git-cliff/issues/67))
- Add more regex examples for commit_preprocessors
- Update GitHub Actions reference link in README.md
- Add `cliff-jumper` to similar projects ([#83](https://github.com/orhun/git-cliff/issues/83))
- Update the title of projects section
- Clarify that `--tag` argument can be an unexisting tag
- Switch chronological and topological ([#99](https://github.com/orhun/git-cliff/issues/99))
- Add test repository link to README.md
- Add MacPorts install info ([#111](https://github.com/orhun/git-cliff/issues/111))
- Update badge URL for Docker builds
- Fix GitHub badges in README.md
- Disable Liquid parsing in README.md by using raw blocks
- Update copyright years

### Features

- Add parser powered by structopt
- Setup logging via pretty_env_logger
- Add custom error
- Parse commits
- Add parser
- Add common Release type
- Add template renderer for changelog generation
- Use commit messages in template
- Generate changelog based on commit types
- Support custom commit groups
- Support header and footer
- Support excluding invalid groups
- Generate changelog based on repository tags
- Add release dates to the template
- Add yaml support
- Add `--tag` argument
- Add capitalize_first filter
- Support skipping tags
- Support skipping commits
- Add other conventional fields to serialize
- Allow disabling conventional commits
- Support grouping based on commit body
- Allow generating changelog using a range of commits
- Support using metadata from the previous release
- Support generating changelog based on tagging status
- Add `--strip` argument for trimming changelog
- Support prepending entries to an existing changelog
- Show debug log for skipped tags
- Add trim option to changelog
- Extend the commit parsers
- Add `--workdir` argument
- Show the processsed commit message
- Add `--output` argument
- Support setting the body template via args
- Embed the default configuration file into the binary
- Add `--init` flag for creating the default config
- Support a global location for configuration file ([#2](https://github.com/orhun/git-cliff/issues/2))
- Add `breaking_description` to the template context ([#4](https://github.com/orhun/git-cliff/issues/4))
- Support parsing the missing scopes with `default_scope` ([#8](https://github.com/orhun/git-cliff/issues/8))
- Support generating a changelog scoped to a directory ([#11](https://github.com/orhun/git-cliff/issues/11))
- Add `--sort` argument for sorting commits ([#15](https://github.com/orhun/git-cliff/issues/15))
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
- Add `link_parsers` for parsing/extracting links ([#42](https://github.com/orhun/git-cliff/issues/42))
- Make the `git` section optional ([#45](https://github.com/orhun/git-cliff/issues/45))
- Make the `changelog` section optional ([#45](https://github.com/orhun/git-cliff/issues/45))
- [**breaking**] Use conventional commit body to check against commit parsers
- [**breaking**] Replace --topo-order by --date-order ([#58](https://github.com/orhun/git-cliff/issues/58))
- Show a message if a newer version is available ([#69](https://github.com/orhun/git-cliff/issues/69))
- Add `--context` flag for outputting context ([#71](https://github.com/orhun/git-cliff/issues/71))
- Support placing configuration inside Cargo.toml ([#46](https://github.com/orhun/git-cliff/issues/46))
- [**breaking**] Prefix environment variables with `GIT_CLIFF_` ([#76](https://github.com/orhun/git-cliff/issues/76))
- Print more debug information when `-vv` is used ([#79](https://github.com/orhun/git-cliff/issues/79))
- Support preprocessing commit messages using regex ([#62](https://github.com/orhun/git-cliff/issues/62))
- Add man page generation script ([#35](https://github.com/orhun/git-cliff/issues/35))
- Support external commands for commit preprocessors ([#86](https://github.com/orhun/git-cliff/issues/86))
- Support changing commit scope with `commit_parsers` ([#94](https://github.com/orhun/git-cliff/issues/94))
- [**breaking**] Pass footer token and separator to template ([#97](https://github.com/orhun/git-cliff/issues/97))
- Support splitting commits by lines ([#101](https://github.com/orhun/git-cliff/issues/101))
- Support setting commit SHA while using `--with-commit`
- Add commit author and committer to the context ([#100](https://github.com/orhun/git-cliff/issues/100))
- Do not skip breaking changes if configured ([#114](https://github.com/orhun/git-cliff/issues/114))
- Changelog for the last n commits ([#116](https://github.com/orhun/git-cliff/issues/116))
- Add a short variant `-d` for specifying `--date-order` flag
- [**breaking**] Replace `--date-order` by `--topo-order`
- Allow running with `--prepend` and `--output` ([#120](https://github.com/orhun/git-cliff/issues/120))
- [**breaking**] Use current time for `--tag` argument ([#107](https://github.com/orhun/git-cliff/issues/107))
- Include completions and mangen in binary releases ([#115](https://github.com/orhun/git-cliff/issues/115))
- Publish Debian package via release workflow ([#113](https://github.com/orhun/git-cliff/issues/113))
- Execute git cliff binary using nodejs
- Add publish step for npm binary artifacts
- Add step for publishing the base NPM package

### Miscellaneous Tasks

- Add workflow
- Use structopt without minor version
- Fix rustfmt arguments
- Fetch the entire git history for test suite
- Use nightly toolchain
- Use the forked git repository for git-conventional
- Use stable channel for cargo check
- Use stable channel for cargo commands except fmt
- Update git-conventional dependency
- Update the branch for git-conventional dependency
- Use grcov for test coverage
- Update git2 dependency
- Update code owners
- Rename
- Change default branch to main
- Fix grcov command
- Update details
- Remove normalize_comments from rustfmt.toml
- Add codecov.yml
- Update description
- Remove default features from git2
- Update description
- Add Dockerfile
- Add shell completion generation script
- Add workflow
- Update config
- Set the release body text
- Add docker releases
- Add release script
- Update the release script about arguments
- Strip the markdown format from tag message
- Add release title to the tag message
- Set the release name explicitly
- Remove user directive from Dockerfile
- Set the changelog as release body
- Fix setting the release body
- Update the skip_tags regex
- Update template to include commit ids
- Set the release body on linux
- Set a custom changelog for the tag message
- Override the changelog template
- Rename the docker automated builds action
- Remove the custom changelog template
- Update .editorconfig about shell scripts
- Include the commit id in the custom template
- Use seperate steps for uploading releases
- Use only one step for uploading releases
- Enable crates.io releases
- Set the new version in release script
- Verify the created tag after creation
- Indicate which versions are managed by the script
- Update project details
- Upgrade dependencies
- Remove etc directory from .gitignore
- Bump the rust version
- Upgrade dependencies
- Rename the shell completions binary
- Upgrade dependencies
- Move `cliff.toml` to config/
- Show the committed changes before creating a tag
- Bump `git-conventional` to `0.10.1` ([#6](https://github.com/orhun/git-cliff/issues/6))
- Bump dependencies
- Bump cargo-chef version in Dockerfile
- Upgrade dependencies
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
- Run CI workflows periodically
- Remove unnecessary Cargo.lock entry from .gitignore
- Upgrade dependencies
- Migrate to Rust 2021 edition
- Bump the Rust version in Dockerfile
- Improve the workflow for test fixtures
- Run test fixtures on ubuntu-latest
- Indicate the breaking changes via default config
- Update arg parsing to clap v3 ([#49](https://github.com/orhun/git-cliff/issues/49))
- Upgrade dependencies
- Bump the Rust version in Dockerfile
- Run cargo-audit for checking vulnerabilities
- Update the runner to macos-11
- Upgrade regex dependency to fix CVE-2022-24713
- Upgrade dependencies
- Return to nightly builds ([#73](https://github.com/orhun/git-cliff/issues/73))
- Include man page in the release assets
- Upgrade git-conventional dependency ([#82](https://github.com/orhun/git-cliff/issues/82))
- Upgrade versions in Dockerfile
- Build Docker images for arm64
- Disable default features for the Docker image
- Strip the binaries in Docker image
- Upgrade dependencies
- Set MSRV to 1.58.1 ([#87](https://github.com/orhun/git-cliff/issues/87))
- Update tera to 1.16.0 ([#70](https://github.com/orhun/git-cliff/issues/70))
- Disable building arm64 docker images temporarily
- Upgrade dependencies
- Update windows runners to windows-2022
- Use an alternative method to fetch registry
- Enable building arm64 docker images
- Update the description on Docker Hub on push
- Disable updating the description on Docker Hub
- Add GitHub Sponsors option for funding
- Upgrade dependencies
- Update MSRV to 1.60.0
- Upgrade versions in Dockerfile
- Enable strip option for release profile
- Remove ansi_term dependency for fixing RUSTSEC-2021-0139
- Upgrade dependencies
- Remove cargo-audit config
- Switch to cargo-tarpaulin for measuring code coverage ([#110](https://github.com/orhun/git-cliff/issues/110))
- Upgrade dependencies
- Update versions in Dockerfile
- Upgrade core dependencies
- Run all test fixtures
- Remove deprecated set-output usage
- Update actions/checkout to v3
- Comment out custom commit preprocessor ([#112](https://github.com/orhun/git-cliff/issues/112))
- Bump git-cliff-action to v2
- Add Jekyll theme configuration for GitHub pages
- Add Jekyll configuration to .dockerignore
- Bump `git-conventional` dependency ([#130](https://github.com/orhun/git-cliff/issues/130))
- Publish binaries for more platforms/architectures
- Scaffold a typescript node project
- Add packaging folder to .gitignore

### Performance

- Process only the last 'previous' release
- Optimize the release vector size

### Refactor

- Use githolit-core's Result type
- Use git_conventional crate
- Update `Release` struct
- Remove unreleased_title entry from config file
- Use `Release` type as changelog context
- Use template from config file
- Use git-conventional crate from crates.io
- Rename tag_regex to tag_pattern
- Use timestamp for release dates
- Deserialize regex while parsing
- Rename group_parsers to commit_parsers
- Remove bare unwrap from capitalize_first_filter
- Rename changelog module to template
- Add changelog module
- Remove `ReleaseRoot` struct
- Add section to configuration file
- Rename capitalize_first filter to upper_first
- Make commit parsers optional
- Make changelog header/footer optional
- Rename prepend argument to changelog
- Add possible values to strip argument
- Make skip_tags optional
- Update the order and value names
- Update the value name for `--strip`
- Improve logging
- Update value names and description
- Make tag_pattern optional
- Use custom error message for GroupError
- Rename changelog argument to prepend
- Create a constant for default configuration file
- Update the log message for unprocessed tags
- Use a better error message for invalid repo path
- Rename the config value for commit order
- Apply clippy suggestions
- [**breaking**] Change the default value of `trim` to `true`
- Unify serde and serde_derive using derive feature ([#57](https://github.com/orhun/git-cliff/issues/57))
- Make update-informer opt-out via feature flag ([#69](https://github.com/orhun/git-cliff/issues/69))
- Use implicit Result type in completions script
- Apply clippy suggestions
- Apply clippy suggestions
- Run clippy for tests
- Use a more concise conversion for string
- Apply clippy suggestions
- Improve cargo-chef caching in Dockerfile
- Utilize workspace dependencies
- Apply clippy suggestions

### Styling

- Update the formatting
- Update formatting
- Update formatting
- Update structure
- Update the order of entries in config
- Center the badges
- Update the comments in template context
- Remove comments from template context
- Wrap table of contents into summary
- Remove quotes from rendered output
- Update the message of `--init` flag
- Fix the newline issues in scoped-sorted example
- [**breaking**] Rename `--commit-path` argument to `--include-path`
- Update the styling
- Comply with MD022 and fix minor typos ([#61](https://github.com/orhun/git-cliff/issues/61))
- Update the changelog template for tag message
- Update styling for with-commit example
- Update README.md about the styling of footer field

### Testing

- Update tests
- Add tests
- Add integration tests
- Add tests
- Add tests
- Update repository tests about getting the latest tag
- Comment out specific steps

### Revert

- Chore(config): update template to include commit ids

<!-- generated by git-cliff -->

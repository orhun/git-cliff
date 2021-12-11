<p align="center">
    <a href="https://github.com/orhun/git-cliff">
        <img src="https://user-images.githubusercontent.com/24392180/121790699-8808dc80-cbea-11eb-8ab6-2fb6b08b66d8.png" width="300"></a>
    <br>
    <a href="https://github.com/orhun/git-cliff/releases">
        <img src="https://img.shields.io/github/v/release/orhun/git-cliff?style=flat&labelColor=1C2C2E&color=C96329&logo=GitHub&logoColor=white">
    </a>
    <a href="https://crates.io/crates/git-cliff/">
        <img src="https://img.shields.io/crates/v/git-cliff?style=flat&labelColor=1C2C2E&color=C96329&logo=Rust&logoColor=white">
    </a>
    <a href="https://codecov.io/gh/orhun/git-cliff">
        <img src="https://img.shields.io/codecov/c/gh/orhun/git-cliff?style=flat&labelColor=1C2C2E&color=C96329&logo=Codecov&logoColor=white">
    </a>
    <br>
    <a href="https://github.com/orhun/git-cliff/actions?query=workflow%3A%22Continuous+Integration%22">
        <img src="https://img.shields.io/github/workflow/status/orhun/git-cliff/Continuous%20Integration?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=GitHub%20Actions&logoColor=BEC5C9">
    </a>
    <a href="https://github.com/orhun/git-cliff/actions?query=workflow%3A%22Continuous+Deployment%22">
        <img src="https://img.shields.io/github/workflow/status/orhun/git-cliff/Continuous%20Deployment?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=GitHub%20Actions&logoColor=BEC5C9&label=deploy">
    </a>
    <a href="https://hub.docker.com/r/orhunp/git-cliff">
        <img src="https://img.shields.io/docker/cloud/build/orhunp/git-cliff?style=flat&labelColor=1C2C2E&color=BEC5C9&label=docker&logo=Docker&logoColor=BEC5C9">
    </a>
    <a href="https://docs.rs/git-cliff-core/">
        <img src="https://img.shields.io/docsrs/git-cliff-core?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=Rust&logoColor=BEC5C9E">
    </a>
    <br>
    <a href="https://matrix.to/#/#git-cliff:matrix.org">
        <img src="https://img.shields.io/matrix/git-cliff:matrix.org?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=matrix&logoColor=BEC5C9E&label=join%20matrix">
    </a>
</p>

## About

**git-cliff** can generate [changelog](https://en.wikipedia.org/wiki/Changelog) files from the [Git](https://git-scm.com/) history by utilizing [conventional commits](#conventional_commits) as well as regex-powered [custom parsers](#commit_parsers). The [changelog template](#templating) can be customized with a [configuration file](#configuration-file) to match the desired format.

![preview](https://user-images.githubusercontent.com/24392180/128637997-5713ba25-d8f3-40c7-8ba8-ea7f333ead88.png)

<details>
  <summary>Table of Contents</summary>

- [About](#about)
- [Installation](#installation)
  - [From crates.io](#from-cratesio)
  - [Using pacman](#using-pacman)
  - [Binary releases](#binary-releases)
  - [Build from source](#build-from-source)
- [Usage](#usage)
  - [Command Line Arguments](#command-line-arguments)
  - [Examples](#examples)
- [Docker](#docker)
- [GitHub Action](#github-action)
- [GitLab CI/CD](#gitlab-cicd)
- [Configuration File](#configuration-file)
  - [changelog](#changelog)
    - [header](#header)
    - [body](#body)
    - [trim](#trim)
    - [footer](#footer)
  - [git](#git)
    - [conventional_commits](#conventional_commits)
    - [filter_unconventional](#filter_unconventional)
    - [commit_parsers](#commit_parsers)
    - [filter_commits](#filter_commits)
    - [tag_pattern](#tag_pattern)
    - [skip_tags](#skip_tags)
    - [ignore_tags](#ignore_tags)
    - [topo_order](#topo_order)
    - [sort_commits](#sort_commits)
- [Templating](#templating)
  - [Context](#context)
    - [Conventional Commits](#conventional-commits)
      - [Breaking Changes](#breaking-changes)
    - [Non-Conventional Commits](#non-conventional-commits)
  - [Syntax](#syntax)
  - [Examples](#examples-1)
    - [Basic](#basic)
    - [Detailed](#detailed)
    - [Scoped](#scoped)
    - [Scoped (Sorted)](#scoped-sorted)
    - [Keep a Changelog](#keep-a-changelog)
    - [Unconventional](#unconventional)
- [Similar Projects](#similar-projects)
- [License](#license)
- [Copyright](#copyright)

</details>

## Installation

<details>
  <summary>Packaging status</summary>

[![Packaging status](https://repology.org/badge/vertical-allrepos/git-cliff.svg)](https://repology.org/project/git-cliff/versions)

</details>

### From crates.io

[git-cliff](crates.io/crates/git-cliff) can be installed from crates.io:

```sh
cargo install git-cliff
```

### Using pacman

If you are using Arch Linux, **git-cliff** can be installed from the [community repository](https://archlinux.org/packages/community/x86_64/git-cliff/):

```sh
pacman -S git-cliff
```

### Binary releases

See the available binaries for different operating systems/architectures from the [releases page](https://github.com/orhun/git-cliff/releases).

\* Release tarballs are signed with the following PGP key: [1D2D410A741137EBC544826F4A92FA17B6619297](https://keyserver.ubuntu.com/pks/lookup?search=0x4A92FA17B6619297&op=vindex)

### Build from source

* Linux dependencies: [zlib](https://zlib.net/)

```sh
# binary will be located at `target/release/git-cliff`
CARGO_TARGET_DIR=target cargo build --release
```

## Usage

### Command Line Arguments

```
git-cliff [FLAGS] [OPTIONS] [RANGE]
```

**Flags:**

```
-v, --verbose       Increases the logging verbosity
-i, --init          Writes the default configuration file to cliff.toml
-l, --latest        Processes the commits starting from the latest tag
    --current       Processes the commits that belong to the current tag
-u, --unreleased    Processes the commits that do not belong to a tag
    --topo-order    Sorts the tags topologically
-h, --help          Prints help information
-V, --version       Prints version information
```

**Options:**

```
-c, --config <PATH>               Sets the configuration file [env: CONFIG=]  [default: cliff.toml]
-w, --workdir <PATH>              Sets the working directory [env: WORKDIR=]
-r, --repository <PATH>           Sets the git repository [env: REPOSITORY=]
    --include-path <PATTERN>...   Sets the path to include related commits [env: INCLUDE_PATH=]
    --exclude-path <PATTERN>...   Sets the path to exclude related commits [env: EXCLUDE_PATH=]
    --with-commit <MSG>...        Sets custom commit messages to include in the changelog [env: WITH_COMMIT=]
-p, --prepend <PATH>              Prepends entries to the given changelog file [env: PREPEND=
-o, --output <PATH>               Writes output to the given file [env: OUTPUT=]
-t, --tag <TAG>                   Sets the tag for the latest version [env: TAG=]
-b, --body <TEMPLATE>             Sets the template for the changelog body [env: TEMPLATE=]
-s, --strip <PART>                Strips the given parts from the changelog [possible values: header, footer, all]
    --sort <sort>                 Sets sorting of the commits inside sections [default: oldest] [possible values: oldest, newest]
```

**Args:**

```
<RANGE>    Sets the commit range to process
```

### Examples

The default [configuration file](#configuration-file) (`cliff.toml`) can be generated using the `--init` flag:

```sh
# create cliff.toml
git cliff --init
```

Then simply create a changelog at your projects git root directory:

```sh
# same as running `git-cliff --config cliff.toml --repository .`
# same as running `git-cliff --workdir .`
git cliff
```

Set a tag for the "unreleased" changes:

```sh
git cliff --tag 1.0.0
```

Generate a changelog for a certain part of git history:

```sh
# only takes the latest tag into account
# (requires at least 2 tags)
git cliff --latest

# only takes the current tag into account
# useful if you checkout a specific tag (e.g. `git checkout v0.0.1`)
# (requires a tag to be present for the current commit (i.e. HEAD))
git cliff --current

# generate changelog for unreleased commits
git cliff --unreleased
git cliff --unreleased --tag 1.0.0

# generate changelog for a specific commit range
git cliff 4c7b043..a440c6e
git cliff 4c7b043..HEAD
git cliff HEAD~2..
```

Generate a changelog scoped to a specific directory (useful for monorepos):

```sh
git cliff --include-path "**/*.toml" --include-path "*.md"
git cliff --exclude-path ".github/*"
```

Generate a changelog that includes yet unexisting commit messages:

```sh
commit_msg="chore(release): update CHANGELOG.md for 1.0.0"

# You might need to include the commit messages that do not exist
# for testing purposes or solving the chicken-egg problem.
git cliff --with-commit "$commit_msg" -o CHANGELOG.md

git add CHANGELOG.md && git commit -m "$commit_msg"
```

Sort the commits inside sections:

```sh
# The oldest commit will be on top.
# (default)
git cliff --sort oldest

# The newest commit will be on top.
git cliff --sort newest
```

Sort the tags in topological order:

```sh
# Process in topological order instead of chronological.
git cliff --topo-order
```

Save the changelog file to the specified file:

```sh
git cliff --output CHANGELOG.md
```

Prepend new changes to an existing changelog file:

```sh
# 1- changelog header is removed from CHANGELOG.md
# 2- new entries are prepended to CHANGELOG.md without footer part
git cliff --unreleased --tag 1.0.0 --prepend CHANGELOG.md
```

Set/remove the changelog parts:

```sh
git cliff --body $template --strip footer
```

Also, see the [release script](./release.sh) of this project which sets the changelog as a message of an annotated tag.

## Docker

The easiest way of running **git-cliff** (in the git root directory with [configuration file](#configuration-file) present) is to use the [available tags](https://hub.docker.com/repository/docker/orhunp/git-cliff/tags) from [Docker Hub](https://hub.docker.com/repository/docker/orhunp/git-cliff):

```sh
docker run -t -v "$(pwd)/.git":/app/ orhunp/git-cliff:latest
```

Or you can use the image from the [GitHub Package Registry](https://github.com/orhun/git-cliff/packages/841947):

```sh
docker run -t -v "$(pwd)/.git":/app/ docker.pkg.github.com/orhun/git-cliff/git-cliff:latest
```

Also, you can build the image yourself using `docker build -t git-cliff .` command.

## GitHub Action

It is possible to generate changelogs using [GitHub Actions](https://docs.github.com/en/actions) via [git-cliff-action](https://github.com/orhun/git-cliff-action).

```yml
- name: Generate a changelog
  uses: orhun/git-cliff-action@v1
  with:
    config: cliff.toml
    args: --verbose
  env:
    OUTPUT: CHANGELOG.md
```

See the [repository](https://github.com/orhun/git-cliff-action) for other [examples](https://github.com/orhun/git-cliff-action#examples).

Also, see the [continuous deployment workflow](./.github/workflows/cd.yml) of this project which sets the release notes for GitHub releases using this action.

## GitLab CI/CD

It is possible to generate changelogs using [GitLab CI/CD](https://docs.gitlab.com/ee/ci/).

This minimal example creates artifacts that can be used on another job.

```yml
- changelog:
    image:
      name: orhunp/git-cliff:latest
      entrypoint: [""]
    variables:
      GIT_STRATEGY: clone # clone entire repo instead of reusing workspace
      GIT_DEPTH: 0 # avoid shallow clone to give cliff all the info it needs
    stage: doc
    script:
      - git-cliff -r . > CHANGELOG.md
    artifacts:
      paths:
        - CHANGELOG.md
```

Please note that the stage is `doc` and has to be changed accordingly to your need. 

## Configuration File

**git-cliff** configuration file supports [TOML](https://github.com/toml-lang/toml) (preferred) and [YAML](https://yaml.org) formats.

The configuration file is read from `$HOME/git-cliff/cliff.toml` if the file exists. This location depends on the platform, for example:

- on Linux: `/home/<user>/.config/git-cliff/cliff.toml`
- on Windows: `C:\Users\<user>\AppData\Roaming\git-cliff\cliff.toml`
- on macOS: `/Users/<user>/Library/Application Support/git-cliff/cliff.toml`

See [config/cliff.toml](./config/cliff.toml) for the default configuration values.

### changelog

This section contains the configuration options for changelog generation.

```toml
[changelog]
header = "Changelog"
body = """
{% for group, commits in commits | group_by(attribute="group") %}
    ### {{ group | upper_first }}
    {% for commit in commits %}
        - {{ commit.message | upper_first }}
    {% endfor %}
{% endfor %}
"""
trim = true
footer = "<!-- generated by git-cliff -->"
```

#### header

Header text that will be added to the beginning of the changelog.

#### body

Body template that represents a single release in the changelog.

See [templating](#templating) for more detail.

#### trim

If set to `true`, leading and trailing whitespaces are removed from the [body](#body).

It is useful for adding indentation to the template for readability, as shown [in the example](#changelog).

#### footer

Footer text that will be added to the end of the changelog.

### git

This section contains the parsing and git related configuration options.

```toml
[git]
conventional_commits = true
filter_unconventional = true
commit_parsers = [
    { message = "^feat", group = "Features"},
    { message = "^fix", group = "Bug Fixes"},
    { message = "^doc", group = "Documentation"},
    { message = "^perf", group = "Performance"},
    { message = "^refactor", group = "Refactor"},
    { message = "^style", group = "Styling"},
    { message = "^test", group = "Testing"},
]
filter_commits = false
tag_pattern = "v[0-9]*"
skip_tags = "v0.1.0-beta.1"
ignore_tags = ""
topo_order = false
sort_commits = "oldest"
```

#### conventional_commits

If set to `true`, commits are parsed according to the [Conventional Commits specifications](https://www.conventionalcommits.org).

> The Conventional Commits specification is a lightweight convention on top of commit messages. It provides an easy set of rules for creating an explicit commit history; which makes it easier to write automated tools on top of. This convention dovetails with SemVer, by describing the features, fixes, and breaking changes made in commit messages.

> The commit message should be structured as follows:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

e.g. `feat(parser): add ability to parse arrays`

#### filter_unconventional

If set to `true`, commits that are not conventional are excluded. This option can be used to generate changelogs with conventional and unconvential commits mixed together. For example:

```toml
conventional_commits = true
filter_unconventional = false
commit_parsers = [
  { message = ".*", group = "Other", default_scope = "other"},
]
```

With the configuration above, conventional commits are parsed as usual and unconventional commits will be also included in the changelog as "Other".

To completely exclude unconventional commits from the changelog:

```toml
# default behaviour
conventional_commits = true
filter_unconventional = true
```

To include any type of commit in the changelog without parsing:

```toml
conventional_commits = false
filter_unconventional = false
```

#### commit_parsers

An array of commit parsers for determining the commit groups by using regex.

Examples:

- `{ message = "^feat", group = "Features"}`
  - Group the commit as "Features" if the commit message (description) starts with "feat".
- `{ body = ".*security", group = "Security"}`
  - Group the commit as "Security" if the commit body contains "security".
- `{ message = ".*deprecated", body = ".*deprecated", group = "Deprecation"}`
  - Group the commit as "Deprecation" if the commit body and message contains "deprecated".
- `{ message = "^revert", skip = true}`
  - Skip processing the commit if the commit message (description) starts with "revert".
- `{ message = "^doc", group = "Documentation", default_scope = "other"},`
  - If the commit starts with "doc", group the commit as "Documentation" and set the default scope to "other". (e.g. `docs: xyz` will be processed as `docs(other): xyz`)

#### filter_commits

If set to `true`, commits that are not matched by [commit parsers](#commit_parsers) are filtered out.

#### tag_pattern

A glob pattern for matching the git tags.

e.g. It processes the same tags as the output of the following git command:

```sh
git tag --list 'v[0-9]*'
```

#### skip_tags

A regex for skip processing the matched tags.

#### ignore_tags

A regex for ignore processing the matched tags.

While `skip_tags` drop commits from the changelog, `ignore_tags` include ignored commits into the next tag.

#### topo_order

If set to `true`, tags are processed in topological order instead of chronological.

This can also be achieved by using the `--topo-order` command line flag.

#### sort_commits

Sort the commits inside sections by specified order.

Possible values:

- `oldest`
- `newest`

This can also be achieved by specifying the `--sort` command line argument.

## Templating

A template is a text where variables and expressions get replaced with values when it is rendered.

### Context

Context is the model that holds the required data for a template rendering. The [JSON](https://en.wikipedia.org/wiki/JSON) format is used in the following examples for the representation of a context.

#### Conventional Commits

> conventional_commits = **true**

For a [conventional commit](#conventional_commits) like this,

```
<type>[scope]: <description>

[body]

[footer(s)]
```

following context is generated to use for templating:

```json
{
  "version": "v0.1.0-rc.21",
  "commits": [
    {
      "id": "e795460c9bb7275294d1fa53a9d73258fb51eb10",
      "group": "<type> (overrided by commit_parsers)",
      "scope": "[scope]",
      "message": "<description>",
      "body": "[body]",
      "footers": ["[footer]", "[footer]"],
      "breaking_description": "<description>",
      "breaking": false,
      "conventional": true
    }
  ],
  "commit_id": "a440c6eb26404be4877b7e3ad592bfaa5d4eb210 (release commit)",
  "timestamp": 1625169301,
  "previous": {
    "version": "previous release"
  }
}
```

##### Breaking Changes

`breaking` flag is set to `true` when the commit has an exclamation mark after the commit type and scope, e.g.:

```
feat(scope)!: this is a breaking change
```

Or when the `BREAKING CHANGE:` footer is defined:

```
feat: add xyz

BREAKING CHANGE: this is a breaking change
```

`breaking_description` is set to the explanation of the breaking change. This description is expected to be present in the `BREAKING CHANGE` footer. However, if it's not provided, the `message` is expected to describe the breaking change.

#### Non-Conventional Commits

> conventional_commits = **false**

If [conventional_commits](#conventional_commits) is set to `false`, then some of the fields are omitted from the context or squashed into the `message` field:

```json
{
  "version": "v0.1.0-rc.21",
  "commits": [
    {
      "id": "e795460c9bb7275294d1fa53a9d73258fb51eb10",
      "group": "(overrided by commit_parsers)",
      "scope": "(overrided by commit_parsers)",
      "message": "(full commit message including description, footers, etc.)",
      "conventional": false,
    }
  ],
  "commit_id": "a440c6eb26404be4877b7e3ad592bfaa5d4eb210 (release commit)",
  "timestamp": 1625169301,
  "previous": {
    "version": "previous release"
  }
}
```

### Syntax

**git-cliff** uses [Tera](https://github.com/Keats/tera) as the template engine. It has a syntax based on [Jinja2](http://jinja.pocoo.org/) and [Django](https://docs.djangoproject.com/en/3.1/topics/templates/) templates.

There are 3 kinds of delimiters and those cannot be changed:

- `{{` and `}}` for expressions
- `{%` or `{%-` and `%}` or `-%}` for statements
- `{#` and `#}` for comments

See the [Tera Documentation](https://tera.netlify.app/docs/#templates) for more information about [control structures](https://tera.netlify.app/docs/#control-structures), [built-ins filters](https://tera.netlify.app/docs/#built-ins), etc.

Custom built-in filters that **git-cliff** uses:

- `upper_first`: Converts the first character of a string to uppercase.

### Examples

Examples are based on the following Git history:

```log
* df6aef4 (HEAD -> master) feat(cache): use cache while fetching pages
* a9d4050 feat(config): support multiple file formats
* 06412ac (tag: v1.0.1) chore(release): add release script
* e4fd3cf refactor(parser): expose string functions
* ad27b43 (tag: v1.0.0) docs(example)!: add tested usage example
* 9add0d4 fix(args): rename help argument due to conflict
* a140cef feat(parser): add ability to parse arrays
* 81fbc63 docs(project): add README.md
* a78bc36 Initial commit
```

See [examples](./examples/) directory for example configuration files.

If you have a custom configuration file that you are using for your project(s), consider sharing it with us by [submitting a pull request](./CONTRIBUTING.md)!

#### [Basic](./config/cliff.toml)

<details>
  <summary>Raw Output</summary>

```
# Changelog
All notable changes to this project will be documented in this file.

## [unreleased]

### Features

- Support multiple file formats
- Use cache while fetching pages

## [1.0.1] - 2021-07-18

### Miscellaneous Tasks

- Add release script

### Refactor

- Expose string functions

## [1.0.0] - 2021-07-18

### Bug Fixes

- Rename help argument due to conflict

### Documentation

- Add README.md
- Add tested usage example

### Features

- Add ability to parse arrays

<!-- generated by git-cliff -->
```

</details>

<details>
  <summary>Rendered Output</summary>

# Changelog
All notable changes to this project will be documented in this file.

## [unreleased]

### Features

- Support multiple file formats
- Use cache while fetching pages

## [1.0.1] - 2021-07-18

### Miscellaneous Tasks

- Add release script

### Refactor

- Expose string functions

## [1.0.0] - 2021-07-18

### Bug Fixes

- Rename help argument due to conflict

### Documentation

- Add README.md
- Add tested usage example

### Features

- Add ability to parse arrays

<!-- generated by git-cliff -->

</details>

#### [Detailed](./examples/detailed.toml)

<details>
  <summary>Raw Output</summary>

```
# Changelog
All notable changes to this project will be documented in this file.

## [unreleased]

### Features

- Support multiple file formats ([a9d4050](a9d4050212a18f6b3bd76e2e41fbb9045d268b80))
- Use cache while fetching pages ([df6aef4](df6aef41292f3ffe5887754232e6ea7831c50ba5))

## [1.0.1] - 2021-07-18

[ad27b43](ad27b43e8032671afb4809a1a3ecf12f45c60e0e)...[06412ac](06412ac1dd4071006c465dde6597a21d4367a158)

### Miscellaneous Tasks

- Add release script ([06412ac](06412ac1dd4071006c465dde6597a21d4367a158))

### Refactor

- Expose string functions ([e4fd3cf](e4fd3cf8e2e6f49c0b57f66416e886c37cbb3715))

## [1.0.0] - 2021-07-18

### Bug Fixes

- Rename help argument due to conflict ([9add0d4](9add0d4616dc95a6ea8b01d5e4d233876b6e5e00))

### Documentation

- Add README.md ([81fbc63](81fbc6365484abf0b4f4b05d384175763ad8db44))
- Add tested usage example ([ad27b43](ad27b43e8032671afb4809a1a3ecf12f45c60e0e))

### Features

- Add ability to parse arrays ([a140cef](a140cef0405e0bcbfb5de44ff59e091527d91b38))

<!-- generated by git-cliff -->
```

</details>

<details>
  <summary>Rendered Output</summary>

# Changelog
All notable changes to this project will be documented in this file.

## [unreleased]

### Features

- Support multiple file formats ([a9d4050](a9d4050212a18f6b3bd76e2e41fbb9045d268b80))
- Use cache while fetching pages ([df6aef4](df6aef41292f3ffe5887754232e6ea7831c50ba5))

## [1.0.1] - 2021-07-18

[ad27b43](ad27b43e8032671afb4809a1a3ecf12f45c60e0e)...[06412ac](06412ac1dd4071006c465dde6597a21d4367a158)

### Miscellaneous Tasks

- Add release script ([06412ac](06412ac1dd4071006c465dde6597a21d4367a158))

### Refactor

- Expose string functions ([e4fd3cf](e4fd3cf8e2e6f49c0b57f66416e886c37cbb3715))

## [1.0.0] - 2021-07-18

### Bug Fixes

- Rename help argument due to conflict ([9add0d4](9add0d4616dc95a6ea8b01d5e4d233876b6e5e00))

### Documentation

- Add README.md ([81fbc63](81fbc6365484abf0b4f4b05d384175763ad8db44))
- Add tested usage example ([ad27b43](ad27b43e8032671afb4809a1a3ecf12f45c60e0e))

### Features

- Add ability to parse arrays ([a140cef](a140cef0405e0bcbfb5de44ff59e091527d91b38))

<!-- generated by git-cliff -->

</details>

#### [Scoped](./examples/scoped.toml)

<details>
  <summary>Raw Output</summary>

```
# Changelog
All notable changes to this project will be documented in this file.

## [unreleased]

### Features

#### Cache

- Use cache while fetching pages

#### Config

- Support multiple file formats

## [1.0.1] - 2021-07-18

### Miscellaneous Tasks

#### Release

- Add release script

### Refactor

#### Parser

- Expose string functions

## [1.0.0] - 2021-07-18

### Bug Fixes

#### Args

- Rename help argument due to conflict

### Documentation

#### Example

- Add tested usage example

#### Project

- Add README.md

### Features

#### Parser

- Add ability to parse arrays

<!-- generated by git-cliff -->
```

</details>

<details>
  <summary>Rendered Output</summary>

# Changelog
All notable changes to this project will be documented in this file.

## [unreleased]

### Features

#### Cache

- Use cache while fetching pages

#### Config

- Support multiple file formats

## [1.0.1] - 2021-07-18

### Miscellaneous Tasks

#### Release

- Add release script

### Refactor

#### Parser

- Expose string functions

## [1.0.0] - 2021-07-18

### Bug Fixes

#### Args

- Rename help argument due to conflict

### Documentation

#### Example

- Add tested usage example

#### Project

- Add README.md

### Features

#### Parser

- Add ability to parse arrays

<!-- generated by git-cliff -->

</details>

#### [Scoped (Sorted)](./examples/scopesorted.toml)

<details>
  <summary>Raw Output</summary>

```
# Changelog
All notable changes to this project will be documented in this file.

## [unreleased]

### Features

- *(cache)* Use cache while fetching pages
- *(config)* Support multiple file formats

## [1.0.1] - 2021-07-18

### Miscellaneous Tasks

- *(release)* Add release script

### Refactor

- *(parser)* Expose string functions

## [1.0.0] - 2021-07-18

### Bug Fixes

- *(args)* Rename help argument due to conflict

### Documentation

- *(example)* Add tested usage example
  - **BREAKING**: add tested usage example
- *(project)* Add README.md

### Features

- *(parser)* Add ability to parse arrays

<!-- generated by git-cliff -->
```

</details>

<details>
  <summary>Rendered Output</summary>

# Changelog
All notable changes to this project will be documented in this file.

## [unreleased]

### Features

- *(cache)* Use cache while fetching pages
- *(config)* Support multiple file formats

## [1.0.1] - 2021-07-18

### Miscellaneous Tasks

- *(release)* Add release script

### Refactor

- *(parser)* Expose string functions

## [1.0.0] - 2021-07-18

### Bug Fixes

- *(args)* Rename help argument due to conflict

### Documentation

- *(example)* Add tested usage example
  - **BREAKING**: add tested usage example
- *(project)* Add README.md

### Features

- *(parser)* Add ability to parse arrays

<!-- generated by git-cliff -->

</details>

#### [Keep a Changelog](./examples/keepachangelog.toml)

<details>
  <summary>Raw Output</summary>

```
# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

### Added

- Support multiple file formats

### Changed

- Use cache while fetching pages

## [1.0.1] - 2021-07-18

### Added

- Add release script

### Changed

- Expose string functions

## [1.0.0] - 2021-07-18

### Added

- Add README.md
- Add ability to parse arrays
- Add tested usage example

### Fixed

- Rename help argument due to conflict

<!-- generated by git-cliff -->
```

</details>

<details>
  <summary>Rendered Output</summary>

# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

### Added

- Support multiple file formats

### Changed

- Use cache while fetching pages

## [1.0.1] - 2021-07-18

### Added

- Add release script

### Changed

- Expose string functions

## [1.0.0] - 2021-07-18

### Added

- Add README.md
- Add ability to parse arrays
- Add tested usage example

### Fixed

- Rename help argument due to conflict

<!-- generated by git-cliff -->

</details>

#### [Unconventional](./examples/unconventional.toml)

<details>
  <summary>Raw Output</summary>

```
# Changelog
All notable changes to this project will be documented in this file.

## [unreleased]

### Features

- Support multiple file formats ✔️
- Use cache while fetching pages ✔️

## [1.0.1] - 2021-07-18

### Miscellaneous Tasks

- Add release script ✔️

### Refactor

- Expose string functions ✔️

## [1.0.0] - 2021-07-18

### Bug Fixes

- Rename help argument due to conflict ✔️

### Documentation

- Add README.md ✔️
- Add tested usage example ✔️

### Features

- Add ability to parse arrays ✔️

### Other (unconventional)

- Initial commit ❌

<!-- generated by git-cliff -->
```

</details>

<details>
  <summary>Rendered Output</summary>

# Changelog
All notable changes to this project will be documented in this file.

## [unreleased]

### Features

- Support multiple file formats ✔️
- Use cache while fetching pages ✔️

## [1.0.1] - 2021-07-18

### Miscellaneous Tasks

- Add release script ✔️

### Refactor

- Expose string functions ✔️

## [1.0.0] - 2021-07-18

### Bug Fixes

- Rename help argument due to conflict ✔️

### Documentation

- Add README.md ✔️
- Add tested usage example ✔️

### Features

- Add ability to parse arrays ✔️

### Other (unconventional)

- Initial commit ❌

<!-- generated by git-cliff -->

</details>

## Similar Projects

- [git-journal](https://github.com/saschagrunert/git-journal) - The Git Commit Message and Changelog Generation Framework
- [clog-cli](https://github.com/clog-tool/clog-cli) - Generate beautiful changelogs from your Git commit history
- [relnotes](https://crates.io/crates/relnotes) - A tool to automatically generate release notes for your project.
- [cocogitto](https://github.com/oknozor/cocogitto) - A set of CLI tools for the conventional commit
and semver specifications.

## License

GNU General Public License ([v3.0](https://www.gnu.org/licenses/gpl.txt))

## Copyright

Copyright © 2021, [git-cliff contributors](mailto:git-cliff@protonmail.com)

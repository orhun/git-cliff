<p align="center">
    <a href="https://github.com/orhun/git-cliff">
        <img src="https://user-images.githubusercontent.com/24392180/121790699-8808dc80-cbea-11eb-8ab6-2fb6b08b66d8.png" width="300"></a>
    <br>
</p>

<center>

[![GitHub Release](https://img.shields.io/github/v/release/orhun/git-cliff?style=flat&labelColor=1C2C2E&color=C96329&logo=GitHub&logoColor=white)](https://github.com/orhun/git-cliff/releases)
[![Crate Release](https://img.shields.io/crates/v/git-cliff?style=flat&labelColor=1C2C2E&color=C96329&logo=Rust&logoColor=white)](https://crates.io/crates/git-cliff/)
[![Coverage](https://img.shields.io/codecov/c/gh/orhun/git-cliff?style=flat&labelColor=1C2C2E&color=C96329&logo=Codecov&logoColor=white)](https://codecov.io/gh/orhun/git-cliff)
<br>
[![Continuous Integration](https://img.shields.io/github/workflow/status/orhun/git-cliff/Continuous%20Integration?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=GitHub%20Actions&logoColor=BEC5C9)](https://github.com/orhun/git-cliff/actions?query=workflow%3A%22Continuous+Integration%22)
[![Continuous Deployment](https://img.shields.io/github/workflow/status/orhun/git-cliff/Continuous%20Deployment?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=GitHub%20Actions&logoColor=BEC5C9&label=deploy)](https://github.com/orhun/git-cliff/actions?query=workflow%3A%22Continuous+Deployment%22)
[![Docker Builds](https://img.shields.io/docker/cloud/build/orhunp/git-cliff?style=flat&labelColor=1C2C2E&color=BEC5C9&label=docker&logo=Docker&logoColor=BEC5C9)](https://hub.docker.com/r/orhunp/git-cliff)
[![Documentation](https://img.shields.io/docsrs/git-cliff-core?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=Rust&logoColor=BEC5C9E)](https://docs.rs/git-cliff-core/)

</center>

## About

**git-cliff** can generate [changelog](https://en.wikipedia.org/wiki/Changelog) files from the [Git](https://git-scm.com/) history by utilizing [conventional commits](#conventional_commits) as well as regex-powered [custom parsers](#commit_parsers). The [changelog template](#templating) can be customized with a [configuration file](#configuration-file) to match the desired format.

- [About](#about)
- [Installation](#installation)
  - [From crates.io](#from-cratesio)
  - [Binary Releases](#binary-releases)
- [Usage](#usage)
  - [Command Line Arguments](#command-line-arguments)
- [Configuration File](#configuration-file)
  - [changelog](#changelog)
    - [header](#header)
    - [body](#body)
    - [trim](#trim)
    - [footer](#footer)
  - [git](#git)
    - [conventional_commits](#conventional_commits)
    - [commit_parsers](#commit_parsers)
    - [filter_commits](#filter_commits)
    - [tag_pattern](#tag_pattern)
    - [skip_tags](#skip_tags)
- [Templating](#templating)
  - [Context](#context)
    - [Conventional Commits](#conventional-commits)
    - [Non-Conventional Commits](#non-conventional-commits)
  - [Syntax](#syntax)
- [Examples](#examples)
  - [Simple](#simple)
  - [Other](#other)
- [Docker](#docker)
- [GitHub Action](#github-action)
- [Similar Projects](#similar-projects)
- [License](#license)
- [Copyright](#copyright)

## Installation

### From crates.io

[git-cliff](crates.io/crates/git-cliff) can be installed from crates.io:

```sh
cargo install git-cliff
```

### Binary Releases

See the available binaries for different operating systems/architectures from the [releases page](https://github.com/orhun/git-cliff/releases).

## Usage

### Command Line Arguments

```
git-cliff [FLAGS] [OPTIONS] [RANGE]
```

**Flags:**

```
-v, --verbose       Increases the logging verbosity
-l, --latest        Processes the commits starting from the latest tag
-u, --unreleased    Processes the commits that do not belong to a tag
-h, --help          Prints help information
-V, --version       Prints version information
```

**Options:**

```
-c, --config <PATH>        Sets the configuration file [env: CONFIG=]  [default: cliff.toml]
-w, --workdir <PATH>       Sets the working directory [env: WORKDIR=]
-r, --repository <PATH>    Sets the repository to parse commits from [env: REPOSITORY=]
-p, --changelog <PATH>     Prepends entries to the given changelog file [env: CHANGELOG=]
-o, --output <PATH>        Writes output to the given file [env: OUTPUT=]
-t, --tag <TAG>            Sets the tag for the latest version [env: TAG=]
-b, --body <TEMPLATE>      Sets the template for the changelog body [env: TEMPLATE=]
-s, --strip <PART>         Strips the given parts from the changelog [possible values: header, footer, all]
```

**Args:**

```
<RANGE>    Sets the commit range to process
```

## Configuration File

**git-cliff** configuration file supports [TOML](https://github.com/toml-lang/toml) (preferred) and [YAML](https://yaml.org) formats.

See [cliff.toml](./cliff.toml) for an example.

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
commit_parsers = [
    { message = "^feat*", group = "Features"},
    { message = "^fix*", group = "Bug Fixes"},
    { message = "^doc*", group = "Documentation"},
    { message = "^perf*", group = "Performance"},
    { message = "^refactor*", group = "Refactor"},
    { message = "^style*", group = "Styling"},
    { message = "^test*", group = "Testing"},
]
filter_commits = false
tag_pattern = "v[0-9]*"
skip_tags = "v0.1.0-beta.1"
```

#### conventional_commits

If set to `true`, parses the commits according to the [Conventional Commits specifications](https://www.conventionalcommits.org).

> The Conventional Commits specification is a lightweight convention on top of commit messages. It provides an easy set of rules for creating an explicit commit history; which makes it easier to write automated tools on top of. This convention dovetails with SemVer, by describing the features, fixes, and breaking changes made in commit messages.

> The commit message should be structured as follows:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

e.g. `feat(parser): add ability to parse arrays`

#### commit_parsers

An array of commit parsers for determining the commit groups by using regex.

Examples:

- `{ message = "^feat*", group = "Features"}`
  - Group the commit as "Features" if the commit message (description) starts with "feat".
- `{ body = ".*security", group = "Security"}`
  - Group the commit as "Security" if the commit body contains "security".
- `{ message = ".*deprecated", body = ".*deprecated", group = "Deprecation"}`
  - Group the commit as "Deprecation" if the commit body and message contains "deprecated".
- `{ message = "^revert*", skip = true}`
  - Skip processing the commit if the commit message (description) starts with "revert".

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

[footer]
```

following context is generated to use for templating:

<!-- json cannot have comments but whatever -->
```json
// represents a release
{
  "version": "v0.1.0-rc.21",
  "commits": [
    {
      "id": "e795460c9bb7275294d1fa53a9d73258fb51eb10",
      "group": "<type>", // overrided by commit_parsers
      "scope": "[scope]",
      "message": "<description>",
      "body": "[body]",
      "footer": "[footer]",
      "breaking": false
    }
  ],
  "commit_id": "a440c6eb26404be4877b7e3ad592bfaa5d4eb210", // release commit
  "timestamp": 1625169301,
  "previous": {} // previous release
}
```

#### Non-Conventional Commits

> conventional_commits = **false**

If [conventional_commits](#conventional_commits) is set to `false`, then some of the fields are omitted from the context or squashed into the `message` field:

<!-- json cannot have comments but whatever -->
```json
// represents a release
{
  "version": "v0.1.0-rc.21",
  "commits": [
    {
      "id": "e795460c9bb7275294d1fa53a9d73258fb51eb10",
      "group": "", // overrided by commit_parsers
      "message": "", // whole commit message including description, footer, etc.
    }
  ],
  "commit_id": "a440c6eb26404be4877b7e3ad592bfaa5d4eb210", // release commit
  "timestamp": 1625169301,
  "previous": {} // previous release
}
```

### Syntax

**git-cliff** uses [Tera](https://github.com/Keats/tera) as a template engine which has a syntax based on [Jinja2](http://jinja.pocoo.org/) and [Django](https://docs.djangoproject.com/en/3.1/topics/templates/) templates.

There are 3 kinds of delimiters and those cannot be changed:

- `{{` and `}}` for expressions
- `{%` or `{%-` and `%}` or `-%}` for statements
- `{#` and `#}` for comments

See the [Tera Documentation](https://tera.netlify.app/docs/#templates) for more information about [control structures](https://tera.netlify.app/docs/#control-structures), [built-ins](https://tera.netlify.app/docs/#built-ins), etc.

## Examples

### Simple

TODO

### Other

- [Setting the release notes for GitHub releases using git-cliff-action](./.github/workflows/cd.yml)
- [Setting the changelog as a message of an annotated tag](./release.sh)

## Docker

The easiest way of running **git-cliff** (in the git root directory with [configuration file](#configuration-file) present) is to use the [available tags](https://hub.docker.com/repository/docker/orhunp/git-cliff/tags) from [Docker Hub](https://hub.docker.com/repository/docker/orhunp/git-cliff):

```sh
docker run -t -v "$(pwd)":/app/ orhunp/git-cliff:latest
```

Or you can use the image from the [GitHub Package Registry](https://github.com/orhun/git-cliff/packages/841947):

```sh
docker run -t -v "$(pwd)":/app/ docker.pkg.github.com/orhun/git-cliff/git-cliff:latest
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

## Similar Projects

- [clog-cli](https://github.com/clog-tool/clog-cli) - Generate beautiful changelogs from your Git commit history
- [relnotes](https://crates.io/crates/relnotes) - A tool to automatically generate release notes for your project.
- [cocogitto](https://github.com/oknozor/cocogitto) - A set of CLI tools for the conventional commit
and semver specifications.

## License

GNU General Public License ([v3.0](https://www.gnu.org/licenses/gpl.txt))

## Copyright

Copyright © 2021, [git-cliff contributors](mailto:git-cliff@protonmail.com)

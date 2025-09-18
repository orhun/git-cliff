---
sidebar_position: 1
---

# Getting Started

**git-cliff** can generate [changelog](https://en.wikipedia.org/wiki/Changelog) files from the [Git](https://git-scm.com/) history by utilizing [conventional commits](/docs/configuration/git#conventional_commits) as well as regex-powered [custom parsers](/docs/configuration/git#commit_parsers).

The [changelog template](category/templating) can be customized with a [configuration file](configuration) to match the desired format.

## Quickstart

1. Install **git-cliff**:

```bash
cargo install git-cliff
```

Also, see the other [installation options](installation).

2. Initialize **git-cliff**:

```bash
git-cliff --init
```

Edit the default [configuration](configuration) (`cliff.toml`) as you like. Check out the [examples](templating/examples) for different templates.

3. Generate a changelog:

```bash
git-cliff -o CHANGELOG.md
```

See the [command-line usage examples](usage/examples).

:::note
The `git cliff` command can also be used interchangeably with `git-cliff` (with a `-`) in most environments (when `git` is installed). However, when using the [NPM](/docs/installation/npm) installation method, the `git-cliff` command should be used.
:::

## FAQ

<details>
<summary>Commit and PR Strategy — How to write commits and manage PRs for changelog generation</summary>

### How should I write my commits?

We recommend using a [Git][1] history that follows the [Conventional Commits][2] specification as the primary strategy. For example:

```
fix(parser): handle empty commit messages gracefully
feat(cli): add support for --dry-run flag
refactor(core)!: change internal API to use async/await
```

**git-cliff**’s [default configuration][3] is built around this convention, making it easy to generate clear, structured, and consistent [Changelog][4]s by grouping commits (e.g., `feat`, `fix`, `docs`). The most important prefixes you should have in mind are:

* `fix:` which represents bug fixes, and correlates to a [SemVer][5] patch.
* `feat:` which represents a new feature, and correlates to a [SemVer][5] minor.
* `feat!:`,  or `fix!:`, `refactor!:`, etc., which represent a breaking change (indicated by the `!`) and will result in a [SemVer][5] major.

In addition to commit messages, **git-cliff** also supports parsing [remote metadata][6] from supported [Git][1] hosting services—such as pull request titles, numbers, and authors—using customizable regular expressions.

For example, [GitHub pull request labels can be used as grouping keys][7], allowing [Changelog][4]s to be organized based on custom [PR][8] label categories such as `breaking-change`, `type/enhancement`, or `area/documentation`.

### How should I manage PRs?

When working with a [PR][8]-based development flow, it’s important to adopt a merge strategy that preserves a clean and readable [Git][1] history—especially when [Changelog][4]s are generated from commit metadata.

We recommend using **squash merges** for integrating [PR][8]s into the main branch. This approach has several benefits:

* Linear history — [PR][8]s are merged as single commits, making the history easier to read and traverse.
* Easier bug tracking — Tools like **git bisect** become more effective with a linear history.
* Better compatibility with **git-cliff** — Since **git-cliff** generates [Changelog][4]s from commit messages, using **squash merges** helps ensure that each [PR][8] corresponds to a single, coherent commit. Other merge strategies, such as rebase merges or merge commits, may fail to consistently associate [PR][8]-level context (e.g., title, labels, issue references) with a single commit.

[1]: https://git-scm.com/
[2]: https://git-cliff.org/docs/configuration/git#conventional_commits
[3]: https://github.com/orhun/git-cliff/blob/main/config/cliff.toml
[4]: https://en.wikipedia.org/wiki/Changelog
[5]: https://semver.org/
[6]: https://git-cliff.org/docs/configuration/remote
[7]: https://git-cliff.org/docs/configuration/remote
[8]: https://en.wikipedia.org/wiki/Fork_and_pull_model

</details>

## Contribute

Contributions are highly appreciated! See the [contribution guidelines](https://github.com/orhun/git-cliff/blob/main/CONTRIBUTING.md) for getting started.

Feel free to submit issues and toss ideas!

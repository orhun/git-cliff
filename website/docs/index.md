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

## Contribute

Contributions are highly appreciated! See the [contribution guidelines](https://github.com/orhun/git-cliff/blob/main/CONTRIBUTING.md) for getting started.

Feel free to submit issues and toss ideas!

## FAQ

### How should I write my commits?

We recommend using a Git history that follows the [Conventional Commits][2] specification as the primary strategy. For example:

```
fix(parser): handle empty commit messages gracefully
feat(cli): add support for --dry-run flag
refactor(core)!: change internal API to use async/await
```

**git-cliff**’s [default configuration][3] is built around this convention, making it easy to generate clear, structured, and consistent changelogs by grouping commits (e.g., `feat`, `fix`, `docs`). The most important prefixes you should have in mind are:

- `fix:` which represents bug fixes, and correlates to a [SemVer][5] patch.
- `feat:` which represents a new feature, and correlates to a [SemVer][5] minor.
- `feat!:`, or `fix!:`, `refactor!:`, etc., which represent a breaking change (indicated by the `!`) and will result in a [SemVer][5] major.

In addition to commit messages, **git-cliff** also supports parsing [remote metadata][6] from supported Git hosting services — such as pull request titles, numbers, and authors — using customizable regular expressions.

For example, [GitHub pull request labels can be used as grouping keys][7], allowing changelogs to be organized based on custom PR label categories such as `breaking-change`, `type/enhancement`, or `area/documentation`.

### How should I manage PRs?

When working with a [PR-based development flow][8], it’s important to adopt a merge strategy that preserves a clean and readable Git history—especially when changelogs are generated from commit metadata.

We recommend using **squash merges** for integrating PRs into the main branch. This approach has several benefits:

- Linear history: PRs are merged as single commits, making the history easier to read and traverse.
- Easier bug tracking: Tools like `git bisect` become more effective with a linear history.
- Better compatibility with **git-cliff**: Since **git-cliff** generates changelogs from commit messages, using **squash merges** helps ensure that each PR corresponds to a single, coherent commit. Other merge strategies, such as rebase merges or merge commits, may fail to consistently associate PR-level context (e.g., title, labels, issue references) with a single commit.

[2]: https://git-cliff.org/docs/configuration/git#conventional_commits
[3]: https://github.com/orhun/git-cliff/blob/main/config/cliff.toml
[5]: https://semver.org/
[6]: https://git-cliff.org/docs/configuration/remote
[7]: http://localhost:3000/docs/tips-and-tricks#use-github-pr-labels-as-groups
[8]: https://en.wikipedia.org/wiki/Fork_and_pull_model

---
sidebar_position: 3
---

# Commit and PR Strategy

## How should I write my commits?

**git-cliff** can generate [changelog](https://en.wikipedia.org/wiki/Changelog) files from the [Git](https://git-scm.com/) history by utilizing [Conventional Commits](https://git-cliff.org/docs/configuration/git#conventional_commits) as well as regex-powered [custom parsers](https://git-cliff.org/docs/configuration/git#commit_parsers).

We recommend using a [Git](https://git-scm.com/) history that follows the [Conventional Commits](https://git-cliff.org/docs/configuration/git#conventional_commits) specification as the primary strategy.
This convention provides a widely adopted standard for categorizing and grouping commits in a meaningful and predictable way.

**git-cliff**’s [default configuration](https://github.com/orhun/git-cliff/blob/main/config/cliff.toml) is built around this convention, making it easy to generate clear, structured, and consistent [changelog](https://en.wikipedia.org/wiki/Changelog)s with minimal customization—by grouping commits (e.g., `feat`, `fix`, `docs`) according to industry best practices.
The most important prefixes you should have in mind are:

* `fix:` which represents bug fixes, and correlates to a [SemVer](https://semver.org/) patch.
* `feat:` which represents a new feature, and correlates to a SemVer minor.
* `feat!:`,  or `fix!:`, `refactor!:`, etc., which represent a breaking change (indicated by the `!`) and will result in a SemVer major.

In addition to commit messages, **git-cliff** also supports parsing [remote metadata](https://git-cliff.org/docs/configuration/remote) from supported [Git](https://git-scm.com/) hosting services—such as pull request titles, numbers, and authors—using customizable regular expressions.
This makes it possible to group commits by PR, author, or other metadata, offering greater flexibility in how [changelog](https://en.wikipedia.org/wiki/Changelog)s are structured.
For example, [GitHub pull request labels can be used as grouping keys](https://git-cliff.org/docs/tips-and-tricks#use-github-pr-labels-as-groups), allowing [changelog](https://en.wikipedia.org/wiki/Changelog)s to be organized based on custom categories such as `breaking-change`, `type/enhancement`, or `area/documentation`.

## How should I manage PRs?

When working with a [pull request (PR)](https://en.wikipedia.org/wiki/Fork_and_pull_model)-based development flow, it’s important to adopt a merge strategy that preserves a clean and readable [Git](https://git-scm.com/) history—especially when [changelog](https://en.wikipedia.org/wiki/Changelog)s are automatically generated from commit metadata.

We recommend using **squash merges** for integrating [PR](https://en.wikipedia.org/wiki/Fork_and_pull_model)s into the main branch. This approach has several benefits:
* Linear Git history — [PR](https://en.wikipedia.org/wiki/Fork_and_pull_model)s are merged as single commits, making the history easier to read and traverse.
* Simplified [changelog](https://en.wikipedia.org/wiki/Changelog) generation — Redundant or interim commits (e.g. `fix: typo`, `test: update`, `revert: feat: something`, etc.) within the [PR](https://en.wikipedia.org/wiki/Fork_and_pull_model) won’t pollute the [changelog](https://en.wikipedia.org/wiki/Changelog).
* Easier bug tracking — Tools like **git bisect** become more effective with a linear history.
* Better compatibility with **git-cliff** — Since **git-cliff** generates [changelog](https://en.wikipedia.org/wiki/Changelog)s from commit messages, using **squash merges** helps ensure that each [PR](https://en.wikipedia.org/wiki/Fork_and_pull_model) corresponds to a single, coherent commit. This improves the consistency of [changelog](https://en.wikipedia.org/wiki/Changelog) entries and allows for better association of [PR](https://en.wikipedia.org/wiki/Fork_and_pull_model) metadata such as titles, descriptions, and issue references. Other merge strategies, such as rebase merges or merge commits, may fail to consistently associate [PR](https://en.wikipedia.org/wiki/Fork_and_pull_model)-level context (e.g., title, description, issue references) with a single commit.
---
slug: 1.3.0
title: What's new in 1.3.0?
date: 2023-08-30T00:00:00.000Z
authors: orhun
tags:
  - release
---
<center>

 <a href="https://github.com/orhun/git-cliff">
    <img src="/img/git-cliff-banner.jpg" />
</a>

</center>

> [**git-cliff**](https://github.com/orhun/git-cliff) is a command-line tool (written in [Rust](https://www.rust-lang.org/)) that provides a highly customizable way to generate changelogs from git history. It supports using [custom regular expressions](/docs/configuration/git#commit_parsers) to alter changelogs which are mostly based on [conventional commits](/docs/configuration/git#conventional_commits). With a single [configuration file](/docs/configuration), a wide variety of formats can be applied for a changelog, thanks to the Jinja2/Django-inspired [template engine](/docs/category/templating). More information and examples can be found in the [GitHub repository](https://github.com/orhun/git-cliff).

## What's new?

The full changelog can be found [here](https://github.com/orhun/git-cliff/blob/main/CHANGELOG.md).

### Fancier changelog 🍬

The changelog of `git-cliff` is looking more fancy now!

For example:

```md
## [1.3.0-rc.1](https://github.com/orhun/git-cliff/compare/v1.2.0..v1.3.0-rc.1) - 2023-08-24

### ⛰️ Features

- _(changelog)_ [**breaking**] Add postprocessors ([#155](https://github.com/orhun/git-cliff/issues/155)) - ([5dc5fb7](https://github.com/orhun/git-cliff/commit/5dc5fb786db922322faacf928cc571a2d785cab2))

### 🐛 Bug Fixes

- _(cd)_ Do not publish release notes for pre-releases ([#249](https://github.com/orhun/git-cliff/issues/249)) - ([7a82aa1](https://github.com/orhun/git-cliff/commit/7a82aa1a769b2170ea7563d7df3c59da5a134201))
```

- The title now has links to the compare changes page on GitHub
- Each entry shows the issue/PR number and related commit
- Emojis!

Configuration: [https://github.com/orhun/git-cliff/blob/main/cliff.toml](https://github.com/orhun/git-cliff/blob/main/cliff.toml)

### Postprocessors ⚙️

Now you can post-process the changelog _after generation_:

> An array of commit postprocessors for manipulating the changelog before outputting. Can e.g. be used for replacing commit author with GitHub usernames.

For example:

```toml
[changelog]
postprocessors = [{ pattern = "foo", replace = "bar"}]
```

A practical example is present in the [default configuration](https://github.com/orhun/git-cliff/blob/main/cliff.toml):

```toml
[changelog]
# <REPO> will be replaced via postprocessors
body = """
## [{{ version }}](<REPO>/compare/{{ previous.version }}..{{ version }})
<!--trim-->
"""
# replace <REPO> with actual repository URL
postprocessors = [
  { pattern = '<REPO>', replace = "https://github.com/orhun/git-cliff" },
]

[git]
# replace issue numbers with <REPO>/issues/<num>
commit_preprocessors = [
  { pattern = '\((\w+\s)?#([0-9]+)\)', replace = "([#${2}](<REPO>/issues/${2}))" },
]
```

Imagine you created a tag (e.g. `0.2.0`) with the following commit:

```
feat: add xyz (#1)
```

In the changelog, it will turn into:

```md
## [0.2.0](https://github.com/orhun/git-cliff/compare/v0.1.0..v0.2.0)

### Features

- Add xyz ([#1](https://github.com/orhun/git-cliff/issues/1))
```

The way that it works is:

1. The numbers in commit messages are replaced with `<REPO>/issues/<num>` with the help of **git.preprocessors**.
2. The changelog is generated using **changelog.body** which has a couple of `<REPO>` usages.
3. `<REPO` is replaced with the original repository URL in the final changelog using **changelog.postprocessors**.

### PyPI Releases 🐍

`git-cliff` is now packaged for [PyPI](https://pypi.org/), the Python packaging index.

- [https://pypi.org/project/git-cliff](https://pypi.org/project/git-cliff)
- [https://test.pypi.org/project/git-cliff](https://test.pypi.org/project/git-cliff) (test package)

You can download it with `pip`:

```sh
pip install git-cliff
```

### Optional git2 🍦

If you are using `git-cliff` as a library, you can now get rid of [`git2`](https://crates.io/crates/git2) dependency by disabling the `repo` feature.

> `repo`: Enable parsing commits from a git repository. Enabled by default.
> You can turn this off if you already have the commits to put in the changelog and you don't need `git-cliff` to parse them.

Here is an example from [`release-plz`](https://github.com/MarcoIeni/release-plz):

```toml
[dependencies]
git-cliff-core = { version = "1.3.0", default-features = false }
```

### Cocogitto example 🐓

[`cocogitto`](https://github.com/cocogitto/cocogitto) is one other great release tool and conventional commits toolbox written in Rust.

With the newly added [`cocogitto.toml`](https://github.com/orhun/git-cliff/blob/main/examples/cocogitto.toml) example, you can generate changelogs similar to `cocogitto`'s changelog format.

For example:

```sh
git cliff -c examples/cocogitto.toml
```

Results in:

```md
# Changelog

All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

---

## [unreleased]

### Bug Fixes

- **(cd)** do not publish release notes for pre-releases (#249) - ([7a82aa1](https://github.com/cocogitto/cocogitto/commit/7a82aa1a769b2170ea7563d7df3c59da5a134201)) - Orhun Parmaksız
```

### Docker improvement 🐋

To avoid [CVE-2022-24765](https://github.blog/2022-04-12-git-security-vulnerability-announced/) (safe directory vulnerability), we were copying the project files into the container. After [#142](https://github.com/orhun/git-cliff/pull/142) is merged, this is no longer the case and the Docker container can be run as follows:

```diff
- docker run -t -v "$(pwd)/.git":/app/ "orhunp/git-cliff:${TAG:-latest}"
+ docker run -t -v "$(pwd)":/app/ "orhunp/git-cliff:${TAG:-latest}"
```

### RustLab 2023 📢

I'm happy to announce that I will be talking about `git-cliff` at [**RustLab 2023**](https://rustlab.it/)! 🎉

![rustlab2023](/img/rustlab2023.png)

<center>

**[https://rustlab.it/talks/turning-git-commits-into-changelog-with-git-cliff](https://rustlab.it/talks/turning-git-commits-into-changelog-with-git-cliff)**

</center>

> In this talk, I will be sharing the story behind git-cliff, implementation details with certain design choices, and most importantly how to work with Git objects using Rust. Also, I will be sharing examples of how to use git-cliff and integrate it with your project.

> Additionally, I will be giving tips on creating a successful command-line tool in Rust and publishing it as open source.

- **Dates**: November 19th -> November 21th
- **Location**: Florence, Italy
- **Tickets**: [https://rustlab.it/tickets](https://rustlab.it/tickets)

## Contributions

Any contribution is highly appreciated! There are [contribution guidelines](https://github.com/orhun/git-cliff/blob/main/CONTRIBUTING.md) for getting started.

Feel free to [submit issues](https://github.com/orhun/git-cliff/issues/new/choose) and join [Discord](https://discord.gg/W3mAwMDWH4) / [Matrix](https://matrix.to/#/#git-cliff:matrix.org)!

## Donate

If you liked `git-cliff` and/or my other projects [on GitHub](https://github.com/orhun), consider donating to support my open source endeavors.

💖 [https://donate.orhun.dev](https://donate.orhun.dev)

Have an awesome day! ⛰️

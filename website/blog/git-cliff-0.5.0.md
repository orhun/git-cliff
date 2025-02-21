---
slug: git-cliff-0.5.0
title: What's new in 0.5.0?
date: 2021-12-15T00:00:00.000Z
authors: orhun
tags:
  - release
---
In this post, I'm explaining the new features in the 0.5.0 release while giving insight into the different use-cases.

<center>

 <a href="https://github.com/orhun/git-cliff">
    <img src="/img/git-cliff-banner.jpg" />
</a>

</center>

> [**git-cliff**](https://github.com/orhun/git-cliff) is a command-line tool (written in [Rust](https://www.rust-lang.org/)) that provides a highly customizable way to generate changelogs from git history. It supports using [custom regular expressions](/docs/configuration/git#commit_parsers) to alter changelogs which are mostly based on [conventional commits](/docs/configuration/git#conventional_commits). With a single [configuration file](/docs/configuration), a wide variety of formats can be applied for a changelog, thanks to the Jinja2/Django-inspired [template engine](/docs/category/templating). More information and examples can be found in the [GitHub repository](https://github.com/orhun/git-cliff).

Today I released the new version (0.5.0) of `git-cliff`. There are a couple of major features that I believe are interesting and they can potentially help with different use cases. Must be exciting, let's have a look!

## What's new?

### `--topo-order`

`--topo-order : Sorts the tags topologically`

Imagine you are working on parallel code lines and you have the following git history:

```
* 0000025 (tag: v0.1.1, fix_v1) fix: patch on v0.1.x
| * 0000050 (HEAD -> master) feat: fifth commit on master
| * 0000040 (tag: v0.2.0) feat: fourth commit on master
| * 0000030 chore: third commit on master
|/
* 0000020 (tag: v0.1.0) fix: second commit on master
* 0000010 feat: first commit on master
```

In this scenario, we can pretend that after your fifth commit on `master` you had to fix something about `v0.1.0` and check out a new branch (`fix_v1`). After that, you committed a patch and created a new tag. (`v0.1.1`)

Now let's say you decided to generate a changelog for the unreleased commits. Since previous versions of `git-cliff` sort the tags chronologically as default, you would get something like this:

```bash
$ git cliff --unreleased

# Changelog
## [unreleased]
### Features
- Fifth commit on master

## [0.2.0] - 2021-10-22
### Features
- Fourth commit on master

### Miscellaneous Tasks
- Third commit on master
```

This is because `--unreleased` flag implicitly uses a commit range such as `0000025..HEAD`, since it sorts the tags chronologically, as previously stated. This situation can now be prevented by using the `--topo-order` flag, which disables the automatic sorting and processes the tags as they appear in the git history:

```bash
$ git cliff --topo-order --unreleased

# Changelog
## [unreleased]
### Features
- Fifth commit on master
```

Now, the correct range of commits (`0000040..HEAD`) is processed. In other words, `v0.2.0` is accepted as the latest tag in the `master` branch.

Tracking issue: [#29](https://github.com/orhun/git-cliff/issues/29)

### `--include-path` && `--exclude-path`

`--include-path <PATTERN>... : Sets the path to include related commits`
`--exclude-path <PATTERN>... : Sets the path to exclude related commits`

Let's say you have a [monorepo](https://en.wikipedia.org/wiki/Monorepo) and you want to generate a changelog that includes or excludes some commits that concern certain files/directories in the working directory. To explain it further, let's think that you have the following directory structure:

```
Cargo.toml
apps/
└── application-related files
libs/
└── library files
other/
└── miscellaneous files
```

In the previous versions of `git-cliff`, it was possible to only include commits in the changelog if the changes are against a path under e.g. "apps". This could be done by using the `--commit-path` argument. But now, functionality is extended much further and you can specify multiple paths, use glob patterns and even exclude files/directories by using the brand new `--include-path` and `--exclude-path` arguments.

```bash
# include commits related to any TOML file and also application directory
$ git cliff --include-path "**/*.toml" --include-path "apps/*"

# exclude commits that are related to miscellaneous files
$ git cliff --exclude-path "other/*"
```

With this change, `--commit-path` argument is <s>removed</s> replaced with `--include-path` which supports glob patterns.

Tracking issue: [#34](https://github.com/orhun/git-cliff/issues/34)

### `--current`

`--current: Processes the commits that belong to the current tag`

Let's suppose this situation:

1. First, `v0.10.0` is released.
2. After that, `v0.11.0` is released.
3. After that, a bug was found and `v0.10.1` is released.

The problem <s>is</s> was, when you check out to a tag and try to generate a changelog for the _latest_ tag, it always points out to the most recent tag (which is `v0.10.1` in our example). So how do you generate changelog for the currently checked out tag? Well, simple:

```bash
$ git checkout v0.11.0

# changelog is generated for v0.11.0
$ git cliff --current

# changelog is generated for v0.10.1
$ git cliff --latest
```

`--current` flag behaves the same as running the following git command:

```bash
$ git describe --tags $(git rev-parse HEAD)
```

So it is expected to always use the current tag if it exists.

Tracking issue: [#37](https://github.com/orhun/git-cliff/issues/37)

### `ignore_tags`

A new configuration file entry makes an appearance in the `[git]` section!

```toml
[git]
ignore_tags = "v.*-beta.*"
```

The simplest explanation would be: while [`skip_tags`](/docs/configuration/git#skip_tags) drop commits from the changelog, `ignore_tags` include ignored commits into the next tag.

So for example if you have the following git history:

```
* 2d77bc3 (HEAD -> master, tag: v0.2.0) feat: add feature 3
* 072eba9 (tag: v0.2.0-beta.1) feat: add feature 2
* 656564c (tag: v0.1.0) feat: fix feature 1
* 9d98e6b feat: add feature 1
* 6807c82 (tag: v0.1.0-beta.1) feat: add skip feature
* 93d62ef Initial commit
```

Setting `ignore_tags = "v.*-beta.*"` in the configuration will result in a changelog like this:

```
# Changelog
## [0.2.0] - 2021-01-23
### Features
- Add feature 2
- Add feature 3

## [0.1.0] - 2021-01-23
### Features
- Add feature 1
- Fix feature 1
```

Thanks to [@kenji-miyake](https://github.com/kenji-miyake) for reporting/implementing this!

Tracking issue: [#36](https://github.com/orhun/git-cliff/issues/36)

### Sorting configuration

Following values can be specified in the configuration file now (in `[git]` section):

- [`topo_order`](/docs/configuration/git#topo_order)
- [`sort_commits`](/docs/configuration/git#sort_commits)

For example:

```toml
[git]
topo_order = false
sort_commits = "newest"
```

Tracking issue: [#31](https://github.com/orhun/git-cliff/issues/31)

### `filter_unconventional`

With this new configuration file entry, it is now possible to have both conventional and unconventional commits in the changelog.

```toml
[git]
# parse conventional commits
conventional_commits = true

# do not skip unconventional commits
filter_unconventional = false

# instead, override the group and scope of unconventional commits as "Other".
commit_parsers = [
  { message = ".*", group = "Other", default_scope = "other"},
]
```

Other use cases of [`filter_unconventional`](/docs/configuration/git#filter_unconventional) are the following:

```toml
# allow only conventional commits (default)
[git]
conventional_commits = true
filter_unconventional = true

# allow any type of commit in the changelog without parsing
[git]
conventional_commits = false
filter_unconventional = false
```

There is also a new field called `conventional` added to the [template context](/docs/templating/context). It can be used like this in the templates:

```
{% if commit.conventional %} ✅ {% else %} ❌ {% endif %}
```

### `--with-commit`

`--with-commit <MSG>... : Sets custom commit messages to include in the changelog`

In some cases, you might want to include commit messages in the changelog that yet don't exist. One example would be having "the commit message that updates the changelog" in the changelog. (🤔)

```bash
git cliff -o CHANGELOG.md
git add CHANGELOG.md
git commit -m "chore(release): update CHANGELOG.md for 1.0.0"
```

In the example above, CHANGELOG.md will not have the latest commit message since the commit is created afterward. So if you want to include custom commit messages like that in the changelog, you can use the `--with-commit` argument as follows:

```bash
# define the commit message
commit_msg="chore(release): update CHANGELOG.md for 1.0.0"

# generate changelog and pretend a commit exists as "$commit_msg"
git cliff --with-commit "$commit_msg" -o CHANGELOG.md

# create the actual commit
git add CHANGELOG.md
git commit -m "$commit_msg"
```

### Better error messages

`git-cliff` now outputs more explanatory error messages about templates, instead of just saying "Failed to parse template":

```bash
$ git cliff

 ERROR git_cliff > Template parse error:
 --> 3:12
  |
3 | {% endif %}{% forx group, commits in commits | group_by(attribute="group") %}␊
  |            ^---
  |
  = expected end of input or some content
```

## Looking forward

For the future versions of `git-cliff`,

- I'm planning to improve performance by parallelizing the _computations_ using [rayon](https://github.com/rayon-rs/rayon). There is not a tracking issue yet, but feel free to create one and share your thoughts!
- Benchmarking would be cool to reveal the performance as well as performance-related issues.
- [Why not add a manpage?](https://github.com/orhun/git-cliff/issues/35)

### Contributions

Any contribution is highly appreciated! There are [contribution guidelines](https://github.com/orhun/git-cliff/blob/main/CONTRIBUTING.md) for getting started. Feel free to submit issues and throw ideas! 🧠

### Support

If you liked `git-cliff` and/or my other projects [on GitHub](https://github.com/orhun/), consider supporting me on [GitHub Sponsors](https://github.com/sponsors/orhun) or [Patreon](https://www.patreon.com/orhunp) ☕

Have a wonderful day! ⛰️

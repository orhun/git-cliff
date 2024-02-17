---
slug: 1.4.0
title: What's new in 1.4.0?
date: 2023-10-29T00:00:00.000Z
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

### Bump version 🆙

`git-cliff` can calculate the next version based on conventional commits and bump the version for the unreleased changes for you!

```
--bump: Bumps the version for unreleased changes
```

For example, if you have `1.0.0` and committed "feat: xyz", `git-cliff --bump --unreleased` will create a changelog for `1.1.0`.

How it works is that for a semantic versioning such as `<MAJOR>.<MINOR>.<PATCH>`:

- "fix:" -> increments `PATCH`
- "feat:" -> increments `MINOR`
- "scope!" (breaking changes) -> increments `MAJOR`

### Better grouping 👥

Now you can group commits by their attributes, i.e. name of the author, email, etc.

For example, to group the commits that belong to `Dependabot` under "Dependency Updates" in the changelog:

```toml
[git]
# regex for parsing and grouping commits
commit_parsers = [
  { field = "author.name", pattern = "dependabot\\[bot\\]", group = "Dependency Updates"},
]
```

This will result in:

```md
### Dependency Updates

- _(deps)_ Bump regex from 1.9.6 to 1.10.0
- _(deps)_ Bump toml from 0.8.1 to 0.8.2
- _(deps)_ Bump regex from 1.9.5 to 1.9.6
```

The supported commit attributes (`field`s) are:

- `id`
- `message`
- `body`
- `author.name`
- `author.email`
- `committer.email`
- `committer.name`

### Glob -> Regex 🧶

`[git].tag_pattern` was only supporting glob patterns for matching (mostly due to the underlying support of such glob by git2), now it directly supports regular expressions:

```diff
[git]
- # glob pattern for matching git tags
+ # regex for matching git tags
- tag_pattern = "v[0-9]*"
+ tag_pattern = "v[0-9].*"
```

### Auto-fix typos ✍️

Here is a `git-cliff` configuration for automatically fixing the typos in the commit messages before they appear in the changelog:

```toml
# regex for preprocessing the commit messages
commit_preprocessors = [
  # Check spelling of the commit with https://github.com/crate-ci/typos
  # If the spelling is incorrect, it will be automatically fixed.
  { pattern = '.*', replace_command = 'typos --write-changes -' },
]
```

This configuration was added to the `git-cliff`'s [repository config](https://github.com/orhun/git-cliff/blob/main/cliff.toml) (not default) in [#316](https://github.com/orhun/git-cliff/pull/316) and runs [`typos`](https://github.com/crate-ci/typos/) for each commit. There is also a [good first issue](https://github.com/orhun/git-cliff/issues/333) to improve this.

### Emacs support 😈

Check out [`git-cliff.el`](https://github.com/liuyinz/git-cliff.el) to generate, update and release changelog in Emacs.

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

Have a fantastic day! ⛰️

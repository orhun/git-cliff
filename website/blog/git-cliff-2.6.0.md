---
slug: 2.6.0
title: "What's new in 2.6.0? \U0001F195"
date: 2024-09-22T00:00:00.000Z
authors: orhun
tags:
  - release
---

<center>

  <a href="https://github.com/orhun/git-cliff">
    <img src="/img/git-cliff-anim.gif" />
  </a>

</center>

> [**git-cliff**](https://github.com/orhun/git-cliff) is a command-line tool (written in [Rust](https://www.rust-lang.org/)) that provides a highly customizable way to generate changelogs from git history.
>
> It supports using [custom regular expressions](/docs/configuration/git#commit_parsers) to alter changelogs which are mostly based on [conventional commits](/docs/configuration/git#conventional_commits). With a single [configuration file](/docs/configuration), a wide variety of formats can be applied for a changelog, thanks to the Jinja2/Django-inspired [template engine](/docs/category/templating).
>
> More information and examples can be found in the [GitHub repository](https://github.com/orhun/git-cliff).

## What's new? ⛰️

The full changelog can be found [here](https://github.com/orhun/git-cliff/blob/main/CHANGELOG.md).

---

### 🛠️ Deprecated integration fields

The following fields are deprecated and will be removed in the next releases:

- `commit.github`, `commit.gitea`, `commit.gitlab`, `commit.bitbucket`

You can now use the `commit.remote` field instead. For example:

```diff
-{% if commit.github.username %}
+{% if commit.remote.username %}
```

---

### 🌲 Better branch support

If you have diverged branches for your project and want to changelog for each branch, you can now use the `--use-branch-tags` option.

```bash
$ git cliff --use-branch-tags
```

The generated changelog above will only include the tags from the current branch.

Also, you can use it from the configuration file:

```toml
[git]
use_branch_tags  = true
```

:::info

See the [implementation](https://github.com/orhun/git-cliff/pull/772) for more explanation and the coolest hand-drawn diagram ever!

:::

---

### ♾️ Render always

Do you want to always render the changelog even if there are no changes? Boom, now you can now use the `render_always` option:

```toml
[changelog]
render_always = true
```

---

### 📤 Output from configuration

This is pretty self-explanatory:

```toml
[changelog]
output = "CHANGELOG.md"
```

This option does not take precedence over command-line arguments which means you can override it with the `--output` option.

---

### 📦 Improve Typescript API

We added the missing options and documented all options with tsdoc comments.

Also, we improved the `skipCommit` option to accept an array of values.

:::info

See the [implementation](https://github.com/orhun/git-cliff/pull/843) for more information.

:::

---

### ✂️ Trim commit messages

We now remove the trailing newline for commits, which means you can use `$` anchor in your regular expressions:

```toml
[git]
commit_preprocessors = [
  # remove the issue number at the end of the commit message (e.g. #123)
  { pattern = ' #\d+$', replace = ""}
]
```

---

### 🌟 Better example templates

The example templates are now more intuitive and conventionally correct. We removed the non-beginner-friendly options and changed the defaults to be easier to start with. Weheee!

---

### 🧰 Other

- _(template)_ [**breaking/core**] Add name parameter to the constructor - ([e577113](https://github.com/orhun/git-cliff/commit/e577113bd69147936e391976c8b06cba76764eec))
- _(bump)_ Suppress template warning when `--bumped-version` is used ([#855](https://github.com/orhun/git-cliff/issues/855)) - ([8bebbf9](https://github.com/orhun/git-cliff/commit/8bebbf9f575e6e3f1bc50332e5703fde9dd1b55f))
- _(changelog)_ Do not change the tag date if tag already exists ([#861](https://github.com/orhun/git-cliff/issues/861)) - ([fbb643b](https://github.com/orhun/git-cliff/commit/fbb643b2e1096ac74a6ea9e9881ed4fd8161d3be))
- _(changelog)_ Correctly set the tag message for the latest release ([#854](https://github.com/orhun/git-cliff/issues/854)) - ([e41e8dd](https://github.com/orhun/git-cliff/commit/e41e8dd4a2e5ed12149078492cf6fd6eedebd0fa))
- _(changelog)_ Don't change the context when provided via `--from-context` ([#820](https://github.com/orhun/git-cliff/issues/820)) - ([ff72406](https://github.com/orhun/git-cliff/commit/ff7240633fcb46e6190dfed22150cbf8d3012df5))

---

## Contributions 👥

- @nejcgalof made their first contribution in [#853](https://github.com/orhun/git-cliff/pull/853)
- @pplmx made their first contribution in [#824](https://github.com/orhun/git-cliff/pull/824)

Any contribution is highly appreciated! See the [contribution guidelines](https://github.com/orhun/git-cliff/blob/main/CONTRIBUTING.md) for getting started.

Feel free to [submit issues](https://github.com/orhun/git-cliff/issues/new/choose) and join our [Discord](https://discord.gg/W3mAwMDWH4) / [Matrix](https://matrix.to/#/#git-cliff:matrix.org) for discussion!

Follow `git-cliff` on [Twitter](https://twitter.com/git_cliff) & [Mastodon](https://fosstodon.org/@git_cliff) to not miss any news!

## Support 🌟

If you liked `git-cliff` and/or my other projects [on GitHub](https://github.com/orhun), consider [donating](https://donate.orhun.dev) to support my open source endeavors.

- 💖 GitHub Sponsors: [@orhun](https://github.com/sponsors/orhun)
- ☕ Buy Me A Coffee: [https://www.buymeacoffee.com/orhun](https://www.buymeacoffee.com/orhun)

Have a fantastic day! ⛰️

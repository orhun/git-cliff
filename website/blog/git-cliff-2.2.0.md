---
slug: 2.2.0
title: What's new in 2.2.0?
date: 2024-03-30T00:00:00.000Z
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

### 🎈 Configurable Bump Rules

If you are a frequent user of `--bump`/`--bumped-version` flags then this new feature is for you!

`git-cliff` now supports customizing the behavior of version bumps.

Add the following section to your `cliff.toml` for configuration:

```toml
[bump]
# Configures automatic minor version increments for feature changes.
#
# When true, a feature will always trigger a minor version update.
#
# When false, a feature will trigger:
# - a patch version update if the major version is 0.
# - a minor version update otherwise.
features_always_bump_minor = true

# Configures 0 -> 1 major version increments for breaking changes.
#
# When true, a breaking change commit will always trigger a major version update
# (including the transition from version 0 to 1)
#
# When false, a breaking change commit will trigger:
# - a minor version update if the major version is 0.
# - a major version update otherwise.
breaking_always_bump_major = true
```

---

### 🛠️ Better Template Errors

Template rendering errors are now more verbose!

For example, let's throw an error in the template with using [throw](https://keats.github.io/tera/docs/#throw) function:

```toml
[changelog]
body = """
{{ throw(message="something happened!") }}
"""
```

When you run `git-cliff`:

```
 ERROR git_cliff > Template render error:
Function call 'throw' failed
something happened!
```

---

### 🤖 Auto Detecting Config

If you configured `git-cliff` from `Cargo.toml` via metadata table (`[workspace.metadata.git-cliff.changelog]`), running `git cliff` is now simply enough!

```sh
$ git cliff

# is same as
$ git cliff --config Cargo.toml
```

We also updated the config detection mechanism to support the following cases:

| `cliff.toml` | project manifest (e.g. `Cargo.toml`) | use config from: |
| ------------ | ------------------------------------ | ---------------- |
| ✅           | ✅                                   | `cliff.toml`     |
| ✅           | ❌                                   | `cliff.toml`     |
| ❌           | ✅                                   | `Cargo.toml`     |
| ❌           | ❌                                   | builtin          |

See [Rust](https://git-cliff.org/docs/integration/rust) & [Python](https://git-cliff.org/docs/integration/python) integration for more information.

---

### 🚦 Commit Processing Order

The order of commit processing is changed from:

1. Split commits
2. Process commits

To:

1. Process commits
2. Split commits (and process the split commits)

This makes it possible to e.g. [preprocess](https://git-cliff.org/docs/configuration/git#commit_preprocessors) commits, [split them](https://git-cliff.org/docs/configuration/git#split_commits) by newline and then process each line as conventional commit.

See [#555](https://github.com/orhun/git-cliff/issues/555) for an example.

---

### ✂️ Trim Text

We changed the commit parser behavior to always trim the text (commit message, body, etc.) before matching it with a regex.

This means that you will be able to use `$` in the regex for matching until the end.

For example:

```toml
[git]
commit_parsers = [
  { message = '^(fix|feat|setup|doc|refactor|test|optimization)\([A-Za-z0-9_-]+?\))+(:\ .*)$', group = "test"},
]
```

---

### 🚀 Quick Installation in CI

You can now install `git-cliff` in your GitHub Actions CI easily with [`taiki-e/install-action`](https://github.com/taiki-e/install-action)!

```yml
- name: Check out repository
  uses: actions/checkout@v3
  with:
    fetch-depth: 0

- name: Install git-cliff
  uses: taiki-e/install-action@git-cliff

- name: Generate changelog
  run: git-cliff
```

---

### 🧰 Other

- _(changelog)_ Return the last version if there is nothing to bump - ([45c87f2](https://github.com/orhun/git-cliff/commit/45c87f2f307e8441c128b81835b662362e6b380a))
- _(command)_ Add missing environment variables for Windows ([#532](https://github.com/orhun/git-cliff/issues/532)) - ([9722784](https://github.com/orhun/git-cliff/commit/972278439613d6187699fec02db8e1c4826ec92b))
- _(npm)_ Publish rc version for prereleases ([#528](https://github.com/orhun/git-cliff/issues/528)) - ([16bea51](https://github.com/orhun/git-cliff/commit/16bea5179a89af26dd0bfb07c7d6b7d1efa3c54e))
- _(pypi)_ Update maturin version ([#539](https://github.com/orhun/git-cliff/issues/539)) - ([10b7ab8](https://github.com/orhun/git-cliff/commit/10b7ab829f30beba19d13437ebafc35b9bb38476))

---

## Contributions 👥

Any contribution is highly appreciated! See the [contribution guidelines](https://github.com/orhun/git-cliff/blob/main/CONTRIBUTING.md) for getting started.

Feel free to [submit issues](https://github.com/orhun/git-cliff/issues/new/choose) and join our [Discord](https://discord.gg/W3mAwMDWH4) / [Matrix](https://matrix.to/#/#git-cliff:matrix.org) for discussion!

Follow `git-cliff` on [Twitter](https://twitter.com/git_cliff) & [Mastodon](https://fosstodon.org/@git_cliff) to not miss any news!

## Support 🌟

If you liked `git-cliff` and/or my other projects [on GitHub](https://github.com/orhun), consider [donating](https://donate.orhun.dev) to support my open source endeavors.

- 💖 GitHub Sponsors: [@orhun](https://github.com/sponsors/orhun)
- ☕ Buy Me A Coffee: [https://www.buymeacoffee.com/orhun](https://www.buymeacoffee.com/orhun)

Have a fantastic day! ⛰️

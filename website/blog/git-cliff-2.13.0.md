---
slug: 2.13.0
title: "What's new in 2.13.0? \U0001F195"
date: 2026-04-26T00:00:00.000Z
authors: orhun
tags:
  - release
---

<center>

  <a href="https://github.com/orhun/git-cliff">
    <img src="/img/git-cliff-anim.gif" />
  </a>

</center>

> [**git-cliff**](https://github.com/orhun/git-cliff) is a command-line tool that provides a highly customizable way to generate changelogs from the Git history.

---

## What's new? ⛰️

The full changelog can be found [here](https://github.com/orhun/git-cliff/blob/main/CHANGELOG.md).

---

### 🔢 Configurable Processing Order

**git-cliff** now supports defining your own pipeline of commit processing steps via [`processing_order`](/docs/configuration/git#processing_order) configuration option!

```toml
[git]
processing_order = [
  "commit_preprocessors",
  "split_commits",
  "conventional_commits",
  "commit_parsers",
  "link_parsers",
]
```

The available processing steps are:

- [`commit_preprocessors`](/docs/configuration/git#commit_preprocessors)
- [`split_commits`](/docs/configuration/git#split_commits)
- [`conventional_commits`](/docs/configuration/git#conventional_commits)
- [`commit_parsers`](/docs/configuration/git#commit_parsers)
- [`link_parsers`](/docs/configuration/git#link_parsers)

:::info

This is useful for advanced users who want to have more control over the commit processing pipeline, for example, to run custom preprocessors before the conventional commit parsing step.

:::

---

### 🌀 Migrate Logging to Tracing

We now use the [tracing](https://crates.io/crates/tracing) crate for logging in **git-cliff**!

Before:

<img src="https://raw.githubusercontent.com/orhun/git-cliff/main/website/static/img/logs-before.gif"/>

After:

<img src="https://raw.githubusercontent.com/orhun/git-cliff/main/website/static/img/logs-after.gif"/>

Please let us know if you encounter any bugs or UX issues!

---

### ⚙️ Alternative Config Locations

**git-cliff** now supports more configuration file locations!

- `cliff.toml`
- `.cliff.toml`
- `.config/cliff.toml`
- `$HOME/cliff.toml`
- `$HOME/.cliff.toml`
- `$HOME/.config/cliff.toml`

---

### 📊 Per-Commit Statistics

You can now get per-commit statistics in the release context:

```jinja2
{% for commit in commits %}
  - {{ commit.message }} ({{ commit.statistics.files_changed }} files, +{{ commit.statistics.additions }}, -{{ commit.statistics.deletions }})
{% endfor %}
```

Results in:

```md
- Fix a bug (3 files, +10, -2)
```

The available statistics are:

- `{{ commit.statistics.files_changed }}`: Number of files changed in the commit
- `{{ commit.statistics.additions }}`: Number of lines added in the commit
- `{{ commit.statistics.deletions }}`: Number of lines deleted in the commit

---

### 🏷️ Bump Type in Context

The determined bump type is now available in the release context as `{{ bump_type }}`.
This can be used to conditionally render content based on the bump type, for example:

```jinja2
{% if bump_type == "major" %}
  - This is a major release!
{% endif %}
```

The available bump types are `major`, `minor` and `patch`.

---

### 📡 Environment Variable for Offline

You can now also set the `GIT_CLIFF_OFFLINE` environment variable to execute in offline mode:

```sh
$ GIT_CLIFF_OFFLINE=true git-cliff
```

Is the same as:

```toml
[remote]
offline = false
```

Or passing the `--offline` flag.

---

### 🐋 Docker Tag Updates

There were some updates to the Docker tags pushed from the CI:

- `latest`: only on version tag builds
- `main`: only on pushes to the `main` branch
- `sha-<short>`: commit SHA builds (e.g. `sha-954106f`)
- `X.Y.Z`: SemVer tag derived from Git tag (e.g. `2.13.0`)

e.g. to pull the latest stable version, you can now use:

```sh
$ docker pull orhun/git-cliff:latest
```

---

### 🧰 Other

- _(lib)_ Raise MSRV to 1.87.0 ([#1479](https://github.com/orhun/git-cliff/issues/1479)) - ([9b38cb4](https://github.com/orhun/git-cliff/commit/9b38cb451e799590d43ef86d0b57917dd2cb256c))
- _(args)_ Correctly parse multiple env values for include/exclude paths ([#1450](https://github.com/orhun/git-cliff/issues/1450)) - ([f1874b8](https://github.com/orhun/git-cliff/commit/f1874b85362cec70f346f812109e72e754e323ca))
- _(cli)_ Warn when `--with-commit` does not change version ([#1484](https://github.com/orhun/git-cliff/issues/1484)) - ([3d6a7cb](https://github.com/orhun/git-cliff/commit/3d6a7cbdbbc922dea9d780ec0320de926341d7b9))
- _(remote)_ Deserialize GitLab API data models safely ([#1368](https://github.com/orhun/git-cliff/issues/1368)) - ([954106f](https://github.com/orhun/git-cliff/commit/954106f3a7d8a6ddea5a51e304d449d4fa728614))
- _(docker)_ Install ca-certificates in docker image ([#1425](https://github.com/orhun/git-cliff/issues/1425)) - ([1732b9a](https://github.com/orhun/git-cliff/commit/1732b9a5d41029daa6577a1374c4eeb1fb714e40))
- _(cd)_ Publish musl wheels to PyPI by matching matrix.build.NAME ([#1490](https://github.com/orhun/git-cliff/issues/1490)) - ([9b5e732](https://github.com/orhun/git-cliff/commit/9b5e73232d164294d701003ed1f6b690fa6f4bc7))

---

## New Contributors ❤️

- @truffle-dev made their first contribution in [#1490](https://github.com/orhun/git-cliff/pull/1490)
- @WaterWhisperer made their first contribution in [#1487](https://github.com/orhun/git-cliff/pull/1487)
- @ChihebBENCHEIKH1 made their first contribution in [#1483](https://github.com/orhun/git-cliff/pull/1483)
- @sermuns made their first contribution in [#1486](https://github.com/orhun/git-cliff/pull/1486)
- @danielpza made their first contribution in [#1448](https://github.com/orhun/git-cliff/pull/1448)
- @niklasmarderx made their first contribution in [#1456](https://github.com/orhun/git-cliff/pull/1456)
- @lawrence3699 made their first contribution in [#1484](https://github.com/orhun/git-cliff/pull/1484)
- @mixator made their first contribution in [#1392](https://github.com/orhun/git-cliff/pull/1392)
- @saudademjj made their first contribution in [#1450](https://github.com/orhun/git-cliff/pull/1450)
- @nbelsterling made their first contribution in [#1425](https://github.com/orhun/git-cliff/pull/1425)
- @y5 made their first contribution in [#1427](https://github.com/orhun/git-cliff/pull/1427)
- @Garbee made their first contribution in [#1371](https://github.com/orhun/git-cliff/pull/1371)

Any contribution is highly appreciated! See the [contribution guidelines](https://github.com/orhun/git-cliff/blob/main/CONTRIBUTING.md) for getting started.

Feel free to [submit issues](https://github.com/orhun/git-cliff/issues/new/choose) and join our [Discord](https://discord.gg/W3mAwMDWH4) / [Matrix](https://matrix.to/#/#git-cliff:matrix.org) for discussion!

Follow `git-cliff` on [Twitter](https://twitter.com/git_cliff) & [Mastodon](https://fosstodon.org/@git_cliff) to not miss any news!

## Support 🌟

If you liked `git-cliff` and/or my other projects [on GitHub](https://github.com/orhun), consider [donating](https://donate.orhun.dev) to support my open source endeavors.

- 💖 GitHub Sponsors: [@orhun](https://github.com/sponsors/orhun)
- ☕ Buy Me A Coffee: [https://www.buymeacoffee.com/orhun](https://www.buymeacoffee.com/orhun)

Have a fantastic day! ⛰️

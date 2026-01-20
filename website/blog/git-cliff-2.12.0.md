---
slug: 2.12.0
title: "What's new in 2.12.0? \U0001F195"
date: 2026-01-20T00:00:00.000Z
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

### 📡 Offline Mode

Now you can run **git-cliff** in offline mode using the `--offline` flag!

This feature disables contacting any external services, even if they are configured. This can be useful in scenarios where you want to avoid network calls or when working in a restricted environment.

```sh
$ git cliff --offline
```

This can be also configured as a part of the the remote configuration, for example:

```toml
[remote.gitlab]
owner = "archlinux"
repo = "arch-repro-status"
offline = true
```

---

### ⏩ Skip Tags via CLI

Skipping certain tags with regex was already possible via the configuration file:

```toml
[git]
skip_tags = "beta|alpha"
```

Now you can also specify the same via the command-line using the `--skip-tags` argument:

```sh
$ git cliff --skip-tags "beta|alpha"
```

---

### ↩️ Revert Log Verbosity

A couple of users reported the new verbosity level introduced in 2.11.0 was too noisy for their use cases.

With this release, we reverted that change and started exploring alternative ways to provide more detailed logs in a less-overwhelming way.

Related issues: [#1352](https://github.com/orhun/git-cliff/issues/1352), [#1354](https://github.com/orhun/git-cliff/pull/1354), [#1327](https://github.com/orhun/git-cliff/issues/1327)

---

### 🌀 Rename Azure DevOps variable

⚠️ This is a breaking change for those using [Azure DevOps remote integration](https://git-cliff.org/docs/integration/azure-devops).

In your template, rename `{{ azureDevops.contributors }}` to `{{ azure_devops.contributors }}`.

```diff
- {% for contributor in azureDevops.contributors | filter(attribute="is_first_time", value=true) %}
+ {% for contributor in azure_devops.contributors | filter(attribute="is_first_time", value=true) %}
```

See [#1318](https://github.com/orhun/git-cliff/issues/1318) for the rationale behind this change.

---

### 🧰 Other

- _(config)_ Respect the changelog.output configuration ([#1349](https://github.com/orhun/git-cliff/issues/1349)) - ([cfcc5ae](https://github.com/orhun/git-cliff/commit/cfcc5ae1c2c3bbb125cae27186649aaaeb32eb10))
- _(remote)_ Avoid false first-time contributors when tag timestamp missing ([#1348](https://github.com/orhun/git-cliff/issues/1348)) - ([de7cf02](https://github.com/orhun/git-cliff/commit/de7cf022e9d33a8ecdaf44fe56445a9d02fc1f1a))
- _(remote)_ Remove reqwest::Response::error_for_status ([#1336](https://github.com/orhun/git-cliff/issues/1336)) - ([081ba68](https://github.com/orhun/git-cliff/commit/081ba68753d388d5d6369da4460fe56060d7b359))

---

## New Contributors ❤️

- @taladar made their first contribution in [#1319](https://github.com/orhun/git-cliff/pull/1319)
- @barskern made their first contribution in [#1321](https://github.com/orhun/git-cliff/pull/1321)
- @ooooo-create made their first contribution in [#1334](https://github.com/orhun/git-cliff/pull/1334)
- @jylenhof made their first contribution in [#1320](https://github.com/orhun/git-cliff/pull/1320)

Any contribution is highly appreciated! See the [contribution guidelines](https://github.com/orhun/git-cliff/blob/main/CONTRIBUTING.md) for getting started.

Feel free to [submit issues](https://github.com/orhun/git-cliff/issues/new/choose) and join our [Discord](https://discord.gg/W3mAwMDWH4) / [Matrix](https://matrix.to/#/#git-cliff:matrix.org) for discussion!

Follow `git-cliff` on [Twitter](https://twitter.com/git_cliff) & [Mastodon](https://fosstodon.org/@git_cliff) to not miss any news!

## Support 🌟

If you liked `git-cliff` and/or my other projects [on GitHub](https://github.com/orhun), consider [donating](https://donate.orhun.dev) to support my open source endeavors.

- 💖 GitHub Sponsors: [@orhun](https://github.com/sponsors/orhun)
- ☕ Buy Me A Coffee: [https://www.buymeacoffee.com/orhun](https://www.buymeacoffee.com/orhun)

Have a fantastic day! ⛰️

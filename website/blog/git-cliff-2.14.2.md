---
slug: 2.14.2
title: "What's new in 2.14.2? \U0001F195"
date: 2026-09-18T00:00:00.000Z
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

This release mainly restores the **npm** and **PyPI** packages, alongside a few parser and CLI improvements.

The full changelog can be found [here](https://github.com/orhun/git-cliff/blob/main/CHANGELOG.md).

---

### 📦 npm & PyPI Packages

The packages for v2.14.1 were not published to these registries due to an expired npm token interrupting the release workflow. v2.14.2 restores both distribution channels, so the latest version of **git-cliff** is available again on [npm](https://www.npmjs.com/package/git-cliff) and [PyPI](https://pypi.org/project/git-cliff/) 🥳

```sh
npm install git-cliff
pip install git-cliff
```

---

### 🧩 Multiple Commit Parsers

Multiple commit parsers can now be applied to a single commit, allowing for more complex grouping and scoping rules!

Set `continue = true` on a parser to keep evaluating the parsers that follow it.

For example, a commit footer can set the scope before another parser assigns its group from the conventional commit type:

```toml
[git]
commit_parsers = [
  { footer = "^Component:Billing$", scope = "billing", continue = true },
  { footer = "^Component:Auth$", scope = "auth", continue = true },
  { message = "^feat", group = "Features" },
  { message = "^fix", group = "Bug Fixes" },
]
```

Given commits with `Component: Billing` and `Component: Auth` footers, this can produce:

```md
### Bug Fixes

- (billing) correct totals rounding

### Features

- (billing) add invoices
- (auth) add login page
```

---

### ✅ Match Every Commit Parser Field

When a commit parser defines multiple matching fields, **all of them must now match** before the parser is applied.

```toml
[git]
commit_parsers = [
  { message = "^feat:.*?(remove|delete|drop)", footer = "^BREAKING CHANGE:", group = "Removed" },
  { message = "^feat", group = "Added" },
  { message = "^fix", group = "Fixed" },
]
```

In the example above, a commit must match both the `message` and `footer` patterns to be grouped under "Removed". Previously, a commit matching either field would have been grouped under "Removed".

---

### 📁 Reliable `--workdir` Filtering

Using `--workdir` could previously produce an empty changelog because its derived include pattern did not match Git's repository-relative paths. The working directory is now resolved relative to the repository root before filtering commits.

```sh
# Generate a changelog scoped to a subdirectory
git cliff --workdir ./crates/my-crate

# Generate a changelog for the entire repository
git cliff --workdir .
```

When the working directory is the repository root, no path filter is added, ensuring all relevant commits are retained, including commits without file changes.

---

## ❤️ New Contributors

- @heaths made their first contribution in [#1641](https://github.com/orhun/git-cliff/pull/1641)
- @Jorge-Polanco-Roque made their first contribution in [#1627](https://github.com/orhun/git-cliff/pull/1627)
- @genx7up made their first contribution in [#1632](https://github.com/orhun/git-cliff/pull/1632)
- @sisp made their first contribution in [#1616](https://github.com/orhun/git-cliff/pull/1616)

Any contribution is highly appreciated! See the [contribution guidelines](https://github.com/orhun/git-cliff/blob/main/CONTRIBUTING.md) for getting started.  
Feel free to [submit issues](https://github.com/orhun/git-cliff/issues/new/choose) and join our [Discord](https://discord.gg/W3mAwMDWH4) / [Matrix](https://matrix.to/#/#git-cliff:matrix.org) for discussion!  
Follow `git-cliff` on [X](https://x.com/git_cliff) & [Mastodon](https://fosstodon.org/@git_cliff) to not miss any news!

## Support 🌟

If you like `git-cliff`, consider:

- 💖 GitHub Sponsors: [@orhun](https://github.com/sponsors/orhun)
- ☕ Buy Me A Coffee: [https://www.buymeacoffee.com/orhun](https://www.buymeacoffee.com/orhun)

Have a fantastic day! ⛰️

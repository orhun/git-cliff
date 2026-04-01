---
slug: 2.11.0
title: "What's new in 2.11.0?"
date: 2025-12-14T00:00:00.000Z
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

:::info[Happy new year!]

This is going to be the last release of 2025!

Wishing you all a fantastic new year ahead filled with Git commits, automated changelogs and cliff jumps! 🎄⛰️

:::

---

### 🌀 Azure DevOps Integration

**git-cliff** now supports [Azure Devops](https://azure.microsoft.com/en-us/products/devops) for remote integration, enabling changelog generation with metadata from Azure DevOps repositories (commits, pull requests, and contributors). 🥳

Simply configure your `cliff.toml` for your own repository as follows:

```toml
# Azure DevOps integration for fetching commit metadata.
[remote.azure_devops]
owner = "shiftme/gitcliff"
repo = "git-cliff-readme-example"
```

And then update your `[changelog].body` with the relevant template variables, e.g. `{{ commit.remote.pr_number }}`, `{{ commit.remote.username }}` and so on.

e.g. results in:

```md
## What's Changed in v1.0.0

- Initial commit by @orhun
- docs(project): add README.md by @orhun
- feat(parser): add ability to parse arrays by @orhun
- fix(args): rename help argument due to conflict by @orhun
- docs(example)!: add tested usage example by @orhun

### New Contributors

- @orhun made their first contribution
```

For more information, see the [documentation](https://git-cliff.org/docs/integration/azure-devops).

Thanks to [@amd989](https://github.com/amd989) for the implementation in [#1283](https://github.com/orhun/git-cliff/pull/1283)!

---

### ❎ Failing on unmatched commits

A new configuration variable was added for enforcing that all commits are matched by a commit parser:

```toml
[git]
commit_parsers = [
    { message = "^feat", group = "Should be matched" },
]

fail_on_unmatched_commit = true
```

If `fail_on_unmatched_commit` is set to `true`, **git-cliff** will fail when any commit included in the changelog is not matched by any of the configured [`commit_parsers`](https://git-cliff.org/docs/configuration/git#commit_parsers).

---

### 🧩 New built-in filters

**git-cliff** now has new custom filters you can use inside templates:

- `upper_first`: Converts the first character of a string to uppercase.

  ```jinja
    {{ "hello" | upper_first }} →  Hello
  ```

- `find_regex`: Finds all occurrences of a regex pattern in a string.

  ```jinja
  {{ "hello world, hello universe" | find_regex(pat="hello") }} →  [hello, hello]
  ```

- `replace_regex`: Replaces all occurrences of a regex pattern with a string.

  ```jinja
  {{ "hello world" | replace_regex(from="o", to="a") }} →  hella warld
  ```

- `split_regex`: Splits a string by a regex pattern.

  ```jinja
  {{ "hello world, hello universe" | split_regex(pat=" ") }} →  [hello, world,, hello, universe]
  ```

---

### 🆙 Increased log verbosity

We have evaluated and increased the verbosity of _some_ log messages to provide better insights into the internal workings of **git-cliff**.

To get more detailed logs, provide one or multiple `-v` flags when running:

```sh
$ git cliff -vv
```

---

### ✨ Better include-path handling

1. The `--include_path`'s behavior has been revised and several reported issues have been addressed in [#1290](https://github.com/orhun/git-cliff/pull/1290) thanks to [@ognis1205](https://github.com/ognis1205)!

2. `--include-path` is now automatically set to the value of `--workdir` if the latter is provided. This ensures that commit parsing works as expected when a different working directory is specified.

Before:

```sh
git cliff --workdir my_crate --include-path my_crate
```

After:

```sh
git cliff --workdir my_crate
```

---

---

### 🦀 Better API

The **git-cliff** library crates (`git_cliff` & `git_cliff_core`) has been improved with several new features and enhancements!

- `git_cliff::run` now returns the generated `git_cliff_core::changelog::Changelog`,
- `git_cliff::write_changelog` helper writes it to a file or stdout,
- `git_cliff::init_config` function handles config creation,
- `git_cliff::check_new_version` is now public.

Breaking changes:

- `Changelog::new` / `Changelog::from_context` take `Config` by value

Here is how you can create a minimal **git-cliff** application in Rust:

```rust
use clap::Parser;
use git_cliff::args::Opt;
use git_cliff_core::error::Result;

fn main() -> Result<()> {
    let args = Opt::parse();
    let changelog = git_cliff::run(args.clone())?;
    git_cliff::write_changelog(&args, changelog, std::io::stdout())?;
    Ok(())
}
```

---

### 🧰 Other

- _(bump)_ Write bumped version to stdout even when output config is set ([#1307](https://github.com/orhun/git-cliff/issues/1307)) - ([314ff57](https://github.com/orhun/git-cliff/commit/314ff57d9138da86027164b7cbeb7045c6f550f7))
- _(args)_ Group remote-related CLI arguments under REMOTE OPTIONS heading ([#1271](https://github.com/orhun/git-cliff/issues/1271)) - ([0b6af12](https://github.com/orhun/git-cliff/commit/0b6af122bb8d39f000591a4a700f8c011ac1827d))
- _(remote)_ Expose commits and PRs as streams ([#1272](https://github.com/orhun/git-cliff/issues/1272)) - ([b82221a](https://github.com/orhun/git-cliff/commit/b82221abd1981b6ecce8ab428fede8165ebb4246))
- _(ci)_ Stabilize lychee link checking in CI ([#1295](https://github.com/orhun/git-cliff/issues/1295)) - ([7ed1db0](https://github.com/orhun/git-cliff/commit/7ed1db0527211c9be2f54425498f823c503482c2))

---

## New Contributors ❤️

- @Lewiscowles1986 made their first contribution in [#1226](https://github.com/orhun/git-cliff/pull/1226)
- @OpenSauce made their first contribution in [#1314](https://github.com/orhun/git-cliff/pull/1314)
- @amd989 made their first contribution in [#1283](https://github.com/orhun/git-cliff/pull/1283)
- @asweet-confluent made their first contribution in [#1272](https://github.com/orhun/git-cliff/pull/1272)
- @linus-skold made their first contribution in [#1287](https://github.com/orhun/git-cliff/pull/1287)
- @simoncdn made their first contribution in [#1305](https://github.com/orhun/git-cliff/pull/1305)
- @haidaraM made their first contribution in [#1285](https://github.com/orhun/git-cliff/pull/1285)
- @ritoban23 made their first contribution in [#1271](https://github.com/orhun/git-cliff/pull/1271)

Any contribution is highly appreciated! See the [contribution guidelines](https://github.com/orhun/git-cliff/blob/main/CONTRIBUTING.md) for getting started.

Feel free to [submit issues](https://github.com/orhun/git-cliff/issues/new/choose) and join our [Discord](https://discord.gg/W3mAwMDWH4) / [Matrix](https://matrix.to/#/#git-cliff:matrix.org) for discussion!

Follow `git-cliff` on [Twitter](https://twitter.com/git_cliff) & [Mastodon](https://fosstodon.org/@git_cliff) to not miss any news!

## Support 🌟

If you liked `git-cliff` and/or my other projects [on GitHub](https://github.com/orhun), consider [donating](https://donate.orhun.dev) to support my open source endeavors.

- 💖 GitHub Sponsors: [@orhun](https://github.com/sponsors/orhun)
- ☕ Buy Me A Coffee: [https://www.buymeacoffee.com/orhun](https://www.buymeacoffee.com/orhun)

Have a fantastic day! ⛰️

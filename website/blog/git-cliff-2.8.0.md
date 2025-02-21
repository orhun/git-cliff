---
slug: 2.8.0
title: "What's new in 2.8.0? \U0001F195"
date: 2025-01-24T00:00:00.000Z
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

Happy new year! This version of **git-cliff** comes with quality of life improvements and bug fixes.

The full changelog can be found [here](https://github.com/orhun/git-cliff/blob/main/CHANGELOG.md).

---

### 🔥 Improved Monorepo Support

There were numerous improvements to the monorepo support in this release:

1. **git-cliff** now _discovers_ the Git repositories automatically even though when you run from sub directories.

2. The configuration file is now _automatically found_ when running from a sub directory.

3. The `include-path` is now _automatically set_ to the current directory when running from a sub directory.

As a result, the following command:

```bash
$ cd packages/some_library

$ git cliff --include-path "packages/some_library/**/*" --repository "../../"
```

becomes:

```bash
$ cd packages/some_library

$ git cliff # just works!
```

---

### 🛡️ Native TLS Support

**git-cliff** now supports enabling native TLS for remote requests. This is useful when you rely on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.

To enable it:

```bash
$ git cliff --use-native-tls
```

Or configure it in your `cliff.toml`:

```toml
[remote.gitlab]
owner = "archlinux"
repo = "arch-repro-status"
api_url = "https://gitlab.archlinux.org/api/v4"
native_tls = true
```

---

### ⚙️ Custom Config Name

You can now specify a custom filename for the configuration while initializing **git-cliff**:

```bash
$ git-cliff --init --config custom.toml
```

---

### 🚨 Better Errors

Before:

```
$ git cliff test
 ERROR git_cliff > Git error: `unable to parse OID - contains invalid characters; class=Invalid (3)`
```

After:

```
$ git cliff test
 ERROR git_cliff > Failed to set the commit range: unable to parse OID - contains invalid characters; class=Invalid (3)
"test" is not a valid commit range. Did you provide the correct arguments?
```

---

### 🔄 Run with Callback API

If you are using **git-cliff** in your Rust project as a library, you can now run it with a callback function to modify the changelog before it's printed:

```rust
use clap::Parser;
use git_cliff::args::Opt;
use git_cliff_core::error::Result;

fn main() -> Result<()> {
    let args = Opt::parse();

    git_cliff::run_with_changelog_modifier(args, |changelog| {
        println!("Releases: {:?}", changelog.releases);
        Ok(())
    })?;

    Ok(())
}
```

---

### 🧰 Other

- _(config)_ Allow environment overwrites when using builtin config ([#961](https://github.com/orhun/git-cliff/issues/961)) - ([7ba3b55](https://github.com/orhun/git-cliff/commit/7ba3b55448bdbf7a4a475df2081b6d7c2e2ceb34))
- _(remote)_ Fix detection of GitLab merge request sha ([#968](https://github.com/orhun/git-cliff/issues/968)) - ([1297655](https://github.com/orhun/git-cliff/commit/12976550d35bad8d535518010046bd136875736b))
- _(tips)_ Extend the merge commit filter example ([#963](https://github.com/orhun/git-cliff/issues/963)) - ([09c0f90](https://github.com/orhun/git-cliff/commit/09c0f905d8b20b585b0bc8183f14250d1a381ca0))
- _(build)_ Bump MSRV to 1.83.0 - ([37598c2](https://github.com/orhun/git-cliff/commit/37598c2d417a1646ec90590ab2a1f6d9da66296c))

---

## Contributions 👥

- @xsadia made their first contribution in [#992](https://github.com/orhun/git-cliff/pull/992)
- @chenrui333 made their first contribution in [#1002](https://github.com/orhun/git-cliff/pull/1002)
- @hackenbergstefan made their first contribution in [#968](https://github.com/orhun/git-cliff/pull/968)
- @paul-uz made their first contribution in [#963](https://github.com/orhun/git-cliff/pull/963)
- @jmartens made their first contribution in [#959](https://github.com/orhun/git-cliff/pull/959)

Any contribution is highly appreciated! See the [contribution guidelines](https://github.com/orhun/git-cliff/blob/main/CONTRIBUTING.md) for getting started.

Feel free to [submit issues](https://github.com/orhun/git-cliff/issues/new/choose) and join our [Discord](https://discord.gg/W3mAwMDWH4) / [Matrix](https://matrix.to/#/#git-cliff:matrix.org) for discussion!

Follow `git-cliff` on [Twitter](https://twitter.com/git_cliff) & [Mastodon](https://fosstodon.org/@git_cliff) to not miss any news!

## Support 🌟

If you liked `git-cliff` and/or my other projects [on GitHub](https://github.com/orhun), consider [donating](https://donate.orhun.dev) to support my open source endeavors.

- 💖 GitHub Sponsors: [@orhun](https://github.com/sponsors/orhun)
- ☕ Buy Me A Coffee: [https://www.buymeacoffee.com/orhun](https://www.buymeacoffee.com/orhun)

Have a fantastic day! ⛰️

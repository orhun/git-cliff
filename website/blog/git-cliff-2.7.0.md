---
slug: 2.7.0
title: "What's new in 2.7.0?"
date: 2024-11-20T00:00:00.000Z
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

### 🥋 Jujutsu Support

`git-cliff` now supports opening a repository that has been cloned using [Jujutsu](https://jj-vcs.github.io/jj/latest/)!

For example:

```bash
$ jj git clone --colocate https://github.com/orhun/git-cliff

$ cd git-cliff

$ git cliff # works!
```

:::caution

This works differently with colocated and non-colocated repositories. See the [documentation](https://git-cliff.org/docs/usage/jujutsu) for more information.

:::

:::tip

Watch my first live reaction to Jujutsu on this stream: [Learning Jujutsu (a version control system)](https://www.youtube.com/watch?v=VcKKhrb4E6s)

:::

---

### ☘️ Add missing fields to context

A bug causing some fields such as `footer` to be missing in the context JSON has been fixed.

This means that the following command now yields an identical result with `git-cliff`:

```bash
# hey look, a snake eating its own tail! 🐍
git cliff --context | git cliff --from-context
```

- [`--context`](https://git-cliff.org/docs/usage/print-context): prints the changelog context as JSON
- [`--from-context`](https://git-cliff.org/docs/usage/load-context): generates a changelog from the context JSON

---

### 📩 Raw message in context

The context now contains the raw/unprocessed full commit message in the `raw_message` field. For example:

```json
{
  "version": "v0.1.0-rc.21",
  "message": "The annotated tag message for the release",
  "commits": [
    {
      "raw_message": "<type>[scope]: <description>\n[body]\n[footer(s)]"
    }
  ]
}
```

You can use it like so:

```jinja2
{% for commit in commits %}
  {{ commit.raw_message }}
{% endfor %}
```

---

### ⚙️ Remote API URL configuration

In addition to the command-line/environment variables, you can now override the remote API URL in the configuration file as follows:

```toml
[remote.gitlab]
owner = "archlinux"
repo = "arch-repro-status"
api_url = "https://gitlab.archlinux.org/api/v4" # new!
```

This is useful when you have a self-hosted Git service and want to use the API for fetching metadata.

See the [`remote` configuration](https://git-cliff.org/docs/configuration/remote) for more information.

---

### ✨ Preserve first time contributors

There was a bug causing the first time contributors to be removed from the changelog when there was a new release. This has been fixed and now the first time contributors are preserved in the changelog.

So if you run `git cliff` now, you might get new names in the changelog! Don't be surprised.

See this [pull request](https://github.com/orhun/git-cliff/pull/925) for more details.

---

### 🐋 ARM Docker images

We brought back the Docker images for ARM64! 🎉 See them [here](https://hub.docker.com/r/orhunp/git-cliff).

```bash
docker run --platform linux/arm64 -t -v "$(pwd)":/app/ "orhunp/git-cliff:${TAG:-latest}"
```

There was a problem building these images due to the timeouts in the GitHub Actions workflow. This turned out to be a problem related to needlessly fetching the Rust toolchain in the build step of `cargo-chef` and is now fixed [in this pull request](https://github.com/orhun/git-cliff/pull/919).

See the related discussion [here](https://github.com/orhun/git-cliff/issues/879).

---

### ❄️ Nix environment

We now have a basic and reproducible dev environment using Nix along with CI checks for it!

[Here](https://github.com/orhun/git-cliff/blob/main/flake.nix) is the Nix flake and you can use it by running `nix build` and `nix run` commands.

---

### 🎨 Colored help

A small cosmetic change, but the output of `git cliff --help` is now colorful!

Try it for yourself :)

---

### 💖 User testimonials

Do you like `git-cliff`? Spread the word on social media and let me know your thoughts! I will be featuring your testimonials.

I collected the testimonials that I could find so far and added them to the [website](https://git-cliff.org). It picks one randomly on each page load.

Shoutout to those amazing people!

---

### 🚀 Stabilize remote integration

The remote integration with GitHub/GitLab/Gitea/Bitbucket has been stabilized and now works as expected (apart from a couple of bugs that come and go occasionally).

---

### 🧰 Other

- _(log)_ Add trace log about which command is being run - ([a9b2690](https://github.com/orhun/git-cliff/commit/a9b26901e38aa3d3b1042d3bc10d2fe7c6c06565))
- _(bitbucket)_ Match PR and release metadata correctly ([#907](https://github.com/orhun/git-cliff/issues/907)) - ([e936ed5](https://github.com/orhun/git-cliff/commit/e936ed571533ea6c41a1dd2b1a29d085c8dbada5))
- _(changelog)_ Include the root commit when `--latest` is used with one tag ([#901](https://github.com/orhun/git-cliff/issues/901)) - ([508a97e](https://github.com/orhun/git-cliff/commit/508a97edb088f77d01f232676d1e3c7f129071b2))
- _(config)_ Add the 'other' parser to the default config - ([12cb1df](https://github.com/orhun/git-cliff/commit/12cb1df561cde39a9a0d0f719156a000f3f4d61b))
- _(git)_ Improve docs for commit_preprocessors and commit_parsers ([#928](https://github.com/orhun/git-cliff/issues/928)) - ([c1f1215](https://github.com/orhun/git-cliff/commit/c1f12154e7efa75f19ce632dc3052dae390c9211))

---

## Contributions 👥

- @pauliyobo made their first contribution in [#896](https://github.com/orhun/git-cliff/pull/896)
- @blackheaven made their first contribution in [#939](https://github.com/orhun/git-cliff/pull/939)
- @Muhammad-Owais-Warsi made their first contribution in [#928](https://github.com/orhun/git-cliff/pull/928)
- @kemitix made their first contribution in [#930](https://github.com/orhun/git-cliff/pull/930)
- @mcwarman made their first contribution in [#925](https://github.com/orhun/git-cliff/pull/925)
- @LtdSauce made their first contribution in [#919](https://github.com/orhun/git-cliff/pull/919)
- @dqkqd made their first contribution in [#920](https://github.com/orhun/git-cliff/pull/920)
- @gsquire made their first contribution in [#909](https://github.com/orhun/git-cliff/pull/909)
- @rarescosma made their first contribution in [#901](https://github.com/orhun/git-cliff/pull/901)
- @vsn4ik made their first contribution in [#894](https://github.com/orhun/git-cliff/pull/894)

Any contribution is highly appreciated! See the [contribution guidelines](https://github.com/orhun/git-cliff/blob/main/CONTRIBUTING.md) for getting started.

Feel free to [submit issues](https://github.com/orhun/git-cliff/issues/new/choose) and join our [Discord](https://discord.gg/W3mAwMDWH4) / [Matrix](https://matrix.to/#/#git-cliff:matrix.org) for discussion!

Follow `git-cliff` on [Twitter](https://twitter.com/git_cliff) & [Mastodon](https://fosstodon.org/@git_cliff) to not miss any news!

## Support 🌟

If you liked `git-cliff` and/or my other projects [on GitHub](https://github.com/orhun), consider [donating](https://donate.orhun.dev) to support my open source endeavors.

- 💖 GitHub Sponsors: [@orhun](https://github.com/sponsors/orhun)
- ☕ Buy Me A Coffee: [https://www.buymeacoffee.com/orhun](https://www.buymeacoffee.com/orhun)

Have a fantastic day! ⛰️

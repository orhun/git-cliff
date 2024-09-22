---
slug: 2.5.0
title: "What's new in 2.5.0?"
date: 2024-08-24T00:00:00.000Z
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

### 🔥 Generate changelog from context

Meet our powerful new command-line argument: `--from-context`.

```bash
# create a context
$ git cliff --context -o context.json

# generate changelog from context
$ git cliff --from-context context.json
```

This new extension point allows transformations on the context and can be especially useful when preprocessor/postprocessor/linkprocessor capabilities are limited.

One example use case is:

1. Print context
2. Modify it with an external tool
3. _Pipe_ it back into `git-cliff`

If you need additional data in the changelog, you can also use the newly added `extra` free-form metadata in the context:

```json
{
  "id": "5061081d6272b1da2146fab49d803c193db309d9",
  "message": "commit message",
  "extra": {
    "note": "this can be anything"
  }
}
```

---

### 🧩 Grouping by arbitrary fields

`git-cliff` now supports grouping commits by arbitrary context fields instead of just a limited set. This means that you can use any context field for `commit_parsers` as `field`.

For example, to group by GitHub PR labels:

```toml
[git]
commit_parsers = [
  { field = "github.pr_labels", pattern = "breaking-change", group = "<!-- 0 --> 🏗️ Breaking changes" },
  { field = "github.pr_labels", pattern = "type/enhancement", group = "<!-- 1 --> 🚀 Features" },
  { field = "github.pr_labels", pattern = "type/bug", group = "<!-- 2 --> 🐛 Fixes" },
  { field = "github.pr_labels", pattern = "type/update", group = "<!-- 3 --> 🧪 Dependencies" },
  { field = "github.pr_labels", pattern = "type/refactor", group = "<!-- 4 --> 🏭 Refactor" },
  { field = "github.pr_labels", pattern = "area/documentation", group = "<!-- 5 --> 📝 Documentation" },
  { field = "github.pr_labels", pattern = ".*", group = "<!-- 6 --> 🌀 Miscellaneous" },
]
```

See the [`commit_parsers`](https://git-cliff.org/docs/configuration/git#commit_parsers) documentation for more information.

---

### ⬆️ Bump specific versions

Now you can specify the semver type while using `--bump`:

```bash
$ git cliff --bump [major|minor|patch]
```

See the [`bump`](https://git-cliff.org/docs/usage/bump-version) documentation for more information.

---

### ⚡ Gotta go fast

`git-cliff` now runs 258x faster for `--include-path`/`--exclude-path` arguments thanks to caching the commit retain checks.

```
Now: 0.080 s
Before: 20.633 s
```

We also improved handling of include/exclude patterns (e.g., by considering the first commit).

See the [implementation](https://github.com/orhun/git-cliff/pull/772) for _cool_ flamegraphs and more!

---

### 💯 Performance profiling

`git-cliff` now supports building with performance profiling instrumentation, which helps identify bottlenecks.

To create a flame graph SVG:

```bash
$ cargo run --profile=bench --features=profiler
```

See the [documentation](https://git-cliff.org/docs/development/profiling) for more information.

---

### ⚗️ Better integration activation

Before this change, the only way to activate a remote integration (and fetch remote data) was by incorporating the related variables in a template.

This meant that the changelog context wouldn't contain GitHub-related fields unless you used something like `github.contributors` in your template.

Now we’ve added support for enabling the remote integration in the following cases:

- If the `[remote]` table is configured.
- If the remote is set via command-line arguments (e.g., `--github-repo`).

So, the following output will contain GitHub variables even with the default template (since the remote is set):

```sh
$ git cliff --context --github-repo orhun/git-cliff
```

Additionally, we fixed [a bug](https://github.com/orhun/git-cliff/issues/812) where some of the GitHub-related variables were not recognized in the template.

---

### 🔢 `count_tags`

A new configuration option has been added to the `[git]` section!

```toml
[git]
count_tags = "v.*-beta.*"
```

:::info

`count_tags` works like an inverted version of `ignore_tags`, including all the commits but only counting the specific tags.

:::

See [the implementation](https://github.com/orhun/git-cliff/pull/599) for more details and an example use case.

---

### 🏆 KaiCode: Open Source Festival

`git-cliff` won a prize for finishing second place in the [KaiCode Open Source Festival](https://www.kaicode.org/2024.html)!

> The orhun/git-cliff project (8.3K★), a customizable changelog generator, impressed us with its excellent easy-to-read source code, build pipeline organization, integration testing, and active issue triaging. However, code coverage is rather low, some functions are too long, there is a lack of peer reviews, and a lack of clarity in the repository structure. $1024 was the reward.

---

### 🦊 GitLab integration fixes

- _(gitlab)_ URL-encode the owner in remote requests for GitLab ([#742](https://github.com/orhun/git-cliff/issues/742)) - ([e3e7c07](https://github.com/orhun/git-cliff/commit/e3e7c0794082e418a78f99e7d9c09161f4d14d5f))
- _(args)_ Allow GitLab groups with `--gitlab-repo` ([#807](https://github.com/orhun/git-cliff/issues/807)) - ([6fbfdb5](https://github.com/orhun/git-cliff/commit/6fbfdb5963ad7d39a389001b660df5bf7f38dd37))

---

### 🧰 Other

- _(changelog)_ Skip ssh and x509 signatures in tag messages ([#748](https://github.com/orhun/git-cliff/issues/748)) - ([ecbabbf](https://github.com/orhun/git-cliff/commit/ecbabbfb39b986e8445d2feb3189bab4307fd854))
- _(changelog)_ Allow using `--bumped-version` without conventional commits ([#806](https://github.com/orhun/git-cliff/issues/806)) - ([e74080c](https://github.com/orhun/git-cliff/commit/e74080cec4283a45f0f81b1b656af466ae4bd693))
- _(config)_ Allow using environment variables without config file present ([#783](https://github.com/orhun/git-cliff/issues/783)) - ([2471745](https://github.com/orhun/git-cliff/commit/2471745e110955be49310afe11e24719ab79b658))
- _(config)_ Make example templates more user-friendly - ([6f8ea19](https://github.com/orhun/git-cliff/commit/6f8ea19baafea2718a00a046b74f0cbbfacc8d46))
- _(lib)_ Clean up some code ([#709](https://github.com/orhun/git-cliff/issues/709)) - ([4b0c0eb](https://github.com/orhun/git-cliff/commit/4b0c0eb09abf1264b5cc92bf40f75c8e05e17da6))

---

## Contributions 👥

- @oberrich made their first contribution in [#809](https://github.com/orhun/git-cliff/pull/809)
- @tisonkun made their first contribution in [#599](https://github.com/orhun/git-cliff/pull/599)
- @DerTiedemann made their first contribution in [#758](https://github.com/orhun/git-cliff/pull/758)
- @DaniPopes made their first contribution in [#709](https://github.com/orhun/git-cliff/pull/709)
- @artrz made their first contribution in [#779](https://github.com/orhun/git-cliff/pull/779)
- @braineo made their first contribution in [#744](https://github.com/orhun/git-cliff/pull/744)
- @myl7 made their first contribution in [#776](https://github.com/orhun/git-cliff/pull/776)
- @pawamoy made their first contribution in [#774](https://github.com/orhun/git-cliff/pull/774)
- @tonybutt made their first contribution in [#742](https://github.com/orhun/git-cliff/pull/742)
- @PigeonF made their first contribution in [#748](https://github.com/orhun/git-cliff/pull/748)
- @janbuchar made their first contribution in [#784](https://github.com/orhun/git-cliff/pull/784)
- @weichweich made their first contribution in [#807](https://github.com/orhun/git-cliff/pull/807)

Any contribution is highly appreciated! See the [contribution guidelines](https://github.com/orhun/git-cliff/blob/main/CONTRIBUTING.md) for getting started.

Feel free to [submit issues](https://github.com/orhun/git-cliff/issues/new/choose) and join our [Discord](https://discord.gg/W3mAwMDWH4) / [Matrix](https://matrix.to/#/#git-cliff:matrix.org) for discussion!

Follow `git-cliff` on [Twitter](https://twitter.com/git_cliff) & [Mastodon](https://fosstodon.org/@git_cliff) to not miss any news!

## Support 🌟

If you liked `git-cliff` and/or my other projects [on GitHub](https://github.com/orhun), consider [donating](https://donate.orhun.dev) to support my open source endeavors.

- 💖 GitHub Sponsors: [@orhun](https://github.com/sponsors/orhun)
- ☕ Buy Me A Coffee: [https://www.buymeacoffee.com/orhun](https://www.buymeacoffee.com/orhun)

Have a fantastic day! ⛰️

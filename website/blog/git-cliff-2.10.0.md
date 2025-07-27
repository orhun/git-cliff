---
slug: 2.10.0
title: "What's new in 2.10.0? \U0001F195"
date: 2025-07-27T00:00:00.000Z
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

### 📈 Release statistics

**git-cliff** now supports adding various release-related metrics to the changelog via `statistics` variable!

You can use it in your template as follows:

```toml
[changelog]
body = """
### Commit Statistics

- {{ statistics.commit_count }} commit(s) contributed to the release.
- {{ statistics.commits_timespan | default(value=0) }} day(s) passed between the first and last commit.
- {{ statistics.conventional_commit_count }} commit(s) parsed as conventional.
- {{ statistics.links | length }} linked issue(s) detected in commits.

{%- for link in statistics.links %}
      {{ "  " }}- [{{ link.text }}]({{ link.href }}) (referenced {{ link.count }} time(s))
{%- endfor %}

- {{ statistics.days_passed_since_last_release }} day(s) passed between releases.

"""
```

This will render a section like this in the changelog:

```markdown
## Commit Statistics

- 5 commit(s) contributed to the release.
- 0 day(s) passed between the first and last commit.
- 5 commit(s) parsed as conventional.
- 3 linked issue(s) detected in commits.
  - [#452](https://github.com/orhun/git-cliff/issues/452) (referenced 2 time(s))
  - [#1148](https://github.com/orhun/git-cliff/issues/1148) (referenced 1 time(s))
  - [ietf-rfc3986](https://datatracker.ietf.org/doc/html/rfc3986) (referenced 1 time(s))
- 1430 day(s) passed between releases.
```

See [release statistics](/docs/templating/context#release-statistics) for the available variables and more details.

Thanks to [Shingo OKAWA](https://github.com/ognis1205) for the implementation in [#1151](https://github.com/orhun/git-cliff/pull/1151)!

---

### 📝 New template

Related to the new statistics feature, we added a new built-in template called [`statistics.toml`](https://github.com/orhun/git-cliff/blob/main/examples/statistics.toml)!

It can be used as follows:

```bash
$ git cliff --config statistics
```

To initialize `cliff.toml` with it:

```bash
$ git cliff --init statistics

 INFO  git_cliff > Saving the configuration file (statistics) to "cliff.toml"
```

It serves the purpose of providing a basic template that includes release statistics. You can use it as a starting point for your own changelog template or simply use it as is.

---

### 📁 Include/exclude paths in config

As highly requested, you can now include or exclude specific paths in your changelog generation via the [`include_paths`](/docs/configuration/git#include_paths) and [`exclude_paths`](/docs/configuration/git#exclude_paths) options in the configuration file.

```
[git]
include_paths = ["src/", "doc/**/*.md"]
exclude_paths = ["unrelated/"]
```

These options are the same as providing `--include-paths` and `--exclude-paths` command line arguments.

Thanks to [@Kriskras99](https://github.com/Kriskras99) for implementing this in [#1173](https://github.com/orhun/git-cliff/pull/1173)!

---

### 🧮 Support matching arrays via parsers

The commit parser has been extended to support regex matching on array values, such as `remote.pr_labels`.

For example, this makes it possible to group commits based on their GitHub labels as follows:

```toml
[git]
commit_parsers = [
  { field = "remote.pr_labels", pattern = "duplicate|invalid|wontfix|skip changelog", skip = true },
  { field = "remote.pr_labels", pattern = "breaking change", group = "<!-- 0 -->🏗️ Breaking Changes" },
  { field = "remote.pr_labels", pattern = "feature|deprecation", group = "<!-- 1 -->🚀 Features" },
  { field = "remote.pr_labels", pattern = "enhancement|refactor", group = "<!-- 1 -->🛠️ Enhancements" },
  { field = "remote.pr_labels", pattern = "bug|regression", group = "<!-- 2 -->🐛 Bug Fixes" },
  { field = "remote.pr_labels", pattern = "security", group = "<!-- 3 -->🔐 Security" },
  { field = "remote.pr_labels", pattern = "documentation", group = "<!-- 4 -->📝 Documentation" },
  { message = ".*", group = "<!-- 5 -->🌀 Miscellaneous" },
]
```

---

### 🗑️ Empty header/footer as default

In the previous release, we internally started initializing the configuration file with default values. This sadly made it impossible to render a changelog without a header or footer.

This behavior has been reverted in this release and the default values for `[changelog.header]` and `[changelog.footer]` are now empty. Meaning that the following is a minimal configuration that will render a changelog without a header or footer:

```toml
[changelog]
body = """
{% if version %}\
    ## {{ version | trim_start_matches(pat="v") }} - {{ timestamp | date(format="%Y-%m-%d") }}\
{% else %}\
    ## Unreleased\
{% endif %}\
{% for group, commits in commits | group_by(attribute="group") %}
    ### {{ group | upper_first }}
    {% for commit in commits %}\
        - {% if commit.breaking %}[**breaking**] {% endif %}{{ commit.message | upper_first }}
    {% endfor %}\
{% endfor %}\n
"""
```

---

### 🐧 Gentoo support

**git-cliff** made its way into the [Gentoo Linux](https://www.gentoo.org/) package repository! 🎉

It can be installed via the following command:

```bash
emerge git-cliff
```

See the package page [here](https://packages.gentoo.org/packages/dev-vcs/git-cliff).

Thanks to [@aspann](https://github.com/aspann) for packaging!

---

### 🏴 Spaces instead of tabs

<iframe width="100%" height="315" src="https://www.youtube.com/embed/oRva7UxGQDw?si=GZUdkxKZHjrzZ6OG" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

**git-cliff** now uses [spaces instead of tabs](https://github.com/orhun/git-cliff/issues/1184) throughout the codebase! This change made the code more consistent with the Rust community's conventions and improved readability.

:::info[Fun Fact]

Hard tabs are used in around [0.1% of Rust projects](https://github.com/orhun/git-cliff/pull/1184#issue-3164547412). I don't know why I went with that config option when I first started this project. I guess I was a rebel back then.

:::

---

### 🧰 Other

- _(config)_ Check if commit.footers is defined in detailed example ([#1170](https://github.com/orhun/git-cliff/issues/1170)) - ([078545f](https://github.com/orhun/git-cliff/commit/078545f55facbd0a82717b723e98589155bedd7e))
- _(generation)_ Ensure skip_tags condition is evaluated first ([#1190](https://github.com/orhun/git-cliff/issues/1190)) - ([318be66](https://github.com/orhun/git-cliff/commit/318be6637609c289cf58270222f1fcd29bf893ec))
- _(repo)_ Use the correct order while diffing paths ([#1188](https://github.com/orhun/git-cliff/issues/1188)) - ([ff6c310](https://github.com/orhun/git-cliff/commit/ff6c3105012b5827145ba4bf2bb660cce0b9c7bf))
- _(config)_ Implement FromStr instead of Config::parse_from_str() ([#1185](https://github.com/orhun/git-cliff/issues/1185)) - ([692345e](https://github.com/orhun/git-cliff/commit/692345e4454127e31c44fe46aaccc065ac0854cc))
- _(ci)_ Apply security best practices ([#1180](https://github.com/orhun/git-cliff/issues/1180)) - ([a32deca](https://github.com/orhun/git-cliff/commit/a32deca80823cf99fd968647217f72fa58c8ccc2))
- _(fixture)_ Add test fixture for overriding the conventional scope ([#1166](https://github.com/orhun/git-cliff/issues/1166)) - ([cb84a08](https://github.com/orhun/git-cliff/commit/cb84a08e60ca4c0f6108c95b4f2a62d47069014b))
- _(build)_ Bump MSRV to 1.85.1 - ([d8279d4](https://github.com/orhun/git-cliff/commit/d8279d4d047ebf9e7c00948bbba266ccc75d262a))
- _(crate)_ Remove Rust nightly requirement - ([4f3e5af](https://github.com/orhun/git-cliff/commit/4f3e5af46bb51e412dff88001b3d135d8575bbe8))

---

## New Contributors ❤️

- @Nick2bad4u made their first contribution in [#1180](https://github.com/orhun/git-cliff/pull/1180)
- @aspann made their first contribution in [#1203](https://github.com/orhun/git-cliff/pull/1203)
- @muzimuzhi made their first contribution in [#1200](https://github.com/orhun/git-cliff/pull/1200)
- @j-g00da made their first contribution in [#1188](https://github.com/orhun/git-cliff/pull/1188)
- @Kriskras99 made their first contribution in [#1173](https://github.com/orhun/git-cliff/pull/1173)
- @wetneb made their first contribution in [#1165](https://github.com/orhun/git-cliff/pull/1165)
- @gmeligio made their first contribution in [#1170](https://github.com/orhun/git-cliff/pull/1170)
- @LitoMore made their first contribution in [#1164](https://github.com/orhun/git-cliff/pull/1164)

Any contribution is highly appreciated! See the [contribution guidelines](https://github.com/orhun/git-cliff/blob/main/CONTRIBUTING.md) for getting started.

Feel free to [submit issues](https://github.com/orhun/git-cliff/issues/new/choose) and join our [Discord](https://discord.gg/W3mAwMDWH4) / [Matrix](https://matrix.to/#/#git-cliff:matrix.org) for discussion!

Follow `git-cliff` on [Twitter](https://twitter.com/git_cliff) & [Mastodon](https://fosstodon.org/@git_cliff) to not miss any news!

## Support 🌟

If you liked `git-cliff` and/or my other projects [on GitHub](https://github.com/orhun), consider [donating](https://donate.orhun.dev) to support my open source endeavors.

- 💖 GitHub Sponsors: [@orhun](https://github.com/sponsors/orhun)
- ☕ Buy Me A Coffee: [https://www.buymeacoffee.com/orhun](https://www.buymeacoffee.com/orhun)

Have a fantastic day! ⛰️

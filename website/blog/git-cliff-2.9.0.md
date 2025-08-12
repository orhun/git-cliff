---
slug: 2.9.0
title: "What's new in 2.9.0?"
date: 2025-05-24T00:00:00.000Z
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

## 10k Stars! 🌟

_Started from the [cd69e764f68e5f09cf6e14975e6a876cdccbcfb9](https://github.com/orhun/git-cliff/commit/cd69e764f68e5f09cf6e14975e6a876cdccbcfb9), now we're here._

<details>
  <summary>Click here for the star history</summary>

![star history](/img/git-cliff-star-history.png)

</details>

**git-cliff** has reached a whopping **10000** stars on GitHub and I wanted to celebrate this huge milestone with giving away a limited edition T-shirt!

![giveaway](/img/10k-giveaway.jpg)

You can join the giveaway by [**clicking here**](https://git-cliff.org/10k). Nothing else is required!

If you want to buy the T-shirt and support the project, visit our shop at [Grindhouse](https://grindhouse.dev/products/git-cliff-10k-stars-limited-edition) for different sizes and colors!

Thank you all for the support and love you have shown to **git-cliff**! ⛰️🧡

---

## What's new? ⛰️

The full changelog can be found [here](https://github.com/orhun/git-cliff/blob/main/CHANGELOG.md).

---

### 🌀 Submodule Support

**git-cliff** now supports submodules! You can recurse into submodules and generate changelogs for them as well.

Just set the following option in your configuration file:

```toml
[git]
recurse_submodules = true
```

And then you can use the `submodule_commits` template variable to access the commits of submodules as follows:

```toml
[changelog]
body = """
{% for submodule_path, commits in submodule_commits %}
    ### {{ submodule_path | upper_first }}
    {% for group, commits in commits | group_by(attribute="group") %}
        #### {{ group | upper_first }}
        {% for commit in commits %}
            - {{ commit.message | upper_first }}\
        {% endfor %}
    {% endfor %}
{% endfor %}\n
"""
```

Thanks [@lehmanju](https://github.com/lehmanju) for the implementation in [#1082](https://github.com/orhun/git-cliff/pull/1082)!

---

### ⚠️ Conventional commit check

**git-cliff** can now check if the commits in the repository follow the [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) specification.

To enable this check, set the `require_conventional` option in your configuration file:

```toml
[git]
require_conventional = true
```

If any unconventional commits are found, an error will be thrown and the changelog generation will fail.

---

### 🛰️ Remote config

Have a configuration file elsewhere on the internet? No probs.

```bash
$ git cliff --config-url https://github.com/orhun/git-cliff/blob/main/examples/github-keepachangelog.toml?raw=true
```

The new `--config-url` option allows you to specify a URL to a configuration file!

---

### ↔️ Commit range variable

The template context now includes a `commit_range` variable that contains the range of commits that were used to generate the changelog.

Can be used as follows:

```
{{ commit_range.from }}..{{ commit_range.to }}

```

Results in:

```
a140cef0405e0bcbfb5de44ff59e091527d91b38..a9d4050212a18f6b3bd76e2e41fbb9045d268b80
```

:::tip

You can use the [`truncate`](https://keats.github.io/tera/docs/#truncate) filter to shorten the commit range:

```jinja
{{ commit_range.from | truncate(length=7, end="") }}..{{ commit_range.to | truncate(length=7, end="") }}
```

Results in:

```
a140cef..a9d4050
```

:::

---

### 🌿 Better branch support

**git-cliff** used to only support the default branches of the remotes (e.g., `main` branch on GitHub).

Now, it can fetch commits from the correct branch automatically based on the commit range that you provide.

For example:

```bash
$ git cliff v1.0.0..v1.0.1 --github-repo my-org/my-private-repo
```

This command used to default to the `main` branch of the `my-org/my-private-repo` repository. Now, it will use the `v1.1` branch thus using the correct commits for the changelog.

Similarly:

```bash
$ git cliff 9f66ac0f76..89de5e8e50 --gitlab-repo my-org/my-private-repo
```

The changelog will contains commits up to the commit `89de5e8e50`.

Thanks to [@william-stacken](https://github.com/william-stacken) for the implementation in [#1086](https://github.com/orhun/git-cliff/pull/1086)!

---

### 🔢 Disable topological sorting

The topological sorting of commits can now be disabled by setting the `topological_sort` option to `false` in your configuration file:

```toml
[git]
topo_order_commits = false
```

- If `false`, the commits will be sorted in the order they were committed, without considering their parent-child relationships.
  - This is equivalent to running `git log`.
- Otherwise, if `true` (default), the commits will be sorted topologically, which means that the commits will be ordered in such a way that all parent commits come before their children.
  - This is equivalent to running `git log --topo-order`.

---

### 📝 New blog posts

Check out the new blog posts from the community members:

- [An introduction to git-cliff for release management](https://substack.evancarroll.com/p/git-cliff-for-automated-release-management): Learn how to automate your software releases
- [Git-cliff and monorepos](https://substack.evancarroll.com/p/git-cliff-and-monorepos): An introduction to the monorepo capabilities of git-cliff

---

### 🛡️ Remove tj-actions

There was a security issue reported in the [tj-actions organization](https://semgrep.dev/blog/2025/popular-github-action-tj-actionschanged-files-is-compromised/).

There is also a GitHub Action created for **git-cliff**: [tj-actions/git-cliff](https://github.com/tj-actions/git-cliff).

The action seems to be unaffected by the compromise, but I have removed all the references to it from the documentation and the website for safety.

---

### 🐛 Various Bug Fixes

- _(bump)_ Check the next version against tag_pattern regex ([#1070](https://github.com/orhun/git-cliff/issues/1070)) - ([c4f0d28](https://github.com/orhun/git-cliff/commit/c4f0d28c39f34bc886c00c51cdeff851beda93de))
- _(bump)_ Accept lowercase values for bump_type config ([#1101](https://github.com/orhun/git-cliff/issues/1101)) - ([77632b2](https://github.com/orhun/git-cliff/commit/77632b276001a879ed4e3328a38a1cedfef67ca3))
- _(git)_ Handle worktrees while retrieving the path of repository ([#1054](https://github.com/orhun/git-cliff/issues/1054)) - ([fab02b0](https://github.com/orhun/git-cliff/commit/fab02b09833eb18be6ca540a436b254d13d7c678))
- _(remote)_ Fix detection of GitLab merge request sha if commits were squashed ([#1043](https://github.com/orhun/git-cliff/issues/1043)) - ([5f3a3d0](https://github.com/orhun/git-cliff/commit/5f3a3d0b4dbae5ec3239c79148258ba4fb47f376))
- _(submodules)_ Fix submodules handling when using custom range ([#1136](https://github.com/orhun/git-cliff/issues/1136)) - ([451a694](https://github.com/orhun/git-cliff/commit/451a694ad4d9db1f0545ef92bd0c6643b3d26600))
- _(template)_ Correctly serialize JSON for the commit fields ([#1145](https://github.com/orhun/git-cliff/issues/1145)) - ([e981e1d](https://github.com/orhun/git-cliff/commit/e981e1d1b27a65fc2d2fd51b025c27692a6c8910))

---

### 🧰 Other

- _(project)_ Migrate to Rust 2024 edition ([#1128](https://github.com/orhun/git-cliff/issues/1128)) - ([4445f06](https://github.com/orhun/git-cliff/commit/4445f063518bd8514ac19381e3ee6c61828c72a9))
- _(config)_ Initialize config structs with default values ([#1090](https://github.com/orhun/git-cliff/issues/1090)) - ([9e4bd07](https://github.com/orhun/git-cliff/commit/9e4bd077b5a18922021afd8ffd671b7d7958ee5c))
- _(quickstart)_ Clarify git-cliff command ([#1051](https://github.com/orhun/git-cliff/issues/1051)) - ([cd26bb2](https://github.com/orhun/git-cliff/commit/cd26bb2de35ebd6ba293ee969c3796ac32a08e21))
- _(security)_ Extend security policy ([#1142](https://github.com/orhun/git-cliff/issues/1142)) - ([4c3c946](https://github.com/orhun/git-cliff/commit/4c3c94692d88ee3d9d2931cabeeec67946aba381))

---

## New Contributors ❤️

- @ognis1205 made their first contribution in [#1145](https://github.com/orhun/git-cliff/pull/1145)
- @janderssonse made their first contribution in [#1142](https://github.com/orhun/git-cliff/pull/1142)
- @jdrst made their first contribution in [#1138](https://github.com/orhun/git-cliff/pull/1138)
- @lehmanju made their first contribution in [#1136](https://github.com/orhun/git-cliff/pull/1136)
- @Jean-Beru made their first contribution in [#1132](https://github.com/orhun/git-cliff/pull/1132)
- @william-stacken made their first contribution in [#1086](https://github.com/orhun/git-cliff/pull/1086)
- @SebClapie made their first contribution in [#1121](https://github.com/orhun/git-cliff/pull/1121)
- @okydk made their first contribution in [#1051](https://github.com/orhun/git-cliff/pull/1051)

Any contribution is highly appreciated! See the [contribution guidelines](https://github.com/orhun/git-cliff/blob/main/CONTRIBUTING.md) for getting started.

Feel free to [submit issues](https://github.com/orhun/git-cliff/issues/new/choose) and join our [Discord](https://discord.gg/W3mAwMDWH4) / [Matrix](https://matrix.to/#/#git-cliff:matrix.org) for discussion!

Follow `git-cliff` on [Twitter](https://twitter.com/git_cliff) & [Mastodon](https://fosstodon.org/@git_cliff) to not miss any news!

## Support 🌟

If you liked `git-cliff` and/or my other projects [on GitHub](https://github.com/orhun), consider [donating](https://donate.orhun.dev) to support my open source endeavors.

- 💖 GitHub Sponsors: [@orhun](https://github.com/sponsors/orhun)
- ☕ Buy Me A Coffee: [https://www.buymeacoffee.com/orhun](https://www.buymeacoffee.com/orhun)

Have a fantastic day! ⛰️

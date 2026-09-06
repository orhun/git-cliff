---
slug: 2.14.0
title: "What's new in 2.14.0? \U0001F195"
date: 2026-09-01T00:00:00.000Z
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

So many things...

The full changelog can be found [here](https://github.com/orhun/git-cliff/blob/main/CHANGELOG.md).

---

### ⚠️ Migration Guide

If you pass multiple values to `--include-path`, `--exclude-path`, `--with-commit`, or `--skip-commit`: you need to repeat the option for each value:

```diff
- git cliff --include-path "src/**" "docs/**"
+ git cliff --include-path "src/**" --include-path "docs/**"
```

:::note[For Rust API users 🦀]

- `Opt::config` is now an `Option<PathBuf>`. The public changelog and remote context types also gained fields, so code constructing them with struct literals must initialize the new fields.
- `git-cliff-core` now uses `git2` 0.21. If you use `git2` types exposed by its public API, update your `git2` dependency as well.

:::

:::note[For packagers 📦]

- Building **git-cliff** from source now requires Rust 1.88.0 or newer.
- The upgrade to `git2` 0.21 includes a libgit2 SONAME change. Dynamically linked packages must be rebuilt against the new libgit2 version.

:::

---

### 💻 CLI

- **Smarter config discovery:** when `--config` is omitted, **git-cliff** automatically looks for a configuration file. ([#1584](https://github.com/orhun/git-cliff/issues/1584))
- **Custom templates:** you can now keep your own configuration templates in a directory and initialize them by name with `--init`.

  Use `--templates-dir` (or `GIT_CLIFF_TEMPLATES_DIR`) to set the directory and `--list-templates` to see all built-in and custom templates. ([#1583](https://github.com/orhun/git-cliff/issues/1583))

  ```sh
  $ tree ~/my-templates
  ~/my-templates
  ├── company.toml
  └── minimal.toml

  $ git cliff --templates-dir ~/my-templates --init company

  $ git cliff --list-templates --templates-dir ~/my-templates
    azure-devops-keepachangelog
    cocogitto
    company
    ...
  ```

- **Templates from files:** you can now place your changelog body template in a file and load it with `--body-file`, making multiline templates easier to manage than passing them directly with `--body`. ([#1574](https://github.com/orhun/git-cliff/issues/1574))

  ```sh
  $ git cliff --body-file changelog-body.tera
  ```

- **Safer multi-value arguments:** path and commit options now consume one value per occurrence, preventing a trailing positional range from being mistaken for another option value. ([#1614](https://github.com/orhun/git-cliff/issues/1614))

  ```sh
  # Repeat the option for each path
  $ git cliff --include-path "src/**" --include-path "docs/**" v1.0.0..v2.0.0

  # Or pass path patterns as one quoted, space-delimited value
  $ git cliff --include-path "src/** docs/**" v1.0.0..v2.0.0

  # The positional range can also come first
  $ git cliff v1.0.0..v2.0.0 --include-path "src/**" --include-path "docs/**"
  ```

---

### 🧩 Templating

- **git-cliff** can now **format markdown!** ([#1610](https://github.com/orhun/git-cliff/issues/1610))

  It is opt-in and can be enabled via the `format` option:

  ```toml
  [changelog]
  format = true
  ```

  :::info

  Formatting normalizes headings, list markers and excessive blank lines. It supports GitHub-flavored Markdown features such as tables, strikethrough, task lists and footnotes and only runs for stdout, extension-less paths and `.md` output.

  Ambiguous bare brackets such as `[unreleased]` are escaped according to CommonMark rules.

  :::

- **New filters:** two new filters give you more control over how releases and commits are grouped:
  - [`commit_groups`](/docs/templating/syntax#custom-built-in-filters) preserves first appearance order or follows the order of `commit_parsers_groups`. ([#1518](https://github.com/orhun/git-cliff/issues/1518))

    :::tip

    If you were using numbered HTML comments to control the group order, you can now remove them:

    ```diff
    commit_parsers = [
    - { message = "^feat", group = "<!-- 0 -->Features" },
    - { message = "^fix", group = "<!-- 1 -->Bug Fixes" },
    + { message = "^feat", group = "Features" },
    + { message = "^fix", group = "Bug Fixes" },
    ]
    ```

    Then replace the `group_by` loop with `commit_groups(groups=commit_parsers_groups)`:

    ```diff
    -{% for group, commits in commits | group_by(attribute="group") %}
    -  ### {{ group | striptags | trim | upper_first }}
    -  {% for commit in commits %}- {{ commit.message }}
    +{% for entry in commits | commit_groups(groups=commit_parsers_groups) %}
    +  ### {{ entry.group | trim | upper_first }}
    +  {% for commit in entry.commits %}- {{ commit.message }}
      {% endfor %}
    {% endfor %}
    ```

    Each returned entry contains the group name in `entry.group` and its commits in `entry.commits`.

    `striptags` is no longer needed either.

    :::

  - [`group_by_scope`](/docs/templating/syntax#custom-built-in-filters) groups releases at a chosen semantic-version scope such as `major`, `minor`, or `patch`, with support for version prefixes. ([#1547](https://github.com/orhun/git-cliff/issues/1547))

    ```jinja2
    {% for version, releases in releases | group_by_scope(scope="minor", prefix="v") %}
      ## {{ version }}
      {% for release in releases %}
        - {{ release.version }}
      {% endfor %}
    {% endfor %}
    ```

    Releases `v0.1.0`, `v0.1.1` and `v0.2.0` render as:

    ```md
    ## v0.1

    - v0.1.1
    - v0.1.0

    ## v0.2

    - v0.2.0
    ```

- **New variables:** remote metadata now exposes more information for changelog templates:
  - `commit.remote.pr_author` contains the author of the matched pull request. ([#1613](https://github.com/orhun/git-cliff/issues/1613))

    ```json
    {
      "commits": [
        {
          "remote": {
            "username": "merge-maintainer",
            "pr_author": "pull-request-author",
            "pr_number": 42
          }
        }
      ]
    }
    ```

    :::info

    `pr_author` is more reliable than resolving the commit author's email for squash merges, where `username` can point to the maintainer who merged the change.

    :::

  - `[github|gitlab|etc].contributors[].pr_numbers` contains every pull request attributed to a contributor in the release, sorted by number. The existing `pr_number` field remains available for compatibility. ([#1546](https://github.com/orhun/git-cliff/issues/1546))

    ```json
    {
      "github": {
        "contributors": [
          {
            "username": "contributor",
            "pr_number": 42,
            "pr_numbers": [42, 57, 81]
          }
        ]
      }
    }
    ```

- **Built-in GitLab templates:** ([#1561](https://github.com/orhun/git-cliff/issues/1561))
  - `gitlab` generates concise release notes with merge request, contributor and tag links:

    ```sh
    $ git cliff --config gitlab
    ```

  - `gitlab-keepachangelog` generates a more detailed [Keep a Changelog](https://keepachangelog.com/) layout with GitLab commit and merge request links:

    ```sh
    $ git cliff --config gitlab-keepachangelog
    ```

  You can also use either preset as the starting point for your own configuration:

  ```sh
  $ git cliff --init gitlab-keepachangelog
  ```

---

### ⚙️ Configuration

- **Skip version bumps for selected commits:** different commit types can now be excluded from version bump calculations with [`no_increment_regex`](/docs/configuration/bump#no_increment_regex). ([#1522](https://github.com/orhun/git-cliff/issues/1522))

  ```toml
  [bump]
  no_increment_regex = "chore|ci|docs"
  ```

- **Configure remote request timeouts:** remote metadata requests now have a configurable [`http_timeout`](/docs/configuration/remote#http_timeout). ([#1580](https://github.com/orhun/git-cliff/issues/1580))

  ```toml
  [remote.github]
  http_timeout = "60s"
  ```

- **Better prepend support:** the new [`header_marker`](/docs/configuration/changelog#header_marker) tells **git-cliff** where the header ends, so it can remove the old header before writing the new one. ([#1603](https://github.com/orhun/git-cliff/issues/1603))

  ```toml
  [changelog]
  header = """
  # Changelog

  Tracked releases: {{ releases | length }}
  """
  header_marker = "<!-- git-cliff: end of header -->"
  ```

  Then prepend a new release as usual:

  ```sh
  $ git cliff --unreleased --prepend CHANGELOG.md
  ```

  The marker is written automatically after the rendered header:

  ```md
  # Changelog

  Tracked releases: 1

  <!-- git-cliff: end of header -->

  ## Unreleased
  ```

  On the next `--prepend`, **git-cliff** removes everything through the marker before writing the updated header.

- **Configuration schema:** the **git-cliff** configuration schema is now available on [SchemaStore](https://www.schemastore.org/git-cliff.json), enabling validation and autocompletion in supported editors. ([#1577](https://github.com/orhun/git-cliff/issues/1577))

  :::tip

  To select it explicitly in a [Taplo-compatible editor](https://taplo.tamasfe.dev/configuration/directives.html), add this directive at the top of your configuration:

  ```toml title="cliff.toml"
  #:schema https://www.schemastore.org/git-cliff.json
  ```

  :::

---

### 🌳 Git

- **Correct releases across merged branches:** commits are now assigned to releases using Git graph reachability. ([#1601](https://github.com/orhun/git-cliff/issues/1601))

  :::info[Why this is big?]

  Consider a feature branch that splits before `v1.0.0` but is merged afterward:

  ```text
            F---G
           /     \
  A---B---C---D---M  main
              |
            v1.0.0
  ```

  `F` and `G` are not part of `v1.0.0`. However, a flattened `git log` can interleave commits from both branches and make them look like they belong to that release.

  **git-cliff** now checks the actual commit graph instead. In this example, `F` and `G` stay under Unreleased until they are included in a later tag.

  This fixes changelogs that list changes under a release that never shipped them, omit those changes from Unreleased or disagree with the actual `previous_tag..tag` history. It is especially useful for repositories with long-lived release branches, backports or branches that are merged after a release.

  :::

- **Limit processed tags:** ([#1493](https://github.com/orhun/git-cliff/issues/1493))

  ```sh
  $ git cliff --limit-tags 10
  ```

  You can also set [`limit_tags`](/docs/configuration/git#limit_tags) in your configuration:

  ```toml
  [git]
  limit_tags = 10
  ```

- **Nested annotated tags:** tags that point to other annotated tags are peeled all the way to their commit and are no longer silently omitted. ([#1360](https://github.com/orhun/git-cliff/issues/1360))

- **Respect commits listed in `.git-blame-ignore-revs`:** they are now automatically excluded from the changelog. ([#1585](https://github.com/orhun/git-cliff/issues/1585))

  ```text title=".git-blame-ignore-revs"
  # Mass formatting
  67b8f240063d0d5b8f6c58be198d31e36fbf251a
  ```

---

### ⚡ Performance

Commit statistics are no longer calculated unless a template, parser, or `--context` output actually needs them. This avoids walking every diff for changelogs that do not use statistics, significantly reducing unnecessary work on larger repositories. ([#1543](https://github.com/orhun/git-cliff/issues/1543))

---

## ❤️ New Contributors

- [@ChrisJr404](https://github.com/ChrisJr404) made their first contribution in [#1610](https://github.com/orhun/git-cliff/pull/1610)
- [@tianrking](https://github.com/tianrking) made their first contribution in [#1603](https://github.com/orhun/git-cliff/pull/1603)
- [@lazizbekravshanov](https://github.com/lazizbekravshanov) made their first contribution in [#1584](https://github.com/orhun/git-cliff/pull/1584)
- [@jimisola](https://github.com/jimisola) made their first contribution in [#1613](https://github.com/orhun/git-cliff/pull/1613)
- [@Jonnobrow](https://github.com/Jonnobrow) made their first contribution in [#1601](https://github.com/orhun/git-cliff/pull/1601)
- [@YuriNachos](https://github.com/YuriNachos) made their first contribution in [#1605](https://github.com/orhun/git-cliff/pull/1605)
- [@hasezoey](https://github.com/hasezoey) made their first contribution in [#1609](https://github.com/orhun/git-cliff/pull/1609)
- [@artshmelev](https://github.com/artshmelev) made their first contribution in [#1597](https://github.com/orhun/git-cliff/pull/1597)
- [@Cyrus580529](https://github.com/Cyrus580529) made their first contribution in [#1594](https://github.com/orhun/git-cliff/pull/1594)
- [@Noai-oss](https://github.com/Noai-oss) made their first contribution in [#1587](https://github.com/orhun/git-cliff/pull/1587)
- [@nabsei](https://github.com/nabsei) made their first contribution in [#1585](https://github.com/orhun/git-cliff/pull/1585)
- [@JDanRibeiro](https://github.com/JDanRibeiro) made their first contribution in [#1561](https://github.com/orhun/git-cliff/pull/1561)
- [@arieleli01212](https://github.com/arieleli01212) made their first contribution in [#1546](https://github.com/orhun/git-cliff/pull/1546)
- [@ychampion](https://github.com/ychampion) made their first contribution in [#1574](https://github.com/orhun/git-cliff/pull/1574)
- [@CatBraaain](https://github.com/CatBraaain) made their first contribution in [#1549](https://github.com/orhun/git-cliff/pull/1549)
- [@SAY-5](https://github.com/SAY-5) made their first contribution
- [@GChernikov](https://github.com/GChernikov) made their first contribution in [#1360](https://github.com/orhun/git-cliff/pull/1360)
- [@signekb](https://github.com/signekb) made their first contribution in [#1527](https://github.com/orhun/git-cliff/pull/1527)
- [@guerda](https://github.com/guerda) made their first contribution in [#1526](https://github.com/orhun/git-cliff/pull/1526)

Any contribution is highly appreciated! See the [contribution guidelines](https://github.com/orhun/git-cliff/blob/main/CONTRIBUTING.md) for getting started.  
Feel free to [submit issues](https://github.com/orhun/git-cliff/issues/new/choose) and join our [Discord](https://discord.gg/W3mAwMDWH4) / [Matrix](https://matrix.to/#/#git-cliff:matrix.org) for discussion!  
Follow `git-cliff` on [X](https://x.com/git_cliff) & [Mastodon](https://fosstodon.org/@git_cliff) to not miss any news!

## Support 🌟

If you like `git-cliff`, consider:

- 💖 GitHub Sponsors: [@orhun](https://github.com/sponsors/orhun)
- ☕ Buy Me A Coffee: [https://www.buymeacoffee.com/orhun](https://www.buymeacoffee.com/orhun)

Have a fantastic day! ⛰️

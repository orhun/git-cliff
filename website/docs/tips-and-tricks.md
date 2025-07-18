---
sidebar_position: 6
---

# Tips And Tricks

## Commit and PR Strategy

### How should I write my commits?

We recommend using a [Git][1] history that follows the [Conventional Commits][2] specification as the primary strategy. For example:

```
fix(parser): handle empty commit messages gracefully
feat(cli): add support for --dry-run flag
refactor(core)!: change internal API to use async/await
```

**git-cliff**’s [default configuration][3] is built around this convention, making it easy to generate clear, structured, and consistent [Changelog][4]s by grouping commits (e.g., `feat`, `fix`, `docs`). The most important prefixes you should have in mind are:

* `fix:` which represents bug fixes, and correlates to a [SemVer][5] patch.
* `feat:` which represents a new feature, and correlates to a [SemVer][5] minor.
* `feat!:`,  or `fix!:`, `refactor!:`, etc., which represent a breaking change (indicated by the `!`) and will result in a [SemVer][5] major.

In addition to commit messages, **git-cliff** also supports parsing [remote metadata][6] from supported [Git][1] hosting services—such as pull request titles, numbers, and authors—using customizable regular expressions.

For example, [GitHub pull request labels can be used as grouping keys][7], allowing [Changelog][4]s to be organized based on custom [PR][8] label categories such as `breaking-change`, `type/enhancement`, or `area/documentation`.

### How should I manage PRs?

When working with a [PR][8]-based development flow, it’s important to adopt a merge strategy that preserves a clean and readable [Git][1] history—especially when [Changelog][4]s are generated from commit metadata.

We recommend using **squash merges** for integrating [PR][8]s into the main branch. This approach has several benefits:

* Linear history — [PR][8]s are merged as single commits, making the history easier to read and traverse.
* Easier bug tracking — Tools like **git bisect** become more effective with a linear history.
* Better compatibility with **git-cliff** — Since **git-cliff** generates [Changelog][4]s from commit messages, using **squash merges** helps ensure that each [PR][8] corresponds to a single, coherent commit. Other merge strategies, such as rebase merges or merge commits, may fail to consistently associate [PR][8]-level context (e.g., title, labels, issue references) with a single commit.

[1]: https://git-scm.com/
[2]: https://git-cliff.org/docs/configuration/git#conventional_commits
[3]: https://github.com/orhun/git-cliff/blob/main/config/cliff.toml
[4]: https://en.wikipedia.org/wiki/Changelog
[5]: https://semver.org/
[6]: https://git-cliff.org/docs/configuration/remote
[7]: https://git-cliff.org/docs/configuration/remote
[8]: https://en.wikipedia.org/wiki/Fork_and_pull_model

## Changing the group order

Since the groups come out in alphabetical order, use HTML comments to force them into their desired positions:

```toml
[git]
commit_parsers = [
    { message = "^feat*", group = "<!-- 0 -->:rocket: New features" },
    { message = "^fix*", group = "<!-- 1 -->:bug: Bug fixes" },
    { message = "^perf*", group = "<!-- 2 -->:zap: Performance" },
    { message = "^chore*", group = "<!-- 3 -->:gear: Miscellaneous" },
]
```

This produces the following order:

- 🚀 New features
- 🐛 Bug fixes
- ⚡ Performance
- ⚙️ Miscellaneous

Then strip the tags in the template with the series of filters:

```jinja2
### {{ group | striptags | trim | upper_first }}
```

## Discard duplicate commits

```jinja2
{% for commit in commits | unique(attribute="message") %}
```

## Filter merge commits

```jinja2
{% for group, commits in commits | filter(attribute="merge_commit", value=false) | group_by(attribute="group") %}
```

## Remove gitmoji

```toml
[git]
commit_preprocessors = [
  # Remove gitmoji, both actual UTF emoji and :emoji:
  { pattern = ' *(:\w+:|[\p{Emoji_Presentation}\p{Extended_Pictographic}](?:\u{FE0F})?\u{200D}?) *', replace = "" },
]
```

## Skip commits with an empty body

```toml
[git]
commit_parsers = [
  { body = "$^", skip = true },
]
```

## Skip commits by GitHub PR label

```jinja2
{% if commit.remote.pr_labels is containing("skip-release-notes") %}
    {% continue %}
{% endif %}
```

## Use GitHub PR labels as groups

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

## Use GitLab CI variables

```jinja2
{{ get_env(name="CI_PROJECT_URL") }}/-/tags/{{ version }}
```

## Convert markdown output to PDF

```bash
pandoc --from=gfm --to=pdf -o CHANGELOG.pdf CHANGELOG.md
```

To support unicode characters, use `xelatex` as PDF engine and a font family which includes the needed unicode characters:

```bash
pandoc --from=gfm --to=pdf --pdf-engine=xelatex -o CHANGELOG.pdf CHANGELOG.md --variable mainfont="Segoe UI Emoji"
```

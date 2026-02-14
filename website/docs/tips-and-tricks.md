---
sidebar_position: 6
---

# Tips And Tricks

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

## Handling remote Git service API rate limits

When retrieving information from a remote Git repository, you may encounter HTTP 403 errors due to rate limiting.

As a simple workaround, you can run `git-cliff` in offline mode to skip remote API calls:

```bash
git-cliff --offline
```

:::note

This will generate the changelog using only local Git commit information.
Note that PR titles, labels, and other remote metadata will not be included in offline mode.

:::

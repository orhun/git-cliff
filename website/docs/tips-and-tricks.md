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
{% for group, commits in commits | filter(attribute="merge_commit", value=false) %}
```

## Skip commits by PR label

```jinja2
{% if commit.github.pr_labels is containing("skip-release-notes") %}
    {% continue %}
{% endif %}
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

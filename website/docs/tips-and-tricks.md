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

## Use release statistics in your template

```jinja2
* {{ statistics.commit_count }} commit{% if statistics.commit_count != 1 %}s{% endif %} contributed to the release.
{%- if statistics.commits_timespan is defined %}
  * {{ statistics.commits_timespan }} day{% if statistics.commits_timespan != 1 %}s{% endif %} passed between the first and last commit.
{%- endif %}
* {{ statistics.conventional_commit_count }} commit{% if statistics.conventional_commit_count != 1 %}s{% endif %} {% if statistics.conventional_commit_count > 1 %}were{% else %}was{% endif %} understood as conventional.
* {{ statistics.link_counts | length }} issue{% if statistics.link_counts | length != 1 %}s{% endif %} like '(#ID)' {% if statistics.link_counts | length != 1 %}were{% else %}was{% endif %} seen in commit messages.
{%- if statistics.link_counts | length > 0 %}
  {%- for link in statistics.link_counts %}
    ** [{{ link.text }}]({{ link.href }}) ({{ link.count }} time{% if link.count != 1 %}s{% endif %} referenced)
  {%- endfor %}
{%- endif %}
{%- if statistics.days_passed_since_last_release is defined %}
  * {{ statistics.days_passed_since_last_release }} day{% if statistics.days_passed_since_last_release != 1 %}s{% endif %} passed between releases.
{%- endif %}
```

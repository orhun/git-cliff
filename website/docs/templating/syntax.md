---
sidebar_position: 2
---

# Syntax

**git-cliff** uses [Tera](https://github.com/Keats/tera) as the template engine. It has a syntax based on [Jinja2](http://jinja.pocoo.org/) and [Django](https://docs.djangoproject.com/en/3.1/topics/templates/) templates.

There are 3 kinds of delimiters and those cannot be changed:

<!-- {% raw %} -->

- `{{` and `}}` for expressions
- `{%` or `{%-` and `%}` or `-%}` for statements
- `{#` and `#}` for comments

<!-- {% endraw %} -->

See the [Tera Documentation](https://keats.github.io/tera/#templates) for more information about [control structures](https://keats.github.io/tera/#control-structures), [built-ins filters](https://keats.github.io/tera/#built-ins), etc.

## Custom built-in filters

**git-cliff** provides a few custom filters you can use inside templates:

- `upper_first`: Converts the first character of a string to uppercase.

  ```jinja
    {{ "hello" | upper_first }} →  Hello
  ```

- `find_regex`: Finds all occurrences of a regex pattern in a string.

  ```jinja
  {{ "hello world, hello universe" | find_regex(pat="hello") }} →  [hello, hello]
  ```

- `replace_regex`: Replaces all occurrences of a regex pattern with a string.

  ```jinja
  {{ "hello world" | replace_regex(from="o", to="a") }} →  hella warld
  ```

- `split_regex`: Splits a string by a regex pattern.

  ```jinja
  {{ "hello world, hello universe" | split_regex(pat=" ") }} →  [hello, world,, hello, universe]
  ```

- `commit_groups`: Groups commits by their `group` field while preserving a custom order.

  ```jinja
  {% for entry in commits | commit_groups(groups=commit_parsers_groups) %}
    ### {{ entry.group }}
  {% endfor %}
  ```

  The filter returns an array of objects like `{ group: "...", commits: [...] }`.

  When you use the `commit_parsers_groups` context field, the filter renders groups in the same order as the configured `commit_parsers` instead of sorting them alphabetically.

- `group_by_scope`: Groups releases by the semantic version scope (`major`, `minor`, or `patch`) of their `version` field.

  ```jinja
  {% for version, releases in releases | group_by_scope(scope="minor", prefix="v") %}
  {% set_global commits = [] %}
  {% for release in releases %}
    {% set_global commits = commits | concat(with=release.commits) %}
  {% endfor %}
    {% for group, commits in commits | group_by(attribute="group") %}
      ### {{ group }}
      {% for commit in commits %}
        - {{ commit.message }}
      {% endfor %}
    {% endfor %}
  {% endfor %}
  ```

  The filter returns an array of objects like `{ version: "...", releases: [...] }`.

  Set `prefix` for prefixed tags (for example, `prefix="v"`); otherwise, versions are parsed as-is. Unparsable versions are left unchanged.

  Use this in `header` or `footer`; `body` is rendered once per release and does not include the full `releases` array.

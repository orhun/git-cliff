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

See the [Tera Documentation](https://keats.github.io/tera/docs/#templates) for more information about [control structures](https://keats.github.io/tera/docs/#control-structures), [built-ins filters](https://keats.github.io/tera/docs/#built-ins), etc.

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

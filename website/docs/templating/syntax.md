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

Custom built-in filters that **git-cliff** uses:

- `upper_first`: Converts the first character of a string to uppercase.
```jinja
    {{ "hello world" | upper_first }} → Hello world
```

- `find_regex`: Finds all occurrences of a regex pattern in a string.
```jinja
    {{ "hello world, hello universe" | find_regex(pat="hello") }} → ["hello", "hello"]
```

- `replace_regex`: Replaces all occurrences of a regex pattern with a string.
```jinja
    {{ "hello world, hello universe" | replace_regex(from="hello", to="hi") }} → hi world, hi universe
```

- `split_regex`: Splits a string by a regex pattern.
```jinja
    {{ "one,two,three" | split_regex(pat=",") }} → ["one", "two", "three"]
```

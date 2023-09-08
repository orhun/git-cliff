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

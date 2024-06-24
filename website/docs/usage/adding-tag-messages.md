---
sidebar_position: 9
---

# Adding version (tag) message

Sometimes, you might want to include a special message or note related to a version of your project.
This can be used to highlight significant milestones, provide additional context, or share information not captured by individual commit messages.

There are currently 2 ways of doing this, in both ways, the message is available in the context of the template under the name `message`:

```
{% if message %}
    {{ message }}
{% endif %}\
```

## Using annotated tags

The recommended way of adding a version message is to add the message to the tag:

```bash
git tag v1.0.0 -m "first release, yay!"
```

So in the release's context, `message` will be "first release, yay!" (even if it is signed).

## Using `--with-tag-message`

If for some reason you don't want to have the message in the tag (or don't have a tag yet) but want to include it in the generated changelog, you can use the `--with-tag-message` flag:

```bash
git cliff --bump --unreleased --with-tag-message "some text"
```

In this case, you can only add a message to the latest release.

:::note

Please note that if you use `--with-tag-message` on a version it will ignore the original tag's message and use the one from the argument.

:::

---
sidebar_position: 3
---

# Gitea Integration 🍵

:::warning

This is still an experimental feature, please [report bugs](https://github.com/orhun/git-cliff/issues/new/choose).

:::

:::note

If you have built from source, enable the `gitea` feature flag for the integration to work.

:::

For projects hosted on Gitea/Forgejo, you can use **git-cliff** to add the following to your changelog:

- Gitea usernames
- Contributors list (all contributors / first time)
- Pull request links (associated with the commits)

## Setting up the remote

As default, remote upstream URL is automatically retrieved from the Git repository.

If that doesn't work or if you want to set a custom remote, there are a couple of ways of doing it:

- Use the [remote option](/docs/configuration/remote) in the configuration file:

```toml
[remote.gitea]
owner = "orhun"
repo = "git-cliff"
token = "***"
```

- Use the `--gitea-repo` argument (takes values in `OWNER/REPO` format, e.g. "orhun/git-cliff")

- Use the `GITEA_REPO` environment variable (same format as `--gitea-repo`)

## Authentication

:::tip

[Gitea REST API](https://gitea.com/api/swagger) is being used to retrieve data from Gitea.
It does not require authentication for public repositories. If your project uses a private
repository, you need to create an access token under *Settings* > *Applications* > *Access tokens*.

:::

To set an access token, you can use the [configuration file](/docs/configuration/remote) (not recommended), `--gitea-token` argument or `GITEA_TOKEN` environment variable.

For example:

```bash
GITEA_TOKEN="***" git cliff --gitea-repo "orhun/git-cliff"
```

:::tip

You can use the `GITEA_API_URL` environment variable want to override the API URL. This is useful if you are using your own Gitea instance.

:::

## Templating

:::tip

See the [templating documentation](/docs/category/templating) for general information about how the template engine works.

:::

### Remote

You can use the following [context](/docs/templating/context) for adding the remote to the changelog:

```json
{
  "gitea": {
    "owner": "orhun",
    "repo": "git-cliff"
  }
}
```

For example:

```jinja2
https://codeberg.org/{{ remote.gitea.owner }}/{{ remote.gitea.repo }}/commits/tag/{{ version }}
```

### Commit authors

For each commit, Gitea related values are added as a nested object (named `gitea`) to the [template context](/docs/templating/context):

```json
{
  "id": "8edec7fd50f703811d55f14a3c5f0fd02b43d9e7",
  "message": "refactor(config): remove unnecessary newline from configs\n",
  "group": "🚜 Refactor",

  "...": "<strip>",

  "remote": {
    "username": "orhun",
    "pr_title": "some things have changed",
    "pr_number": 420,
    "pr_labels": ["rust"],
    "is_first_time": false
  }
}
```

This can be used in the template as follows:

```
{% for commit in commits %}
  * {{ commit.message | split(pat="\n") | first | trim }}\
    {% if commit.remote.username %} by @{{ commit.remote.username }}{%- endif %}\
    {% if commit.remote.pr_number %} in #{{ commit.remote.pr_number }}{%- endif %}
{%- endfor -%}
```

The will result in:

```md
- feat(commit): add merge_commit flag to the context by @orhun in #389
- feat(args): set `CHANGELOG.md` as default missing value for output option by @sh-cho in #354
```

### Contributors

For each release, following contributors data is added to the [template context](/docs/templating/context) as a nested object:

```json
{
  "version": "v1.4.0",
  "commits": [],
  "commit_id": "0af9eb24888d1a8c9b2887fbe5427985582a0f26",
  "timestamp": 0,
  "previous": null,
  "gitea": {
    "contributors": [
      {
        "username": "orhun",
        "pr_title": "some things have changed",
        "pr_number": 420,
        "pr_labels": ["rust"],
        "is_first_time": true
      },
      {
        "username": "cliffjumper",
        "pr_title": "I love jumping",
        "pr_number": 999,
        "pr_labels": ["rust"],
        "is_first_time": true
      }
    ]
  }
}
```

This can be used in the template as follows:

```
{% for contributor in gitea.contributors | filter(attribute="is_first_time", value=true) %}
  * @{{ contributor.username }} made their first contribution in #{{ contributor.pr_number }}
{%- endfor -%}
```

The will result in:

```md
- @orhun made their first contribution in #420
- @cliffjumper made their first contribution in #999
```

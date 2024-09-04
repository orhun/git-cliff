---
sidebar_position: 1
---

# GitHub Integration 🐙

:::warning

This is still an experimental feature, please [report bugs](https://github.com/orhun/git-cliff/issues/new/choose).

:::

:::note

If you have built from source, enable the `github` feature flag for the integration to work.

:::

For projects hosted on GitHub, you can use **git-cliff** to add the following to your changelog:

- GitHub usernames
- Contributors list (all contributors / first time)
- Pull request links (associated with the commits)

And simply generate the same changelog that you can typically generate from the GitHub interface.

## Setting up the remote

As default, remote upstream URL is automatically retrieved from the Git repository.

If that doesn't work or if you want to set a custom remote, there are a couple of ways of doing it:

- Use the [remote option](/docs/configuration/remote) in the configuration file:

```toml
[remote.github]
owner = "orhun"
repo = "git-cliff"
token = "***"
```

- Use the `--github-repo` argument (takes values in `OWNER/REPO` format, e.g. "orhun/git-cliff")

- Use the `GITHUB_REPO` environment variable (same format as `--github-repo`)

## Authentication

[GitHub REST API](https://docs.github.com/en/rest) is being used to retrieve data from GitHub and it has the rate limit of _60 requests per hour_ for unauthenticated users.

Although this is enough for a couple of runs of **git-cliff**, it is suggested that you create an access token to increase the request limit.

:::tip

Follow [this guide](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens) for creating an access token. It can be either a classic or fine-grained token **without permissions**. Also, if you are running **git-cliff** in GitHub Actions, using `${{ secrets.GITHUB_TOKEN }}` is also enough.

:::

To set the access token, you can use the [configuration file](/docs/configuration/remote) (not recommended), `--github-token` argument or `GITHUB_TOKEN` environment variable.

For example:

```bash
GITHUB_TOKEN="***" git cliff --github-repo "orhun/git-cliff"
```

:::tip

You can use the [`GITHUB_API_URL`](https://docs.github.com/en/actions/learn-github-actions/variables) environment variable want to override the API URL. This is useful if you are using GitHub enterprise.

:::

## Templating

:::tip

See the [templating documentation](/docs/category/templating) for general information about how the template engine works.

:::

### Remote

You can use the following [context](/docs/templating/context) for adding the remote to the changelog:

```json
{
  "github": {
    "owner": "orhun",
    "repo": "git-cliff"
  }
}
```

For example:

```jinja2
https://github.com/{{ remote.github.owner }}/{{ remote.github.repo }}/compare/{{ previous.version }}...{{ version }}
```

### Commit authors

For each commit, GitHub related values are added as a nested object (named `github`) to the [template context](/docs/templating/context):

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
  "github": {
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
{% for contributor in github.contributors | filter(attribute="is_first_time", value=true) %}
  * @{{ contributor.username }} made their first contribution in #{{ contributor.pr_number }}
{%- endfor -%}
```

The will result in:

```md
- @orhun made their first contribution in #420
- @cliffjumper made their first contribution in #999
```

## GitHub Changelog

If you would like to create a changelog similar to the GitHub's default format, you can use the [`github.toml`](https://github.com/orhun/git-cliff/tree/main/examples/github.toml) example.

Since it is already embedded into the binary, you can simply run:

```bash
git cliff -c github
```

This will generate a changelog such as:

```md
## What's Changed

- feat(commit): add merge_commit flag to the context by @orhun in #389
- test(fixture): add test fixture for bumping version by @orhun in #360

## New Contributors

- @someone made their first contribution in #360
- @cliffjumper made their first contribution in #389

<!-- generated by git-cliff -->
```

Alternatively, you can use [`github-keepachangelog.toml`](https://github.com/orhun/git-cliff/tree/main/examples/github-keepachangelog.toml) template which is a mix of GitHub and [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) formats.

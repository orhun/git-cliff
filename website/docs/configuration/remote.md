# `remote`

This section contains the Git remote related configuration options.

```toml
[remote.github]
owner = "orhun"
repo = "git-cliff"
token = ""
```

:::tip

See the [GitHub integration](/docs/integration/github).

:::

Or if you are using GitLab:

```toml
[remote.gitlab]
owner = "orhun"
repo = "git-cliff"
token = ""
```

:::tip

See the [GitLab integration](/docs/integration/gitlab).

:::

### owner

Sets the owner (username) of the Git remote.

### repo

Sets the name of the repository.

If you are using GitHub, you can use the `--github-repo` argument or `GITHUB_REPO` environment variable.

e.g.

```bash
git cliff --github-repo orhun/git-cliff
```

Same applies for GitLab with `--gitlab-repo` and `GITLAB_REPO` environment variables.

### token

Sets the access token for the remote.

If you are using GitHub, then you can also pass this value via `--github-token` argument or `GITHUB_TOKEN` environment variable as follows:

```bash
git cliff --github-token <TOKEN>
```

Same applies for GitLab with `--gitlab-token` and `GITLAB_TOKEN` environment variables.

# `remote`

This section contains the Git remote related configuration options.

You can configure a remote for GitHub, GitLab, Gitea/Forgejo or Bitbucket as follows:

```toml
[remote.github]
owner = "orhun"
repo = "git-cliff"
token = ""
```

Change this to `remote.gitlab`, `remote.gitea` or `remote.bitbucket` accordingly to your project.

:::tip

- See the [GitHub integration](/docs/integration/github).
- See the [GitLab integration](/docs/integration/gitlab).
- See the [Gitea integration](/docs/integration/gitea).
- See the [Bitbucket integration](/docs/integration/bitbucket).

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

Same applies for GitLab/Bitbucket with `--gitlab-repo`/`--gitea-repo`/`--bitbucket-repo` and `GITLAB_REPO`/`GITEA_REPO`/`BITBUCKET_REPO` environment variables.

### token

Sets the access token for the remote.

If you are using GitHub, then you can also pass this value via `--github-token` argument or `GITHUB_TOKEN` environment variable as follows:

```bash
git cliff --github-token <TOKEN>
```

Same applies for GitLab/Bitbucket with `--gitlab-token`/`--gitea-token`/`--bitbucket-token` and `GITLAB_TOKEN`/`GITEA_TOKEN`/`BITBUCKET_TOKEN` environment variables.

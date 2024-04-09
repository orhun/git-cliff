# git-cliff-action

It is possible to generate changelogs using [GitHub Actions](https://github.com/features/actions) via [git-cliff-action](https://github.com/orhun/git-cliff-action).

```yml
- name: Check out repository
  uses: actions/checkout@v3
  with:
    fetch-depth: 0

- name: Generate a changelog
  uses: orhun/git-cliff-action@v3
  with:
    config: cliff.toml
    args: --verbose
  env:
    OUTPUT: CHANGELOG.md
    GITHUB_REPO: ${{ github.repository }}
```

See the [repository](https://github.com/orhun/git-cliff-action) for other [examples](https://github.com/orhun/git-cliff-action#examples).

Also, see the [continuous deployment workflow](https://github.com/orhun/git-cliff/tree/main/.github/workflows/cd.yml) of this project which sets the release notes for GitHub releases using this action.

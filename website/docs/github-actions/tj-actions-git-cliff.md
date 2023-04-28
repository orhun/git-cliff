# tj-actions/git-cliff

[tj-actions/git-cliff](https://github.com/tj-actions/git-cliff) is another GitHub Action that you can use to generate changelogs for your project.

It uses a generic `cliff-template.toml` without the need to maintain multiple configuration files for each project or you can optionally provide a customized template as a path or URL which falls back to the project's `cliff.toml` if it exists.

Additionally, it utilizes the `cliff-template.toml` and dynamically replaces values via [GitHub context object](https://docs.github.com/en/actions/learn-github-actions/contexts) and runs the [git-cliff-action](https://github.com/orhun/git-cliff-action) using a generated `cliff.toml`.

```yml
- name: Check out repository
  uses: actions/checkout@v3
  with:
    fetch-depth: 0

- name: Run git-cliff
  uses: tj-actions/git-cliff@v1
```

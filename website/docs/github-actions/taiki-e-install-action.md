# taiki-e/install-action

[taiki-e/install-action](https://github.com/taiki-e/install-action) enables a manual workflow
where `git-cliff` is automatically installed into your GitHub Actions environment from pre-built releases
and you can invoke it in subsequent shell script steps.

For example:

```yml
- name: Check out repository
  uses: actions/checkout@v3
  with:
    fetch-depth: 0

- name: Install git-cliff
  uses: taiki-e/install-action@git-cliff

- name: Generate changelog
  run: git-cliff
```

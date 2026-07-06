---
sidebar_position: 10
---

# GitLab CI/CD

It is possible to generate changelogs using [GitLab CI/CD](https://docs.gitlab.com/ee/ci/).

This minimal example generates a `CHANGELOG.md` file and stores it as an artifact for use by a release job.

```yml
- changelog:
    image:
      name: orhunp/git-cliff:latest
      entrypoint: [""]
    variables:
      GIT_STRATEGY: clone # clone entire repo instead of reusing workspace
      GIT_DEPTH: 0 # avoid shallow clone to give cliff all the info it needs
    stage: build
    script:
      - git-cliff -r . > CHANGELOG.md
    artifacts:
      paths:
        - CHANGELOG.md
```

If you're using a GitLab self-managed instance with corporate trust roots, some additional setup is required.

This example builds upon the previous by adding the GitLab server's TLS CA certificate to the Docker image's native certificate store before calling `git-cliff` with the `--use-native-tls` option.

```yml
changelog:
  image:
    name: orhunp/git-cliff:latest
    entrypoint: [""]
  variables:
    GIT_STRATEGY: clone # clone entire repo instead of reusing workspace
    GIT_DEPTH: 0 # avoid shallow clone to give cliff all the info it needs
  stage: build
  before_script:
    - mkdir -p /usr/local/share/ca-certificates/
    - cp "$CI_SERVER_TLS_CA_FILE" /usr/local/share/ca-certificates/gitlab-ca.crt
    - update-ca-certificates
  script:
    - git-cliff --use-native-tls -r . > CHANGELOG.md
  artifacts:
    paths:
      - CHANGELOG.md
```

:::info

This example assumes that your GitLab runners are configured with the [`tls-ca-file`](https://docs.gitlab.com/runner/configuration/advanced-configuration/#the-runners-section) setting.

:::

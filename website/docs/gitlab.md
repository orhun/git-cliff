---
sidebar_position: 10
---

# GitLab CI/CD

It is possible to generate changelogs using [GitLab CI/CD](https://docs.gitlab.com/ee/ci/).

This minimal example creates artifacts that can be used on another job.

```yml
- changelog:
    image:
      name: orhunp/git-cliff:latest
      entrypoint: [""]
    variables:
      GIT_STRATEGY: clone # clone entire repo instead of reusing workspace
      GIT_DEPTH: 0 # avoid shallow clone to give cliff all the info it needs
    stage: doc
    script:
      - git-cliff -r . > CHANGELOG.md
    artifacts:
      paths:
        - CHANGELOG.md
```

Please note that the stage is `doc` and has to be changed accordingly to your need.

---
sidebar_position: 11
---

# Sourcehut Builds

It is possible to generate changelogs using [SourceHut builds](https://builds.sr.ht).

```yaml
image: alpine/edge
packages:
  - git-cliff
secrets:
  - <your-builds.sr.ht-secret>
sources:
  - git://git@git.sr.ht:~<username>/<repo-name>
environment:
  dir: <repo-name>
  source: <your-source>
tasks:
  - git-cliff: |
      cd $dir
      git cliff -o CHANGELOG.md
      ssh-keyscan -t rsa git.sr.ht >> ~/.ssh/known_hosts
      git remote set-url origin $source
      git checkout main
      git add CHANGELOG.md
      git commit -m "chore(release): Update CHANGELOG"
      git push -o skip-ci
```

1. Generate a new SSH-key for SourceHut builds:
   `ssh-keygen -t ed25519 -C "builds.sr.ht" -f ~/.ssh/builds-srht`
2. Add the newly generated public key to your
   [SourceHut account](https://meta.sr.ht/keys).
3. Add the private key as a secret to your
   [Sourcehut Builds](https://builds.sr.ht/secrets).
4. Replace all the placeholders in your `.build.yml`:
   - `<your-builds.sr.ht-secret>`
   - `<username>`
   - `<repo-name>`
   - `<your-source>`

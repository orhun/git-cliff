# Creating a Release

[GitHub](https://github.com/orhun/git-cliff/releases), [crates.io](https://crates.io/crates/git-cliff/) and [Docker Hub](https://hub.docker.com/repository/docker/orhunp/git-cliff) releases are automated via [GitHub actions](./.github/workflows/cd.yml) and triggered by pushing a tag.

1. Run the [release script](./release.sh): `./release.sh v[X.Y.Z]`
2. Push the changes: `git push`
3. Check if [Continuous Integration](https://github.com/orhun/git-cliff/actions) workflow is completed successfully.
4. Push the tags: `git push --tags`
5. Wait for [Continuous Deployment](https://github.com/orhun/git-cliff/actions) workflow to finish.
   - Do not forget to set `vars.USE_TESTPYPI` variable to `false` before release.

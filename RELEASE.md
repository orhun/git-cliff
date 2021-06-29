# Creating a Release

[GitHub](https://github.com/orhun/git-cliff/releases), [crates.io](https://crates.io/crates/git-cliff/) and [Docker Hub](https://hub.docker.com/repository/docker/orhunp/git-cliff) releases are automated via [GitHub actions](./.github/workflows/cd.yml) and triggered by pushing a tag.

1. Bump the version in [Cargo.toml](./Cargo.toml) according to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
2. Update [Cargo.lock](./Cargo.lock) by building the project via `cargo build`.
3. Run the [release script](./release.sh): `./release.sh v[X.Y.Z]`
4. Push the changes: `git push`
5. Check if [Continuous Integration](https://github.com/orhun/git-cliff/actions) workflow is completed successfully.
6. Push the tags: `git push --tags`
7. Wait for [Continuous Deployment](https://github.com/orhun/git-cliff/actions) workflow to finish.

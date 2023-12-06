# Fixtures

[Test fixtures](https://en.wikipedia.org/wiki/Test_fixture) are a way of testing the `git-cliff`'s functionality against a configuration file (`cliff.toml`) and asserting the output (`expected.md`).

They are being [run](https://github.com/orhun/git-cliff/actions/workflows/test-fixtures.yml) as a part of the GitHub Actions and the workflow file can be viewed [here](../workflows/test-fixtures.yml).

## Adding new fixtures

1. Copy the `new-fixture-template` as `test-<functionality>`.
2. Update the files accordingly to the function that is being tested.
3. Run the fixture locally. (see below)
4. Add the fixture to [`test-fixtures.yml`](../workflows/test-fixtures.yml) matrix.
   - If you need to specify custom arguments to `git-cliff` for the fixture test, you can do it here with using `command`.
5. Commit your changes.

See [this commit](https://github.com/orhun/git-cliff/commit/c94cb6a37ae268953ab29dd35cb43b6a4fec47cc) as an example.

## Running locally

To run the fixtures locally:

```sh
./test-fixtures-locally.sh <fixture_name>
```

## See also

- [Contribution Guidelines](https://github.com/orhun/git-cliff/blob/main/CONTRIBUTING.md)

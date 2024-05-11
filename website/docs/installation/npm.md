# NPM

You can install and run **git-cliff** from [NPM](https://www.npmjs.com/package/git-cliff) with a single command:

```bash
npx git-cliff@latest
```

Also, if you want to add **git-cliff** to your project:

```bash
# with yarn
yarn add -D git-cliff

# with npm
npm install git-cliff --save-dev
```

Afterwards, you can run **git-cliff** via `npm exec git-cliff` or `npx git-cliff@latest`.

## Programmatic API

**git-cliff** also provides a fully typed programmatic API. You can use it to integrate **git-cliff** into your own tooling.

```typescript
import { runGitCliff, type Options } from "git-cliff";

const options: Options = {
  // ...
};

runGitCliff(options);
```

Under the hood this will parse the options, set up the environment and call the **git-cliff** binary for you.

## Supported Node.js Versions

The following minimum versions of Node are currently supported:

- `>=18.19`
- `>=20.6`
- `>=21`

## Supported Platforms

NPM packages are distributed for the following platforms:

- Linux (x64, arm64)
- Windows (x64, arm64)
- macOS (x64, arm64)

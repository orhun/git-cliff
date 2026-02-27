'use strict';

var execa = require('execa');
var url = require('url');
var os = require('os');

var __defProp = Object.defineProperty;
var __name = (target, value) => __defProp(target, "name", { value, configurable: true });
async function getExePath() {
  const platform = os.platform();
  const arch = os.arch();
  let os$1 = platform;
  let extension = "";
  if (platform === "win32" || platform === "cygwin") {
    os$1 = "windows";
    extension = ".exe";
  }
  try {
    return undefined(
      `git-cliff-${os$1}-${arch}/bin/git-cliff${extension}`
    );
  } catch (e) {
    throw new Error(
      `Couldn't find git-cliff binary inside node_modules for ${os$1}-${arch} (${e})`
    );
  }
}
__name(getExePath, "getExePath");

// src/optionsToStringArgs.ts
function optionsToStringArgs(options) {
  const args = [];
  for (const [key, value] of Object.entries(options)) {
    const hyphenCaseKey = key.replace(/([A-Z])/g, "-$1").toLowerCase();
    if (Array.isArray(value)) {
      for (const arrValue of value) {
        args.push(`--${hyphenCaseKey}`, arrValue);
      }
    } else if (value === true) {
      args.push(`--${hyphenCaseKey}`);
    } else if (value === false || value === null) {
      continue;
    } else {
      args.push(`--${hyphenCaseKey}`, value);
    }
  }
  return args;
}
__name(optionsToStringArgs, "optionsToStringArgs");

// src/index.ts
async function runGitCliff(argsOrOptions, execaOptions) {
  const exePath = await getExePath();
  const args = Array.isArray(argsOrOptions) ? argsOrOptions : optionsToStringArgs(argsOrOptions);
  return execa.execa(url.fileURLToPath(exePath), args, {
    stdio: "inherit",
    ...execaOptions
  });
}
__name(runGitCliff, "runGitCliff");

exports.runGitCliff = runGitCliff;
//# sourceMappingURL=index.cjs.map
//# sourceMappingURL=index.cjs.map
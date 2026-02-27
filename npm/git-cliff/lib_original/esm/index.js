import { execa } from 'execa';
import { fileURLToPath } from 'url';
import { platform, arch } from 'os';

var __defProp = Object.defineProperty;
var __name = (target, value) => __defProp(target, "name", { value, configurable: true });
async function getExePath() {
  const platform$1 = platform();
  const arch$1 = arch();
  let os = platform$1;
  let extension = "";
  if (platform$1 === "win32" || platform$1 === "cygwin") {
    os = "windows";
    extension = ".exe";
  }
  try {
    return import.meta.resolve(
      `git-cliff-${os}-${arch$1}/bin/git-cliff${extension}`
    );
  } catch (e) {
    throw new Error(
      `Couldn't find git-cliff binary inside node_modules for ${os}-${arch$1} (${e})`
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
  return execa(fileURLToPath(exePath), args, {
    stdio: "inherit",
    ...execaOptions
  });
}
__name(runGitCliff, "runGitCliff");

export { runGitCliff };
//# sourceMappingURL=index.js.map
//# sourceMappingURL=index.js.map
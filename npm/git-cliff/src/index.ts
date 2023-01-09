#!/usr/bin/env node

import { spawnSync } from "child_process"

/**
  * Returns the executable path for git-cliff located inside node_modules
  * The naming convention is git-cliff-${os}-${arch}
  * If the platform is `win32` or `cygwin`, executable will include a `.exe` extension
  * @see https://nodejs.org/api/os.html#osarch
  * @see https://nodejs.org/api/os.html#osplatform
  * @example "x/xx/node_modules/git-cliff-darwin-arm64"
  */
function getExePath() {
  const arch = process.arch;
  let os = process.platform as string;
  let extension = '';
  if (['win32', 'cygwin'].includes(process.platform)) {
    os = 'windows';
    extension = '.exe';
  }

  try {
    // Since the bin will be located inside `node_modules`, we can simply call require.resolve
    return require.resolve(`git-cliff-${os}-${arch}/bin/git-cliff${extension}`)
  } catch (e) {
    throw new Error(`Couldn't find git-cliff binary inside node_modules for ${os}-${arch}`)
  }
}

/**
  * Runs `git-cliff` with args using nodejs spawn
  */
function runGitCliff() {
  const args = process.argv.slice(2)
  const processResult = spawnSync(getExePath(), args, { stdio: "inherit" })
  process.exit(processResult.status ?? 0)
}

runGitCliff()

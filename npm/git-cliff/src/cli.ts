#!/usr/bin/env node

import { runGitCliff } from "./index.js";

async function run() {
  const args = process.argv.slice(2);
  const processResult = await runGitCliff(args);

  process.exit(processResult.exitCode ?? 0);
}

void run();

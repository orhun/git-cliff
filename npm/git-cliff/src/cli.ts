#!/usr/bin/env node

import { runGitCliff } from "./index.js";

const args = process.argv.slice(2);

void runGitCliff(args);

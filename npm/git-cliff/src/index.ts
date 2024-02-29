import { execa } from "execa";
import { fileURLToPath } from "node:url";
import { getExePath } from "./getExePath.js";
import type { Options } from "./options.js";
import { optionsToStringArgs } from "./optionsToStringArgs.js";

export type { Options } from "./options.js";

/**
 * Runs `git-cliff` with the provided options as a JavaScript object.
 *
 * @param options - The options to pass to `git-cliff`.
 * These get transformed into an array strings.
 * - Values that are `true` will be passed as flags (`--flag`).
 * - Values that are `false` or `null` will be ignored.
 * - All other values will be passed as options (`--option value`).
 */
export async function runGitCliff(options: Options): Promise<void>;
/**
 * Runs the `git-cliff` with the provided arguments.
 *
 * @param args - The arguments to pass to `git-cliff`.
 * These should be in an array of string format.
 * Every option and their value should be its own entry in the array.
 *
 * @returns A promise that resolves when the `git-cliff` has finished running.
 *
 * @example
 * Options with values
 * ```typescript
 * await runGitCliff(["--tag", "1.0.0", "--config", "github"]);
 * ```
 *
 * @example
 * Boolean flags
 * ```typescript
 * await runGitCliff(["--unreleased", "--topo-order"]);
 * ```
 *
 * @example
 * Combining options and flags
 * ```typescript
 * await runGitCliff(["--tag", "1.0.0", "--config", "github", "--topo-order"]);
 * ```
 */
export async function runGitCliff(args: string[]): Promise<void>;
export async function runGitCliff(argsOrOptions: Options | string[]) {
  const exePath = await getExePath();
  const args = Array.isArray(argsOrOptions)
    ? argsOrOptions
    : optionsToStringArgs(argsOrOptions);

  const processResult = await execa(fileURLToPath(exePath), args, {
    stdio: "inherit",
  });
  process.exit(processResult.exitCode ?? 0);
}

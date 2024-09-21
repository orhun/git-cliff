import type { Options } from "./options.js";

/**
 * Transforms a JavaScript object of options into an array
 * of strings that can be passed to {@link execa} for calling `git-cliff`
 *
 * @param options The options to transform
 * @returns The options as an array of strings
 */
export function optionsToStringArgs(options: Options): string[] {
  const args: string[] = [];

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

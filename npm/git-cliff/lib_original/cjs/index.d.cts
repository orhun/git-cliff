import { Options as Options$1, ResultPromise } from 'execa';

type Options = Partial<{
    /** Prints help information */
    help: boolean;
    /** Prints version information */
    version: boolean;
    /** Increases the logging verbosity */
    verbose: boolean;
    /** Prints bumped version for unreleased changes */
    bumpedVersion: boolean;
    /** Processes the commits starting from the latest tag */
    latest: boolean;
    /** Processes the commits that belong to the current tag */
    current: boolean;
    /** Processes the commits that do not belong to a tag */
    unreleased: boolean;
    /** Sorts the tags topologically */
    topoOrder: boolean;
    /** Include only the tags that belong to the current branch */
    useBranchTags: boolean;
    /** Disables the external command execution */
    noExec: boolean;
    /** Prints changelog context as JSON */
    context: boolean;
    /** Writes the default configuration file to cliff.toml */
    init: boolean | string;
    /**
     * Bumps the version for unreleased changes
     * @default 'auto'
     */
    bump: "auto" | "major" | "minor" | "patch";
    /**
     * Sets the configuration file
     * @default 'cliff.toml'
     */
    config: string;
    /** Sets the working directory */
    workdir: string;
    /** Sets the git repository */
    repository: string;
    /** Sets the path to include related commits */
    includePath: string;
    /** Sets the path to exclude related commits */
    excludePath: string;
    /** Sets the regex for matching git tags */
    tagPattern: string;
    /** Sets custom commit messages to include in the changelog */
    withCommit: string;
    /** Sets custom message for the latest release */
    withTagMessage: string;
    /** Sets the tags to ignore in the changelog */
    ignoreTags: string | string[];
    /** Sets the tags to count in the changelog */
    countTags: string | string[];
    /** Sets commits that will be skipped in the changelog */
    skipCommit: string | string[];
    /** Prepends entries to the given changelog file */
    prepend: string;
    /** Writes output to the given file */
    output: string;
    /** Sets the tag for the latest version */
    tag: string;
    /** Sets the template for the changelog body */
    body: string;
    /** Generates changelog from a JSON context */
    fromContext: string;
    /** Strips the given parts from the changelog */
    strip: "header" | "footer" | "all";
    /**
     * Sets sorting of the commits inside sections
     * @default 'oldest'
     */
    sort: "oldest" | "newest";
    /** Sets the GitHub API token */
    githubToken: string;
    /** Sets the GitHub repository */
    githubRepo: string;
    /** Sets the GitLab API token */
    gitlabToken: string;
    /** Sets the GitLab repository */
    gitlabRepo: string;
    /** Sets the Gitea API token */
    giteaToken: string;
    /** Sets the Gitea repository */
    giteaRepo: string;
    /** Sets the Bitbucket API token */
    bitbucketToken: string;
    /** Sets the Bitbucket repository */
    bitbucketRepo: string;
}>;

/**
 * Runs `git-cliff` with the provided options as a JavaScript object.
 *
 * @param options - The options to pass to `git-cliff`.
 * These get transformed into an array strings.
 * - Values that are `true` will be passed as flags (`--flag`).
 * - Values that are `false` or `null` will be ignored.
 * - All other values will be passed as options (`--option value`).
 *
 * @param execaOptions - Options to pass to {@link execa}.
 */
declare function runGitCliff(options: Options, execaOptions?: Options$1): Promise<ResultPromise<Options$1>>;
/**
 * Runs the `git-cliff` with the provided arguments.
 *
 * @param args - The arguments to pass to `git-cliff`.
 * These should be in an array of string format.
 * Every option and their value should be its own entry in the array.
 *
 * @param execaOptions - Options to pass to {@link execa}.
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
declare function runGitCliff(args: string[], execaOptions?: Options$1): Promise<ResultPromise<Options$1>>;

export { type Options, runGitCliff };

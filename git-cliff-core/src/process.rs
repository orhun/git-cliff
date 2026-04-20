use crate::commit::Commit;
use crate::config::{GitConfig, ProcessingStep};
use crate::error::{Error as AppError, Result};
use crate::summary::Summary;

/// Processes commits using either legacy behavior or a configured processing order.
pub fn process_commit_list<'a>(
    commits: &mut Vec<Commit<'a>>,
    git_config: &GitConfig,
    summary: &mut Summary,
) -> Result<()> {
    CommitProcessor::new(git_config, summary).run(commits)
}

/// Stateful commit-processing pipeline.
pub struct CommitProcessor<'cfg, 'sum> {
    config: &'cfg GitConfig,
    summary: &'sum mut Summary,
}

impl<'cfg, 'sum> CommitProcessor<'cfg, 'sum> {
    /// Creates a processor bound to config and summary output.
    #[must_use]
    pub fn new(config: &'cfg GitConfig, summary: &'sum mut Summary) -> Self {
        Self { config, summary }
    }

    /// Runs commit processing and final validation checks.
    pub fn run<'a>(&mut self, commits: &mut Vec<Commit<'a>>) -> Result<()> {
        if let Some(order) = &self.config.processing_order {
            self.run_with_order(commits, order);
        } else {
            self.run_legacy(commits);
        }

        if self.config.require_conventional {
            self.check_conventional_commits(commits)?;
        }
        if self.config.fail_on_unmatched_commit {
            self.check_unmatched_commits(commits)?;
        }

        Ok(())
    }

    /// Applies commit processing steps in the configured linear order.
    fn run_with_order<'a>(&mut self, commits: &mut Vec<Commit<'a>>, order: &[ProcessingStep]) {
        for step in order {
            match step {
                ProcessingStep::CommitPreprocessors => self.apply_commit_preprocessors(commits),
                ProcessingStep::SplitCommits => self.apply_split_commits(commits),
                ProcessingStep::ConventionalCommits => self.apply_conventional_commits(commits),
                ProcessingStep::CommitParsers => self.apply_commit_parsers(commits),
                ProcessingStep::LinkParsers => self.apply_link_parsers(commits),
            }
        }
    }

    /// Preserves the historical non-linear processing flow for compatibility.
    fn run_legacy<'a>(&mut self, commits: &mut Vec<Commit<'a>>) {
        let mut processed = Vec::new();
        for commit in commits.iter() {
            if let Some(commit) = self.process_single_commit(commit) {
                if self.config.split_commits {
                    for line in commit.message.lines() {
                        let mut split_commit = commit.clone();
                        split_commit.message = line.to_string();
                        split_commit.links.clear();
                        if split_commit.message.is_empty() {
                            continue;
                        }
                        if let Some(split_commit) = self.process_single_commit(&split_commit) {
                            processed.push(split_commit);
                        }
                    }
                } else {
                    processed.push(commit);
                }
            }
        }
        *commits = processed;
    }

    /// Applies commit preprocessors to all commits.
    fn apply_commit_preprocessors<'a>(&mut self, commits: &mut Vec<Commit<'a>>) {
        let mut processed = Vec::new();
        for commit in commits.iter() {
            match commit.clone().preprocess(&self.config.commit_preprocessors) {
                Ok(commit) => {
                    self.summary.record_ok();
                    processed.push(commit);
                }
                Err(error) => {
                    self.summary.record_err(&error);
                    self.on_processing_error(commit, &error);
                }
            }
        }
        *commits = processed;
    }

    /// Splits commit messages by line when `split_commits` is enabled.
    fn apply_split_commits<'a>(&mut self, commits: &mut Vec<Commit<'a>>) {
        if !self.config.split_commits {
            return;
        }
        let mut split_commits = Vec::new();
        for commit in commits.iter() {
            for line in commit.message.lines() {
                if line.is_empty() {
                    continue;
                }
                let mut split_commit = commit.clone();
                split_commit.message = line.to_string();
                split_commit.links.clear();
                split_commits.push(split_commit);
            }
        }
        *commits = split_commits;
    }

    /// Parses commits as conventional according to current config rules.
    fn apply_conventional_commits<'a>(&mut self, commits: &mut Vec<Commit<'a>>) {
        let mut processed = Vec::new();
        for commit in commits.iter() {
            if !self.config.conventional_commits {
                self.summary.record_ok();
                processed.push(commit.clone());
                continue;
            }

            if !self.config.require_conventional &&
                self.config.filter_unconventional &&
                !self.config.split_commits
            {
                match commit.clone().into_conventional() {
                    Ok(commit) => {
                        self.summary.record_ok();
                        processed.push(commit);
                    }
                    Err(error) => {
                        self.summary.record_err(&error);
                        self.on_processing_error(commit, &error);
                    }
                }
            } else {
                match commit.clone().into_conventional() {
                    Ok(commit) => {
                        self.summary.record_ok();
                        processed.push(commit);
                    }
                    Err(_) => {
                        self.summary.record_ok();
                        processed.push(commit.clone());
                    }
                }
            }
        }
        *commits = processed;
    }

    /// Applies commit parsers for grouping/filtering.
    fn apply_commit_parsers<'a>(&mut self, commits: &mut Vec<Commit<'a>>) {
        let mut processed = Vec::new();
        for commit in commits.iter() {
            match commit.clone().parse(
                &self.config.commit_parsers,
                self.config.protect_breaking_commits,
                self.config.filter_commits,
            ) {
                Ok(commit) => {
                    self.summary.record_ok();
                    processed.push(commit);
                }
                Err(error) => {
                    self.summary.record_err(&error);
                    self.on_processing_error(commit, &error);
                }
            }
        }
        *commits = processed;
    }

    /// Applies link parsers without filtering commits.
    fn apply_link_parsers<'a>(&mut self, commits: &mut Vec<Commit<'a>>) {
        let mut processed = Vec::new();
        for commit in commits.iter() {
            processed.push(commit.clone().parse_links(&self.config.link_parsers));
        }
        *commits = processed;
    }

    /// Processes one commit with the legacy single-pass pipeline.
    fn process_single_commit<'a>(&mut self, commit: &Commit<'a>) -> Option<Commit<'a>> {
        match commit.process(self.config) {
            Ok(commit) => {
                self.summary.record_ok();
                Some(commit)
            }
            Err(error) => {
                self.summary.record_err(&error);
                self.on_processing_error(commit, &error);
                None
            }
        }
    }

    /// Validates that all processed commits are conventional.
    fn check_conventional_commits(&self, commits: &[Commit<'_>]) -> Result<()> {
        log::debug!("Verifying that all commits are conventional");
        let mut unconventional_count = 0;
        commits.iter().for_each(|commit| {
            if commit.conv.is_none() {
                log::error!(
                    "Commit {id} is not conventional:\n{message}",
                    id = &commit.id[..7],
                    message = commit
                        .message
                        .lines()
                        .map(|line| { format!("    | {}", line.trim()) })
                        .collect::<Vec<String>>()
                        .join("\n")
                );
                unconventional_count += 1;
            }
        });

        if unconventional_count > 0 {
            return Err(AppError::UnconventionalCommitsError(unconventional_count));
        }
        Ok(())
    }

    /// Validates that all processed commits matched at least one parser.
    fn check_unmatched_commits(&self, commits: &[Commit<'_>]) -> Result<()> {
        log::debug!("Verifying that no commits are unmatched by commit parsers");
        let mut unmatched_count = 0;
        commits.iter().for_each(|commit| {
            let is_unmatched = commit.group.is_none();
            if is_unmatched {
                log::error!(
                    "Commit {id} was not matched by any commit parser:\n{message}",
                    id = &commit.id[..7],
                    message = commit
                        .message
                        .lines()
                        .map(|line| { format!("    | {}", line.trim()) })
                        .collect::<Vec<String>>()
                        .join("\n")
                );
                unmatched_count += 1;
            }
        });

        if unmatched_count > 0 {
            return Err(AppError::UnmatchedCommitsError(unmatched_count));
        }
        Ok(())
    }

    /// Emits a trace log entry for a commit-processing failure.
    fn on_processing_error(&self, commit: &Commit<'_>, error: &AppError) {
        let short_id = commit.id.chars().take(7).collect::<String>();
        let summary = commit.message.lines().next().unwrap_or_default().trim();
        log::trace!("{short_id} - {error} ({summary})");
    }
}

//! End-to-end benchmarks for generating a changelog from a git repository.
//!
//! The repositories are generated at startup with a fixed shape (linear
//! history, a tag every [`COMMITS_PER_TAG`] commits) so the results are
//! reproducible and nothing large has to be committed.
//!
//! Run with `cargo bench -p git-cliff --bench changelog`.

use std::io;
use std::path::Path;

use clap::Parser;
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use git_cliff::args::Opt;
use git2::{Oid, Repository, Signature, Time};
use temp_dir::TempDir;

/// Number of commits between two consecutive tags.
const COMMITS_PER_TAG: usize = 10;

/// Tag counts to benchmark. The largest one keeps an iteration within a
/// couple of seconds while still exposing super-linear scaling in the
/// number of tags.
const TAG_COUNTS: &[usize] = &[10, 50, 200];

/// Commit messages cycled through when generating the history.
const MESSAGES: &[&str] = &[
    "feat(parser): support parsing arrays",
    "fix(changelog): handle empty releases\n\nCloses #12",
    "docs: update the configuration guide",
    "refactor(repo)!: simplify the tag lookup\n\nBREAKING CHANGE: `tags` now takes a pattern",
    "perf(template): cache compiled templates",
    "chore(deps): bump dependencies",
    "test(config): cover the default values",
    "style: apply formatting",
    "Merge pull request #34 from feature/branch",
    "fix: correct a typo in the output",
];

/// Creates a repository in `dir` with `tags * COMMITS_PER_TAG` commits on a
/// linear history and a lightweight tag on every [`COMMITS_PER_TAG`]th commit.
fn generate_repository(dir: &Path, tags: usize) -> Result<(), git2::Error> {
    let repo = Repository::init(dir)?;
    let tree_id = repo.index()?.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let mut parent: Option<Oid> = None;
    for i in 1..=tags * COMMITS_PER_TAG {
        let time = Time::new(1_600_000_000 + i64::try_from(i).unwrap_or_default() * 60, 0);
        let signature = Signature::new("git-cliff", "bench@git-cliff.org", &time)?;
        let parents = parent
            .map(|id| repo.find_commit(id))
            .transpose()?
            .into_iter()
            .collect::<Vec<_>>();
        let parents = parents.iter().collect::<Vec<_>>();
        let message = MESSAGES[i % MESSAGES.len()];
        let id = repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &parents,
        )?;
        if i % COMMITS_PER_TAG == 0 {
            let target = repo.find_object(id, None)?;
            repo.tag_lightweight(&format!("v0.{}.0", i / COMMITS_PER_TAG), &target, false)?;
        }
        parent = Some(id);
    }
    Ok(())
}

/// Benchmarks running git-cliff against repositories with a growing number of
/// tags, from reading the history to rendering the changelog.
fn bench_changelog(c: &mut Criterion) {
    let mut group = c.benchmark_group("changelog");
    group.sample_size(10);
    for &tags in TAG_COUNTS {
        let dir = TempDir::new().expect("failed to create a temporary directory");
        generate_repository(dir.path(), tags).expect("failed to generate the repository");
        let repository = dir.path().to_string_lossy().into_owned();
        group.throughput(Throughput::Elements((tags * COMMITS_PER_TAG) as u64));
        group.bench_with_input(
            BenchmarkId::new("tags", tags),
            &repository,
            |b, repository| {
                b.iter(|| {
                    let args = Opt::parse_from([
                        "git-cliff",
                        "--repository",
                        repository,
                        "--config",
                        "keepachangelog",
                    ]);
                    let changelog = git_cliff::run(args.clone()).expect("failed to run git-cliff");
                    git_cliff::write_changelog(&args, changelog, io::sink())
                        .expect("failed to write the changelog");
                });
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_changelog);
criterion_main!(benches);

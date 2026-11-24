//! Benchmarks for processing commits and rendering the changelog, without
//! touching a git repository.
//!
//! Run with `cargo bench -p git-cliff-core --bench changelog`.

use std::hint::black_box;
use std::io;

use criterion::{BatchSize, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use git_cliff_core::changelog::Changelog;
use git_cliff_core::commit::Commit;
use git_cliff_core::config::Config;
use git_cliff_core::embed::BuiltinConfig;
use git_cliff_core::release::Release;

/// Number of commits in each release.
const COMMITS_PER_RELEASE: usize = 10;

/// Release counts to benchmark.
const RELEASE_COUNTS: &[usize] = &[10, 100, 1000];

/// Commit messages cycled through when generating the releases.
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

/// Returns `count` releases, oldest first, each with
/// [`COMMITS_PER_RELEASE`] unprocessed commits.
fn releases(count: usize) -> Vec<Release<'static>> {
    (0..count)
        .map(|index| Release {
            version: Some(format!("v0.{index}.0")),
            commits: (0..COMMITS_PER_RELEASE)
                .map(|offset| {
                    let n = index * COMMITS_PER_RELEASE + offset;
                    Commit::new(
                        format!("{n:040x}"),
                        MESSAGES[n % MESSAGES.len()].to_string(),
                    )
                })
                .collect(),
            commit_id: Some(format!("{:040x}", (index + 1) * COMMITS_PER_RELEASE - 1)),
            timestamp: Some(1_600_000_000 + i64::try_from(index).unwrap_or_default() * 86_400),
            ..Release::default()
        })
        .collect()
}

/// Loads one of the built-in configuration files.
fn config(name: &str) -> Config {
    BuiltinConfig::parse(name.to_string())
        .expect("failed to parse the built-in configuration")
        .0
}

/// Benchmarks processing the commits of every release (conventional commit
/// parsing, commit parsers and link parsers).
fn bench_process(c: &mut Criterion) {
    let config = config("keepachangelog");
    let mut group = c.benchmark_group("process");
    for &count in RELEASE_COUNTS {
        let releases = releases(count);
        group.throughput(Throughput::Elements((count * COMMITS_PER_RELEASE) as u64));
        group.bench_with_input(
            BenchmarkId::new("releases", count),
            &releases,
            |b, releases| {
                b.iter_batched(
                    || (releases.clone(), config.clone()),
                    |(releases, config)| {
                        black_box(
                            Changelog::new(releases, config, None).expect("failed to process"),
                        )
                    },
                    BatchSize::LargeInput,
                );
            },
        );
    }
    group.finish();
}

/// Benchmarks rendering already processed releases with the built-in
/// templates.
fn bench_render(c: &mut Criterion) {
    let mut group = c.benchmark_group("render");
    for name in ["keepachangelog", "detailed", "scoped"] {
        let count = 100;
        let changelog = Changelog::new(releases(count), config(name), None)
            .expect("failed to process the releases");
        group.throughput(Throughput::Elements((count * COMMITS_PER_RELEASE) as u64));
        group.bench_with_input(BenchmarkId::new(name, count), &changelog, |b, changelog| {
            b.iter(|| {
                changelog
                    .generate(&mut io::sink())
                    .expect("failed to render");
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_process, bench_render);
criterion_main!(benches);

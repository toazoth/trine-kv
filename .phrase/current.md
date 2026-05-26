# Current Phase

## Status

Complete

## Goal

Run measured benchmark/performance tuning before CI push without changing
public API or the v1 storage contract.

## Entry Condition

- Phase 11 Windows directory-sync hardening is complete.
- User wants benchmark/performance tuning before pushing to CI.

## Scope

- Run the existing v1 benchmark harness and compare against local baseline.
- Choose one measured hotspot with a clear implementation path.
- Apply a small tuning change with focused verification.
- Update benchmark notes and phase evidence.

## Out Of Scope

- Changing the table, manifest, WAL, blob, or recovery file formats.
- Adding new public API.
- Publishing the crate.
- Broad benchmark harness redesign.
- Speculative tuning without a benchmark signal.

## Acceptance Gate

- Current benchmark baseline is recorded.
- A measured hotspot is selected and classified before implementation.
- After tuning, the relevant benchmark improves or evidence explains why the
  change is rejected.
- Local verification passes for `cargo fmt --check`,
  `cargo clippy --all-targets --all-features -- -D warnings`,
  `cargo test --all-targets --all-features`, examples, package list, package
  verification, publish dry-run, and `git diff --check`.

## Active Task Slice

```text
task047 [x] goal:one benchmark-backed performance tuning slice lands with before/after evidence | scope:benches,src,docs/benchmarks,.phrase | verify:cargo bench --bench v1_bench before/after + full release gate
```

## Known Blockers

- GitHub Actions cannot be executed locally in this environment; remote CI must
  run after push.
- Local benchmark numbers are machine- and load-sensitive; compare only within
  this session.
- The tuning improved the separated-blob benchmark path; flush did not improve
  because its directory sync count is unchanged, and compaction remained noisy.

## Evidence To Record

- Benchmark baseline and after-tuning result.
- Selected hotspot and rejected options.
- Full local release gate result.

## Next Recommendation

- If this gate passes, configure the publish secret/environment and run the
  `Publish` workflow with `mode=dry-run` before any real publish.

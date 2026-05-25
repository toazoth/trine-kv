# Current Phase

## Status

Complete

## Goal

Prepare Trine KV for a clean first crate package using Semantic Versioning.

## Entry Condition

- Phase 6 production hardening is complete.
- User chose release packaging before integration examples.

## Scope

- Cargo package metadata for the first `0.1.0` SemVer release candidate.
- Crate package contents, license files, changelog, and release checklist.
- README and usage docs that explain released dependency syntax.
- Package verification using Cargo and the existing test/example gate.

## Out Of Scope

- Publishing to crates.io.
- Creating GitHub releases or CI pipelines.
- Adding integration examples; that is the next planned phase.
- Changing v1 storage contracts.

## Acceptance Gate

- `cargo package --list` excludes local workflow files such as `.phrase/`,
  `.rust-skills/`, and `.claude/`.
- `cargo package` verifies the package.
- `cargo fmt --check`, `cargo clippy`, `cargo test`,
  `cargo run --example quickstart`, and `git diff --check` pass.
- Release docs record the SemVer rule and package checklist.

## Active Task Slice

```text
task042 [x] goal:package metadata, package contents, release docs, and SemVer rule are ready for 0.1.0 | scope:Cargo.toml,README.md,docs,CHANGELOG.md,LICENSE*,.phrase | verify:cargo package --list + cargo package + cargo fmt --check + cargo clippy + cargo test + cargo run --example quickstart + git diff --check
```

## Known Blockers

- None recorded for Phase 7.
- Initial `cargo package --list` included local workflow and skill files; the
  package include list now excludes them.
- `cargo package` required network access to refresh the crates.io index in
  this environment.

## Evidence To Record

- Package list result.
- Package verification result.
- Full release gate result.

## Next Recommendation

- Start Phase 8 integration examples.

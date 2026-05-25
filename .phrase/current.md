# Current Phase

## Status

Complete

## Goal

Polish the public v1 API so common usage paths need less caller-side boilerplate
without changing storage behavior.

## Entry Condition

- Phase 4 usage documentation is complete.
- Quickstart example and v1 tests pass.

## Scope

- Ergonomic constructors and helpers for public options.
- Keyspace convenience methods that preserve batch/transaction semantics.
- Documentation and examples updated only where the API becomes simpler.

## Out Of Scope

- Changing MVCC, WAL, SSTable, manifest, compaction, transaction,
  prefix-filter, compression, or search-policy behavior.
- Production-hardening audits; that is the next phase.
- Publishing or packaging work.

## Acceptance Gate

- API examples compile and run.
- `cargo fmt --check`, `cargo clippy`, `cargo test`, and `git diff --check`
  pass.
- Usage docs remain accurate and avoid forbidden terminology.

## Active Task Slice

```text
task037 [x] goal:common open and single-key write paths need less struct boilerplate | scope:src/db.rs,src/options.rs,src/keyspace.rs,README.md,docs/usage.md,examples/quickstart.rs,tests,.phrase/current.md,.phrase/evidence.md | verify:cargo run --example quickstart + cargo fmt --check + cargo clippy + cargo test + git diff --check
```

## Known Blockers

- None recorded for Phase 5.

## Evidence To Record

- API helper validation.
- Example and docs alignment.

## Next Phase Recommendation

Start production hardening from an audit slice. Do not assume the first
hardening change before checking operational failure modes.

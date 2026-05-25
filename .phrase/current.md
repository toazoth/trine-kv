# Current Phase

## Status

Complete

## Goal

Write usage documentation that lets a Rust user try Trine KV and understand the
main v1 API without reading the engine internals.

## Entry Condition

- Phase 3 is complete by the v1 acceptance gate.
- V1 implementation, benchmark baseline, and durability notes are committed.

## Scope

- README entry point.
- Runnable quickstart example.
- Usage guide for opening databases, keyspaces, writes, reads, iteration,
  snapshots, transactions, durability, maintenance, stats, and recovery limits.
- Keep documentation aligned with the implemented public API.

## Out Of Scope

- Changing storage-engine behavior while writing docs, unless documentation
  exposes a clear contract bug.
- Publishing crate/package metadata beyond existing `Cargo.toml`.
- Pre-splitting release, integration, or production-hardening work.

## Acceptance Gate

- `cargo run --example quickstart` passes.
- `cargo fmt --check`, `cargo clippy`, `cargo test`, and `git diff --check`
  pass.
- Usage docs avoid forbidden terminology and do not overstate durability.

## Active Task Slice

```text
task036 [x] goal:usage docs and runnable quickstart cover the v1 public API entry path | scope:README.md,docs/usage.md,examples/quickstart.rs,.phrase/current.md,.phrase/evidence.md | verify:cargo run --example quickstart + cargo fmt --check + cargo clippy + cargo test + git diff --check
```

## Known Blockers

- None recorded for Phase 4.

## Evidence To Record

- Quickstart example validation.
- Documentation coverage and boundary notes.

## Next Phase Recommendation

Start the next phase from fresh evidence. Good candidates are API polish,
release packaging, integration examples, or production-hardening audits.

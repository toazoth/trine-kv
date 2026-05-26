# Current Phase

## Status

Complete

## Goal

Finish the first Titan-like large-value lifecycle by adding snapshot-safe blob
GC rewrite, manifest pending-deletion recovery, and user-facing GC controls.

## Entry Condition

- Phase 35 completed `BlobFile` flush/recovery integration.
- User asked to finish the remaining large-value work.

## Scope

- Add database-level blob GC controls.
- Mark obsolete blob files in manifest metadata before physical deletion.
- Rewrite still-live records from partially stale blob files into new blob
  files during compaction-triggered GC.
- Keep old blob files while snapshots or lazy range iterators can still reach
  old table handles.
- Resume safe pending-deletion cleanup on writable open.
- Expose blob GC counters in `DbStats`.
- Update docs and protocol notes for the implemented GC behavior.

## Out Of Scope

- Titan Level Merge policy and range-locality optimization.
- WAL-time value separation.
- Hole punching or in-place blob-file rewriting.
- A separate in-memory blob store.

## Acceptance Gate

- Compaction records obsolete blob files as manifest pending deletions.
- GC rewrites live records out of partially stale blob files and keeps reads
  correct across reopen.
- Pending obsolete blob files are removed only when no active read can reach
  them.
- Recovery cleans safe pending blob deletions and preserves conflicting
  referenced pending entries.
- Manifest v4 files still decode with empty pending-deletion metadata.
- Rust verification passes.

## Active Task Slice

```text
task119 [x] goal:add blob GC controls and counters | scope:src/options.rs src/stats.rs src/db.rs | verify:cargo check
task120 [x] goal:publish pending blob deletions in manifest | scope:src/manifest.rs src/db.rs | verify:pending deletion recovery tests
task121 [x] goal:rewrite partially stale blob files safely | scope:src/blob.rs src/db.rs src/table.rs | verify:blob GC rewrite tests
task122 [x] goal:run full verification and record evidence | scope:repo .phrase docs | verify:cargo test + clippy + diff checks
```

## Known Blockers

- Remote CI cannot be executed locally; it must run after push.
- Blob GC throughput has correctness coverage but not a dedicated benchmark
  yet.

## Evidence

- `cargo check --all-targets --all-features` passes.
- `cargo test persistent_blob_gc_rewrites_live_records_from_partially_stale_file --all-features`
  passes.
- `cargo test persistent_blob_gc_keeps_old_blob_while_read_pin_can_reach_it --all-features`
  passes.
- `cargo test persistent_recovery_cleans_manifest_pending_blob_deletion --all-features`
  passes.
- `cargo test persistent_recovery_does_not_delete_referenced_pending_blob_deletion --all-features`
  passes.
- `cargo test manifest_decode_accepts_previous_version_without_pending_blob_deletions --all-features`
  passes.
- `cargo test --all-targets --all-features` passes.
- `cargo clippy --all-targets --all-features` passes.
- `cargo fmt --all --check` passes.
- `git diff --check` passes.
- Forbidden-term scan over `.phrase`, `src`, `tests`, `benches`, `examples`,
  `docs`, and `README.md` passes.

## Next Recommendation

- Commit Phase 36 once the user wants a checkpoint, then run a dedicated
  large-value/GC benchmark phase before tuning throughput.

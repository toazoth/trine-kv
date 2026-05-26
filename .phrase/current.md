# Current Phase

## Status

Complete

## Goal

Finish the first pass of post-GC large-value maintenance: optional Titan-style
Level Merge, value-lazy iteration, GC rewrite throughput tightening, and a
more systematic recovery fault matrix.

## Entry Condition

- Phase 37 completed direct `BlobIndex` read tuning and benchmark coverage.
- User asked to finish Level Merge, value-lazy iterator, blob GC throughput
  optimization, and broader crash/recovery fault injection.

## Scope

- Add bucket-level `blob_level_merge_enabled` and persist it in manifest v6.
- Let compaction rewrite retained `BlobIndex` values into output blob files
  only when the bucket enables Level Merge.
- Add public value-lazy range and prefix iterator APIs for `Db`, `Bucket`, and
  `Snapshot`.
- Make blob GC candidate selection read only blob footer/properties metadata,
  and make GC live-record copying use direct indexed blob reads.
- Add focused persistent and in-memory tests plus a table-driven recovery fault
  matrix.
- Update the Titan-like blob protocol, usage docs, README, and benchmark notes.

## Out Of Scope

- Automatic Level Merge policy selection.
- Value caching inside `LazyValue`.
- Multi-file batch planning for blob GC.
- New on-disk blob format changes beyond manifest v6 bucket options.

## Acceptance Gate

- Level Merge rewrites retained large values only when explicitly enabled.
- Value-lazy iterators do not read blob bytes until the caller asks for the
  value, and in-memory mode still returns inline values.
- GC candidate selection no longer decodes every blob record payload.
- Recovery fault matrix covers representative publish, missing-file,
  corruption, and unreferenced-file failures.
- Protocol/docs describe the implemented behavior.
- Full local Rust verification passes.

## Active Task Slice

```text
task126 [x] goal:add optional blob Level Merge | scope:options manifest table compaction tests | verify:persistent_blob_level_merge_rewrites_retained_blob_indexes
task127 [x] goal:add value-lazy iterator API | scope:iterator db bucket snapshot docs tests | verify:persistent_value_lazy_iterator_defers_blob_reads_until_value_access + in-memory lazy test
task128 [x] goal:tighten blob GC throughput path | scope:blob db bench protocol tests | verify:properties_read_skips_record_payload_decode + cargo bench row
task129 [x] goal:add recovery fault matrix | scope:tests/persistent_wal.rs | verify:persistent_recovery_fault_injection_matrix_fails_closed
task130 [x] goal:record docs/evidence and run full gate | scope:.phrase docs README | verify:full Rust verification
```

## Known Blockers

- Remote CI cannot be executed locally; it must run after push.
- Level Merge policy is explicit and manual. Automatic policy should be
  benchmark-driven later.
- GC still rewrites one selected candidate per maintenance pass.

## Evidence

- `blob_level_merge_enabled` defaults to false and is persisted in manifest v6;
  v5 manifests decode the option as false.
- `range_lazy` and `prefix_lazy` return keys plus `LazyValue`; blob bytes are
  read only by `LazyValue::read` or `LazyKeyValue::into_key_value`.
- Lazy blob rows share the iterator read pin through `Arc<Snapshot>`, so each
  returned row does not re-pin the global snapshot tracker.
- GC candidate selection reads blob footer/properties metadata without
  decoding every blob record payload.
- GC rewrite reads live records by exact `BlobIndex` and internal key.
- Release benchmark rows from `cargo bench --bench v1_bench`:
  - `blob range scan`: 17705 us for 32 scans.
  - `blob range lazy keys`: 174 us for 32 scans.
  - `blob GC rewrite`: 154265 us.
  - `blob level merge`: 143299 us.
- `cargo test --all-targets --all-features` passes.
- `cargo clippy --all-targets --all-features` passes.
- `cargo fmt --all --check` passes.
- `git diff --check` passes.
- Forbidden-term scan over `.phrase`, `src`, `tests`, `benches`, `examples`,
  `docs`, and `README.md` passes.

## Next Recommendation

- Commit Phase 38. After CI push, the next large-value work should be
  benchmark-driven policy tuning: when to enable Level Merge, whether GC should
  batch multiple candidates, and how lazy value APIs should expose key-only or
  value-caching ergonomics.

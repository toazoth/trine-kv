# Current Phase

## Status

Complete

## Goal

Tighten persistent read-path resource management by avoiding per-block file
handle clone/open work, pinning hot L0/L1 index/filter metadata, and giving the
block cache a real priority policy.

## Entry Condition

- Phase 41 completed background maintenance scheduling and backpressure.
- User identified descriptor/file-handle cache, per-level index/filter pinning,
  block-cache priority policy, and benchmark-gated key encoding as the next
  release risks.

## Scope

- Replace per-block table file cloning/opening with a shared table file handle
  that performs seek/read under one table-local lock.
- Pin table-level filters and index partitions for L0/L1 tables; keep deeper
  levels lazy.
- Route lazy index partitions through the global block cache when available.
- Split block-cache eviction into low-priority data/blob entries and
  high-priority metadata entries so data pressure does not evict hot
  index/filter/range metadata first.
- Add a shared-prefix key benchmark and decide from evidence whether prefix
  truncation or a tighter key encoding belongs in this phase.
- Update protocol, docs, roadmap, evidence, and focused tests.

## Out Of Scope

- Changing the SSTable file version or existing record encoding unless the new
  benchmark proves it is worth the added format complexity.
- Adding async I/O or platform-specific positional-read APIs.
- Adding public cache tuning knobs before benchmark evidence justifies them.

## Acceptance Gate

- Persistent block reads reuse the table's cached file handle without cloning or
  reopening it per block.
- L0/L1 tables pin table filters and index partitions; deeper levels keep
  partition metadata lazy and cacheable.
- Block-cache eviction protects high-priority metadata entries from low-priority
  data churn.
- Benchmark evidence exists for shared-prefix keys and justifies either
  deferring or implementing key encoding changes.
- Full local Rust verification passes.

## Active Task Slice

```text
task144 [x] goal:shared table file handle | scope:src/table.rs | verify:cached-handle test
task145 [x] goal:L0/L1 metadata pinning | scope:src/table.rs tests | verify:pinning/lazy tests
task146 [x] goal:block cache priority and index partition caching | scope:src/cache.rs src/table.rs | verify:cache policy tests
task147 [x] goal:shared-prefix key benchmark decision | scope:benches/v1_bench.rs .phrase/evidence.md | verify:cargo bench row
task148 [x] goal:protocol/docs/evidence update | scope:.phrase docs README | verify:full Rust verification
```

## Known Blockers

- Remote CI cannot be executed locally; it must run after push.

## Evidence

- Rust skill, performance skill, concurrency skill, SPEC-AGENTS context, and
  coding guidelines were read before implementation.
- Baseline `cargo bench --bench v1_bench` on 2026-05-27 reported `random get`
  at 907 us, `missing get` at 457 us, `block cache warm read` at 1539 us,
  `cold table read` at 176052 us, and `index seek policy auto large` at 3739 us.
- Initial code audit found persistent tables hold an `Arc<File>`, but each
  block read clones the handle before seek/read. Only L0 table filters are
  pinned, and lazy index partitions stay in an unbounded table-local cache.
- Block cache keys already carry a block kind, but eviction does not yet use
  kind-specific priority.
- Persistent tables now hold a shared table file handle and read blocks through
  table-local locked seek/read instead of cloning or opening a file per block.
- L0/L1 table writes now include table filters; persistent open pins table
  filters and all index partitions for L0/L1 tables. Deeper levels keep index
  partitions lazy.
- Deeper-level lazy index partitions use the global block cache when a read path
  supplies it.
- Block cache entries now have high/low priority queues. Index, filter, and
  range-tombstone metadata are high priority; data and blob blocks are low
  priority.
- Release-profile `cargo bench --bench v1_bench` after implementation reported
  `random get` at 1032 us, `missing get` at 497 us, `block cache warm read` at
  1764 us, `cold table read` at 154176 us, `index seek policy auto large` at
  3341 us, and new `long shared-prefix get` at 2867 us.
- Shared-prefix keys are measurably slower than the normal key shape, but a
  tighter key encoding would change SSTable record layout. This phase records
  the benchmark and defers the format change to a dedicated storage-format
  slice instead of bundling it into cache work.
- Verification passed: `cargo test --all-targets --all-features`,
  `cargo clippy --all-targets --all-features -- -D warnings`,
  `cargo fmt --all --check`, `cargo bench --bench v1_bench`,
  `git diff --check`, and the forbidden-term scan.

## Next Recommendation

- Commit Phase 42, then use remote CI as the external release signal. If
  shared-prefix workloads remain important, open a dedicated data-block key
  encoding phase with a file-format compatibility gate.

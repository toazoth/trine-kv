# Current Phase

## Status

Complete

## Goal

Harden iterator merge and write-path maintenance scheduling: make range/prefix
iteration choose the next source through a heap and make
`background_worker_count` drive real foreground-safe background maintenance.

## Entry Condition

- Phase 19 leveled compaction and range tombstone queries passed locally.
- Remaining evidence identifies linear scan source merge and foreground-only
  flush/compaction scheduling as the next production-readiness risks.

## Scope

- Replace lazy scan source selection with a heap keyed by user key and scan
  direction.
- Keep L0 multi-source merge and L1+ table cursor behavior correct under MVCC
  visibility and range tombstones.
- Start background maintenance workers when persistent mode has
  `background_worker_count > 0`.
- Schedule immutable memtable flush and compaction work after commits and
  explicit flushes without publishing partially written tables.
- Surface background maintenance errors through later writes, `flush()`, and
  `compact_range()`.
- Keep in-memory mode free of background worker threads.
- Preserve in-memory mode behavior.

## Out Of Scope

- Changing public API.
- Changing compression policy.
- Parallel compaction inside one compaction job.
- Full asynchronous public API.

## Acceptance Gate

- Iterator `next()` selects candidate source groups through heap operations.
- Range and prefix scans preserve forward/reverse order and MVCC visibility.
- `background_worker_count` starts persistent maintenance workers and `0`
  leaves maintenance synchronous/explicit.
- Background flush/compaction can reduce immutable/L0 pressure without an
  explicit user `flush()`.
- Background errors are not swallowed.
- Full local Rust verification passes.

## Active Task Slice

```text
task064 [x] goal:heap-based lazy iterator source merge | scope:src/iterator.rs,tests | verify:iterator heap unit and persistent scan tests
task065 [x] goal:background maintenance workers honor background_worker_count | scope:src/db.rs,src/db/commit.rs,tests | verify:background flush/compaction persistent tests
task066 [x] goal:background maintenance errors surface to callers | scope:src/db.rs,tests | verify:publish-failure/background-error test
task067 [x] goal:update protocol and evidence for P5/P6 behavior | scope:.phrase | verify:evidence delta and protocol notes
```

## Known Blockers

- Background work remains thread-based and bounded to persistent databases.
- Remote CI cannot be executed locally; it must run after push.
- GitHub Actions cannot be executed locally; remote CI must run after push.

## Evidence

- Iterator tests prove heap ordering for forward/reverse scans and live scan
  correctness.
- Persistent tests prove background workers flush immutable memtables and compact
  L0 pressure.
- A focused failure test proves background maintenance errors are returned to
  later callers.
- A coordinator unit test proves a later success does not hide an unreported
  background failure.
- Full local Rust verification passed with all targets and features.

## Next Recommendation

- Move to remaining benchmark-guided refinements or release-readiness review.

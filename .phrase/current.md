# Current Phase

## Status

Complete

## Goal

Remove avoidable contention and allocation from point reads without changing
public API or v1 storage formats.

## Entry Condition

- Phase 14 lazy range iterator hardening is committed.
- User benchmark review identifies repeated snapshot pinning, global block-cache
  locking, vector-based point lookup, and full memtable scans as point-read
  bottlenecks.

## Scope

- Reuse an existing snapshot pin for snapshot-backed point reads.
- Make active memtable point lookup seek directly to the requested user-key span.
- Make SSTable point lookup choose the newest visible record without building a
  full record vector for the key.
- Reduce block-cache hit-path contention.
- Keep the public `get` API returning owned values for v1 compatibility.

## Out Of Scope

- Changing public return types to borrowed or guarded values.
- Changing WAL, manifest, SSTable, blob, compression, or recovery formats.
- Rewriting range/prefix iterators again.
- Replacing the benchmark harness.

## Acceptance Gate

- Snapshot-backed point reads do not take a second read pin.
- Point reads no longer call the vector-based point collection path.
- Point reads preserve MVCC ordering, point deletes, range deletes, blob values,
  snapshots, and persistent reads.
- Block-cache hit tracking no longer depends on one global exclusive lock.
- Local verification passes for formatting, clippy, full tests, examples,
  benchmark, Windows target check, and `git diff --check`.

## Active Task Slice

```text
task051 [x] goal:point reads avoid duplicate snapshot pins and vector/sort lookup | scope:src/keyspace.rs,src/snapshot.rs,src/db.rs,src/table.rs,src/cache.rs,tests,.phrase | verify:targeted tests + full Rust gate + v1 bench
```

## Known Blockers

- The public v1 `get` API returns owned `Vec<u8>`, so this phase can avoid
  intermediate clones but still must copy the returned value.
- GitHub Actions cannot be executed locally; remote CI must run after push.

## Evidence To Record

- Baseline and after-change benchmark rows for random get, missing get, and
  block-cache warm read.
- Tests proving snapshot, range-delete, blob, and persistent point reads remain
  correct.
- Snapshot-backed point reads reuse an existing pinned snapshot when one is
  available.
- Point reads keep one newest visible candidate instead of collecting and
  sorting all point records for the key.
- Block-cache metadata is split into shards; hits use a shard read lock instead
  of one global exclusive lock.
- Release benchmark spot-check improved random get from 10233 us to 783 us,
  missing get from 9276 us to 403 us, and block cache warm read from 1373 us to
  913 us in local runs.

## Next Recommendation

- If this phase passes, rerun the external multi-thread benchmark to check
  thread scaling under realistic concurrency.

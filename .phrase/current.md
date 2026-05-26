# Current Phase

## Status

Complete

## Goal

Separate the internal LSM core from database-wide coordination without changing
public API behavior or storage formats.

## Entry Condition

- Phase 20 iterator merge and background maintenance passed locally.
- User identified that the current tree data structure and database layer are
  too tightly mixed.
- Current evidence shows `db.rs` still owns tree state, read visibility,
  tombstone checks, flush input selection, and compaction retention helpers.

## Scope

- Use `.phrase/protocol/lsm-core-boundary-spec.md` as the source of truth for
  the extraction.
- Keep WAL, manifest publish, process lock, recovery, background worker
  lifecycle, and cross-keyspace batch coordination in the database layer.
- Move one-keyspace tree state and tree-local rules behind an internal
  `LsmTree` boundary.
- Keep MVCC read visibility, range tombstone checks, scan grouping, transaction
  conflict checks, flush planning, and compaction retention inside LSM core.
- Preserve in-memory mode as the same logical engine over volatile storage.
- Keep public API and storage format unchanged.

## Out Of Scope

- Public API redesign.
- Storage format changes.
- WAL or manifest format changes.
- New compression codecs.
- Replacing the background worker model.
- Cross-keyspace compaction.

## Acceptance Gate

- The boundary spec is written and linked from the v1 protocol.
- `src/lsm/` exists and owns the first tree-local state boundary.
- `Db` keeps database-wide coordination but no longer owns the first extracted
  tree rules for the task slice.
- Full local Rust verification passes after each code slice.
- Evidence records what moved and what remains in DB for the next phase.

## Active Task Slice

```text
task068 [x] goal:write complete LSM core boundary spec | scope:.phrase/protocol,.phrase/current.md,.phrase/roadmap.md | verify:protocol link and doc diff checks
task069 [x] goal:create internal LSM module and move tree state boundary | scope:src/lsm,src/db.rs,src/lib.rs | verify:cargo test --all-targets --all-features
task070 [x] goal:move point read visibility into LsmTree | scope:src/lsm,src/db.rs,tests | verify:point read, tombstone, transaction, persistent tests
task071 [x] goal:move range and prefix scan setup into LsmTree | scope:src/lsm,src/db.rs,src/iterator.rs,tests | verify:scan, prefix, range-delete, persistent iterator tests
task072 [x] goal:move flush planning and install into LsmTree | scope:src/lsm,src/db.rs,tests | verify:flush and persistent WAL tests
task073 [x] goal:move compaction planning and merge retention into LsmTree | scope:src/lsm,src/db.rs,tests | verify:compaction and range-delete tests
task074 [x] goal:move transaction conflict checks into LsmTree | scope:src/lsm,src/db/commit.rs,tests | verify:transaction tests
```

## Known Blockers

- Remote CI cannot be executed locally; it must run after push.
- `AGENTS.md` has a pre-existing unstaged edit outside this phase.

## Evidence

- Boundary spec path and protocol link.
- `src/lsm/` now exists with `LsmTree`, `ImmutableMemtable`, and
  `RangeTombstone`.
- `DbInner.keyspaces` now stores `Arc<LsmTree>`.
- Tree table sorting moved behind `LsmTree::sort_tables_for_reads`.
- Point read visibility moved behind `LsmTree::read_visible_point`.
- Range and prefix scan source construction moved behind `LsmTree::scan`.
- Write application, active memtable freeze, memtable byte accounting, flush
  input planning, and flush install moved behind `LsmTree`.
- Compaction input planning, point-version retention, range tombstone cleanup,
  output splitting, and table install moved behind `LsmTree`.
- Transaction point/range conflict checks moved behind `LsmTree`.
- `Db` still coordinates WAL, manifest publish, sequence assignment,
  snapshots, process lock, background worker lifecycle, and cross-keyspace
  batch boundaries.
- `Db::get_at_with_pin_state` now only finds the tree and supplies
  `read_sequence`, path, and cache handles.
- Local Rust verification passed for the completed LSM core boundary slice.

## Next Recommendation

- Close Phase 21 after remote CI. The next phase should be chosen from fresh
  CI, benchmark, or user review evidence rather than from old coupling notes.

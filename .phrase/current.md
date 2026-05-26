# Current Phase

## Status

Complete

## Goal

Make compaction and range tombstone reads match the v1 leveled-LSM shape:
choose compaction work from level pressure, split outputs by configured table
size, and query ordered tombstone structures instead of scanning every
tombstone.

## Entry Condition

- Phase 18 real Bloom filters passed locally.
- Remaining evidence identifies compaction output sizing and range tombstone
  lookup as the next production-readiness risks.

## Scope

- Add an ordered range tombstone query structure for memtables, SSTables, point
  reads, transaction conflict checks, and scan setup.
- Keep range tombstone blocks on disk and load them only when a query needs
  tombstones from that table.
- Pick compaction work from L0 pressure or level-size pressure using
  `target_table_bytes`, `level_size_multiplier`, and `max_l0_files`.
- Keep L0 special handling: overlapping L0 inputs are grouped and overlapping
  L1 inputs are included before publishing an L1 replacement.
- Move overfull L1+ inputs down one level with overlapping next-level inputs so
  deeper levels stay non-overlapping after publish.
- Split compaction outputs at user-key boundaries according to
  `target_table_bytes`.
- Preserve snapshot-visible versions and range tombstones needed for retained
  records.
- Preserve in-memory mode behavior.

## Out Of Scope

- Background worker scheduling.
- Replacing scan source merge with a heap-based merge.
- Changing public API.
- Changing compression policy.

## Acceptance Gate

- Point reads and transaction conflict checks query only tombstones whose bounds
  can cover the requested key/range.
- Scan setup collects only tombstones that overlap the iterator selector.
- Compaction can split one input set into multiple output SSTables, each under
  the configured target except for a single oversized user-key group.
- L0 pressure is reduced, and overfull L1+ levels move data down by level-score
  rules.
- Full local Rust verification passes.

## Active Task Slice

```text
task060 [x] goal:ordered range tombstone query path for point/range reads | scope:src/range_tombstone.rs,src/db.rs,src/table.rs,src/iterator.rs | verify:targeted tombstone lookup tests
task061 [x] goal:level-score compaction planning with L0 and L1+ rules | scope:src/compaction.rs,src/db.rs,tests | verify:planner and persistent level tests
task062 [x] goal:split compaction outputs by target table size | scope:src/db.rs,src/manifest.rs,src/table.rs,tests | verify:persistent split-output compaction test
task063 [x] goal:update protocol and evidence for P3/P4 behavior | scope:.phrase | verify:evidence delta and protocol notes
```

## Known Blockers

- Compaction still runs synchronously under the writer coordinator.
- Scan source merge remains linear across sources rather than heap-based.
- GitHub Actions cannot be executed locally; remote CI must run after push.

## Evidence To Record

- `covering_key_returns_only_possible_covering_tombstones` and
  `overlapping_range_returns_only_intersecting_tombstones` prove ordered
  tombstone key/range lookup.
- `l0_plan_expands_overlapping_l0_group_and_lower_level_tables`,
  `no_l0_fallback_moves_shallowest_overlapping_level_down`, and
  `overfull_level_score_picks_largest_pressure_ratio` prove planner behavior.
- `persistent_compaction_splits_outputs_and_moves_overfull_l1_down` proves
  split outputs, L1+ down-level compaction, read correctness, and reopen.
- Full local gate passed: `cargo test --all-targets --all-features`,
  `cargo clippy --all-targets --all-features -- -D warnings`, and
  `cargo fmt --all`.

## Next Recommendation

- If this phase passes, move to iterator merge hardening or background flush
  scheduling depending on the next benchmark/audit signal.

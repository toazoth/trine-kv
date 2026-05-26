# Current Phase

## Status

In Progress

## Goal

Harden the compaction picker after filter strategy behavior became observable.

## Entry Condition

- Phase 25 exposed table/block filter hit, miss, and false-positive counters
  and strengthened prefix miss skip tests.
- User review identified P6 as the next LSM tree improvement after filter
  strategy hardening.

## Scope

- Audit the current compaction picker against level score, L0 pressure, overlap
  bytes, read amplification, and trivial move behavior.
- Keep snapshot-aware tombstone and version retention rules intact.
- Prefer picker improvements that keep table format and public API unchanged.
- Add tests before changing picker behavior.

## Out Of Scope

- Public API redesign.
- WAL or manifest format changes.
- Blob GC.
- Benchmark-driven policy defaults without new benchmark evidence.
- Format changes without a protocol update.
- Background compaction scheduling beyond picker behavior.
- Single-delete semantics.

## Acceptance Gate

- Compaction picker uses level score and L0 pressure without broadening work
  beyond the needed key range.
- L0 compaction keeps overlap closure behavior and lower-level overlap inputs.
- L1+ compaction can avoid full-level rewrites when a narrower range is enough.
- Trivial move is supported when an input table has no lower-level overlap.
- Output table splitting continues to respect target table bytes.
- Existing public API and storage formats remain unchanged unless protocol docs
  are updated first.
- Full local Rust verification passes.

## Active Task Slice

```text
task089 [ ] goal:audit compaction picker gaps | scope:src/compaction.rs,src/lsm/compact.rs,tests | verify:evidence note with exact blockers
task090 [ ] goal:add picker tests for score, overlap, and trivial move | scope:src/compaction.rs,tests | verify:focused compaction picker tests
task091 [ ] goal:implement the next picker refinement slice | scope:src/compaction.rs,src/lsm/compact.rs | verify:focused tests plus full Rust gate
```

## Known Blockers

- Remote CI cannot be executed locally; it must run after push.
- Any on-disk format change must update protocol docs first; current phase
  should avoid that unless evidence proves it necessary.

## Evidence

- Phase 25 full local verification passed.
- Current compaction already has level score, L0 overlap closure, output
  splitting, and snapshot-aware cleanup, but the picker still needs a focused
  audit before the next behavior change.

## Next Recommendation

- Start task089 with a focused picker audit before changing compaction inputs
  or move behavior.

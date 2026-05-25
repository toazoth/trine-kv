# Changelog

All public crate releases use Semantic Versioning.

## 0.1.0 - 2026-05-26

Initial packaged release candidate.

### Added

- Embedded LSM MVCC key-value database with in-memory and persistent modes.
- Named keyspaces, point reads/writes, range scans, prefix scans, snapshots,
  optimistic transactions, and atomic write batches.
- WAL recovery, SSTable flush/read, manifest metadata, compaction, block
  compression through `lz4_flex`, prefix filters, block cache stats, and
  blob-backed large values.
- Read-only open, safe temporary file repair policy, durability notes, usage
  guide, quickstart example, and benchmark baseline.

### Hardened

- Manifest publish installs in-memory state only after durable file publish
  succeeds.
- WAL, manifest, and table decoders reject impossible count fields before large
  allocation.
- Failed flush/compaction publish removes unpublished table/blob output files.

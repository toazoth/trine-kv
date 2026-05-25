# Trine KV

Trine KV is an embedded Rust key-value database with MVCC snapshots, optimistic
transactions, WAL recovery, SSTables, compaction, prefix/range scans, block
compression, and blob-backed large values.

The v1 engine is implemented and verified by the repository test suite,
benchmark harness, and durability notes. Start with the runnable quickstart:

```text
cargo run --example quickstart
```

Then read [docs/usage.md](docs/usage.md) for the API path and
[docs/durability.md](docs/durability.md) for persistence guarantees and limits.

## Install

This repository does not assume a publication target yet. For local
development, depend on a path:

```toml
[dependencies]
trine-kv = { path = "../trine-kv" }
```

## Minimal Example

```rust
use trine_kv::{Db, KeyspaceOptions};

fn main() -> trine_kv::Result<()> {
    let db = Db::open_memory()?;
    let users = db.keyspace("users", KeyspaceOptions::default())?;

    users.insert(b"user:001", b"Ada")?;
    assert_eq!(users.get(b"user:001")?, Some(b"Ada".to_vec()));

    Ok(())
}
```

## Common Commands

```text
cargo fmt --check
cargo clippy
cargo test
cargo run --example quickstart
cargo bench --bench v1_bench
```

## Documentation

- [Usage guide](docs/usage.md)
- [Durability notes](docs/durability.md)
- [V1 benchmark baseline](docs/benchmarks/v1-baseline.md)

## Current Boundaries

- Persistent mode uses a single local database directory.
- `SyncAll` is the strongest WAL commit mode, subject to platform filesystem
  behavior.
- Read-only open is for inspecting a stable directory state; v1 does not define
  live multi-process reads against an active writer.
- Repair is intentionally narrow and only removes known safe temporary files
  when explicitly requested.

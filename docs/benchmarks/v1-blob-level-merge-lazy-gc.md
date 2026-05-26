# Trine KV V1 Blob Maintenance And Lazy Value Benchmark

Date: 2026-05-26

Command:

```text
cargo bench --bench v1_bench
```

Context:

- rows: 1024
- ops: 2048
- large-value rows: 128
- large-value ops: 256
- large value size: 16 KiB
- build profile: Cargo bench release profile
- comparison scope: same local machine, same session

## Focus Rows

| name | elapsed_us | units_per_sec |
| --- | ---: | ---: |
| blob point read | 16650 | 15375 |
| blob range scan | 17705 | 1807 |
| blob range lazy keys | 174 | 183205 |
| blob GC rewrite | 154265 | 829 |
| blob level merge | 143299 | 893 |

## Interpretation

`blob range lazy keys` measures scanning keys without asking for blob values.
It is much cheaper than value-returning range scans because it avoids blob
record reads until the caller requests a value.

`blob GC rewrite` now selects candidates from blob footer/properties metadata
and reads only live referenced records by `BlobIndex`. Recovery still validates
full blob files.

`blob level merge` measures optional compaction rewriting of retained large
values into the output blob file when `blob_level_merge_enabled` is set.

## Verification

- `cargo bench --bench v1_bench`

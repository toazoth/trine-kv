# Trine KV V1 Pre-Publish Tuning

Date: 2026-05-26

Command:

```text
cargo bench --bench v1_bench
```

Context:

- rows: 1024
- ops: 2048
- build profile: Cargo bench release profile
- comparison scope: same local machine, same session
- tuning target: reduce parent-directory sync calls for table/blob output files
  before manifest publish

## Selected Hotspot

The pre-tuning run showed persistent write-heavy workloads as the clearest
release-relevant cost after adding parent-directory sync:

| name | before_elapsed_us |
| --- | ---: |
| flush throughput | 34110 |
| compaction throughput | 83298 |
| separated blob values | 52841 |

The selected slice was `separated blob values`, because one table write can
publish both a blob file and an SSTable before the manifest edit. Syncing the
database directory once before manifest publish is enough to make both file
names durable, so per-file parent-directory syncs were avoidable.

## After Tuning

Three post-tuning runs:

| name | run_1_us | run_2_us | run_3_us | median_us |
| --- | ---: | ---: | ---: | ---: |
| flush throughput | 36996 | 36892 | 36847 | 36892 |
| compaction throughput | 72681 | 86151 | 86263 | 86151 |
| separated blob values | 47327 | 43088 | 46869 | 46869 |

`separated blob values` improved from 52841 us to a 46869 us median, about
11.3 percent on this local run. Flush did not improve because it usually writes
only an SSTable before the manifest, so the directory sync count is unchanged.
Compaction remained noisy in this small harness, so this change should not be
claimed as a compaction win until a larger benchmark confirms it.

## Implementation

- Blob and SSTable writes still sync file contents before rename.
- Flush and compaction now sync the database directory once after all new
  table/blob renames and before manifest publish.
- Manifest and recovery-report publish paths still sync the parent directory
  immediately after their own rename because those files are direct durable
  cutover/report files.

## Verification

- `cargo test sync_parent_dir_after_rename_accepts_published_file`
- `cargo test publish_failure_removes_unpublished_table_and_blob_files`
- `cargo bench --bench v1_bench` before and after tuning

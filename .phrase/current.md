# Current Phase

## Status

Complete

## Goal

Harden the public bucket API contract before implementing Titan-like
large-value storage.

## Entry Condition

- Phase 32 completed the large-value storage spec.
- User asked whether the current bucket API is good enough before key-value
  separation work.
- API audit found that default-bucket and named-bucket entry points were still
  mixed in tests, benches, and protocol docs.

## Scope

- Keep `Db` as the direct operation handle for the built-in default bucket.
- Keep `Bucket` as the handle for optional named buckets and advanced default
  bucket use, such as snapshot APIs that need a bucket handle.
- Make default `WriteBatch` and `Transaction` methods target the default
  bucket without a bucket name.
- Keep named-bucket batch and transaction methods explicit with `_bucket`
  suffixes.
- Use `Db::bucket` as the common get-or-create entry for named buckets, and
  `Db::bucket_with_options` when callers need fixed non-default options.
- Reserve `"default"` so callers cannot open the default bucket as a named
  bucket.
- Update protocol, usage docs, tests, examples, and benches.

## Out Of Scope

- Implementing the new Titan-like blob file format.
- Changing WAL/table file formats solely for the bucket API hardening.
- Adding bucket drop/rename APIs.

## Acceptance Gate

- `Db::put/get/range/prefix` operate on the default bucket.
- `Db::bucket("default")` and
  `Db::bucket_with_options("default", ...)` return invalid-options
  errors.
- `Db::bucket(name)` returns an existing named bucket or creates one with
  default bucket options.
- `Db::bucket_with_options(name, options)` returns an existing named bucket
  only when options match, or creates one with those fixed options.
- `WriteBatch::put/delete/delete_range` target the default bucket, while
  `put_bucket/delete_bucket/delete_range_bucket` validate named bucket names
  before staging operations.
- `Transaction::get/put/delete/read_range` target the default bucket, while
  `_bucket` variants validate named bucket names or read named buckets.
- Default bucket options are configured through `DbOptions`.
- Protocol and usage docs describe the final contract.
- Rust verification passes.

## Active Task Slice

```text
task108 [x] goal:audit bucket API boundary | scope:src docs tests benches | verify:rg API scan
task109 [x] goal:harden default/named bucket API | scope:src tests benches examples | verify:cargo check + focused tests
task110 [x] goal:update bucket API docs and evidence | scope:.phrase docs | verify:doc scan
task111 [x] goal:rename named bucket entrypoint | scope:src docs tests examples | verify:cargo test
task112 [x] goal:show common README capabilities | scope:README.md | verify:quickstart + doc scan
```

## Known Blockers

- Remote CI cannot be executed locally; it must run after push.

## Evidence

- API scan found `bucket_with_options("default", ...)` still used outside
  the intended default `Db` path.
- The earlier batch/transaction API still made common default operations look
  like named-bucket operations.
- User feedback found the older named-bucket entrypoint too long and less
  accurate than `bucket`, because the intended behavior is get-or-create.
- User feedback found the README too minimal; it now shows the common API
  surface instead of only `put/get`.
- Full local Rust verification passed after tightening the API and renaming the
  named-bucket entrypoint.

## Next Recommendation

- Continue with Phase 34 `BlobIndex` and `BlobFile` format tests.

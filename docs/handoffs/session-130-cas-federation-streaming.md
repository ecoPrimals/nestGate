# Session 130 — CAS Federation Streaming + Size Guard + Remote Decoupling

**Date**: Aug 3, 2026 (Wave 155n)

## Changes

- **Federation streaming for large blobs**: `content.replicate` and `content.replicate.pull`
  now use chunked streaming (4 MiB chunks via `content.store_stream` / `content.retrieve_stream`)
  for blobs > 16 MiB. Below threshold: inline base64 `content.put`/`content.get` (single call).
- **`content.get` inline size guard**: Blobs > 64 MiB return `use_streaming: true` with size
  metadata instead of the full base64 payload.
- **Transparent streaming fallback**: `pull_blob_from_remote` probes `content.exists` first for
  size, transparently switches to chunked retrieve if needed.
- **Remote decoupling**: Hardcoded `"forgejo"` → `NESTGATE_PREFERRED_REMOTE` env var.
- **Test extraction**: Production file from 980 → 797 lines (under 800 limit).
- **Atomic writes**: `verify_and_write` uses temp-file-then-rename for integrity.

## Validation

- `cargo check --all-features` PASS
- `cargo clippy --all-features -D warnings` zero warnings
- Relevant tests pass

## Files Modified

- `code/crates/nestgate-rpc/src/rpc/unix_socket_server/content_handlers/cas.rs`
- `code/crates/nestgate-rpc/src/rpc/unix_socket_server/content_federation_handlers.rs`
- `code/crates/nestgate-rpc/src/rpc/unix_socket_server/content_federation_handlers_tests.rs` (new)
- `code/crates/nestgate-rpc/src/rpc/unix_socket_server/federation_ops.rs`

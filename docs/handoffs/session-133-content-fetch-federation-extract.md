# Session 133 — content.fetch Streaming Fix + Federation Extraction

**Date**: Aug 4, 2026 (Wave 155u/156b)  
**Commit**: `c3dc2316`

## Changes

### content.fetch Handler Fixed (compile + design)
- **Compile fix**: ureq 3.x `Body` doesn't implement `std::io::Read`; switched to `body.as_reader()` for chunked reads
- **Stream to disk**: Rewrote from full-response `Vec<u8>` buffer to streaming `.part` file — BLAKE3 hash computed during download, O(1) memory for arbitrary file sizes
- **`unwrap()` removed**: `file_name().unwrap()` replaced with hash-based meta path construction
- **Meta write errors propagated**: `let _ = std::fs::write(...)` → proper `map_err` propagation
- **Unused imports cleaned**: `get_storage_base_path`, `ensure_parent_dirs` removed
- **`https_only` correctness**: Now derived from actual URL scheme (was hardcoded `false`)
- **Hash mismatch cleanup**: `.part` file removed on BLAKE3 mismatch (was leaked)
- **Clippy-pedantic clean**: Cast safety via `#[expect]`, `checked_sub` for Duration, collapsible-if

### Federation Blob Transfer Extraction
- **New module**: `federation_blob_transfer.rs` (396 lines) — contains `pull_blob_from_remote`, `pull_blob_inline`, `pull_blob_streamed`, `verify_and_write`, `replicate_blob_to_remote`, `replicate_blob_inline`, `replicate_blob_streamed`
- **Main file**: `content_federation_handlers.rs` reduced from 802 → 420 lines (under 800-line budget)
- Constants `FEDERATION_STREAM_THRESHOLD` and `FEDERATION_CHUNK_SIZE` moved with helpers

### HTTP Client Deduplication
- **Shared utilities in `storage_paths.rs`**:
  - `validate_fetch_url()` — URL scheme validation (HTTPS required unless HTTP explicitly allowed)
  - `build_http_agent()` — ureq agent with `rustls-rustcrypto` TLS provider
  - `http_user_agent()` — standard `NestGate/{version}` User-Agent string
- **Removed duplicates** from `fetch.rs` (inline `validate_fetch_url`, inline agent setup) and `external_handlers.rs` (identical `validate_fetch_url`, inline agent setup)

## Validation
- `cargo check --all-features` — PASS
- `cargo clippy --all-features -- -D warnings` — PASS (0 warnings)
- `cargo test --all-features` — 1,630 passed, 0 failed, 80 ignored

## Files Changed
- `content_handlers/fetch.rs` — rewritten (compile fix + streaming + shared HTTP)
- `content_federation_handlers.rs` — 802→420 lines (helpers extracted)
- `federation_blob_transfer.rs` — NEW (396 lines, extracted helpers)
- `external_handlers.rs` — deduped HTTP client setup
- `storage_paths.rs` — added shared HTTP utilities
- `mod.rs` — registered new module

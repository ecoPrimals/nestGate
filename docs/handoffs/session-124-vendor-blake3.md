# Session 124: Vendor Elimination + BLAKE3 Crypto Consolidation

**Date**: Jul 20, 2026  
**Wave**: 150q  
**Commit**: `33b15599`

## Changes

### Vendor Elimination

- Replaced vendored `rustls-webpki 0.103.12` + `rustls-rustcrypto
  0.0.2-alpha` with published `oxitls-rustcrypto-provider 0.2.1` — a
  fork that drops the `rustls-webpki` dependency and routes through
  `rustls-pki-types`. RUSTSEC-2026-0104 fix included. Pure Rust. MIT/Apache-2.0.
- Package rename (`rustls-rustcrypto = { package = "oxitls-rustcrypto-provider" }`)
  preserves the `rustls_rustcrypto::provider()` import path — zero code changes.
- Removed `vendor/` directory (65 files, 692KB) and `[patch.crates-io]` block.

### BLAKE3 Crypto Consolidation

- Auth token MACs: HMAC-SHA256 → `blake3::keyed_hash` (nestgate-security)
- Certificate fingerprints: SHA-256 → `blake3::hash` (nestgate-security)
- BTSP Phase 3 KDF: HKDF-SHA256 → `blake3::derive_key` (nestgate-rpc)
- Storage checksums: SHA-256 → `blake3::hash` (nestgate-core)
- `SessionKeys::derive` and `derive_handshake_key` made infallible
- `sha2`/`hmac` removed from nestgate-security, nestgate-rpc, nestgate-core
- `hkdf` removed entirely from workspace
- `sha2`/`hmac` optional in nestgate-zfs (`s3-backend` feature gate)

### Silicon Atheism Verification

- `ring` present in lockfile but NOT activated (optional dep of `rustls-webpki`,
  no feature enables it) — confirmed via `cargo tree`
- `cc` crate is a build dep for `blake3` (CPU feature detection only); with
  `pure` feature, no C/asm compilation occurs
- Build closure confirmed C-free

## Scorecard

- Tests: 1,630 passed, 80 ignored
- Clippy: 0 warnings
- Vendor TODOs eliminated: 27
- Vendor >800L files eliminated: 4

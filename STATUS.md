# NestGate - Current Status

**Last Updated**: Jul 18, 2026 (Wave 149b — Session 119: cargo fmt, GAP-038 PID liveness check, dimensional audit)
**Version**: 0.5.0

---

## Quick Metrics

```
Build:              PASS — cargo check --workspace --all-features --all-targets (0 errors)
Clippy:             PASS — cargo clippy --all-features -- -D warnings (zero warnings in nestgate crates)
Format:             CLEAN (cargo fmt --all -- --check passes)
Docs:               PASS — RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features (zero errors/warnings)
Tests:              3,790 passed, 73 ignored (1 pre-existing env-specific) — cargo test --workspace
Coverage:           84%+ line (cargo llvm-cov --workspace; CI floor 80%) — 90% target pending
Files > 800 lines:  ZERO in production src/ (content_handlers.rs split → 4-file directory module; all files with inline tests extracted to siblings)
Unwrap/Expect:      10 documented .expect() in production (OnceLock init + invariant guards + const timestamp)
Inline markers:     none in committed production `.rs` (wateringHole policy)
Unsafe code:        #![forbid(unsafe_code)] on ALL 20 crate roots (zero exceptions)
println! in lib:    ZERO in core libs; installer retains stdout for interactive wizard UX (documented)
Dead code:          ZERO #[allow(dead_code)]; stub modules use #[expect(dead_code, reason=...)] per ecosystem standard
Box<dyn Error>:     ZERO in production library code
async-trait:        ZERO compiled usages, ZERO dependency (not in any Cargo.toml)
Mocks in prod:      ZERO fabricated metrics — 11 ZFS handlers evolved to honest not_implemented; system memory from /proc/meminfo (was hardcoded); ARC fallback 0.0 (was 0.85); dev_environment gated behind `dev-stubs` feature
Stubs:              Feature-gated behind `dev-stubs` cargo feature (opt-in only, zero production leakage)
TLS/crypto:         ureq + rustls-rustcrypto (pure Rust); ring/reqwest/openssl/native-tls ELIMINATED
Encrypt-at-rest:    ChaCha20-Poly1305
Auth mode:          NESTGATE_AUTH_MODE=delegated|external — auth delegated to security capability provider
Discovery:          Environment variables + capability IPC; 6-tier security socket discovery (XDG-based, zero hardcoded FHS paths)
Env aliases:        Legacy BEARDOG_* aliases REMOVED — canonical SECURITY_PROVIDER_SOCKET / FAMILY_SEED only
IPC routes (UDS):   storage.*, content.*, session.*, model.*, templates.*, audit.*, nat.*, beacon.*, zfs.*, bonding.ledger.*, coord.*, footprint.*, health.*, capabilities.*, identity.*, discovery.*, auth.*, lifecycle.*, btsp.* — 90 methods
IPC routes (HTTP):  Aligned with UDS namespace (storage.store not storage.object.store); legacy aliases warn
IPC routes (tarpc): storage.*, content.*, metadata.*, crypto.*, session.*, discovery.*, health.*, capabilities.*, lifecycle.* — 52 semantic-routed methods
content.* parity:   ALL transport paths — UDS dispatch, SemanticRouter, isomorphic IPC, HTTP API
Wire Standard:      Level 3 (Composable) — {primal, version, capabilities} envelope, protocol: "jsonrpc-2.0", transport: ["uds", "tcp", "http"]
BTSP:               Phase 1-3 PASS — family-scoped sockets, server-side handshake, ChaCha20-Poly1305 encrypted channel
MethodGate:         Public/Protected method classification; NESTGATE_AUTH_MODE=enforced rejects unauthed protected calls
TCP JSON-RPC:       Functional — --port, --listen, NESTGATE_API_PORT activates alongside UDS
Constants:          Runtime-configurable via LazyLock + env vars (NESTGATE_ZFS_*, NESTGATE_FALLBACK_PORT_*)
Serial tests:       #[serial]: env-var-sensitive tests in nestgate-rpc (55), nestgate-config (20), ZFS stubs
Supply chain:       cargo deny check — advisories ok, bans ok, licenses ok, sources ok; ring ELIMINATED
Workspace:          20 crates, 100% hoisted deps, Rust 2024 edition
Platforms:          6+ (Linux, FreeBSD, macOS, WSL2, illumos, Android)
Registry:           capability_registry.toml — machine-readable self-knowledge (20 capability domains)
CONTEXT.md:         Present (per wateringHole PUBLIC_SURFACE_STANDARD)
```

---

## Session History

Per-session detail (Sessions 43–119) lives in [`CHANGELOG.md`](CHANGELOG.md) and `docs/handoffs/`.

Recent sessions:
- **Session 119** (Wave 149b): `cargo fmt` (133 files); GAP-038 PID sidecar liveness check (socket conflict detection before unlink); btsp `is_btsp_required` → `#[cfg(test)]`; dimensional audit aligned with ecosystem scorecard; wave stamps → 149b
- **Session 118** (Wave 149b): Deep debt sweep — 292 dead code warnings → 0 (stale imports removed, stub modules gated with `#[expect(dead_code)]`); 8 let-chain modernizations; 30 clippy errors → 0; removed dead `AnalysisConfigCanonical` alias, unfulfilled `async_fn_in_trait` expects; unused `BTreeMap` import
- **Session 117** (Wave 149b): Phase 2 Transport — `TransportStream`/`TransportListener` types, server accept loop unified, client connect consolidated
- **Session 116** (Wave 149b): Typed JSON-RPC errors — canonical `JsonRpcErrorCode` enum + `JsonRpcError` in `nestgate-types`, replaced 6 duplicate structs, ~97 stringly-typed error sites → typed; `pub(crate)` tightening (10 modules, `models.rs`, `rest`); removed `/opt/ecoPrimals/depot` hardcoded fallback, security socket tier-6 → XDG-based
- **Session 115** (Wave 149b): ErrorContextExt trait — 152 map_err(format!()) sites → .io_ctx/.net_ctx/.internal_ctx/.api_ctx/.validation_ctx/.security_ctx, 42 remaining (runtime-interpolated)
- **Session 114** (Wave 143b): PROJECTS_PATH CAS wiring (footPrint composition), String::from R8 sweep (2500+ across 382 files), deep debt continuation
- **Session 113** (Wave 142b): Production mock evolution — ZFS defaults zeroed, tier utilization real, AI confidence computed, String::from R7 (21)
- **Session 112** (Wave 142b): Visibility tightening — `btsp_client`/`btsp_phase3`/`primal_announce` → `pub(crate)`, infallible nonce, unwrap_or sweep (31)
- **Session 111** (Wave 141b): Streaming clone elimination — 4 function signatures `Value` → `&Value`, cast safety fix, String::from R6 (55), cross-arch 14/14
- **Session 110** (Wave 141a): Production mock evolution — 11 ZFS fake handlers → honest `not_implemented`; `/proc/meminfo` for real memory; ARC fallback 0.85→0.0
- **Session 109** (Wave 141a): Cross-architecture adoption — `cargo check --target x86_64-pc-windows-gnu` PASS
- **Session 108** (Wave 140a): Deep debt sweep — test fixture gating, platform FS audit, String::from R4 (63)
- **Session 107** (Wave 139a): Deep debt sweep — ~425 String::from→.into(), ZfsError→thiserror, install path→env override
- **Session 106** (Wave 136b): COORD-ACTIVATE + FP-PERSIST — coordination and footprint wired to all 4 surfaces

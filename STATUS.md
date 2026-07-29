# NestGate - Current Status

**Last Updated**: Jul 29, 2026 (Wave 155i — CAS on ZFS configured, deep debt complete)
**Version**: 0.5.0

---

## Quick Metrics

```
Build:              PASS — cargo check --workspace --all-features (0 errors)
Clippy:             PASS — cargo clippy --all-features -- -D warnings (zero warnings, pedantic+nursery)
Format:             CLEAN — cargo fmt --check passes
Tests:              13,095+ passed, 0 failed, ~430 ignored
Files > 800 lines:  ZERO in production src/
Unwrap/Expect:      deny(unwrap_used), deny(expect_used) in workspace lints — zero in production
Inline markers:     none in committed production .rs (deny(todo), deny(unimplemented))
Unsafe code:        #![forbid(unsafe_code)] on ALL 20 crate roots (zero exceptions)
println! in lib:    ZERO in core libs; installer retains stdout for interactive wizard UX
Dead code:          ZERO #[allow(dead_code)]; stubs use #[expect(dead_code, reason=...)]
Mocks in prod:      ZERO fabricated metrics; all stubs honest not_implemented; dev_environment gated behind dev-stubs feature
TLS/crypto:         ureq + oxitls-rustcrypto-provider (pure Rust TLS); internal crypto BLAKE3; ring/reqwest/openssl ELIMINATED
Encrypt-at-rest:    ChaCha20-Poly1305
External deps:      Pure Rust — zero C build deps, no OpenSSL/ring, no cloud SDKs, 13 top-level runtime deps
Discovery:          Environment variables + capability IPC; XDG-compliant path resolution via etcetera; zero hardcoded FHS paths in consumers
CLI health/status:  Live UDS probe via JsonRpcClient — resolves socket, sends health.check JSON-RPC
Path resolution:    EnvSource injection works correctly (injected HOME > etcetera auto-detect > system fallback)
IPC routes (UDS):   storage.*, content.*, session.*, model.*, templates.*, audit.*, nat.*, beacon.*, zfs.*, bonding.ledger.*, coord.*, footprint.*, health.*, capabilities.*, identity.*, discovery.*, auth.*, lifecycle.*, btsp.* — 90 methods
IPC routes (HTTP):  Aligned with UDS namespace; legacy aliases warn
IPC routes (tarpc): 52 semantic-routed methods
Wire Standard:      Level 3 (Composable) — {primal, version, capabilities} envelope
BTSP:               Phase 1-3 PASS — family-scoped sockets, encrypted channel, CAS federation wired
Workspace:          20 crates, Rust 2024 edition, 100% hoisted deps
Repository:         https://git.primals.eco/ecoPrimals/nestGate
Registry:           capability_registry.toml — machine-readable self-knowledge (20 capability domains)
CONTEXT.md:         Present (per wateringHole PUBLIC_SURFACE_STANDARD)
```

---

## Session History

Per-session detail (Sessions 43–129) lives in [`CHANGELOG.md`](CHANGELOG.md) and `docs/handoffs/`.

Recent sessions:
- **Wave 155i** (Jul 29): CAS on ZFS configured, deep debt sweep complete. CLI probe commands evolved (bypass JWT). Flaky test fixed. File renames for naming accuracy. P1 ghost methods resolved. Live composition verified (8 services, 1,704 capabilities). 113.7 GiB `cargo clean`.
- **Session 126** (Wave 151c): BTSP ClientHello shipped — `btsp_client_handshake.rs` implements full outbound wire handshake (P1 Nest Atomic blocker resolved); dead `BtspClient` stub removed from `btsp_client.rs`; `JsonRpcClient::from_btsp_stream` + `connect_with_btsp` integration hook; 8 dep bumps (cc, clap, either, libc, rustls-pki-types, syn, tokio-stream)
- **Session 125** (Wave 150t): Procfs consolidation phase 3 — 17 `/proc` callsites in nestgate-api consolidated to `linux_proc` delegates (hardware_tuning, metrics_collector, performance_dashboard); 6 dep bumps (tokio, libc, tokio-util, zerocopy); clippy cleanup; wave stamps → 150t
- **Session 124** (Wave 150q): Vendor elimination + BLAKE3 crypto consolidation — replaced vendored TLS crates with `oxitls-rustcrypto-provider 0.2.1`; all internal crypto (auth tokens, BTSP KDF, checksums, cert fingerprints) consolidated to BLAKE3; `sha2`/`hmac`/`hkdf` removed as direct deps (sha2/hmac optional behind `s3-backend` in nestgate-zfs); `vendor/` + `[patch.crates-io]` removed; 27 TODOs + 4 >800L gone; wave stamps → 150q
- **Session 123** (Wave 150o): 150o dimensional audit triage — 27 TODOs, 5 >800L, 52 unsafe all confirmed in vendor/ (zero in nestGate code); procfs consolidation phase 2 — 3 more callsites (ZFS readiness, performance analyzer) → `linux_proc`; 18 dep bumps; wave stamps → 150o
- **Session 122** (Wave 150g): Procfs consolidation — `SystemHealthProvider` evolved from raw `/proc` reads to `nestgate_platform::linux_proc` (platform-agnostic); 4 scattered `/proc/meminfo`/`/proc/uptime`/`/proc/loadavg` reads in discovery, storage, API, and websocket → `linux_proc` functions; wave stamps → 150g
- **Session 121** (Wave 150d): Prod unwrap deep audit — full 14-crate scan confirmed 0 `.unwrap()`, 10 `.expect()` in production (all justified, annotated with `#[expect(clippy::expect_used)]`); wave stamps → 150d
- **Session 120** (Wave 150b): 99 dependency patch bumps (all semver-compatible); socket path ecosystem segment — legacy `$XDG_RUNTIME_DIR/{service}.sock` → `$XDG_RUNTIME_DIR/<ecosystem>/{service}.sock` across discovery, launcher, and server fallback paths (GAP-036 alignment); dimensional scorecard audit (1,710 tests / 0 clippy / 0 fmt / 0 unsafe / 0 >800L); wave stamps → 150b
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

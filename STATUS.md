# NestGate - Current Status

**Last Updated**: Jun 10, 2026 (Session 101)  
**Version**: 0.5.0

---

## Quick Metrics

```
Build:              PASS — cargo check --workspace --all-features --all-targets (0 errors), as of Jun 10, 2026
Clippy:             PASS — cargo clippy --workspace -- -D warnings (zero warnings), as of Jun 10, 2026
Format:             CLEAN (cargo fmt --check passes), as of Jun 10, 2026
Docs:               PASS — cargo doc --workspace --no-deps (zero warnings), as of Jun 10, 2026
Tests:              3,863 total (2,325 lib, 867 RPC), 0 failures — as of Jun 10, 2026 (Session 101)
Coverage:           84%+ line (cargo llvm-cov --workspace; measured May 26, 2026) — wateringHole 80% met; 90% target pending
Files > 800 lines:  ZERO in default production build
Unwrap/Expect:      ZERO in production library code
Inline markers:     none in committed production `.rs` (wateringHole policy)
Unsafe code:        #![forbid(unsafe_code)] on ALL crate roots (zero exceptions)
println! in lib:    ZERO in core libs; installer retains stdout for interactive wizard UX (documented)
Dead code:          ZERO unwired modules, ZERO `if false` stubs, ZERO #[allow(dead_code)] in production
Box<dyn Error>:     ZERO in production library code
async-trait:        ZERO compiled usages, ZERO dependency (not in any Cargo.toml)
Mocks in prod:      ZERO — all mocks test-only (#[cfg(test)]) or feature-gated (dev-stubs)
Stubs:              Feature-gated behind `dev-stubs` cargo feature (opt-in only, zero production leakage)
TLS/crypto:         ureq + rustls-rustcrypto (pure Rust); ring/reqwest/openssl/native-tls ELIMINATED
Encrypt-at-rest:    ChaCha20-Poly1305
Auth mode:          NESTGATE_AUTH_MODE=delegated|external — auth delegated to security capability provider
Discovery:          Environment variables + capability IPC; 6-tier security socket discovery
IPC routes (UDS):   storage.*, content.*, session.*, model.*, templates.*, audit.*, nat.*, beacon.*, zfs.*, bonding.ledger.*, health.*, capabilities.*, identity.*, discovery.*, auth.*, lifecycle.*, btsp.* — 77 methods
IPC routes (HTTP):  storage.*, content.*, health.*, capabilities.*, identity.*, system.*, lifecycle.*
IPC routes (tarpc): storage.*, content.*, metadata.*, crypto.*, session.*, discovery.*, health.*, capabilities.*, lifecycle.* — 52 semantic-routed methods
content.* parity:   ALL transport paths — UDS dispatch, SemanticRouter, isomorphic IPC, HTTP API
Wire Standard:      Level 3 (Composable) — {primal, version, capabilities} envelope, protocol: "jsonrpc-2.0", transport: ["uds", "tcp", "http"]
BTSP:               Phase 1-3 PASS — family-scoped sockets, server-side handshake, ChaCha20-Poly1305 encrypted channel
MethodGate:         Public/Protected method classification; NESTGATE_AUTH_MODE=enforced rejects unauthed protected calls
TCP JSON-RPC:       Functional — --port, --listen, NESTGATE_API_PORT activates alongside UDS
Serial tests:       #[serial]: env-var-sensitive tests in nestgate-rpc (55), nestgate-config (20), ZFS stubs
Supply chain:       cargo deny check — advisories ok, bans ok, licenses ok, sources ok; ring ELIMINATED
Workspace:          22 crates, 100% hoisted deps, Rust 2024 edition
Platforms:          6+ (Linux, FreeBSD, macOS, WSL2, illumos, Android)
Registry:           capability_registry.toml — machine-readable self-knowledge
CONTEXT.md:         Present (per wateringHole PUBLIC_SURFACE_STANDARD)
```

---

## Session History

Per-session detail (Sessions 43–101) lives in [`CHANGELOG.md`](CHANGELOG.md) and `infra/wateringHole/handoffs/`.

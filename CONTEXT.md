# Context — NestGate

**Last Updated**: Aug 6, 2026 (Session 140)

## What This Is

NestGate is a pure Rust **storage and discovery primal** in the ecoPrimals sovereign
computing ecosystem: self-contained binaries that coordinate via JSON-RPC 2.0 and
optional tarpc, with zero compile-time coupling between primals. It provides
substrate-agnostic storage orchestration, runtime capability discovery, and
zero-knowledge infant discovery. Human-oriented detail lives in the root `README.md`.

## Role in the Ecosystem

NestGate owns **persistent storage abstraction** and **discovery** for gates and
sibling primals. Other ecosystem components (security, network, device, compute,
and visualization primals) integrate at runtime via IPC and capabilities rather
than by importing this crate graph.

## Technical Facts

| Field | Value |
|-------|--------|
| **Version** | 0.5.0 |
| **Language** | Rust 2024 edition (env-process-shim: 2021 for safe env mutation); 100% Rust application code |
| **License** | AGPL-3.0-or-later (code); CC-BY-SA 4.0 (documentation) |
| **Tests** | 1,630+ passed, 0 failed (~80 ignored) |
| **Coverage** | 84%+ line (`cargo llvm-cov`); 90% target pending |
| **Architecture** | 18 workspace crates under `code/crates/` |
| **Binary** | Single self-contained release binary |
| **IPC** | JSON-RPC 2.0 (required); tarpc via G65 protocol negotiation on UDS + TCP (G66 transport abstraction); C2 dual-socket retained for backward compat |
| **TLS/crypto** | `ureq` + `oxitls-rustcrypto-provider` (pure Rust TLS); internal crypto BLAKE3; ring/reqwest/openssl eliminated; installer uses system `curl` |
| **Unsafe** | `#![forbid(unsafe_code)]` on ALL crate roots (zero exceptions) |
| **Lint / format** | `cargo clippy --workspace --all-targets --all-features -- -D warnings` zero warnings (pedantic + nursery); `cargo fmt --check` clean |
| **Docs** | `cargo doc --workspace --no-deps` — clean with `-D warnings` |
| **Platforms** | Linux, FreeBSD, macOS, WSL2, illumos, Android; Windows cross-arch builds (G66 transport abstraction) |
| **Registry** | `config/capability_registry.toml` — 21 capability domains, machine-readable self-knowledge |

### Workspace crates (authoritative list)

`nestgate-types`, `nestgate-config`, `nestgate-core`, `nestgate-api`, `nestgate-rpc`,
`nestgate-zfs`, `nestgate-discovery`, `nestgate-security`,
`nestgate-cache`, `nestgate-observe`, `nestgate-storage`, `nestgate-performance`,
`nestgate-canonical`,
`nestgate-installer`, `nestgate-nas`,
`nestgate-platform`, `nestgate-bin`, `nestgate-env-process-shim`.

Deprecated/shed (removed from workspace): `nestgate-network`, `nestgate-automation`, `nestgate-mcp`.

### Canonical trait surface (high level)

`CanonicalProvider`, `CanonicalStorage`, `CanonicalSecurity`, `CanonicalNetwork`,
`CanonicalService` — unified boundaries for configuration, storage, security,
networking, and service lifecycle across the adapter stack.

### Storage & discovery

- **Storage:** Substrate-agnostic — ZFS, ext4, btrfs, xfs, and object-style backends.
- **Discovery:** Zero-knowledge infant discovery plus capability-based runtime
  discovery aligned with ecosystem IPC expectations.

## Key Capabilities

- Expose storage and workspace lifecycle operations over JSON-RPC (and optionally tarpc).
- Discover and advertise capabilities at runtime without hard-wiring peer primals.
- Integrate observability, caching, ZFS-specific paths, and NAS surfaces
  as named workspace crates (see crate list above). MCP is delegated to the
  orchestration layer via `capability.call`.

## What This Does Not Do

- Does **not** compile or own the WGSL/GPU shader pipeline (delegated to compute capability providers).
- Does **not** replace the security primal for the core cryptographic identity and signing stack.
- Does **not** act as the hardware/VFIO dispatch layer (delegated to device capability providers).
- Does **not** import other primals’ code; coordination is IPC-only at ecosystem boundaries.

## Related Repositories

- [wateringHole](https://git.primals.eco/ecoPrimals/wateringHole) — standards, registry, `PUBLIC_SURFACE_STANDARD.md`, `STANDARDS_AND_EXPECTATIONS.md`
- [ecoPrimals org](https://git.primals.eco/ecoPrimals) — sibling primals and springs (security, network, device, compute, visualization, and other capability providers)

## Design Philosophy

Primals are evolved under strong Rust semantics (ownership, lifetimes, types) and
minimal, justified dependencies. Complexity is pushed to **runtime** coordination
(JSON-RPC, capabilities), not compile-time coupling between repositories.

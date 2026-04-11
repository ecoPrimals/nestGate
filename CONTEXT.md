# Context — NestGate

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
| **Version** | 4.7.0-dev |
| **Language** | Rust 2024 edition; 100% Rust application code |
| **License** | AGPL-3.0-or-later (code); CC-BY-SA 4.0 (documentation) |
| **Architecture** | 23 workspace members: 20 `code/crates/*` + `tools/unwrap-migrator` + fuzz + root |
| **Binary** | Single self-contained static release binary (~4.7 MB, musl) |
| **IPC** | JSON-RPC 2.0 (required); tarpc (optional, high-performance path) |
| **TLS/crypto** | `ureq` + `rustls-rustcrypto` (pure Rust); ring/reqwest/openssl eliminated; installer uses system `curl` |
| **Unsafe** | `#![forbid(unsafe_code)]` on ALL crate roots (zero exceptions) |
| **Lint / format** | `cargo clippy --workspace --lib` zero warnings (pedantic + nursery); `cargo fmt --check` clean |
| **Docs** | `cargo doc --workspace --no-deps` — clean in routine runs |
| **Tests** | `cargo test --workspace --all-features` — ~11,856 passing, 461 ignored, 0 failures (see STATUS.md) |
| **Coverage** | ~80% line (llvm-cov) — wateringHole 80% minimum met |
| **Platforms** | Linux, FreeBSD, macOS, WSL2, illumos, Android |
| **Specs** | 16 specification documents under `specs/` |

### Workspace crates (authoritative list)

`nestgate-types`, `nestgate-config`, `nestgate-core`, `nestgate-api`, `nestgate-rpc`,
`nestgate-zfs`, `nestgate-discovery`, `nestgate-security`,
`nestgate-cache`, `nestgate-observe`, `nestgate-storage`, `nestgate-performance`,
`nestgate-canonical`, `nestgate-fsmonitor`,
`nestgate-installer`, `nestgate-middleware`, `nestgate-nas`,
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
- Integrate observability, caching, ZFS-specific paths, and NAS/middleware surfaces
  as named workspace crates (see crate list above). MCP is delegated to biomeOS via
  `capability.call`.

## What This Does Not Do

- Does **not** compile or own the WGSL/GPU shader pipeline (delegated to compute capability providers).
- Does **not** replace the security primal for the core cryptographic identity and signing stack.
- Does **not** act as the hardware/VFIO dispatch layer (delegated to device capability providers).
- Does **not** import other primals’ code; coordination is IPC-only at ecosystem boundaries.

## Related Repositories

- [wateringHole](https://github.com/ecoPrimals/wateringHole) — standards, registry, `PUBLIC_SURFACE_STANDARD.md`, `STANDARDS_AND_EXPECTATIONS.md`
- [ecoPrimals org](https://github.com/ecoPrimals) — sibling primals and springs (security, network, device, compute, visualization, and other capability providers)

## Design Philosophy

Primals are evolved under strong Rust semantics (ownership, lifetimes, types) and
minimal, justified dependencies. Complexity is pushed to **runtime** coordination
(JSON-RPC, capabilities), not compile-time coupling between repositories.

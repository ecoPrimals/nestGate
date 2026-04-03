# NestGate Specifications

**Last Updated**: April 3, 2026

Architectural specifications for nestGate — the storage and discovery primal
in the ecoPrimals ecosystem.

For current build, test, and lint status (not duplicated here), see [STATUS.md](../STATUS.md) and the root [README.md](../README.md).

---

## Core Architecture

| Specification | Purpose |
|---------------|---------|
| [NESTGATE_CORE_DOMAIN_SPEC.md](./NESTGATE_CORE_DOMAIN_SPEC.md) | Core domain boundaries -- what nestGate owns and does not own |
| [ZERO_COST_ARCHITECTURE_FINAL_SPECIFICATION.md](./ZERO_COST_ARCHITECTURE_FINAL_SPECIFICATION.md) | Zero-cost abstraction patterns and native async design |
| [INFANT_DISCOVERY_ARCHITECTURE_SPEC.md](./INFANT_DISCOVERY_ARCHITECTURE_SPEC.md) | Zero-knowledge startup and primal discovery protocol |
| [UNIVERSAL_STORAGE_AGNOSTIC_ARCHITECTURE.md](./UNIVERSAL_STORAGE_AGNOSTIC_ARCHITECTURE.md) | Substrate-agnostic storage: works with ZFS, ext4, btrfs, xfs, object stores, or bare filesystem |
| [PRIMAL_ECOSYSTEM_INTEGRATION_SPEC.md](./PRIMAL_ECOSYSTEM_INTEGRATION_SPEC.md) | Cross-primal sovereignty, discovery, and integration patterns |

## Service Specifications

| Specification | Purpose |
|---------------|---------|
| [UNIVERSAL_RPC_SYSTEM_SPECIFICATION.md](./UNIVERSAL_RPC_SYSTEM_SPECIFICATION.md) | JSON-RPC 2.0 + tarpc IPC system and semantic routing |
| [NESTGATE_DATA_SERVICE_SPECIFICATION.md](./NESTGATE_DATA_SERVICE_SPECIFICATION.md) | Data service scope and API surface |
| [NESTGATE_NETWORK_MODERNIZATION_SPEC.md](./NESTGATE_NETWORK_MODERNIZATION_SPEC.md) | Network layer modernization and transport strategy |
| [UNIVERSAL_ADAPTER_MODULE_ARCHITECTURE_SPEC.md](./UNIVERSAL_ADAPTER_MODULE_ARCHITECTURE_SPEC.md) | Universal adapter module layout and backend abstraction |
| [STEAM_DATA_SERVICE_SPEC.md](./STEAM_DATA_SERVICE_SPEC.md) | Steam/game data service domain spec |

## Data and Compression

| Specification | Purpose |
|---------------|---------|
| [ADAPTIVE_COMPRESSION_ARCHITECTURE.md](./ADAPTIVE_COMPRESSION_ARCHITECTURE.md) | Entropy-aware compression pipeline and cross-primal delegation |
| [CROSS_PRIMAL_COMPRESSION_INTERACTIONS.md](./CROSS_PRIMAL_COMPRESSION_INTERACTIONS.md) | Compression responsibility boundaries across primals |
| [SELF_CONTAINED_STORAGE_IMPLEMENTATION_PLAN.md](./SELF_CONTAINED_STORAGE_IMPLEMENTATION_PLAN.md) | Pure-Rust self-contained storage backend design |
| [SIMD_PERFORMANCE_SPECIFICATION.md](./SIMD_PERFORMANCE_SPECIFICATION.md) | SIMD acceleration architecture for data processing |

## Integration

| Specification | Purpose |
|---------------|---------|
| [COLLABORATIVE_INTELLIGENCE_IMPLEMENTATION.md](./COLLABORATIVE_INTELLIGENCE_IMPLEMENTATION.md) | Collaborative intelligence between primals and biomeOS |

---

## Storage Architecture: Substrate Agnostic

NestGate's internal storage abstraction treats all storage as data
substrates behind a universal interface. The "ZFS" crate provides the
abstraction layer for data handling and distribution, regardless of
whether actual ZFS, ext4, btrfs, xfs, or object storage is underneath.

**Warm tier** (NVMe/SSD): Active datasets, hot caches, metadata indices.
Saturates fast I/O paths for reads and recent writes.

**Cold tier** (HDD): Archival data, snapshots, bulk storage. Optimized
for sequential throughput and capacity.

When on NAS, nestGate prepares and saturates songbird network connections
with data, cycling between warm and cold tiers efficiently.

---

## Fossil Record

Historical status reports, implementation assessments, and dated summaries
have been archived to `ecoPrimals/infra/wateringHole/fossilRecord/nestgate/`.
Git history preserves the complete record.

---

**Source of truth**: [STATUS.md](../STATUS.md) for current metrics.

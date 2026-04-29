# Documentation Quick Guide

**Last Updated**: April 29, 2026 (Session 49)

---

## I Want To...

### Get Started
- [README.md](../../README.md) — Project overview
- [START_HERE.md](../../START_HERE.md) — Quick start guide
- [QUICK_START.md](../../QUICK_START.md) — Build and run

### Understand the Architecture
- [STATUS.md](../../STATUS.md) — Current measured metrics (ground truth)
- [CAPABILITY_MAPPINGS.md](../../CAPABILITY_MAPPINGS.md) — Wire standard and method catalog
- [CONTEXT.md](../../CONTEXT.md) — Project context for new contributors
- [specs/](../../specs/) — Technical specifications

### Contribute
- [CONTRIBUTING.md](../../CONTRIBUTING.md) — Contribution guidelines
- [QUICK_REFERENCE.md](../../QUICK_REFERENCE.md) — Command reference
- [DOCUMENTATION_INDEX.md](../../DOCUMENTATION_INDEX.md) — Full doc index

### Integrate with the Ecosystem
- [docs/integration/biomeos/QUICK_START_BIOMEOS.md](../integration/biomeos/QUICK_START_BIOMEOS.md) — biomeOS integration
- [docs/api/JSONRPC_API_DOCUMENTATION.md](../api/JSONRPC_API_DOCUMENTATION.md) — JSON-RPC API reference

---

## Directory Structure

```
nestgate/
├── README.md, STATUS.md, START_HERE.md    Root docs (ground truth)
├── CAPABILITY_MAPPINGS.md                 Wire standard compliance
├── capability_registry.toml               Machine-readable self-knowledge
├── code/crates/                           23 workspace members
├── specs/                                 Protocol specifications
├── tests/                                 Integration, chaos, e2e tests
├── docs/
│   ├── api/                               JSON-RPC API documentation
│   ├── architecture/                      System design
│   ├── guides/                            Development guides
│   ├── integration/                       Ecosystem integration
│   └── operations/                        Deployment and ops
└── vendor/                                Vendored rustls-rustcrypto
```

---

## Archived Documentation

Historical session reports and handoffs are preserved as fossil record in:
- `ecoPrimals/infra/wateringHole/fossilRecord/nestgate/`
- `ecoPrimals/infra/wateringHole/handoffs/`

These are not in the nestgate repository — they live in the shared infra tree.

---

**Last Updated**: April 29, 2026 (Session 49)

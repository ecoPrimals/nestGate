# Documentation Quick Guide

**Last Updated**: Jul 16, 2026 (Wave 144b)

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
- [capability_registry.toml](../../capability_registry.toml) — Capability and method registry

### Contribute
- [CONTRIBUTING.md](../../CONTRIBUTING.md) — Contribution guidelines
- [QUICK_REFERENCE.md](../../QUICK_REFERENCE.md) — Command reference
- [DOCUMENTATION_INDEX.md](../../DOCUMENTATION_INDEX.md) — Full doc index

### Environment Variables
- [ENVIRONMENT_VARIABLES.md](ENVIRONMENT_VARIABLES.md) — All `NESTGATE_*` env vars

---

## Directory Structure

```
nestgate/
├── README.md, STATUS.md, START_HERE.md    Root docs (ground truth)
├── CAPABILITY_MAPPINGS.md                 Wire standard compliance
├── capability_registry.toml               Machine-readable self-knowledge
├── code/crates/                           20 crates (22 workspace members total incl. root, fuzz)
├── tests/                                 Integration, chaos, e2e tests
├── docs/
│   └── guides/                            Environment variables, this guide
└── vendor/                                Vendored rustls-rustcrypto
```

---

## Archived Documentation

Historical session reports, stale guides, and handoffs are preserved as fossil record in:
- `ecoPrimals/infra/fossilRecord/nestgate/`
- `ecoPrimals/infra/wateringHole/handoffs/`

These are not in the nestgate repository — they live in the shared infra tree.

---

**Last Updated**: Jul 16, 2026 (Wave 144b)

# NestGate Documentation Index

**Last Updated**: Jul 16, 2026 (Wave 150q)  
**Version**: 0.5.0

Paths linked below exist in this repository as of July 2026.

---

## Quick Navigation

### Start Here
- [README.md](./README.md) - Project overview, quick start, current status
- [START_HERE.md](./START_HERE.md) - Getting started guide
- [QUICK_REFERENCE.md](./QUICK_REFERENCE.md) - Essential commands & configuration
- [CONTRIBUTING.md](./CONTRIBUTING.md) - Development guidelines

### Configuration
- [docs/guides/ENVIRONMENT_VARIABLES.md](./docs/guides/ENVIRONMENT_VARIABLES.md) — Environment variable reference

---

## By Topic

### Architecture & API
- [CAPABILITY_MAPPINGS.md](./CAPABILITY_MAPPINGS.md) - Primal capability mappings
- Historical API/architecture docs moved to `ecoPrimals/infra/fossilRecord/nestgate/historical-docs-jun2026/`

### Guides
- [docs/guides/](./docs/guides/) - Environment variables, documentation quick guide
- [docs/guides/ENVIRONMENT_VARIABLES.md](./docs/guides/ENVIRONMENT_VARIABLES.md) - All configuration options
- [docs/guides/DOCS_QUICK_GUIDE.md](./docs/guides/DOCS_QUICK_GUIDE.md) - Doc navigation helper

### Testing
- [tests/](./tests/) - Integration tests, disabled tests reference

---

## Fossil Record

Session archives, planning docs, stale examples, old specs, and historical reports are preserved in
`ecoPrimals/infra/wateringHole/fossilRecord/nestgate/`. Git history retains the full record.

Historical guides and architecture docs live in:
- `ecoPrimals/infra/fossilRecord/nestgate/historical-docs-jun2026/` (Jun 21 cleanup)
- `ecoPrimals/infra/fossilRecord/nestgate/historical-docs-jun2026-wave128b/` (Jun 28: COMMON_TASKS, TROUBLESHOOTING, ZERO_COPY_OPTIMIZATIONS, DEVELOPER_ONBOARDING)

---

## Project Structure

```
nestgate/
├── code/crates/                    # 20 crates (22 workspace packages: + fuzz + root)
│   ├── nestgate-core/              # Core traits, services, adapters
│   ├── nestgate-rpc/               # JSON-RPC 2.0 + tarpc IPC (storage.sock symlink)
│   ├── nestgate-security/          # Crypto delegation (security capability provider)
│   ├── nestgate-bin/               # CLI binary (UniBin)
│   │   └── src/commands/           # CLI command implementations
│   ├── nestgate-api/               # REST + JSON-RPC API server
│   ├── nestgate-zfs/               # ZFS integration (adaptive)
│   └── ...                         # 14 additional crates
├── docs/                           # Documentation (api, architecture, guides)
└── tests/                          # Integration tests
```

Crate-level benchmarks: `code/crates/nestgate-core/benches/`, `code/crates/nestgate-zfs/benches/`.

---

## Document Naming Convention

**Active Documents**: `DOCUMENT_NAME.md` in root  
**Fossil Record**: `ecoPrimals/infra/wateringHole/fossilRecord/nestgate/`  
**Handoffs**: `docs/handoffs/` (in-repo) and upstream `ecoPrimals/infra/wateringHole/handoffs/`

---

**Status**: Documentation cleaned and updated Jul 16, 2026.  
**Ground Truth**: See [STATUS.md](./STATUS.md) for current measured metrics.  
**Last Updated**: Jul 16, 2026

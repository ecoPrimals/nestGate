# NestGate Documentation Index

**Last Updated**: March 30, 2026  
**Version**: 4.7.0-dev

Paths linked below exist in this repository as of March 30, 2026.

---

## Quick Navigation

### Start Here
- [README.md](./README.md) - Project overview, quick start, current status
- [START_HERE.md](./START_HERE.md) - Getting started guide
- [QUICK_REFERENCE.md](./QUICK_REFERENCE.md) - Essential commands & configuration
- [CONTRIBUTING.md](./CONTRIBUTING.md) - Development guidelines

### Configuration
- [config/](./config/) - Production configuration templates
- [production.env.example](./config/production.env.example) — Environment variable examples

---

## By Topic

### Architecture & API
- [docs/api/](./docs/api/) - Complete API reference
- [docs/architecture/](./docs/architecture/) - System design & patterns
- [CAPABILITY_MAPPINGS.md](./CAPABILITY_MAPPINGS.md) - Primal capability mappings
- [specs/](./specs/) - Protocol specifications

### Guides
- [docs/guides/](./docs/guides/) - How-to guides and tutorials
- [docs/guides/ENVIRONMENT_VARIABLES.md](./docs/guides/ENVIRONMENT_VARIABLES.md) - All configuration options
- [docs/guides/TROUBLESHOOTING.md](./docs/guides/TROUBLESHOOTING.md) - Common issues & solutions

### Operations & Testing
- [docs/operations/](./docs/operations/) - Operations runbook
- [tests/](./tests/) - Integration tests, disabled tests reference

---

## Fossil Record

Session archives, planning docs, stale examples, old specs, and historical reports are preserved in
`ecoPrimals/infra/wateringHole/fossilRecord/nestgate/`. Git history retains the full record.

---

## Project Structure

```
nestgate/
├── code/crates/                    # 25 workspace members (see Cargo.toml)
│   ├── nestgate-core/              # Core functionality
│   │   ├── src/rpc/                # JSON-RPC 2.0 + Isomorphic IPC
│   │   ├── src/crypto/             # AES-256-GCM encryption
│   │   └── src/services/storage/   # Universal storage backend
│   ├── nestgate-bin/               # CLI binary (unibin)
│   │   └── src/commands/           # CLI command implementations
│   ├── nestgate-api/               # API server
│   ├── nestgate-zfs/               # ZFS integration
│   ├── nestgate-mcp/               # MCP provider
│   └── ...                         # Additional crates
├── docs/                           # Documentation (api, architecture, guides)
├── tests/                          # Integration tests
├── benches/                        # Performance benchmarks
├── config/                         # Configuration templates
├── scripts/                        # Setup and utility scripts
└── specs/                          # Protocol specifications
```

---

## Document Naming Convention

**Active Documents**: `DOCUMENT_NAME.md` in root  
**Fossil Record**: `ecoPrimals/infra/wateringHole/fossilRecord/nestgate/`  
**Handoffs**: `ecoPrimals/infra/wateringHole/handoffs/`

---

**Status**: Documentation cleaned and updated March 30, 2026.  
**Ground Truth**: See [STATUS.md](./STATUS.md) for current measured metrics.  
**Last Updated**: March 30, 2026

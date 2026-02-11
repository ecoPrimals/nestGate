# NestGate Documentation Index

**Last Updated**: February 11, 2026  
**Version**: 4.1.0-dev

---

## Quick Navigation

### Start Here
- [README.md](./README.md) - Project overview, quick start, current status
- [START_HERE.md](./START_HERE.md) - Getting started guide
- [QUICK_REFERENCE.md](./QUICK_REFERENCE.md) - Essential commands & configuration
- [CONTRIBUTING.md](./CONTRIBUTING.md) - Development guidelines

### Configuration
- [config/](./config/) - Production configuration templates
- [.env.sovereignty](./config/production.env.example) - Environment variable examples

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

### Testing
- [docs/testing/](./docs/testing/) - Test documentation
- [tests/](./tests/) - Integration and unit tests

### Deployment
- [deploy/](./deploy/) - Deployment scripts and configs
- [docker/](./docker/) - Docker production setup
- [examples/](./examples/) - Usage examples

---

## Session Archives (Fossil Record)

### February 2026 (Current)
Located in `docs/sessions/feb_2026/`:
- Evolution Sprint reports
- Deep debt audits and completions
- Model cache integration
- Upstream bug investigations
- Configuration evolution

### January 2026
Located in `docs/sessions/jan_2026/`:
- Isomorphic IPC (Phases 1, 2 & 3)
- Deep Debt Evolution (Phases 1-3)
- Platform code elimination
- genomeBin evolution

### Earlier Sessions
Located in `docs/sessions/` and `docs/session-reports/`:
- Historical evolution records preserved as fossil record

---

## Project Structure

```
nestGate/
├── code/crates/                    # Core crates (13 total)
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
├── docs/                           # Documentation
│   └── sessions/                   # Session archives (fossil record)
├── tests/                          # Integration tests
├── benches/                        # Performance benchmarks
├── examples/                       # Usage examples
├── config/                         # Configuration templates
├── deploy/                         # Deployment scripts
└── showcase/                       # Demo scenarios
```

---

## Document Naming Convention

**Active Documents**: `DOCUMENT_NAME.md` in root
**Session Archives**: `docs/sessions/{month}_{year}/DOCUMENT_NAME_{DATE}.md`
**Preserved as**: Fossil record for historical reference in ecoPrimals

---

**Status**: Documentation cleaned and updated February 11, 2026.  
**Ground Truth**: See [STATUS.md](./STATUS.md) for current measured metrics.  
**Last Updated**: February 11, 2026

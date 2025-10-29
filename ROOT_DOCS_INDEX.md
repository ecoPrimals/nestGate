# NestGate Documentation Index

**Last Updated**: October 28, 2025 - Evening Session

---

## 🚀 **Quick Start**

| Document | Purpose | Audience |
|----------|---------|----------|
| **[START_HERE.md](START_HERE.md)** | First-time setup and orientation | New developers |
| **[QUICK_START_GUIDE.md](QUICK_START_GUIDE.md)** | Fast setup for experienced devs | All developers |
| **[README.md](README.md)** | Project overview and features | Everyone |

---

## 📊 **Current Status** (Updated)

| Document | Last Updated | Status |
|----------|--------------|--------|
| **[CURRENT_STATUS.md](CURRENT_STATUS.md)** | Oct 28, 2025 | ✅ Current |
| **[SESSION_PROGRESS_OCT_28_2025_EVENING.md](SESSION_PROGRESS_OCT_28_2025_EVENING.md)** | Oct 28, 2025 | ✅ Latest Session |
| **[KNOWN_ISSUES.md](KNOWN_ISSUES.md)** | Oct 28, 2025 | ✅ Active |

### Quick Status Summary
```
Test Coverage:      15.94% (baseline) → Target: 90%
Library Tests:      673 passing (100% pass rate) ✅
Integration Tests:  Temporarily disabled (security module fixes needed)
Code Quality:       A- (improving to A+)
```

---

## 🏗️ **Architecture & Design**

| Document | Description |
|----------|-------------|
| **[ARCHITECTURE_OVERVIEW.md](ARCHITECTURE_OVERVIEW.md)** | System architecture and design patterns |
| **[specs/](specs/)** | Detailed specifications and requirements |
| **[docs/architecture/](docs/architecture/)** | In-depth architecture documentation |

---

## 🧪 **Testing & Quality**

| Document | Purpose | Status |
|----------|---------|--------|
| **[KNOWN_ISSUES.md](KNOWN_ISSUES.md)** | Current bugs and limitations | ✅ Active |
| **[TEST_FILES_TO_FIX.md](TEST_FILES_TO_FIX.md)** | Test files needing fixes | ⏸️ In Progress |
| **[TEST_MODERNIZATION_PLAN_OCT_28_2025.md](TEST_MODERNIZATION_PLAN_OCT_28_2025.md)** | Test improvement roadmap | ✅ Active |
| **[UNWRAP_MIGRATOR_ASSESSMENT_OCT_28_2025.md](UNWRAP_MIGRATOR_ASSESSMENT_OCT_28_2025.md)** | Unwrap migration status | ✅ Active |

---

## 🚢 **Deployment**

| Document | Purpose |
|----------|---------|
| **[DEPLOYMENT_GUIDE.md](DEPLOYMENT_GUIDE.md)** | Production deployment instructions |
| **[docker/](docker/)** | Docker configuration files |
| **[k8s-deployment.yaml](k8s-deployment.yaml)** | Kubernetes deployment manifest |
| **[deploy/](deploy/)** | Deployment scripts and configs |

---

## 🤝 **Contributing**

| Document | Purpose |
|----------|---------|
| **[CONTRIBUTING.md](CONTRIBUTING.md)** | How to contribute to NestGate |
| **[CHANGELOG.md](CHANGELOG.md)** | Version history and changes |
| **[LICENSE](LICENSE)** | Project license |

---

## 🛠️ **Development Tools**

| Tool | Purpose |
|------|---------|
| **[QUICK_COMMANDS.sh](QUICK_COMMANDS.sh)** | Common development commands |
| **[TOOL_MIGRATION_QUICKSTART.md](TOOL_MIGRATION_QUICKSTART.md)** | Tool migration guide |
| **[scripts/](scripts/)** | Development and automation scripts |

---

## 📁 **Project Structure**

```
nestgate/
├── code/crates/          # Rust workspace crates
│   ├── nestgate-core/    # Core library (518 tests ✅)
│   ├── nestgate-api/     # REST API (56 tests ✅)
│   ├── nestgate-zfs/     # ZFS integration (99 tests ✅)
│   └── ...
├── docs/                 # Comprehensive documentation
├── specs/                # Technical specifications
├── tests/                # Integration and E2E tests
├── benches/              # Performance benchmarks
├── examples/             # Usage examples
└── tools/                # Development tools
```

---

## 📚 **Detailed Documentation**

### Core Concepts
- [Infant Discovery Architecture](docs/architecture/infant-discovery.md)
- [Zero-Cost Architecture](docs/architecture/zero-cost.md)
- [Sovereignty Layer](docs/sovereignty/)
- [Universal Storage](docs/storage/)

### Development Guides
- [Testing Strategy](docs/testing/)
- [Performance Optimization](docs/performance/)
- [Error Handling Patterns](docs/error-handling.md)
- [Configuration Management](docs/configuration/)

### API Documentation
- [REST API Reference](docs/api/rest.md)
- [RPC Interface](docs/api/rpc.md)
- [WebSocket Events](docs/api/websocket.md)

---

## 🎯 **Current Development Focus**

### Active Initiatives
1. **Test Coverage Expansion** (Phase 2 - In Progress)
   - Target: 90% coverage (currently 15.94%)
   - Added 100+ tests this session
   - Path: [SESSION_PROGRESS_OCT_28_2025_EVENING.md](SESSION_PROGRESS_OCT_28_2025_EVENING.md)

2. **Error Handling Modernization** (In Progress)
   - Migrating unwrap() → Result<T, E>
   - 6 fixed, 1,204 remaining
   - Path: [UNWRAP_MIGRATOR_ASSESSMENT_OCT_28_2025.md](UNWRAP_MIGRATOR_ASSESSMENT_OCT_28_2025.md)

3. **Security Module Fixes** (Pending)
   - Fix syntax errors in security module
   - Estimated: 1-2 hours
   - Path: [KNOWN_ISSUES.md](KNOWN_ISSUES.md)

---

## 📦 **Archived Documentation**

Historical session files and outdated documentation are archived in:
- `archive/oct-28-2025-session/` - Oct 28 session reports
- `sessions/` - Historical development sessions
- `rebuild_workspace/` - Workspace rebuild documentation

---

## 🔗 **External Resources**

- **GitHub**: (Add repository URL)
- **Documentation Site**: (Add docs URL if applicable)
- **Issue Tracker**: (Add issues URL)
- **Community**: (Add community links)

---

## 📝 **Document Status Legend**

| Symbol | Meaning |
|--------|---------|
| ✅ | Current and actively maintained |
| ⏸️ | In progress, actively updating |
| 📚 | Reference material, stable |
| 🗄️ | Archived, historical reference |
| ❌ | Deprecated, do not use |

---

## 🆘 **Getting Help**

1. **New to NestGate?** → Start with [START_HERE.md](START_HERE.md)
2. **Need quick setup?** → See [QUICK_START_GUIDE.md](QUICK_START_GUIDE.md)
3. **Found a bug?** → Check [KNOWN_ISSUES.md](KNOWN_ISSUES.md)
4. **Want to contribute?** → Read [CONTRIBUTING.md](CONTRIBUTING.md)
5. **Looking for API docs?** → See [docs/api/](docs/api/)

---

**Maintained by**: NestGate Development Team  
**Last Review**: October 28, 2025  
**Next Review**: November 11, 2025

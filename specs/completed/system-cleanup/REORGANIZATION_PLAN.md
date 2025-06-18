# Project Reorganization Plan

## Overview

NestGate's project structure needs to be reorganized to improve maintainability, clarity, and organization. This document outlines the plan for reorganizing the project structure.

## Target Structure

```
nestgate/
├── crates/              # All implementation code
│   ├── nestgate-api/    # API implementation
│   ├── nestgate-bin/    # Binary executables
│   ├── nestgate-core/   # Core functionality
│   ├── nestgate-fsmonitor/ # Filesystem monitoring
│   ├── nestgate-mcp/    # Machine Context Protocol
│   ├── nestgate-network/ # Network functionality
│   ├── nestgate-port-manager/ # Port management
│   ├── nestgate-ui/     # User interface
│   └── nestgate-zfs/    # ZFS integration
├── docs/                # User documentation
│   ├── api/             # API documentation
│   ├── guides/          # User guides
│   └── references/      # Reference documentation
├── specs/               # System design and specifications
│   ├── architecture/    # Architecture specifications
│   ├── implementation/  # Implementation specifications
│   └── integration/     # Integration specifications
├── scripts/             # Utility scripts
├── examples/            # Example code
└── tests/               # System-level tests
```

## Migration Plan

1. **Documentation Files**
   - Move user-facing documentation to `docs/`
   - Move internal specifications to `specs/`
   - Create appropriate subdirectories for organization

2. **Implementation Files**
   - Ensure all implementation code is in `crates/`
   - Move any stray implementation files to the appropriate crate

3. **Scripts**
   - Organize scripts into a consistent location
   - Remove redundant or obsolete scripts

4. **Cleanup**
   - Remove any obsolete files
   - Migrate remaining mock data to test directories
   - Update imports and references

## Priority Files for Reorganization

### Documentation to Move to docs/
- FORDEVS.md → docs/guides/for_developers.md
- API_REFERENCE.md → docs/api/reference.md
- UI_INTEGRATION_GUIDE.md → docs/guides/ui_integration.md
- quickstart.md → docs/guides/quickstart.md

### Specifications to Move to specs/
- ARCHITECTURE.md → specs/architecture/overview.md
- IMPLEMENTATION.md → specs/implementation/overview.md
- COMPATIBILITY.md → specs/integration/compatibility.md
- SECURITY_IMPLEMENTATION_PLAN.md → specs/implementation/security.md

### Implementation Files to Move to crates/
- Any stray implementation files not already in crates/

### Scripts to Organize
- Consolidate start scripts
- Organize build scripts
- Clean up test scripts

## Timeline

1. Documentation Reorganization (1 day)
2. Implementation File Migration (1-2 days)
3. Script Organization (1 day)
4. Final Cleanup and Verification (1 day)

## Post-Reorganization Tasks

- Update all import paths
- Update build scripts
- Update documentation references
- Verify all functionality still works correctly 
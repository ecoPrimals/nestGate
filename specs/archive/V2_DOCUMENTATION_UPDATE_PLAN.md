---
title: NestGate v2 Documentation Update Plan
description: Comprehensive plan for updating all specs to reflect orchestrator-centric sovereign architecture
version: 1.0.0
date: 2025-01-26
author: MCP Sovereign Engineering
priority: CRITICAL
---

# NestGate v2 Documentation Update Plan

## Executive Summary

Following the successful implementation of the v2 sovereign rebuild with orchestrator-centric architecture, ALL existing documentation needs systematic updates to reflect:

1. **Port Manager → Orchestrator Migration**: Complete rebranding and architectural documentation
2. **Sovereign-First Design**: Updated focus on standalone operation with optional federation
3. **Orchestrator-Centric Connectivity**: All connections now flow through nestgate-orchestrator
4. **Simplified Architecture**: Removal of complex port management in favor of centralized orchestrator
5. **MCP Federation**: Optional connectivity model instead of required dependency

## Documentation Audit Results

### ✅ COMPLETED - Up to Date
- `specs/NESTGATE_V2_SOVEREIGN_REBUILD.md` - **CURRENT** (master v2 spec)

### 🔄 NEEDS MAJOR UPDATES - Architecture Changes

#### Core Architecture Documents
- `specs/SPECS.md` - **CRITICAL** (1,286 lines) - Still references port manager extensively
- `specs/architecture/overview.md` - **CRITICAL** (375 lines) - Outdated system diagrams
- `specs/architecture/new_architecture.md` - **HIGH** - Needs v2 orchestrator patterns
- `specs/architecture/codebase_overview.md` - **HIGH** - Crate structure changed

#### Port Manager Legacy Documents 
- `specs/legacy/PORT_MANAGER_HANDOFF.md` - **ARCHIVE** - Move to completed/deprecated
- All port manager references in SPECS.md - **REPLACE** with orchestrator patterns

#### Network Architecture
- `specs/network/nestgate-network/architecture.md` - **HIGH** - Update connectivity patterns
- `specs/network/nestgate-network/protocol_support.md` - **MEDIUM** - Update to orchestrator model
- `specs/network/nestgate-network/mcp_integration.md` - **HIGH** - Update federation model

### 🔄 NEEDS MODERATE UPDATES - Service Integration

#### Storage Integration
- `specs/storage/TIERED_STORAGE_INTEGRATION.md` - **MEDIUM** - Update for orchestrator integration
- `specs/network/nestgate-network/zfs_integration.md` - **MEDIUM** - Update service discovery

#### Service Management
- `specs/DYNAMIC-SERVICE-SYSTEM.md` - **HIGH** - Update for orchestrator service registry
- `specs/services/` (entire directory) - **HIGH** - Service definitions need orchestrator patterns

### 🔄 NEEDS MINOR UPDATES - References and Examples

#### Development Documents
- `specs/IMPLEMENTATION.md` - **MEDIUM** - Update build and deployment instructions
- `specs/COMPONENTS.md` - **MEDIUM** - Update component list (orchestrator vs port-manager)
- `specs/README.md` - **LOW** - Update overview and quick start

#### Planning Documents
- `specs/NAS_ROADMAP.md` - **MEDIUM** - Reflect v2 completion and next priorities
- `specs/REORGANIZATION_PLAN.md` - **LOW** - Mark as completed, reference v2 success

#### Status Documents
- `specs/SPECIFICATION_STATUS.md` - **HIGH** - Update to reflect v2 completion
- `specs/NAS_PROGRESS.md` - **HIGH** - Document v2 achievements

## Update Priority Matrix

### Phase 1: CRITICAL Architecture Updates (Immediate)
```yaml
timeline: "Sprint 1 (Week 1-2)"
priority: CRITICAL
documents:
  - specs/SPECS.md                           # Master specification rewrite
  - specs/architecture/overview.md           # Core architecture diagrams
  - specs/SPECIFICATION_STATUS.md            # Mark v2 as complete
  - specs/INDEX.md                          # Update document index
```

### Phase 2: HIGH Impact Service Updates (Sprint 2)
```yaml
timeline: "Sprint 2 (Week 3-4)"
priority: HIGH
documents:
  - specs/DYNAMIC-SERVICE-SYSTEM.md         # Service system patterns
  - specs/network/nestgate-network/mcp_integration.md  # Federation model
  - specs/architecture/new_architecture.md   # v2 patterns
  - specs/services/                         # All service definitions
```

### Phase 3: MEDIUM Integration Updates (Sprint 3)
```yaml
timeline: "Sprint 3 (Week 5-6)"
priority: MEDIUM
documents:
  - specs/storage/TIERED_STORAGE_INTEGRATION.md  # Storage patterns
  - specs/network/nestgate-network/architecture.md  # Network patterns
  - specs/IMPLEMENTATION.md                 # Build instructions
  - specs/NAS_ROADMAP.md                   # Updated roadmap
```

### Phase 4: LOW Reference Updates (Sprint 4)
```yaml
timeline: "Sprint 4 (Week 7-8)"
priority: LOW
documents:
  - specs/COMPONENTS.md                     # Component reference
  - specs/README.md                        # Overview update
  - specs/CONTRIBUTING.md                  # Update for v2
```

## Detailed Update Requirements

### 1. Master Specification (specs/SPECS.md)

**Current Issues:**
- 57 references to "Port Manager" - replace with "Orchestrator"
- Hardcoded port allocations - replace with orchestrator patterns
- Service registry model outdated
- No sovereign/federation mode documentation

**Required Changes:**
```yaml
architectural_changes:
  - "Port Manager First" → "Orchestrator-Centric" philosophy
  - Dynamic port allocation → Centralized connectivity hub
  - Service discovery via Port Manager → Service registry via Orchestrator
  
federation_model:
  - Add standalone vs federated mode documentation
  - Document auto-detection patterns
  - Update MCP integration to optional model
  
connectivity_patterns:
  - All external connections → orchestrator → services
  - Service-to-service communication via orchestrator
  - Health monitoring through orchestrator
```

### 2. Architecture Overview (specs/architecture/overview.md)

**Required Mermaid Diagram Updates:**
```mermaid
# OLD: Port Manager Architecture
Port Manager → Service Registry → Individual Services

# NEW: Orchestrator-Centric Architecture  
External Clients → Orchestrator → Service Registry → Local Services
MCP Federation ←→ Orchestrator ←→ Connection Proxy ←→ Services
```

**Documentation Updates:**
- Replace all "Port Manager" component references
- Update sequence diagrams for orchestrator flow
- Document sovereign vs federated modes
- Update component interaction patterns

### 3. Service System (specs/DYNAMIC-SERVICE-SYSTEM.md)

**Required Updates:**
- Service registration → Orchestrator service registry
- Port allocation → Orchestrator port management  
- Health checks → Orchestrator health monitoring
- Process management → Orchestrator process coordination

### 4. Network Integration Documents

**MCP Integration Updates:**
- Required dependency → Optional federation
- Always-on connection → Auto-detect and graceful degradation
- Direct MCP communication → Through orchestrator proxy

**Architecture Updates:**
- Protocol handling through orchestrator
- Service discovery via orchestrator
- Connection pooling and proxying

## Implementation Guidelines

### Documentation Standards for v2

#### 1. Terminology Updates
```yaml
old_terms:
  "Port Manager": "Orchestrator"
  "Service Manager": "Orchestrator Service Registry"  
  "Port Allocation": "Orchestrator Port Management"
  "Dynamic Port Discovery": "Orchestrator Service Discovery"
  "Health Management": "Orchestrator Health Monitoring"
  
new_patterns:
  connectivity: "All connectivity flows through nestgate-orchestrator"
  federation: "Optional MCP federation with standalone fallback"
  sovereignty: "Fully autonomous operation capability"
```

#### 2. Architectural Patterns
```yaml
old_pattern:
  External → Port Manager → Service Discovery → Individual Services
  
new_pattern:
  External → Orchestrator → Connection Proxy → Service Registry → Services
  MCP Federation ←→ Orchestrator (optional)
  Standalone Mode: Orchestrator (no MCP dependency)
```

#### 3. Code Examples
All code examples must show:
- Orchestrator-based service registration
- Optional MCP federation patterns
- Standalone operation examples
- Service discovery through orchestrator

### Validation Checklist

For each updated document:
- [ ] No references to "Port Manager" remain
- [ ] Orchestrator-centric patterns documented  
- [ ] Sovereign/federation modes explained
- [ ] Standalone operation capability highlighted
- [ ] MCP integration shown as optional
- [ ] All connectivity flows through orchestrator
- [ ] Code examples updated to v2 patterns

## Success Metrics

### Documentation Completion Metrics
- [ ] 100% of "Port Manager" references replaced with "Orchestrator"
- [ ] All architecture diagrams reflect v2 orchestrator-centric design
- [ ] Sovereign operation modes documented in all relevant specs
- [ ] MCP integration consistently shown as optional federation
- [ ] Service patterns updated to orchestrator-based discovery

### Validation Metrics  
- [ ] New developer can understand v2 architecture from specs alone
- [ ] All code examples in specs compile and run with v2 codebase
- [ ] Architecture diagrams match actual implemented system
- [ ] No outdated references to deprecated components

## Risk Mitigation

### Documentation Debt Prevention
1. **Version Control**: Tag all current docs as "pre-v2" before updates
2. **Incremental Updates**: Update critical path documents first
3. **Cross-Reference Validation**: Ensure consistency across all updates
4. **Example Validation**: Test all code examples with v2 implementation

### Knowledge Preservation  
1. **Legacy Archive**: Move outdated docs to `specs/legacy/v1/`
2. **Migration Guide**: Document v1→v2 changes for historical reference
3. **Decision Log**: Document architectural decisions and rationale

## Next Actions

### Immediate (This Sprint)
1. **Archive Legacy**: Move port-manager docs to `specs/legacy/v1/`
2. **Update Master Spec**: Rewrite `specs/SPECS.md` for orchestrator architecture
3. **Fix Architecture Overview**: Update `specs/architecture/overview.md` diagrams
4. **Update Status**: Mark v2 completion in `specs/SPECIFICATION_STATUS.md`

### Sprint Planning
- Assign documentation updates across team members
- Set up documentation review process
- Create validation checklist for each document type
- Establish timeline for phased completion

---

## Summary

The v2 sovereign rebuild fundamentally changed NestGate's architecture from port-manager-centric to orchestrator-centric design. This documentation update plan ensures our specs accurately reflect the new reality:

- **Orchestrator as central connectivity hub**
- **Sovereign standalone operation capability**  
- **Optional MCP federation model**
- **Simplified service discovery and management**

Completing this documentation update is CRITICAL for:
- Onboarding new developers
- Planning future development
- Maintaining system knowledge
- Supporting deployment and operations

**Total Estimated Effort**: 4 sprints (8 weeks) with team coordination
**Priority**: CRITICAL - Required for continued development 
# External Primal Routing Specification

## Overview

This specification defines the migration of all external primal integrations from hardcoded implementations to the Universal Adapter pattern, eliminating technical debt and establishing proper ecosystem boundaries.

## Current State Analysis

### External Dependencies Requiring Migration

**Toadstool Compute Primal Integration:**
- `ToadstoolComputeClient` - Direct hardware tuning client
- `MockToadstoolCompute` - Test implementation
- Location: `code/crates/nestgate-api/src/hardware_tuning/client.rs`

**Songbird Orchestration Integration:**
- `SongbirdIntegration` - Direct orchestration calls
- Mock implementations in tests
- Location: `code/crates/nestgate-api/src/universal_primal.rs`

**BearDog Security Integration:**
- `BearDogConfig` - Hardcoded security client
- Authentication bypass implementations
- Location: Various security modules

**Squirrel AI Integration:**
- Direct AI model calls
- Hardcoded inference endpoints
- Location: `code/crates/nestgate-core/src/data_sources/huggingface.rs`

### Technical Debt Metrics
- **67 TODOs** related to external primal integrations
- **23 Mock implementations** that should be adapter-based
- **156 Hardcoded endpoints** and configurations
- **89 Direct external calls** bypassing adapter pattern

## Target Architecture

### Universal Adapter Integration

```rust
// Current problematic pattern:
let toadstool = ToadstoolComputeClient::new(hardcoded_config);
toadstool.optimize_hardware().await?;

// Target adapter pattern:
let adapter = self.universal_adapter.get_capability("compute.hardware_optimization").await?;
adapter.execute(OptimizationRequest::new(params)).await?;
```

### Capability Categories

**Compute Capabilities (Toadstool):**
- `compute.hardware_optimization`
- `compute.resource_allocation`
- `compute.performance_tuning`

**Orchestration Capabilities (Songbird):**
- `orchestration.service_coordination`
- `orchestration.workflow_management`
- `orchestration.event_routing`

**Security Capabilities (BearDog):**
- `security.authentication`
- `security.authorization`
- `security.encryption`

**Intelligence Capabilities (Squirrel):**
- `ai.model_inference`
- `ai.data_analysis`
- `ai.optimization_suggestions`

## Implementation Strategy

### Phase 1: Adapter Interface Definition
1. Define capability interfaces for each external primal
2. Create adapter configuration schemas
3. Implement mock adapters for testing

### Phase 2: Gradual Migration
1. Replace direct calls with adapter calls
2. Maintain backward compatibility during transition
3. Update tests to use adapter pattern

### Phase 3: Cleanup and Optimization
1. Remove hardcoded implementations
2. Eliminate mock classes
3. Optimize adapter routing performance

## Success Metrics

### Reduction Targets
- **90% reduction** in external primal TODOs
- **100% elimination** of hardcoded external endpoints
- **80% reduction** in mock implementations
- **Complete removal** of direct external primal imports

### Quality Improvements
- Consistent error handling across all external integrations
- Unified logging and monitoring for external calls
- Improved testability through adapter mocking
- Enhanced security through centralized external access

## Implementation Requirements

### Code Quality Standards
- All adapter implementations must have 100% test coverage
- Error handling must use unified Result types
- Logging must follow NestGate standards
- Documentation must include capability schemas

### Performance Requirements
- Adapter routing overhead < 1ms
- Connection pooling for external calls
- Circuit breaker pattern for fault tolerance
- Metrics collection for all external interactions

## Migration Checklist

### Per External Primal
- [ ] Define capability interface
- [ ] Implement adapter client
- [ ] Create configuration schema
- [ ] Add comprehensive tests
- [ ] Update calling code
- [ ] Remove direct implementations

### System-wide
- [ ] Update universal adapter registry
- [ ] Add capability discovery mechanisms
- [ ] Implement adapter health checks
- [ ] Create monitoring dashboards
- [ ] Update documentation

## Risk Mitigation

### Breaking Changes
- Feature flags for gradual rollout
- Fallback mechanisms during migration
- Comprehensive integration testing
- Rollback procedures documented

### External Dependencies
- Version compatibility matrices
- Graceful degradation for unavailable primals
- Timeout and retry configurations
- Circuit breaker implementations

## Validation Criteria

### Technical Validation
- All tests pass with adapter implementations
- Performance benchmarks meet requirements
- Security scans show no external exposure
- Code coverage maintains 90%+ levels

### Integration Validation
- End-to-end ecosystem tests pass
- External primal interactions work correctly
- Error scenarios handled gracefully
- Monitoring and alerting functional 
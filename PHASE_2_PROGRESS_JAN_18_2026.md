# Phase 2 Execution: Async Evolution & Capability Migration

**Started**: January 18, 2026  
**Status**: IN PROGRESS  
**Target**: 50 unwraps + 100 hardcoded values

---

## Session Plan

### Target 1: Migrate Hardcoded Ports to Capability-Based (100 values)

**Priority Files** (from codebase search):
1. ✅ `discovery/network_discovery.rs` - Hardcoded ports 8081, 9001, 8443, 9000, 7000, 8000
2. `constants/network_hardcoded.rs` - Central constants (316+ localhost instances)
3. `config/discovery_config.rs` - Configuration hardcoding
4. `dev_stubs/primal_discovery.rs` - Stub with hardcoded fallbacks

**Pattern to Apply**:
```rust
// ❌ OLD: Hardcoded ports
let health_port = 8081;
let websocket_port = 9001;

// ✅ NEW: Environment-driven with capability discovery
let health_port = env_config
    .network
    .health_port
    .unwrap_or_else(|| discover_capability_port("health").await);
```

### Target 2: Evolve Unwraps to Async Result (50 critical)

**Good News**: RPC modules already have excellent error handling!
- `unix_socket_server.rs` - Proper `map_err` usage ✅
- `tarpc_client.rs` - Good error propagation ✅
- `connection_pool.rs` - Proper Result returns ✅

**Focus**: Find remaining `.unwrap()` calls in these modules and convert

---

## Progress Tracker

### Hardcoding Migration

| File | Hardcoded Values | Migrated | Status |
|------|-----------------|----------|--------|
| `discovery/network_discovery.rs` | 6+ ports | 0 | 🔄 Next |
| `constants/network_hardcoded.rs` | 316+ addresses | 0 | Pending |
| `config/discovery_config.rs` | TBD | 0 | Pending |
| `dev_stubs/primal_discovery.rs` | Multiple | 0 | Pending |

**Total**: 0/100

### Unwrap Evolution

| Module | Unwraps Found | Evolved | Status |
|--------|--------------|---------|--------|
| RPC modules | TBD | 0 | Pending |
| Network modules | TBD | 0 | Pending |
| API handlers | TBD | 0 | Pending |

**Total**: 0/50

---

## Next Actions

1. **Migrate discovery/network_discovery.rs** (30 min)
   - Replace 6 hardcoded ports with env/capability discovery
   - Test with capability-based lookup

2. **Find unwraps in RPC** (15 min)
   ```bash
   rg "\.unwrap\(\)|\.expect\(" code/crates/nestgate-core/src/rpc/
   ```

3. **Evolve first 10 unwraps** (30 min)
   - Add proper error context
   - Use `.context()` pattern
   - Add retry where appropriate

---

**Session Started**: Now  
**Expected Duration**: 2-3 hours for both targets

# Hardcoding Elimination Guide
**Created**: November 13, 2025  
**Status**: 🚀 Ready for Migration  
**Target**: 888+ hardcoded instances → Environment-driven configuration

---

## Overview

This guide helps migrate from hardcoded values to the new `constants::consolidated` module.

### Current State
- **Hardcoded IPs**: 447 instances across 91 files
- **Hardcoded ports**: 441 instances across 67 files
- **Total targeted for elimination**: 888 instances

### Solution
New `constants::consolidated` module provides:
- ✅ Environment-driven configuration
- ✅ Thread-safe initialization (`Arc<T>` + `OnceLock`)
- ✅ Zero runtime overhead
- ✅ Type-safe API
- ✅ Comprehensive test coverage

---

## Quick Start

### 1. Import the Module

```rust
use nestgate_core::constants::consolidated::*;
```

### 2. Replace Hardcoded Values

#### Before (Hardcoded):
```rust
fn connect_to_api() -> String {
    format!("http://127.0.0.1:8080/api")
}

fn get_postgres_connection() -> String {
    "postgresql://127.0.0.1:5432/nestgate".to_string()
}

const MAX_CONNECTIONS: usize = 1000;
const TIMEOUT_MS: u64 = 30000;
```

#### After (Consolidated):
```rust
use nestgate_core::constants::consolidated::*;

fn connect_to_api() -> String {
    let net = NetworkConstants::get();
    format!("{}/api", net.api_url())
}

fn get_postgres_connection() -> String {
    let storage = StorageConstants::get();
    storage.postgres_url()
}

fn get_max_connections() -> usize {
    let perf = PerformanceConstants::get();
    perf.max_connections()
}

fn get_timeout() -> std::time::Duration {
    let perf = PerformanceConstants::get();
    perf.request_timeout()
}
```

---

## Migration Patterns

### Pattern 1: Network Addresses

#### Hardcoded IP Addresses
```rust
// ❌ OLD: Hardcoded
let host = "127.0.0.1";
let bind = "0.0.0.0";

// ✅ NEW: Consolidated
let net = NetworkConstants::get();
let host = net.api_host();
let bind = net.bind_address();
```

#### Hardcoded Ports
```rust
// ❌ OLD: Hardcoded
let api_port = 8080;
let metrics_port = 9090;

// ✅ NEW: Consolidated
let net = NetworkConstants::get();
let api_port = net.api_port();
let metrics_port = net.metrics_port();
```

#### Full URLs
```rust
// ❌ OLD: Hardcoded
let url = "http://localhost:8080";
let ws_url = "ws://localhost:8082/ws";

// ✅ NEW: Consolidated
let net = NetworkConstants::get();
let url = net.api_url();
let ws_url = net.websocket_url();
```

### Pattern 2: Database Connections

#### PostgreSQL
```rust
// ❌ OLD: Hardcoded
let db_url = format!("postgresql://{}:{}/{}", 
    "127.0.0.1", 5432, "nestgate");

// ✅ NEW: Consolidated
let storage = StorageConstants::get();
let db_url = storage.postgres_url();
```

#### Redis
```rust
// ❌ OLD: Hardcoded
let redis_host = "127.0.0.1:6379";

// ✅ NEW: Consolidated
let storage = StorageConstants::get();
let redis_url = storage.redis_url();
```

### Pattern 3: Performance Tuning

#### Timeouts
```rust
// ❌ OLD: Hardcoded
use std::time::Duration;
let timeout = Duration::from_millis(5000);
let request_timeout = Duration::from_millis(30000);

// ✅ NEW: Consolidated
let perf = PerformanceConstants::get();
let timeout = perf.connection_timeout();
let request_timeout = perf.request_timeout();
```

#### Connection Pools
```rust
// ❌ OLD: Hardcoded
const MAX_CONNECTIONS: usize = 1000;
const POOL_SIZE: usize = 100;

// ✅ NEW: Consolidated
let perf = PerformanceConstants::get();
let max_conn = perf.max_connections();
let pool_size = perf.connection_pool_size();
```

#### Retries
```rust
// ❌ OLD: Hardcoded
const MAX_RETRIES: u32 = 3;
const RETRY_DELAY_MS: u64 = 1000;

// ✅ NEW: Consolidated
let perf = PerformanceConstants::get();
let max_retries = perf.max_retry_attempts();
let retry_delay = perf.retry_delay();
```

### Pattern 4: Security Settings

#### JWT Configuration
```rust
// ❌ OLD: Hardcoded
let jwt_secret = "some_secret";
let expiration_secs = 3600;

// ✅ NEW: Consolidated
let sec = SecurityConstants::get();
let jwt_secret = sec.jwt_secret();
let expiration = sec.jwt_expiration();
```

#### TLS Configuration
```rust
// ❌ OLD: Hardcoded
let cert_path = "./certs/server.crt";
let tls_enabled = false;

// ✅ NEW: Consolidated
let sec = SecurityConstants::get();
let cert_path = sec.tls_cert_path();
let tls_enabled = sec.tls_enabled();
```

---

## Environment Variable Override

All constants can be overridden via environment variables:

### Network
```bash
export NESTGATE_API_HOST=api.production.com
export NESTGATE_API_PORT=443
export NESTGATE_BIND_ADDRESS=0.0.0.0
export NESTGATE_METRICS_PORT=9090
```

### Storage
```bash
export NESTGATE_POSTGRES_HOST=db.production.com
export NESTGATE_POSTGRES_PORT=5432
export NESTGATE_POSTGRES_DB=nestgate_prod
export NESTGATE_REDIS_HOST=cache.production.com
export NESTGATE_REDIS_PORT=6379
```

### Performance
```bash
export NESTGATE_MAX_CONNECTIONS=5000
export NESTGATE_POOL_SIZE=500
export NESTGATE_CONN_TIMEOUT_MS=3000
export NESTGATE_REQ_TIMEOUT_MS=60000
export NESTGATE_MAX_RETRIES=5
```

### Security
```bash
export NESTGATE_JWT_SECRET=production_secret_key_here
export NESTGATE_JWT_EXP=7200
export NESTGATE_TLS_ENABLED=true
export NESTGATE_TLS_CERT=/etc/ssl/certs/server.crt
```

---

## Migration Priority

### Phase 1: Critical Paths (High Priority)
1. **API Handlers** - 40+ hardcoded URLs
   - Files: `code/crates/nestgate-api/src/handlers/*`
   - Impact: Direct user-facing code

2. **Network Client** - 23 hardcoded IPs in `utils/network.rs`
   - Impact: All outbound connections

3. **Config Defaults** - 44 instances in `config/network_defaults.rs`
   - Impact: Default configuration values

### Phase 2: Service Integration (Medium Priority)
4. **Universal Adapter** - 17 instances in `universal_adapter/adapter_config.rs`
5. **Discovery Services** - 16 instances in `universal_primal_discovery/network_discovery_config.rs`
6. **External Services** - 20 instances in `config/external/services_config.rs`

### Phase 3: Test Infrastructure (Low Priority)
7. **Test Files** - ~200 instances in `*_tests.rs` files
   - Impact: Test reliability, not production

---

## Automated Migration

### Script: Find and Replace

```bash
#!/bin/bash
# Find all hardcoded "127.0.0.1" and "localhost"

echo "=== Hardcoded IPs ==="
rg -n "127\.0\.0\.1|localhost" --type rust code/crates/nestgate-core/src \
  | grep -v "tests.rs" \
  | head -20

echo "=== Hardcoded Ports ==="
rg -n ":\d{4,5}" --type rust code/crates/nestgate-core/src \
  | grep -v "tests.rs" \
  | head -20
```

### Semi-Automated Replacement

```bash
# Example: Replace in a specific file
sed -i 's/127\.0\.0\.1/NetworkConstants::get().api_host()/g' file.rs

# Note: Manual review required after automated replacement!
```

---

## Testing the Migration

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_core::constants::consolidated::*;
    
    #[test]
    fn test_network_configuration() {
        let net = NetworkConstants::get();
        
        // Verify URL construction
        let api_url = net.api_url();
        assert!(api_url.starts_with("http://"));
        assert!(api_url.contains(":"));
        
        // Verify environment override works
        std::env::set_var("NESTGATE_API_PORT", "9999");
        let net2 = NetworkConstants::default();
        assert_eq!(net2.api_port(), 9999);
    }
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_api_connection_with_constants() {
    let net = NetworkConstants::get();
    let client = reqwest::Client::new();
    
    let response = client
        .get(net.health_url())
        .timeout(PerformanceConstants::get().request_timeout())
        .send()
        .await;
    
    assert!(response.is_ok());
}
```

---

## Validation

### Before Merging
Run these checks:

```bash
# 1. All tests pass
cargo test --workspace

# 2. No new hardcoded values introduced
./scripts/check_hardcoding.sh

# 3. Environment override works
export NESTGATE_API_PORT=9999
cargo test --package nestgate-core constants::consolidated

# 4. No performance regression
cargo bench
```

---

## Rollback Plan

If issues arise:

1. **Revert module addition**:
   ```bash
   git checkout HEAD -- code/crates/nestgate-core/src/constants/consolidated.rs
   git checkout HEAD -- code/crates/nestgate-core/src/constants/mod.rs
   ```

2. **Revert migration changes**:
   ```bash
   git revert <migration-commit-sha>
   ```

3. **Fallback**: Old constants remain in `hardcoding.rs` and `canonical_defaults.rs`

---

## Progress Tracking

### Migration Checklist

#### Core Modules
- [ ] `utils/network.rs` (40 .expect(), 23 IPs)
- [ ] `config/network_defaults.rs` (44 IPs, 33 ports)
- [ ] `config/external/network.rs` (21 IPs)
- [ ] `canonical_defaults.rs` (partial, already using helpers)

#### Discovery & Adapter
- [ ] `universal_adapter/adapter_config.rs` (17 ports)
- [ ] `universal_primal_discovery/network_discovery_config.rs` (16 ports)
- [ ] `discovery/capability_scanner.rs` (8 IPs)

#### Configuration
- [ ] `config/external/services_config.rs` (20 ports)
- [ ] `service_discovery/dynamic_endpoints_config.rs` (11 ports)
- [ ] `constants/domains/network.rs` (11 IPs, 2 ports)

#### Tests (Lower Priority)
- [ ] `*_tests.rs` files (~200 instances)

### Metrics
- **Total instances**: 888
- **Migrated**: 0 (baseline)
- **Remaining**: 888
- **Target**: <100 by end of Phase 3

---

## Examples

### Example 1: Migrating a Handler

```rust
// Before
use axum::{extract::State, Json};

async fn health_check() -> Json<HealthStatus> {
    let url = "http://localhost:8080/health";
    // ... rest of handler
}

// After
use axum::{extract::State, Json};
use nestgate_core::constants::consolidated::NetworkConstants;

async fn health_check() -> Json<HealthStatus> {
    let net = NetworkConstants::get();
    let url = net.health_url();
    // ... rest of handler (now environment-configurable!)
}
```

### Example 2: Migrating Configuration

```rust
// Before
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
        }
    }
}

// After
use nestgate_core::constants::consolidated::NetworkConstants;

pub struct ApiConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ApiConfig {
    fn default() -> Self {
        let net = NetworkConstants::get();
        Self {
            host: net.api_host().to_string(),
            port: net.api_port(),
        }
    }
}
```

---

## Benefits

### Before Migration
- ❌ Hardcoded values scattered across 175+ files
- ❌ Difficult to change configuration
- ❌ No environment override support
- ❌ Copy-paste errors and inconsistencies
- ❌ Hard to test different configurations

### After Migration
- ✅ Single source of truth (`constants::consolidated`)
- ✅ Easy configuration changes (environment variables)
- ✅ Consistent values across the codebase
- ✅ Type-safe, compile-time checked
- ✅ Easy to test and mock

---

## Support

Questions? Issues?
- Documentation: `COVERAGE_BASELINE_AND_GAPS_NOV_13_2025.md`
- Audit report: `AUDIT_EXECUTION_REPORT_NOV_13_2025.md`
- Constants source: `code/crates/nestgate-core/src/constants/consolidated.rs`

---

*Generated by Cursor AI Assistant | NestGate Hardcoding Elimination | Nov 13, 2025*


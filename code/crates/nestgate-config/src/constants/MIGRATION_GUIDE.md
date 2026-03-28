# Hardcoding to Environment-Driven Configuration - Migration Guide

**Date**: January 19, 2026  
**Status**: Active Migration  
**Progress**: 10 of 92 critical values (11%)

---

## 🎯 Quick Start

### Before (Hardcoded)

```rust
// ❌ OLD: Hardcoded constant
const API_PORT: u16 = 8080;
let addr = format!("127.0.0.1:{}", API_PORT);
```

### After (Environment-Driven)

```rust
// ✅ NEW: Environment-driven with fallback
use nestgate_core::constants::network_environment::api_port;

let port = api_port(); // Checks NESTGATE_API_PORT, defaults to 8080
let addr = format!("127.0.0.1:{}", port);
```

**Benefits**:
- ✅ Different ports per environment (dev/staging/prod)
- ✅ Zero breaking changes (same defaults)
- ✅ Runtime configuration (no rebuild needed)
- ✅ Testable (override in tests)

---

## 📋 Available Environment Variables

| Variable | Default | Purpose | Status |
|----------|---------|---------|--------|
| `NESTGATE_API_PORT` | 8080 | Main API server | ✅ Available |
| `NESTGATE_ADMIN_PORT` | 8081 | Admin interface | ✅ Available |
| `NESTGATE_METRICS_PORT` | 9090 | Prometheus metrics | ✅ Available |
| `NESTGATE_HEALTH_PORT` | 8082 | Health check | ✅ Available |
| `NESTGATE_WEBSOCKET_PORT` | 8081 | WebSocket connections | ✅ Available |
| `NESTGATE_DEV_PORT` | 3000 | Development server | ✅ Available |
| `NESTGATE_DEV_ALT_PORT` | 5000 | Alt dev server | ✅ Available |
| `NESTGATE_POSTGRES_PORT` | 5432 | PostgreSQL | ✅ Available |
| `NESTGATE_HTTPS_PORT` | 8443 | HTTPS/TLS | ✅ Available |
| `NESTGATE_BIND_ADDRESS` | 0.0.0.0 | Server bind address | ✅ Available |

---

## 🔧 Migration Patterns

### Pattern 1: Simple Port Migration

**Before**:
```rust
use nestgate_core::constants::port_defaults::DEFAULT_API_PORT;

fn start_server() {
    let port = DEFAULT_API_PORT; // Always 8080
    // ...
}
```

**After**:
```rust
use nestgate_core::constants::network_environment::api_port;

fn start_server() {
    let port = api_port(); // Environment-driven!
    // ...
}
```

---

### Pattern 2: Bind Address Migration

**Before**:
```rust
const BIND_ADDRESS: &str = "0.0.0.0";

async fn bind_server() {
    let addr = format!("{}:8080", BIND_ADDRESS);
    // ...
}
```

**After**:
```rust
use nestgate_core::constants::network_environment::{bind_address, api_port};

async fn bind_server() {
    let addr = format!("{}:{}", bind_address(), api_port());
    // ...
}
```

---

### Pattern 3: Full Endpoint Migration

**Before**:
```rust
fn api_endpoint() -> String {
    format!("http://127.0.0.1:8080")
}
```

**After**:
```rust
use nestgate_core::constants::network_environment::{localhost_ipv4, api_port};

fn api_endpoint() -> String {
    format!("http://{}:{}", localhost_ipv4(), api_port())
}
```

---

### Pattern 4: Timeout Migration

**Before**:
```rust
const CONNECT_TIMEOUT_MS: u64 = 5000;

async fn connect() {
    let timeout = Duration::from_millis(CONNECT_TIMEOUT_MS);
    // ...
}
```

**After**:
```rust
use nestgate_core::constants::network_environment::connect_timeout_ms;
use std::time::Duration;

async fn connect() {
    let timeout = Duration::from_millis(connect_timeout_ms());
    // ...
}
```

---

## 🧪 Testing with Environment Overrides

### In Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    #[test]
    fn test_custom_port() {
        // Override for this test
        env::set_var("NESTGATE_API_PORT", "9999");
        
        let port = api_port();
        assert_eq!(port, 9999);
        
        // Cleanup
        env::remove_var("NESTGATE_API_PORT");
    }
}
```

### In Integration Tests

```bash
# Run with custom configuration
NESTGATE_API_PORT=9000 cargo test

# Run with production-like config
NESTGATE_BIND_ADDRESS=0.0.0.0 \
NESTGATE_API_PORT=80 \
NESTGATE_METRICS_PORT=9090 \
cargo test --release
```

---

## 📊 Migration Progress

### Completed (10 values)

- [x] API_PORT → `api_port()`
- [x] ADMIN_PORT → `admin_port()`
- [x] METRICS_PORT → `metrics_port()`
- [x] HEALTH_PORT → `health_port()`
- [x] WEBSOCKET_PORT → `websocket_port()`
- [x] DEV_PORT → `dev_port()`
- [x] DEV_ALT_PORT → `dev_alt_port()`
- [x] POSTGRES_PORT → `postgres_port()`
- [x] HTTPS_PORT → `https_port()`
- [x] BIND_ADDRESS → `bind_address()`

### Next Batch (20 values)

Priority files to migrate:
1. `discovery/network_discovery.rs`
2. `config/capability_discovery.rs`
3. `service_discovery/dynamic_endpoints.rs`
4. `rpc/tarpc_server.rs`
5. `network/client/pool.rs`

---

## 🚀 Deployment Examples

### Development

```bash
# Use defaults (localhost:8080)
cargo run

# Or override for dev
NESTGATE_API_PORT=3000 cargo run
```

### Staging

```bash
# Staging configuration
export NESTGATE_API_PORT=8080
export NESTGATE_BIND_ADDRESS=0.0.0.0
export NESTGATE_METRICS_PORT=9090
cargo run --release
```

### Production

```bash
# Production configuration
export NESTGATE_API_PORT=80
export NESTGATE_HTTPS_PORT=443
export NESTGATE_BIND_ADDRESS=0.0.0.0
export NESTGATE_METRICS_PORT=9090
export NESTGATE_HEALTH_PORT=8080

# Or use systemd environment file
systemctl start nestgate.service
```

### Docker

```dockerfile
# Dockerfile with environment variables
FROM rust:latest
WORKDIR /app
COPY . .
RUN cargo build --release

# Default environment (overridable at runtime)
ENV NESTGATE_API_PORT=8080
ENV NESTGATE_BIND_ADDRESS=0.0.0.0

CMD ["./target/release/nestgate"]
```

```bash
# Run with custom config
docker run -e NESTGATE_API_PORT=9000 nestgate
```

### Kubernetes

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: nestgate-config
data:
  NESTGATE_API_PORT: "8080"
  NESTGATE_BIND_ADDRESS: "0.0.0.0"
  NESTGATE_METRICS_PORT: "9090"
  NESTGATE_HEALTH_PORT: "8082"

---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nestgate
spec:
  template:
    spec:
      containers:
      - name: nestgate
        image: nestgate:latest
        envFrom:
        - configMapRef:
            name: nestgate-config
```

---

## 💡 Best Practices

### 1. Always Provide Defaults

```rust
// ✅ GOOD: Safe fallback
pub fn api_port() -> u16 {
    env::var("NESTGATE_API_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080) // Sensible default
}

// ❌ BAD: No fallback
pub fn api_port() -> u16 {
    env::var("NESTGATE_API_PORT")
        .expect("NESTGATE_API_PORT required") // Panics!
}
```

### 2. Validate at Startup

```rust
pub fn validate_config() -> Result<()> {
    let port = api_port();
    if port < 1024 && !is_privileged() {
        return Err(anyhow!("Port {} requires elevated privileges", port));
    }
    Ok(())
}
```

### 3. Document Environment Variables

```rust
/// Get API port from environment or use default (8080)
///
/// **Environment**: `NESTGATE_API_PORT`  
/// **Default**: 8080  
/// **Range**: 1024-65535 (unprivileged)
///
/// # Examples
///
/// ```
/// use nestgate_core::constants::network_environment::api_port;
///
/// let port = api_port(); // 8080 by default
/// ```
pub fn api_port() -> u16 {
    // implementation
}
```

### 4. Use Type-Safe Defaults

```rust
// ✅ GOOD: Type-safe
use std::net::IpAddr;

pub fn default_host() -> IpAddr {
    env::var("NESTGATE_HOST")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST))
}

// ❌ BAD: String parsing at use site
pub fn default_host() -> String {
    env::var("NESTGATE_HOST")
        .unwrap_or("127.0.0.1".to_string())
}
```

---

## 🎯 Migration Checklist

### For Each File

- [ ] Identify hardcoded constants
- [ ] Replace with `network_environment::*` functions
- [ ] Update imports
- [ ] Add tests with environment overrides
- [ ] Document environment variables
- [ ] Update deployment docs

### Verification

- [ ] All tests pass
- [ ] Defaults unchanged (backward compatible)
- [ ] Environment overrides work
- [ ] Documentation updated
- [ ] No hardcoded constants remain

---

## 📞 Support

**Questions?**
- See: `UNIVERSAL_IPC_EVOLUTION_PLAN_JAN_19_2026.md`
- See: `code/crates/nestgate-core/src/constants/network_environment.rs`
- Pattern examples in this guide

**Issues?**
- Test with default values first
- Verify environment variable format
- Check for typos in variable names
- Ensure valid port ranges (1-65535)

---

**Status**: Living document (updated as migration progresses)  
**Next Update**: When 30 of 92 values migrated (33%)

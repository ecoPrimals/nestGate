> **Historical**: This document was written in November 7, 2025. Current architecture
> and patterns may differ. See root-level docs and `specs/` for current specifications.

# 📖 Constants Usage Guide
**Last Updated**: November 7, 2025  
**Purpose**: Definitive guide for using NestGate constants  
**Audience**: Developers, code reviewers

---

## 🎯 SINGLE SOURCE OF TRUTH

All constants are now consolidated in a clear hierarchy:

```
constants/
├── canonical.rs          ← PRIMARY SOURCE (use this!)
├── port_defaults.rs      ← Port constants with env support
├── timeouts.rs           ← Timeout helpers with env support  
├── network_defaults.rs   ← Network helpers with env support
└── canonical_defaults.rs ← Alternative defaults
```

---

## 📦 IMPORT PATTERNS

### ✅ PREFERRED: Use canonical module

```rust
use nestgate_core::constants::canonical::{
    performance,  // Buffer sizes, limits
    timeouts,     // Timeouts, retries
    network,      // Network constants
    storage,      // Storage constants
    security,     // Security constants
};

// Usage
let buffer_size = performance::DEFAULT_BUFFER_SIZE;
let timeout = timeouts::DEFAULT_TIMEOUT_MS;
```

### ✅ ALSO GOOD: Use re-exports

```rust
use nestgate_core::constants::{
    DEFAULT_API_PORT,           // From port_defaults
    DEFAULT_BUFFER_SIZE,         // From canonical::performance
    DEFAULT_TIMEOUT_MS,          // From canonical::timeouts
    DEFAULT_BIND_ADDRESS,        // From canonical::network
};
```

### ❌ AVOID: Direct imports from fragmented locations

```rust
// DON'T DO THIS:
use nestgate_core::constants::shared::*;
use nestgate_core::constants::hardcoding::*;
```

---

## 🚀 BUFFER SIZES (Domain-Specific)

### **IMPORTANT**: Buffer sizes are intentionally different!

#### DEFAULT_BUFFER_SIZE (4096 bytes)

**Use For**:
- File I/O operations
- Disk reading/writing
- General buffering
- Cache-friendly operations

**Why 4KB**:
- Matches typical OS page size
- Optimal for filesystem operations
- Fits in L1/L2 CPU cache
- Minimizes system call overhead

**Example**:
```rust
use nestgate_core::constants::canonical::performance;

// Reading from disk
let mut file_buffer = vec![0u8; performance::DEFAULT_BUFFER_SIZE];
file.read(&mut file_buffer)?;
```

#### NETWORK_BUFFER_SIZE (8192 bytes or 65536 bytes)

**Two sizes available**:
- `canonical_defaults::performance::NETWORK_BUFFER_SIZE` = 8KB
- `canonical::performance::NETWORK_BUFFER_SIZE` = 64KB

**Use 8KB For**:
- Moderate network operations
- Memory-constrained scenarios
- Socket I/O with many connections

**Use 64KB For**:
- High-throughput network operations
- Large file transfers
- Streaming operations
- Single or few connections

**Why Different**:
- 8KB: Balances memory vs performance
- 64KB: Matches typical TCP window size

**Example**:
```rust
use nestgate_core::constants::canonical::performance;
use nestgate_core::constants::canonical_defaults::performance as defaults;

// High-throughput streaming
let mut stream_buffer = vec![0u8; performance::NETWORK_BUFFER_SIZE]; // 64KB

// Moderate throughput, many connections
let mut socket_buffer = vec![0u8; defaults::NETWORK_BUFFER_SIZE]; // 8KB
```

### ⚠️ DO NOT CONSOLIDATE

These buffer sizes should **NEVER** be consolidated:
- They serve different purposes
- They are performance-tuned for specific operations
- Consolidating will hurt performance

---

## ⏰ TIMEOUT CONSTANTS

### Single Source: `canonical::timeouts`

All timeout constants are now in one place:

```rust
use nestgate_core::constants::canonical::timeouts;

// Default timeout (30 seconds)
let timeout = timeouts::DEFAULT_TIMEOUT_MS;        // 30,000 ms
let timeout_secs = timeouts::DEFAULT_TIMEOUT_SECS; // 30 seconds

// Specific timeouts
let connection = timeouts::CONNECTION_TIMEOUT_SECS;  // 30 seconds
let session = timeouts::SESSION_TIMEOUT_SECS;        // 300 seconds (5 min)
let operation = timeouts::OPERATION_TIMEOUT_SECS;    // 30 seconds
let discovery = timeouts::DISCOVERY_TIMEOUT_MS;      // 5,000 ms
```

### Environment Variable Support

Use the `timeouts` module functions for env variable support:

```rust
use nestgate_core::constants::timeouts;

// These check env vars first, then use defaults
let connection_timeout = timeouts::connection_timeout(); // Duration
let request_timeout = timeouts::request_timeout();       // Duration
```

**Environment Variables**:
- `NESTGATE_CONNECTION_TIMEOUT` - Connection timeout in seconds
- `NESTGATE_REQUEST_TIMEOUT` - Request timeout in seconds
- `NESTGATE_IDLE_TIMEOUT` - Idle timeout in seconds

---

## 🌐 PORT CONSTANTS

### Single Source: `port_defaults`

All port constants are in `port_defaults.rs`:

```rust
use nestgate_core::constants::port_defaults;

// Service ports
let api_port = port_defaults::DEFAULT_API_PORT;         // 8080
let admin_port = port_defaults::DEFAULT_ADMIN_PORT;     // 8081
let metrics_port = port_defaults::DEFAULT_METRICS_PORT; // 9090
let health_port = port_defaults::DEFAULT_HEALTH_PORT;   // 8082

// Database ports
let postgres = port_defaults::DEFAULT_POSTGRES_PORT;    // 5432
let redis = port_defaults::DEFAULT_REDIS_PORT;          // 6379
```

### Environment Variable Support

```rust
use nestgate_core::constants::port_defaults;

// These check env vars first
let api_port = port_defaults::get_api_port();     // checks NESTGATE_API_PORT
let metrics = port_defaults::get_metrics_port();  // checks NESTGATE_METRICS_PORT
```

**Environment Variables**:
- `NESTGATE_API_PORT` - API server port
- `NESTGATE_ADMIN_PORT` - Admin interface port
- `NESTGATE_METRICS_PORT` - Metrics/Prometheus port
- `NESTGATE_HEALTH_PORT` - Health check port
- Plus 10+ more service-specific ports

---

## 🔄 RETRY & BACKOFF

```rust
use nestgate_core::constants::canonical::timeouts;

let retry_attempts = timeouts::DEFAULT_RETRY_ATTEMPTS;   // 3
let retry_delay = timeouts::DEFAULT_RETRY_DELAY_MS;      // 1,000 ms
let rate_limit = timeouts::DEFAULT_RATE_LIMIT_RPM;       // 1000 requests/min
```

---

## 🗄️ STORAGE CONSTANTS

```rust
use nestgate_core::constants::canonical::storage;

// Size units
let kb = storage::KB;  // 1024
let mb = storage::MB;  // 1024 * 1024
let gb = storage::GB;  // 1024 * 1024 * 1024

// ZFS constants
let record_size = storage::RECORD_SIZE;  // 128KB
let arc_size = storage::ARC_SIZE;        // 1GB

// Storage tiers
let hot = storage::TIER_HOT;    // "hot"
let warm = storage::TIER_WARM;  // "warm"
let cold = storage::TIER_COLD;  // "cold"
```

---

## 🔒 SECURITY CONSTANTS

```rust
use nestgate_core::constants::canonical::security;

let token_expiration = security::TOKEN_EXPIRATION_S;  // 3600 seconds (1 hour)
let encryption = security::AES_256_GCM;               // "AES-256-GCM"

// User roles
let admin = security::ROLE_ADMIN;  // "admin"
let user = security::ROLE_USER;    // "user"
```

---

## 📡 NETWORK CONSTANTS

```rust
use nestgate_core::constants::canonical::network;

// Addresses
let bind_addr = network::DEFAULT_BIND_ADDRESS;  // "0.0.0.0"
let localhost = network::LOCALHOST;              // "127.0.0.1"

// Limits
let max_services = network::MAX_SERVICES;               // 1000
let max_requests = network::MAX_CONCURRENT_REQUESTS;    // 10,000
let max_sessions = network::MAX_SESSIONS;               // 1000
let max_message = network::MAX_MESSAGE_SIZE;            // 1024 bytes

// MTU and buffers
let mtu = network::MTU_SIZE;                  // 1500
let send_buf = network::SEND_BUFFER_SIZE;     // 65536
let recv_buf = network::RECV_BUFFER_SIZE;     // 65536
```

---

## 🎭 TESTING CONSTANTS

```rust
use nestgate_core::constants::testing;

// Test-specific constants
let test_timeout = testing::TEST_TIMEOUT_MS;
let test_retries = testing::TEST_RETRY_ATTEMPTS;
```

---

## ⚠️ WHAT NOT TO DO

### ❌ Don't Consolidate Domain-Specific Values

```rust
// WRONG: These are intentionally different!
// DEFAULT_BUFFER_SIZE (4KB) for disk
// NETWORK_BUFFER_SIZE (64KB) for network
// Don't make them the same!
```

### ❌ Don't Hardcode Values

```rust
// WRONG:
let port = 8080;
let timeout = 5000;

// RIGHT:
use nestgate_core::constants::canonical::{timeouts, network};
let port = network::DEFAULT_API_PORT;
let timeout = timeouts::DEFAULT_TIMEOUT_MS;
```

### ❌ Don't Import From Multiple Sources

```rust
// WRONG: Fragmented imports
use nestgate_core::constants::shared::*;
use nestgate_core::constants::network::*;
use nestgate_core::constants::hardcoding::*;

// RIGHT: Use canonical
use nestgate_core::constants::canonical::{performance, timeouts, network};
```

---

## ✅ BEST PRACTICES

### 1. Always Use Canonical Source

```rust
// Best practice
use nestgate_core::constants::canonical::*;
```

### 2. Use Environment Variables for Configurability

```rust
// Configuration flexibility
use nestgate_core::constants::port_defaults;
let port = port_defaults::get_api_port(); // Checks NESTGATE_API_PORT env var
```

### 3. Document Why Values Are Different

```rust
// When values look similar but shouldn't be consolidated
const NETWORK_BUFFER: usize = 8192;  // TCP window optimization
const DISK_BUFFER: usize = 4096;     // Page size optimization
// Different purposes, different values!
```

### 4. Add Tests for Critical Constants

```rust
#[test]
fn test_buffer_sizes_are_intentionally_different() {
    assert_eq!(performance::DEFAULT_BUFFER_SIZE, 4096);
    assert_eq!(performance::NETWORK_BUFFER_SIZE, 65536);
    assert_ne!(performance::DEFAULT_BUFFER_SIZE, performance::NETWORK_BUFFER_SIZE);
    // They're different on purpose!
}
```

---

## 🔍 FINDING CONSTANTS

### Quick Reference

| Need... | Use Module... | Constant... |
|---------|---------------|-------------|
| Port number | `port_defaults` | `DEFAULT_API_PORT` |
| Timeout | `canonical::timeouts` | `DEFAULT_TIMEOUT_MS` |
| Buffer (disk) | `canonical::performance` | `DEFAULT_BUFFER_SIZE` |
| Buffer (network) | `canonical::performance` | `NETWORK_BUFFER_SIZE` |
| Retry count | `canonical::timeouts` | `DEFAULT_RETRY_ATTEMPTS` |
| IP address | `canonical::network` | `DEFAULT_BIND_ADDRESS` |

### Search Commands

```bash
# Find a constant
rg "pub const DEFAULT_API_PORT" code/crates/nestgate-core/src/constants/

# Check where it's used
rg "DEFAULT_API_PORT" code/crates/ -t rust

# See all timeout constants
rg "pub const.*TIMEOUT" code/crates/nestgate-core/src/constants/canonical.rs
```

---

## 📚 RELATED DOCUMENTS

- `PHASE_3_CONSTANTS_STATUS_NOV_7_FINAL.md` - Consolidation progress
- `PHASE_3_EXECUTION_SESSION_NOV_7_EVENING.md` - Latest session work
- `UNIFICATION_ANALYSIS_NOV_7_2025_COMPREHENSIVE.md` - Full analysis

---

## 🎯 SUMMARY

**Remember**:
1. Use `canonical` module as single source
2. Don't consolidate domain-specific buffers
3. Use env variable support where available
4. Document why values are different
5. Test critical constants

**Single Source of Truth**:
- Port → `port_defaults`
- Timeouts → `canonical::timeouts`
- Buffers → `canonical::performance` (domain-specific!)
- Everything else → `canonical::*`

---

**Status**: ✅ COMPLETE  
**Version**: 1.0  
**Last Updated**: November 7, 2025  
**Maintainer**: NestGate Team


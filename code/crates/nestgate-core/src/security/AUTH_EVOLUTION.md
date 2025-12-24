# NestGate Auth Evolution: JWT → BearDog Crypto

**Status**: ✅ Implemented (v0.1.2)  
**Date**: December 23, 2025

---

## 🎯 **Design Philosophy**

**NestGate is a data service, not a security primal.**

Authentication is a **capability**, not a core concern. We delegate authentication to pluggable providers based on the deployment context.

---

## 📊 **Authentication Modes**

| Mode | Use Case | Auth Method | Sovereignty |
|------|----------|-------------|-------------|
| **beardog** (default) | Primal network | DID + cryptographic signatures | ✅ Decentralized |
| **jwt** (legacy) | Standalone NAS, external clients | Shared secret tokens | ⚠️ Centralized |
| **auto** (recommended) | Mixed environment | Try BearDog first, fallback to JWT | ✅ Adaptive |
| **none** (dev only) | Development/testing | No authentication | ❌ Insecure |

---

## 🏗️ **Architecture**

### Pluggable Provider System

```
┌─────────────────────────────────────────────────┐
│            API Request (HTTP)                    │
│  Headers: Authorization, X-Primal-DID, etc.     │
└────────────────┬────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────┐
│         AuthMiddleware (Axum)                    │
│  Extracts: JWT token, DID, signature            │
└────────────────┬────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────┐
│            AuthRouter                            │
│  Mode: beardog | jwt | auto | none              │
└────────────────┬────────────────────────────────┘
                 │
        ┌────────┴────────┐
        ▼                 ▼
┌──────────────┐  ┌──────────────┐
│  BearDog     │  │  JWT         │
│  Provider    │  │  Provider    │
│  (primary)   │  │  (legacy)    │
└──────┬───────┘  └──────┬───────┘
       │                 │
       ▼                 ▼
┌──────────────┐  ┌──────────────┐
│  BearDog     │  │  JWT Secret  │
│  Service     │  │  Validation  │
│  (external)  │  │  (internal)  │
└──────────────┘  └──────────────┘
```

---

## 🔐 **BearDog Mode (Primary)**

### Request Format

```http
POST /api/storage/store HTTP/1.1
Host: nestgate.local
X-Primal-DID: did:primal:beardog:abc123
X-Primal-Signature: 3a4f5b2c...
Content-Type: application/json

{
  "data": "..."
}
```

### Implementation

```rust
use nestgate_core::security::{AuthRequest, create_router_from_env};

// Create router (reads NESTGATE_AUTH_MODE from env)
let router = create_router_from_env();

// Authenticate request
let auth_req = AuthRequest {
    did: Some("did:primal:beardog:abc123".to_string()),
    signature: Some("3a4f5b2c...".to_string()),
    payload: Some(request_body.as_bytes().to_vec()),
    ..Default::default()
};

let auth_resp = router.authenticate(&auth_req).await?;

if auth_resp.authenticated {
    // Proceed with request
    println!("Authenticated as: {}", auth_resp.principal.unwrap());
} else {
    // Reject request
    return Err("Unauthorized");
}
```

### Why BearDog?

- ✅ **Sovereignty**: No shared secrets between primals
- ✅ **Decentralized**: Each primal has its own DID/keypair
- ✅ **HSM Support**: Hardware-backed keys when available
- ✅ **Genetic Crypto**: Algorithm-agnostic, future-proof
- ✅ **Audit Trail**: Cryptographic proof of all actions

---

## 🔑 **JWT Mode (Legacy)**

### Request Format

```http
POST /api/storage/store HTTP/1.1
Host: nestgate.local
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
Content-Type: application/json

{
  "data": "..."
}
```

### Implementation

```rust
use nestgate_core::security::{AuthRequest, create_router_from_env};

// Create router
let router = create_router_from_env();

// Authenticate request
let auth_req = AuthRequest {
    token: Some("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...".to_string()),
    ..Default::default()
};

let auth_resp = router.authenticate(&auth_req).await?;

if auth_resp.authenticated {
    // Proceed with request
    println!("Authenticated as: {}", auth_resp.principal.unwrap());
} else {
    // Reject request
    return Err("Unauthorized");
}
```

### Why JWT?

- ✅ **Compatibility**: Works with existing NAS systems
- ✅ **Simplicity**: Easy to integrate with external clients
- ✅ **Tooling**: Wide ecosystem support
- ⚠️ **Centralized**: Requires shared secret management
- ⚠️ **Less Secure**: Vulnerable to secret leakage

---

## 🔄 **Auto Mode (Recommended)**

Tries BearDog first (preferred), falls back to JWT if BearDog not available.

### Request Handling

```
1. Check for BearDog credentials (DID + signature)
   ├─ Found? → Use BearDog provider
   └─ Not found? → Continue

2. Check for JWT token (Authorization header)
   ├─ Found? → Use JWT provider
   └─ Not found? → Reject (401 Unauthorized)
```

### Configuration

```bash
# Environment variable
export NESTGATE_AUTH_MODE=auto

# Or in config file
[security]
auth_mode = "auto"
```

---

## ⚙️ **Configuration**

### Environment Variables

| Variable | Values | Default | Description |
|----------|--------|---------|-------------|
| `NESTGATE_AUTH_MODE` | `beardog`, `jwt`, `auto`, `none` | `auto` | Authentication mode |
| `BEARDOG_URL` | URL | - | BearDog service endpoint |
| `BEARDOG_ALLOW_FALLBACK` | `true`, `false` | `false` | Allow mock validation if BearDog unavailable |
| `NESTGATE_JWT_SECRET` | String (32+ chars) | - | JWT secret key |
| `NESTGATE_ENFORCE_JWT` | `true`, `false` | `true` | Enforce JWT secret validation |

### Example Configurations

#### Production (Primal Network)
```bash
export NESTGATE_AUTH_MODE=beardog
export BEARDOG_URL=http://beardog.local:8080
export BEARDOG_ALLOW_FALLBACK=false  # Strict
```

#### Development (Mixed)
```bash
export NESTGATE_AUTH_MODE=auto
export BEARDOG_URL=http://localhost:8080
export BEARDOG_ALLOW_FALLBACK=true  # Permissive
export NESTGATE_JWT_SECRET=$(openssl rand -base64 48)
export NESTGATE_ENFORCE_JWT=false
```

#### Standalone NAS
```bash
export NESTGATE_AUTH_MODE=jwt
export NESTGATE_JWT_SECRET=$(openssl rand -base64 48)
export NESTGATE_ENFORCE_JWT=true
```

#### Testing
```bash
export NESTGATE_AUTH_MODE=none  # ⚠️ INSECURE - dev only!
```

---

## 🧪 **Testing**

### Unit Tests

```bash
# Test auth providers
cargo test --package nestgate-core auth_provider

# Test middleware
cargo test --package nestgate-api auth_middleware
```

### Integration Tests

#### Test BearDog Auth (with mock BearDog)
```bash
export BEARDOG_URL=http://localhost:8080
export BEARDOG_ALLOW_FALLBACK=true
export NESTGATE_AUTH_MODE=beardog

cargo test --package nestgate-api --test integration_tests -- beardog
```

#### Test JWT Auth
```bash
export NESTGATE_JWT_SECRET="test-secret-at-least-32-characters-long"
export NESTGATE_AUTH_MODE=jwt

cargo test --package nestgate-api --test integration_tests -- jwt
```

#### Test Auto Mode
```bash
export NESTGATE_AUTH_MODE=auto
export BEARDOG_URL=http://localhost:8080
export BEARDOG_ALLOW_FALLBACK=true
export NESTGATE_JWT_SECRET="test-secret-at-least-32-characters-long"

cargo test --package nestgate-api --test integration_tests -- auto
```

---

## 📚 **API Usage Examples**

### Axum Integration

```rust
use axum::{Router, middleware};
use nestgate_api::middleware::{AuthMiddleware, auth_middleware};
use nestgate_core::security::create_router_from_env;

// Create auth router from environment
let auth_router = create_router_from_env();
let auth_mw = AuthMiddleware::new(auth_router);

// Add to Axum app
let app = Router::new()
    .route("/api/storage/store", post(store_handler))
    .layer(middleware::from_fn_with_state(
        auth_mw.clone(),
        auth_middleware,
    ));
```

### Manual Authentication

```rust
use nestgate_core::security::{AuthRequest, AuthRouter, AuthMode};

// Create router manually
let mut router = AuthRouter::new(AuthMode::Auto);
router.register_provider(Box::new(BearDogAuthProvider::new()));
router.register_provider(Box::new(JwtAuthProvider::new()));

// Authenticate
let request = AuthRequest {
    did: Some("did:primal:beardog:123".to_string()),
    signature: Some("abcd1234".to_string()),
    ..Default::default()
};

match router.authenticate(&request).await {
    Ok(response) if response.authenticated => {
        println!("✅ Authenticated: {}", response.principal.unwrap());
        println!("   Permissions: {:?}", response.permissions);
        println!("   Method: {}", response.auth_method);
    }
    Ok(response) => {
        println!("❌ Authentication failed: {}", response.message);
    }
    Err(e) => {
        println!("❌ Error: {}", e);
    }
}
```

---

## 🚀 **Migration Path**

### Phase 1: Current (v0.1.2)
- ✅ Pluggable auth architecture implemented
- ✅ JWT provider (functional)
- ✅ BearDog provider (placeholder, awaits BearDog integration)
- ✅ Auto mode (with fallback)
- ✅ Configuration via environment

### Phase 2: BearDog Integration (v0.2.0)
- [ ] Implement actual HTTP client to BearDog service
- [ ] Add signature verification via BearDog API
- [ ] Add DID resolution
- [ ] Add HSM support detection
- [ ] Performance testing with real BearDog

### Phase 3: Advanced Features (v0.3.0)
- [ ] Token refresh/rotation
- [ ] Permission caching
- [ ] Audit logging
- [ ] Rate limiting per principal
- [ ] Multi-factor authentication support

---

## 🔗 **Related Documentation**

- [`HANDOFF_TO_BEARDOG.md`](../../../../showcase/03_encryption_storage/HANDOFF_TO_BEARDOG.md) - BearDog encryption integration
- [`jwt_validation.rs`](./jwt_validation.rs) - JWT secret validation
- [`universal_auth_adapter.rs`](./universal_auth_adapter.rs) - Legacy auth adapter

---

## 📝 **Status Summary**

| Component | Status | Notes |
|-----------|--------|-------|
| **Auth Provider Trait** | ✅ Complete | Pluggable architecture |
| **JWT Provider** | ✅ Complete | Functional (simplified validation) |
| **BearDog Provider** | 🔄 Partial | Placeholder, awaits BearDog service |
| **Auth Router** | ✅ Complete | Mode-based routing |
| **Auth Middleware** | ✅ Complete | Axum integration |
| **Configuration** | ✅ Complete | Environment-driven |
| **Unit Tests** | ✅ Complete | All providers tested |
| **Integration Tests** | ⏳ Pending | Awaits BearDog mock service |
| **Documentation** | ✅ Complete | This file |

---

## 🎉 **Result**

**NestGate now has a sovereign, pluggable authentication system!**

- ✅ **No more JWT-only blocking** - Multiple auth methods supported
- ✅ **BearDog-ready** - Architecture in place, awaits service integration
- ✅ **Testable** - Both eco-internal and external auth can be tested
- ✅ **Configurable** - Environment-driven, no code changes needed
- ✅ **Maintainable** - Clear separation of concerns

**Next**: Integrate with actual BearDog service when available. 🐻

---

**End of Document**


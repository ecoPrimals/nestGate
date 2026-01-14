# 🔧 NestGate Transport Evolution - Implementation Plan

**Date**: January 14, 2026  
**Priority**: HIGH (Blocks NUCLEUS persistence)  
**From**: biomeOS Team Handoff  
**Status**: Planning → Implementation  
**Estimated Effort**: 2-4 hours

---

## 🎯 **MISSION**

**Evolve NestGate from HTTP/REST to Unix Sockets + JSON-RPC 2.0**

This is the **final primal evolution** for TRUE PRIMAL perfection.

---

## 📊 **CURRENT STATE ANALYSIS**

### **What NestGate Has** ❌:
```rust
// HTTP/REST on port 8080
nestgate service start --port 8080 --bind 0.0.0.0

Transport:    HTTP/REST
Port:         8080 (hardcoded)
Auth:         JWT tokens
Discovery:    Manual (localhost:8080)
Security:     Fallback JWT
```

### **What Other Primals Have** ✅:
```rust
// Unix socket + JSON-RPC
primal service start \
  --socket /tmp/primal-${FAMILY_ID}.sock \
  --security-provider /tmp/beardog-${FAMILY_ID}-default.sock

Transport:    Unix sockets
Port:         NONE (port-free)
Auth:         BearDog integration
Discovery:    Socket scanning
Security:     Hardware-backed (FIDO2/HSM)
```

### **Status Matrix**:
```
Primal      | Transport      | Security | Socket Pattern                    | Status
------------|----------------|----------|-----------------------------------|--------
BearDog     | Unix socket    | Self     | /tmp/beardog-{family}-default.sock| ✅
Songbird    | Unix socket    | BearDog  | /tmp/songbird-{family}.sock       | ✅
Toadstool   | Unix socket    | BearDog  | /tmp/toadstool-{family}.sock      | ✅
Squirrel    | Unix socket    | BearDog  | /tmp/squirrel-{family}.sock       | ✅
NestGate    | HTTP (8080)    | JWT      | localhost:8080                    | ❌ NEEDS EVOLUTION
```

---

## 🧬 **WHY THIS MATTERS**

### **Blocks NUCLEUS Deployment**:
```
NUCLEUS = Tower + Node + Nest

Tower:  BearDog + Songbird        ✅ Unix sockets
Node:   Tower + Toadstool          ✅ Unix sockets  
Nest:   Tower + NestGate           ❌ HTTP (BLOCKED)
```

**Without Nest working**:
- ❌ No data persistence in NUCLEUS
- ❌ Data won't survive ecosystem restarts
- ❌ Incomplete TRUE PRIMAL compliance
- ❌ Can't deploy to LiveSpore USB

### **Aligns With Our Evolution Goals**:

This perfectly addresses our **existing deep debt**:

1. **Hardcoding Evolution** (F → A):
   - Remove hardcoded port 8080
   - Remove hardcoded localhost
   - Use environment variables + discovery

2. **Primal Self-Knowledge** (Pending):
   - Only know NestGate identity
   - Discover BearDog at runtime
   - Discover via socket scanning

3. **Capability-Based Discovery** (Pending):
   - No hardcoded endpoints
   - Dynamic service discovery
   - Runtime capability detection

**This is NOT new work - it's the NEXT STEP in our planned evolution!**

---

## 🏗️ **IMPLEMENTATION PLAN**

### **Phase 1: Unix Socket Support** (1-2 hours) 🔴 **CRITICAL**

#### **1.1 Add Unix Socket Listener**
```rust
// Location: nestgate-api/src/server/mod.rs

use tokio::net::UnixListener;
use std::path::PathBuf;

pub struct NestGateServer {
    // Existing
    http_config: Option<HttpConfig>,
    
    // NEW: Unix socket (primary)
    socket_path: PathBuf,
    security_provider: PathBuf,
}

impl NestGateServer {
    pub async fn start_unix_socket(&self) -> Result<()> {
        let family_id = std::env::var("NESTGATE_FAMILY_ID")
            .unwrap_or_else(|_| "default".to_string());
            
        let socket_path = std::env::var("NESTGATE_SOCKET_PATH")
            .unwrap_or_else(|_| format!("/tmp/nestgate-{}.sock", family_id));
        
        // Remove old socket if exists
        let _ = std::fs::remove_file(&socket_path);
        
        let listener = UnixListener::bind(&socket_path)?;
        tracing::info!("NestGate listening on Unix socket: {}", socket_path);
        
        loop {
            let (stream, _) = listener.accept().await?;
            tokio::spawn(self.handle_connection(stream));
        }
    }
}
```

#### **1.2 Environment Variables**
```bash
# Configuration via environment
export NESTGATE_FAMILY_ID="nat0"
export NESTGATE_SOCKET_PATH="/tmp/nestgate-nat0.sock"
export NESTGATE_SECURITY_PROVIDER="/tmp/beardog-nat0-default.sock"

# Start NestGate
nestgate service start
```

#### **1.3 Command Line Arguments**
```rust
// CLI flags for backward compatibility
#[derive(Parser)]
pub struct ServeCommand {
    /// Unix socket path (primary)
    #[arg(long, env = "NESTGATE_SOCKET_PATH")]
    socket: Option<PathBuf>,
    
    /// Family ID
    #[arg(long, env = "NESTGATE_FAMILY_ID")]
    family_id: Option<String>,
    
    /// Security provider socket (BearDog)
    #[arg(long, env = "NESTGATE_SECURITY_PROVIDER")]
    security_provider: Option<PathBuf>,
    
    /// HTTP port (optional fallback)
    #[arg(long, env = "NESTGATE_HTTP_PORT")]
    http_port: Option<u16>,
}
```

---

### **Phase 2: JSON-RPC 2.0 Support** (1 hour) 🟡 **HIGH**

#### **2.1 Add JSON-RPC Dependency**
```toml
# Cargo.toml
[dependencies]
jsonrpc-core = "18.0"
jsonrpc-derive = "18.0"
serde_json = "1.0"
```

#### **2.2 JSON-RPC Server Implementation**
```rust
// Location: nestgate-api/src/jsonrpc/mod.rs

use jsonrpc_core::{IoHandler, Params, Result as RpcResult};
use jsonrpc_derive::rpc;

#[rpc]
pub trait StorageRpc {
    /// Store data
    #[rpc(name = "storage.store")]
    fn store(&self, key: String, value: Vec<u8>) -> RpcResult<bool>;
    
    /// Retrieve data
    #[rpc(name = "storage.retrieve")]
    fn retrieve(&self, key: String) -> RpcResult<Option<Vec<u8>>>;
    
    /// Delete data
    #[rpc(name = "storage.delete")]
    fn delete(&self, key: String) -> RpcResult<bool>;
    
    /// List keys
    #[rpc(name = "storage.list")]
    fn list(&self) -> RpcResult<Vec<String>>;
}

#[rpc]
pub trait HealthRpc {
    /// Health check
    #[rpc(name = "health.check")]
    fn check(&self) -> RpcResult<HealthStatus>;
}

#[rpc]
pub trait IdentityRpc {
    /// Get NestGate identity
    #[rpc(name = "identity.get")]
    fn get(&self) -> RpcResult<Identity>;
}
```

#### **2.3 RPC Handler**
```rust
pub async fn handle_jsonrpc_request(
    stream: UnixStream,
    handler: IoHandler,
) -> Result<()> {
    let mut buf = vec![0u8; 8192];
    let n = stream.readable().await?.read(&mut buf).await?;
    
    let request = String::from_utf8(buf[..n].to_vec())?;
    let response = handler.handle_request_sync(&request);
    
    if let Some(response) = response {
        stream.writable().await?.write_all(response.as_bytes()).await?;
    }
    
    Ok(())
}
```

---

### **Phase 3: BearDog Integration** (1-2 hours) 🟡 **HIGH**

#### **3.1 Add BearDog Client**
```rust
// Location: nestgate-core/src/security/beardog_client.rs

use tokio::net::UnixStream;
use serde_json::json;

pub struct BearDogClient {
    socket_path: PathBuf,
}

impl BearDogClient {
    pub fn from_socket(path: impl Into<PathBuf>) -> Self {
        Self {
            socket_path: path.into(),
        }
    }
    
    /// Verify signature via BearDog
    pub async fn verify_signature(
        &self,
        signature: &[u8],
        payload: &[u8],
    ) -> Result<bool> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "crypto.verify_signature",
            "params": {
                "signature": hex::encode(signature),
                "payload": hex::encode(payload),
            },
            "id": 1,
        });
        
        self.send_request(&request).await
    }
    
    /// Encrypt data via BearDog
    pub async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "crypto.encrypt",
            "params": {
                "data": hex::encode(data),
            },
            "id": 2,
        });
        
        self.send_request(&request).await
    }
    
    /// Decrypt data via BearDog
    pub async fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "crypto.decrypt",
            "params": {
                "data": hex::encode(data),
            },
            "id": 3,
        });
        
        self.send_request(&request).await
    }
    
    async fn send_request<T: DeserializeOwned>(
        &self,
        request: &serde_json::Value,
    ) -> Result<T> {
        let stream = UnixStream::connect(&self.socket_path).await?;
        let request_str = serde_json::to_string(request)?;
        
        stream.writable().await?.write_all(request_str.as_bytes()).await?;
        
        let mut buf = vec![0u8; 8192];
        let n = stream.readable().await?.read(&mut buf).await?;
        
        let response: serde_json::Value = serde_json::from_slice(&buf[..n])?;
        
        if let Some(result) = response.get("result") {
            Ok(serde_json::from_value(result.clone())?)
        } else if let Some(error) = response.get("error") {
            Err(NestGateError::rpc_error(error.to_string()))
        } else {
            Err(NestGateError::rpc_error("Invalid JSON-RPC response"))
        }
    }
}
```

#### **3.2 Replace JWT with BearDog**
```rust
// OLD: JWT authentication
async fn authenticate_jwt(&self, token: &str) -> Result<Identity> {
    let claims = decode_jwt(token)?;
    Ok(Identity::from_claims(claims))
}

// NEW: BearDog authentication
async fn authenticate_beardog(&self, request: &Request) -> Result<Identity> {
    let beardog = BearDogClient::from_socket(&self.security_provider)?;
    
    // Verify signature
    beardog.verify_signature(
        &request.signature,
        &request.payload,
    ).await?;
    
    // Get identity from public key
    let identity = beardog.get_identity(&request.public_key).await?;
    
    Ok(identity)
}
```

---

### **Phase 4: Keep HTTP as Optional Fallback** (30 min) 🟢 **MEDIUM**

#### **4.1 Dual-Mode Server**
```rust
pub async fn start(&self) -> Result<()> {
    // Primary: Unix socket
    let unix_handle = tokio::spawn(self.start_unix_socket());
    
    // Optional: HTTP fallback
    let http_handle = if let Some(port) = self.http_config.as_ref().map(|c| c.port) {
        tracing::warn!("HTTP fallback enabled on port {}", port);
        Some(tokio::spawn(self.start_http_server(port)))
    } else {
        tracing::info!("HTTP fallback disabled (TRUE PRIMAL mode)");
        None
    };
    
    // Wait for both
    tokio::select! {
        res = unix_handle => res??,
        res = async { http_handle.unwrap().await }, if http_handle.is_some() => res??,
    }
}
```

---

## ✅ **SUCCESS CRITERIA**

**You'll know it works when:**

1. ✅ NestGate creates Unix socket on startup
   ```bash
   ls -la /tmp/nestgate-nat0.sock
   # Should exist
   ```

2. ✅ biomeOS can discover NestGate via socket scanning
   ```bash
   biomeos scan-sockets
   # Should show: NestGate at /tmp/nestgate-nat0.sock
   ```

3. ✅ BearDog handles all security (no JWT)
   ```bash
   # No JWT secrets in config
   # All crypto via BearDog socket
   ```

4. ✅ Full NUCLEUS deploys
   ```bash
   nucleus deploy --family nat0
   # Deploys: BearDog + Songbird + Toadstool + NestGate
   # All via Unix sockets
   ```

5. ✅ Data persists across restarts
   ```bash
   nucleus restart
   # Data still available after restart
   ```

6. ✅ HTTP is optional
   ```bash
   nestgate service start  # Unix socket only
   nestgate service start --http-port 8080  # Unix + HTTP
   ```

---

## 📋 **IMPLEMENTATION CHECKLIST**

### **Phase 1: Unix Socket** 🔴 **CRITICAL** (1-2 hours)
- [ ] Add `tokio::net::UnixListener` support
- [ ] Environment variable configuration
- [ ] Socket path resolution (`/tmp/nestgate-{family}.sock`)
- [ ] Auto-cleanup old sockets
- [ ] Connection handler
- [ ] Tests for Unix socket communication

### **Phase 2: JSON-RPC** 🟡 **HIGH** (1 hour)
- [ ] Add `jsonrpc-core` dependency
- [ ] Implement `StorageRpc` trait
- [ ] Implement `HealthRpc` trait
- [ ] Implement `IdentityRpc` trait
- [ ] JSON-RPC request handler
- [ ] Tests for JSON-RPC methods

### **Phase 3: BearDog** 🟡 **HIGH** (1-2 hours)
- [ ] Create `BearDogClient` struct
- [ ] Implement `verify_signature()`
- [ ] Implement `encrypt()`
- [ ] Implement `decrypt()`
- [ ] Implement `get_identity()`
- [ ] Replace JWT authentication
- [ ] Tests for BearDog integration

### **Phase 4: HTTP Fallback** 🟢 **MEDIUM** (30 min)
- [ ] Make HTTP optional via flag
- [ ] Dual-mode server (Unix + HTTP)
- [ ] Environment variable: `NESTGATE_HTTP_PORT`
- [ ] Warning when HTTP is enabled
- [ ] Tests for both modes

### **Phase 5: Testing** 🟢 **MEDIUM** (30 min)
- [ ] Unit tests for Unix socket
- [ ] Unit tests for JSON-RPC
- [ ] Unit tests for BearDog client
- [ ] Integration test with real BearDog
- [ ] Integration test with biomeOS
- [ ] NUCLEUS deployment test

---

## 📊 **EFFORT ESTIMATE**

```
Phase 1 (Unix Socket):      1-2 hours  🔴 CRITICAL
Phase 2 (JSON-RPC):         1 hour     🟡 HIGH
Phase 3 (BearDog):          1-2 hours  🟡 HIGH
Phase 4 (HTTP Fallback):    30 min     🟢 MEDIUM
Phase 5 (Testing):          30 min     🟢 MEDIUM

TOTAL:                      4-6 hours
```

**Priority**: Complete Phase 1-3 first (core functionality)

---

## 🎯 **ALIGNMENT WITH EXISTING DEBT**

This evolution **directly addresses** our existing deep debt:

| Debt Category | Current | After Evolution | Impact |
|---------------|---------|-----------------|--------|
| **Hardcoding** | F (45/100) | B+ (85/100) | ⬆️ +40 points |
| **Primal Self-Knowledge** | Pending | Complete | ✅ Full compliance |
| **Capability Discovery** | Pending | Complete | ✅ Runtime discovery |
| **Overall Grade** | B+ (88/100) | A (92/100) | ⬆️ +4 points |

**This is HIGH LEVERAGE work** - one evolution fixes multiple debt categories!

---

## 📖 **REFERENCE IMPLEMENTATIONS**

**Study these for patterns:**

1. **Squirrel** (`phase1/squirrel/`):
   - Recently evolved to Unix sockets (Jan 2026)
   - Clean, modern implementation
   - Good testing examples

2. **Songbird** (`plasmidBin/primals/songbird-orchestrator`):
   - Mature Unix socket implementation
   - BearDog integration examples
   - JSON-RPC patterns

3. **Toadstool** (`plasmidBin/primals/toadstool`):
   - Simple socket setup
   - Good error handling
   - Minimal dependencies

4. **biomeOS** (`biomeOS/crates/biomeos-core/src/clients/`):
   - `beardog/` - BearDog client reference
   - `transport/` - Transport abstraction
   - `discovery/` - Socket scanning

---

## 🤝 **COLLABORATION**

**biomeOS Team Offers**:
- ✅ Code review for PRs
- ✅ Testing NUCLEUS deployment
- ✅ BearDog integration support
- ✅ Documentation updates

**We Should**:
- Create feature branch: `feature/unix-socket-transport`
- Regular commits with tests
- Ask for review after Phase 3
- Test with real NUCLEUS deployment

---

## 🎊 **IMPACT**

### **Technical**:
- ✅ 100x faster IPC (Unix sockets vs HTTP)
- ✅ No port management
- ✅ Hardware-backed security (FIDO2/HSM)
- ✅ TRUE PRIMAL compliant

### **Ecosystem**:
- ✅ **NUCLEUS production-ready**
- ✅ Full ecosystem deployment
- ✅ LiveSpore USB compatibility
- ✅ Final primal evolution complete

### **Debt**:
- ✅ Hardcoding: F → B+ (+40 points)
- ✅ Primal Self-Knowledge: Complete
- ✅ Capability Discovery: Complete
- ✅ Overall: B+ → A (+4 points)

---

## 🚀 **NEXT STEPS**

**Immediate** (Today):
1. Create feature branch
2. Start Phase 1 (Unix socket)
3. Add basic tests

**This Week**:
1. Complete Phase 1-3 (core functionality)
2. Test with local BearDog
3. Request biomeOS code review

**Next Week**:
1. Complete Phase 4-5 (fallback + testing)
2. Integration test with NUCLEUS
3. Merge to main
4. Deploy to production

---

## 📝 **NOTES**

- This is **NOT new technical debt** - it's the next evolution step
- Aligns perfectly with our existing roadmap
- High leverage: fixes multiple debt categories
- Unblocks production NUCLEUS deployment
- Final piece for TRUE PRIMAL ecosystem

---

**Status**: Ready to implement  
**Priority**: HIGH (blocks NUCLEUS)  
**Complexity**: Medium (well-defined patterns)  
**Confidence**: Very High (reference implementations exist)

---

*"The final primal evolution for TRUE PRIMAL perfection."* 🧬🚀✨

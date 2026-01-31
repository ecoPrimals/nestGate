# 🔌 Isomorphic IPC Implementation Plan for NestGate

**Date**: January 31, 2026  
**Priority**: MEDIUM (Part of NEST atomic)  
**Effort**: 6-8 hours  
**Status**: Ready for Implementation

═══════════════════════════════════════════════════════════════════

## 🎯 OBJECTIVE

Implement isomorphic IPC for NestGate following the Try→Detect→Adapt→Succeed pattern validated in songbird v3.33.0 (A++ grade, 205/100).

**Goal**: Same binary works on ALL platforms (Linux, Android, etc.) with automatic TCP fallback when Unix sockets are unavailable.

═══════════════════════════════════════════════════════════════════

## 📊 CURRENT STATE ANALYSIS

### **Existing Infrastructure** ✅

**NestGate already has**:
1. ✅ Unix socket server (`code/crates/nestgate-core/src/rpc/unix_socket_server.rs`)
2. ✅ JSON-RPC 2.0 protocol implementation
3. ✅ Socket configuration (`code/crates/nestgate-core/src/rpc/socket_config.rs`)
4. ✅ Universal trait patterns (from Phase 2 & 3 evolution)
5. ✅ Environment-driven configuration (XDG-compliant)

### **What We Need to Add** 🔄

1. 🔄 Platform constraint detection (`is_platform_constraint()`)
2. 🔄 TCP fallback server (`start_tcp_fallback()`)
3. 🔄 Discovery file system (TCP endpoint discovery)
4. 🔄 Client-side discovery (`IpcEndpoint` enum)
5. 🔄 Polymorphic stream trait (`AsyncStream`)

### **Advantage** 🏆

**NestGate's Recent Evolution**:
- ✅ **56% platform code reduction** (9 → 4 files in Phase 1-3)
- ✅ **Trait pattern proven 4 times** (block storage, service, network, filesystem)
- ✅ **Universal, agnostic, modern idiom

atic Rust** (our specialty!)
- ✅ **Strong foundation** for isomorphic evolution

**This aligns perfectly with our deep debt evolution!**

═══════════════════════════════════════════════════════════════════

## 🏗️ ARCHITECTURE

### **The Try→Detect→Adapt→Succeed Pattern**

```rust
pub async fn start_ipc_server(&self) -> Result<()> {
    info!("🔌 Starting IPC server (isomorphic mode)...");
    
    // 1. TRY Unix socket first (optimal)
    info!("   Trying Unix socket IPC (optimal)...");
    match self.try_unix_server().await {
        Ok(()) => {
            info!("✅ Unix socket IPC active");
            Ok(())
        }
        
        // 2. DETECT platform constraints
        Err(e) if self.is_platform_constraint(&e) => {
            warn!("⚠️  Unix sockets unavailable: {}", e);
            warn!("   Detected platform constraint, adapting...");
            
            // 3. ADAPT to TCP fallback
            info!("🌐 Starting TCP IPC fallback (isomorphic mode)");
            self.start_tcp_fallback().await
        }
        
        // 4. Real error
        Err(e) => {
            error!("❌ Real error (not platform constraint): {}", e);
            Err(e)
        }
    }
}
```

### **Module Structure**

```
code/crates/nestgate-core/src/rpc/
├── mod.rs (update exports)
├── unix_socket_server.rs (existing - refactor)
├── isomorphic_ipc/
│   ├── mod.rs (new)
│   ├── server.rs (new - isomorphic server)
│   ├── platform_detection.rs (new - constraint detection)
│   ├── tcp_fallback.rs (new - TCP server)
│   ├── discovery.rs (new - endpoint discovery)
│   └── streams.rs (new - polymorphic streams)
```

═══════════════════════════════════════════════════════════════════

## 📋 IMPLEMENTATION STEPS

### **Phase 1: Server-Side Isomorphic IPC** (4-5 hours)

#### **Task 1.1: Platform Constraint Detection** (30 min)

**File**: `code/crates/nestgate-core/src/rpc/isomorphic_ipc/platform_detection.rs`

**Implementation**:
```rust
//! Platform constraint detection
//!
//! **RUNTIME DETECTION**: Determines if errors are platform constraints vs real errors

use std::io::ErrorKind;

/// Detects if an error is due to platform constraints (not a real error)
pub fn is_platform_constraint(error: &anyhow::Error) -> bool {
    if let Some(io_err) = error.downcast_ref::<std::io::Error>() {
        match io_err.kind() {
            // Permission denied - check for SELinux/Android restrictions
            ErrorKind::PermissionDenied => is_selinux_enforcing(),
            
            // Platform doesn't support Unix sockets
            ErrorKind::Unsupported => true,
            
            // Address family not supported
            ErrorKind::AddrNotAvailable => true,
            
            _ => false
        }
    } else {
        false
    }
}

/// Check if SELinux is enforcing (Android/Linux)
fn is_selinux_enforcing() -> bool {
    std::fs::read_to_string("/sys/fs/selinux/enforce")
        .ok()
        .and_then(|s| s.trim().parse::<u8>().ok())
        .map(|v| v == 1)
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_selinux_detection() {
        // Should not panic
        let _ = is_selinux_enforcing();
    }
}
```

**Validation**: Copy pattern from songbird's implementation

---

#### **Task 1.2: TCP Fallback Server** (2-3 hours)

**File**: `code/crates/nestgate-core/src/rpc/isomorphic_ipc/tcp_fallback.rs`

**Implementation**:
```rust
//! TCP IPC Fallback Server
//!
//! **ISOMORPHIC**: Automatic fallback when Unix sockets unavailable
//! **PROTOCOL**: Same JSON-RPC 2.0 as Unix sockets
//! **SECURITY**: Localhost only (127.0.0.1), same as Unix sockets

use anyhow::{Context, Result};
use serde_json::Value;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info, warn};

/// TCP fallback server for isomorphic IPC
pub struct TcpFallbackServer {
    /// Service name
    service_name: String,
    /// RPC handler
    handler: Arc<dyn RpcHandler>,
}

impl TcpFallbackServer {
    pub fn new(service_name: String, handler: Arc<dyn RpcHandler>) -> Self {
        Self {
            service_name,
            handler,
        }
    }
    
    /// Start TCP fallback server
    pub async fn start(self: Arc<Self>) -> Result<()> {
        info!("🌐 Starting TCP IPC fallback (isomorphic mode)");
        info!("   Protocol: JSON-RPC 2.0 (same as Unix socket)");
        
        // Bind to localhost only (security: same as Unix socket)
        let listener = TcpListener::bind("127.0.0.1:0").await
            .context("Failed to bind TCP socket for IPC fallback")?;
        
        let local_addr = listener.local_addr()?;
        info!("✅ TCP IPC listening on {}", local_addr);
        
        // Write discovery file for clients
        self.write_tcp_discovery_file(&local_addr)?;
        
        info!("   Status: READY ✅ (isomorphic TCP fallback active)");
        
        // Accept connections (same pattern as Unix)
        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    info!("📥 TCP client connected: {}", addr);
                    let handler = self.clone();
                    
                    tokio::spawn(async move {
                        if let Err(e) = handler.handle_tcp_connection(stream).await {
                            error!("TCP connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Failed to accept TCP connection: {}", e);
                }
            }
        }
    }
    
    /// Handle TCP connection (JSON-RPC over TCP)
    async fn handle_tcp_connection(&self, stream: TcpStream) -> Result<()> {
        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);
        let mut line = String::new();
        
        loop {
            line.clear();
            
            match reader.read_line(&mut line).await {
                Ok(0) => break, // Connection closed
                Ok(_) => {
                    // Parse JSON-RPC request
                    match serde_json::from_str::<Value>(&line) {
                        Ok(request) => {
                            // Handle request (same as Unix socket)
                            let response = self.handler.handle_request(request).await;
                            
                            // Send response
                            let response_str = serde_json::to_string(&response)?;
                            writer.write_all(response_str.as_bytes()).await?;
                            writer.write_all(b"\n").await?;
                        }
                        Err(e) => {
                            warn!("Invalid JSON-RPC request: {}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("TCP read error: {}", e);
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    /// Write TCP discovery file (XDG-compliant)
    fn write_tcp_discovery_file(&self, addr: &SocketAddr) -> Result<()> {
        use std::fs::File;
        use std::io::Write;
        
        // XDG-compliant discovery file paths
        let discovery_dirs = [
            std::env::var("XDG_RUNTIME_DIR").ok(),
            std::env::var("HOME").map(|h| format!("{}/.local/share", h)),
            Some("/tmp".to_string()),
        ];
        
        for dir in discovery_dirs.iter().filter_map(|d| d.as_ref()) {
            let discovery_file = format!("{}/{}-ipc-port", dir, self.service_name);
            
            if let Ok(mut f) = File::create(&discovery_file) {
                // Write in format: tcp:127.0.0.1:PORT
                writeln!(f, "tcp:{}", addr)?;
                info!("📁 TCP discovery file: {}", discovery_file);
                return Ok(());
            }
        }
        
        warn!("⚠️  Could not write TCP discovery file (clients may not find endpoint)");
        Ok(())
    }
}

/// RPC handler trait (same interface as Unix socket handler)
#[async_trait::async_trait]
pub trait RpcHandler: Send + Sync {
    async fn handle_request(&self, request: Value) -> Value;
}
```

---

#### **Task 1.3: Isomorphic Server Facade** (1 hour)

**File**: `code/crates/nestgate-core/src/rpc/isomorphic_ipc/server.rs`

**Implementation**:
```rust
//! Isomorphic IPC Server
//!
//! **UNIVERSAL**: Automatically adapts to platform constraints
//! **PATTERN**: Try→Detect→Adapt→Succeed

use anyhow::Result;
use std::sync::Arc;
use tracing::{info, warn};

use super::platform_detection::is_platform_constraint;
use super::tcp_fallback::TcpFallbackServer;
use crate::rpc::unix_socket_server::JsonRpcUnixServer;

/// Isomorphic IPC server (Unix socket OR TCP fallback)
pub struct IsomorphicIpcServer {
    service_name: String,
    handler: Arc<dyn RpcHandler>,
}

impl IsomorphicIpcServer {
    pub fn new(service_name: String, handler: Arc<dyn RpcHandler>) -> Self {
        Self {
            service_name,
            handler,
        }
    }
    
    /// Start isomorphic IPC server (Try→Detect→Adapt→Succeed)
    pub async fn start(self: Arc<Self>) -> Result<()> {
        info!("🔌 Starting IPC server (isomorphic mode)...");
        info!("   Service: {}", self.service_name);
        
        // 1. TRY Unix socket first (optimal)
        info!("   Trying Unix socket IPC (optimal)...");
        match self.try_unix_server().await {
            Ok(()) => {
                info!("✅ Unix socket IPC active (optimal path)");
                Ok(())
            }
            
            // 2. DETECT platform constraints
            Err(e) if is_platform_constraint(&e) => {
                warn!("⚠️  Unix sockets unavailable: {}", e);
                warn!("   Detected platform constraint, adapting...");
                
                // 3. ADAPT to TCP fallback
                self.start_tcp_fallback().await
            }
            
            // 4. Real error
            Err(e) => {
                error!("❌ Failed to start IPC server: {}", e);
                Err(e)
            }
        }
    }
    
    /// Try to start Unix socket server
    async fn try_unix_server(&self) -> Result<()> {
        let unix_server = JsonRpcUnixServer::new(&self.service_name, self.handler.clone()).await?;
        unix_server.serve().await
    }
    
    /// Start TCP fallback server
    async fn start_tcp_fallback(&self) -> Result<()> {
        let tcp_server = Arc::new(TcpFallbackServer::new(
            self.service_name.clone(),
            self.handler.clone(),
        ));
        
        tcp_server.start().await
    }
}
```

---

### **Phase 2: Client-Side Discovery** (2-3 hours)

#### **Task 2.1: IPC Endpoint Enum** (30 min)

**File**: `code/crates/nestgate-core/src/rpc/isomorphic_ipc/discovery.rs`

**Implementation**:
```rust
//! IPC Endpoint Discovery
//!
//! **UNIVERSAL**: Discovers Unix socket OR TCP endpoints automatically

use anyhow::{Context, Result};
use std::net::SocketAddr;
use std::path::PathBuf;

/// IPC endpoint type (polymorphic)
#[derive(Debug, Clone)]
pub enum IpcEndpoint {
    /// Unix socket path
    UnixSocket(PathBuf),
    /// TCP local address (localhost only)
    TcpLocal(SocketAddr),
}

/// Discover IPC endpoint for a service
///
/// **TRIES**:
/// 1. Unix socket (optimal)
/// 2. TCP discovery file (fallback)
pub fn discover_ipc_endpoint(service_name: &str) -> Result<IpcEndpoint> {
    // 1. Try Unix socket first
    if let Ok(socket_path) = discover_unix_socket(service_name) {
        if socket_path.exists() {
            return Ok(IpcEndpoint::UnixSocket(socket_path));
        }
    }
    
    // 2. Try TCP discovery file
    if let Ok(endpoint) = discover_tcp_endpoint(service_name) {
        return Ok(endpoint);
    }
    
    Err(anyhow::anyhow!(
        "Could not discover IPC endpoint for service: {}",
        service_name
    ))
}

/// Discover Unix socket path (XDG-compliant)
fn discover_unix_socket(service_name: &str) -> Result<PathBuf> {
    // Try XDG_RUNTIME_DIR first
    if let Ok(runtime_dir) = std::env::var("XDG_RUNTIME_DIR") {
        let socket_path = PathBuf::from(format!("{}/{}.sock", runtime_dir, service_name));
        return Ok(socket_path);
    }
    
    // Fallback to /tmp
    Ok(PathBuf::from(format!("/tmp/{}.sock", service_name)))
}

/// Discover TCP endpoint from discovery file
fn discover_tcp_endpoint(service_name: &str) -> Result<IpcEndpoint> {
    let discovery_files = get_tcp_discovery_file_candidates(service_name);
    
    for file in discovery_files {
        if let Ok(contents) = std::fs::read_to_string(&file) {
            // Parse format: tcp:127.0.0.1:PORT
            if let Some(addr_str) = contents.trim().strip_prefix("tcp:") {
                if let Ok(addr) = addr_str.parse::<SocketAddr>() {
                    return Ok(IpcEndpoint::TcpLocal(addr));
                }
            }
        }
    }
    
    Err(anyhow::anyhow!("No TCP discovery file found"))
}

/// Get candidate paths for TCP discovery file
fn get_tcp_discovery_file_candidates(service_name: &str) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    
    // XDG_RUNTIME_DIR
    if let Ok(runtime_dir) = std::env::var("XDG_RUNTIME_DIR") {
        candidates.push(PathBuf::from(format!("{}/{}-ipc-port", runtime_dir, service_name)));
    }
    
    // HOME/.local/share
    if let Ok(home) = std::env::var("HOME") {
        candidates.push(PathBuf::from(format!("{}/.local/share/{}-ipc-port", home, service_name)));
    }
    
    // /tmp
    candidates.push(PathBuf::from(format!("/tmp/{}-ipc-port", service_name)));
    
    candidates
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_discover_unix_socket() {
        let path = discover_unix_socket("nestgate").unwrap();
        assert!(path.to_string_lossy().contains("nestgate"));
    }
    
    #[test]
    fn test_discovery_file_candidates() {
        let candidates = get_tcp_discovery_file_candidates("nestgate");
        assert!(!candidates.is_empty());
    }
}
```

---

#### **Task 2.2: Polymorphic Streams** (1 hour)

**File**: `code/crates/nestgate-core/src/rpc/isomorphic_ipc/streams.rs`

**Implementation**:
```rust
//! Polymorphic IPC Streams
//!
//! **UNIVERSAL**: Unified interface for Unix socket and TCP streams

use anyhow::Result;
use async_trait::async_trait;
use std::pin::Pin;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::{TcpStream, UnixStream};

use super::discovery::IpcEndpoint;

/// Polymorphic async stream trait
#[async_trait]
pub trait AsyncStream: AsyncRead + AsyncWrite + Send + Unpin {}

/// Implement for Unix sockets
impl AsyncStream for UnixStream {}

/// Implement for TCP sockets
impl AsyncStream for TcpStream {}

/// Connect to IPC endpoint (polymorphic)
pub async fn connect_endpoint(endpoint: &IpcEndpoint) -> Result<Box<dyn AsyncStream>> {
    match endpoint {
        IpcEndpoint::UnixSocket(path) => {
            let stream = UnixStream::connect(path).await?;
            Ok(Box::new(stream))
        }
        IpcEndpoint::TcpLocal(addr) => {
            let stream = TcpStream::connect(addr).await?;
            Ok(Box::new(stream))
        }
    }
}
```

---

### **Phase 3: Integration & Testing** (1-2 hours)

#### **Task 3.1: Update Module Exports** (15 min)

**File**: `code/crates/nestgate-core/src/rpc/mod.rs`

**Update**:
```rust
// Add isomorphic IPC exports
pub mod isomorphic_ipc;
pub use isomorphic_ipc::{
    IsomorphicIpcServer,
    IpcEndpoint,
    discover_ipc_endpoint,
    connect_endpoint,
};
```

#### **Task 3.2: Build & Test** (1 hour)

**Commands**:
```bash
# Build for Linux
cargo build --release --target x86_64-unknown-linux-musl

# Build for Android (if needed)
cargo build --release --target aarch64-unknown-linux-musl

# Run tests
cargo test --package nestgate-core isomorphic_ipc

# Integration test
cargo test --test biomeos_integration_tests
```

#### **Task 3.3: Validation on Linux** (15 min)

**Expected logs**:
```
[INFO] 🔌 Starting IPC server (isomorphic mode)...
[INFO]    Service: nestgate
[INFO]    Trying Unix socket IPC (optimal)...
[INFO] ✅ Unix socket IPC active (optimal path)
```

#### **Task 3.4: Validation on Android** (15 min)

**Expected logs**:
```
[INFO] 🔌 Starting IPC server (isomorphic mode)...
[INFO]    Service: nestgate
[INFO]    Trying Unix socket IPC (optimal)...
[WARN] ⚠️  Unix sockets unavailable: Permission denied
[WARN]    Detected platform constraint, adapting...
[INFO] 🌐 Starting TCP IPC fallback (isomorphic mode)
[INFO]    Protocol: JSON-RPC 2.0 (same as Unix socket)
[INFO] ✅ TCP IPC listening on 127.0.0.1:45763
[INFO]    Status: READY ✅ (isomorphic TCP fallback active)
```

═══════════════════════════════════════════════════════════════════

## ✅ VALIDATION CHECKLIST

### **Server-Side**
- [ ] `IsomorphicIpcServer` created with Try→Detect→Adapt pattern
- [ ] `is_platform_constraint()` detects SELinux/platform issues
- [ ] `TcpFallbackServer` binds to `127.0.0.1:0` (ephemeral port)
- [ ] TCP server uses same JSON-RPC 2.0 protocol
- [ ] Discovery file written to XDG-compliant paths
- [ ] Logs show automatic adaptation

### **Client-Side**
- [ ] `IpcEndpoint` enum defined (UnixSocket | TcpLocal)
- [ ] `discover_ipc_endpoint()` tries Unix first, then TCP
- [ ] TCP discovery file parsed correctly
- [ ] `AsyncStream` trait for polymorphic streams
- [ ] `connect_endpoint()` handles both Unix and TCP

### **Integration**
- [ ] Builds for x86_64 and aarch64
- [ ] Works on Linux (Unix sockets)
- [ ] Works on Android (TCP fallback)
- [ ] Zero configuration required
- [ ] Inter-primal communication works

═══════════════════════════════════════════════════════════════════

## 🎯 SUCCESS CRITERIA

**Implementation complete when**:

1. ✅ Same binary runs on Linux AND Android
2. ✅ Automatically uses Unix sockets on Linux
3. ✅ Automatically falls back to TCP on Android
4. ✅ Zero configuration (no env vars or flags)
5. ✅ Logs prove automatic adaptation
6. ✅ Clients discover endpoints (Unix OR TCP)
7. ✅ JSON-RPC communication works
8. ✅ Deep Debt principles maintained

**Timeline**: 6-8 hours (as estimated in guide)

═══════════════════════════════════════════════════════════════════

## 📚 REFERENCES

### **From biomeOS Guide**
- songbird v3.33.0 reference implementation
- Try→Detect→Adapt→Succeed pattern
- Platform constraint detection logic
- Discovery file system

### **NestGate's Foundation**
- Universal trait patterns (Phase 2 & 3)
- Platform code reduction (56% done)
- Modern idiomatic Rust practices
- XDG-compliant configuration

### **Alignment**
This implementation continues NestGate's deep debt evolution:
- ✅ Phase 1-3: Platform code reduction (56%)
- ✅ Phase 3 Task 1: Filesystem detection (complete)
- 🔄 **Next**: Isomorphic IPC (this plan)
- 🎯 **Target**: ZERO platform-specific files

═══════════════════════════════════════════════════════════════════

**Status**: Ready for Implementation  
**Priority**: MEDIUM (Part of NEST atomic)  
**Effort**: 6-8 hours  
**Confidence**: HIGH (proven pattern from songbird)

**🦀 NestGate: From platform-specific → Universal → Isomorphic!** 🚀🌍🧬

**Created**: January 31, 2026  
**Philosophy**: Binary = DNA: Universal, Deterministic, Adaptive

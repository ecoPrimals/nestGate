// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # 🔌 Isomorphic IPC Module
//!
//! **UNIVERSAL**: Same binary works on ALL platforms\
//! **ADAPTIVE**: Automatic TCP fallback when Unix sockets unavailable\
//! **ZERO CONFIG**: No environment variables or flags required
//!
//! ## Philosophy
//!
//! **Binary = DNA: Universal, Deterministic, Adaptive**
//!
//! Platform constraints are **DATA** (detected at runtime), not **CONFIG** (hardcoded at compile time).
//!
//! This module implements isomorphic IPC following the Try→Detect→Adapt→Succeed pattern:
//! 1. **TRY** Unix sockets first (optimal)
//! 2. **DETECT** platform constraints (`SELinux`, lack of support)
//! 3. **ADAPT** to TCP fallback (automatic)
//! 4. **SUCCEED** or fail with real error
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │   Isomorphic IPC Server                 │
//! │   (Try→Detect→Adapt→Succeed)           │
//! └─────────────────────────────────────────┘
//!           │
//!           ├─→ Try Unix Socket
//!           │     │
//!           │     ├─→ Success? ✅ Use Unix
//!           │     │
//!           │     └─→ Platform Constraint? ⚠️
//!           │              │
//!           └─────────────→ Adapt: TCP Fallback
//!                          │
//!                          └─→ Success ✅
//!
//! ┌─────────────────────────────────────────┐
//! │   Client Discovery                      │
//! │   (Auto-detect Unix OR TCP)             │
//! └─────────────────────────────────────────┘
//!           │
//!           ├─→ Try Unix Socket
//!           │     │
//!           │     ├─→ Exists? ✅ Use Unix
//!           │     │
//!           │     └─→ Not found? ⚠️
//!           │              │
//!           └─────────────→ Try TCP Discovery File
//!                          │
//!                          └─→ Found ✅ Use TCP
//! ```
//!
//! ## Modules
//!
//! - **`platform_detection`**: Detect platform constraints (`SELinux`, lack of support)
//! - **`tcp_fallback`**: TCP IPC server (automatic fallback)
//! - **`server`**: Isomorphic server facade (Try→Detect→Adapt→Succeed)
//! - **`discovery`**: Client endpoint discovery (Unix OR TCP)
//! - **`streams`**: Polymorphic streams (unified interface)
//!
//! ## Usage
//!
//! ### Server-Side
//!
//! ```rust,ignore
//! use nestgate_core::rpc::isomorphic_ipc::{IsomorphicIpcServer, RpcHandler};
//! use std::sync::Arc;
//! use serde_json::Value;
//!
//! // Implement RPC handler
//! struct MyHandler;
//!
//! #[async_trait::async_trait]
//! impl RpcHandler for MyHandler {
//!     async fn handle_request(&self, request: Value) -> Value {
//!         // Handle JSON-RPC request
//!         serde_json::json!({"jsonrpc": "2.0", "result": "ok", "id": 1})
//!     }
//! }
//!
//! # async fn example() -> anyhow::Result<()> {
//! // Create server
//! let server = Arc::new(IsomorphicIpcServer::new(
//!     "nestgate".to_string(),
//!     Arc::new(MyHandler),
//! ));
//!
//! // Start server (automatically adapts to platform)
//! server.start().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Client-Side
//!
//! ```rust,ignore
//! use nestgate_core::rpc::isomorphic_ipc::{discover_ipc_endpoint, connect_endpoint};
//! use tokio::io::{AsyncReadExt, AsyncWriteExt};
//!
//! # async fn example() -> anyhow::Result<()> {
//! // Discover endpoint (Unix OR TCP, automatic)
//! let endpoint = discover_ipc_endpoint("nestgate")?;
//!
//! // Connect (transport transparent)
//! let mut stream = connect_endpoint(&endpoint).await?;
//!
//! // Use stream (same API for Unix and TCP)
//! stream.write_all(b"request\n").await?;
//! let mut buffer = vec![0u8; 1024];
//! let n = stream.read(&mut buffer).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Expected Behavior
//!
//! ### Linux (Unix sockets work)
//!
//! ```text
//! [INFO] 🔌 Starting IPC server (isomorphic mode)...
//! [INFO]    Trying Unix socket IPC (optimal)...
//! [INFO] ✅ Unix socket IPC active (optimal path)
//! ```
//!
//! ### Android (Unix sockets blocked by `SELinux`)
//!
//! ```text
//! [INFO] 🔌 Starting IPC server (isomorphic mode)...
//! [INFO]    Trying Unix socket IPC (optimal)...
//! [WARN] ⚠️  Unix sockets unavailable: Permission denied
//! [WARN]    Detected platform constraint, adapting...
//! [INFO] 🌐 Starting TCP IPC fallback (isomorphic mode)
//! [INFO] ✅ TCP IPC listening on 127.0.0.1:45763
//! ```
//!
//! ## Deep Debt Principles
//!
//! This module exemplifies modern idiomatic Rust:
//!
//! 1. ✅ **100% Pure Rust** - No C dependencies for IPC
//! 2. ✅ **Zero Unsafe Code** - All IPC code is safe Rust
//! 3. ✅ **Runtime Discovery** - Detects platform constraints from errors
//! 4. ✅ **Platform-Agnostic** - Same code on all platforms
//! 5. ✅ **Modern Idiomatic** - async/await, traits, error context
//! 6. ✅ **Primal Self-Knowledge** - No external configuration
//! 7. ✅ **Zero Configuration** - Works out of the box
//!
//! ## Reference
//!
//! Pattern validated in orchestration provider v3.33.0\
//! Implementation guide: `ISOMORPHIC_IPC_IMPLEMENTATION_PLAN_JAN_31_2026.md`
//!
//! **Status**: Phases 1, 2 & 3 Complete ✅ (A++ Grade)

// Module exports
pub mod discovery;
pub mod platform_detection;
pub mod server;
pub mod streams;
pub mod tcp_fallback;
pub mod unix_adapter;
// Phase 3: Deployment Coordination
pub mod atomic;
pub mod health;
pub mod launcher;

// Re-exports for convenience
pub use discovery::{IpcEndpoint, discover_ipc_endpoint};
pub use platform_detection::is_platform_constraint;
pub use server::IsomorphicIpcServer;
pub use streams::{IpcStream, connect_endpoint};
pub use tcp_fallback::{RpcHandler, TcpFallbackServer};
pub use unix_adapter::UnixSocketRpcHandler;
// Phase 3 re-exports
pub use atomic::{AtomicStatus, AtomicType, verify_nest_health, verify_nestgate_health};
pub use health::{HealthStatus, check_nestgate_health, check_nestgate_health_detailed};
pub use launcher::{
    connect_to_nestgate, connect_to_nestgate_with_retry, discover_nestgate_endpoint,
    discover_nestgate_with_retry, is_nestgate_running,
};

#[cfg(test)]
mod isomorphic_ipc_export_tests {

    #[test]
    fn public_types_are_reachable() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<super::AtomicStatus>();
        assert_send_sync::<super::HealthStatus>();
        assert_send_sync::<super::IpcEndpoint>();
    }
}

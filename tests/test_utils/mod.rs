//! # Test Utilities for Concurrent Testing
//!
//! Modern concurrent testing primitives for NestGate.
//!
//! ## Philosophy
//!
//! **"Test issues = Production issues"**
//!
//! This module provides utilities that enable truly concurrent testing by:
//! - Event-driven coordination (no sleep-based timing)
//! - Environment isolation (no global state pollution)
//! - Dynamic resource allocation (no hardcoded ports/paths)
//!
//! ## Usage
//!
//! ```rust
//! use tests::test_utils::coordination::*;
//!
//! #[tokio::test]
//! async fn test_concurrent_operation() {
//!     let signal = ReadySignal::new();
//!     
//!     let handle = tokio::spawn({
//!         let signal = signal.clone();
//!         async move {
//!             // Do work
//!             signal.notify_ready().await;
//!         }
//!     });
//!     
//!     signal.wait_ready().await; // Event-driven, no sleep!
//!     // Test continues...
//! }
//! ```

pub mod coordination;
pub mod ports;
pub mod environment;

/// Re-export commonly used items
pub use coordination::{ReadySignal, CompletionBarrier, StateWatcher};
pub use ports::DynamicPort;
pub use environment::IsolatedEnv;


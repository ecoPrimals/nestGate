//! NestGate MCP Integration
//! 
//! MCP protocol integration adapter for NestGate

pub mod provider;
pub mod adapter;
pub mod storage;

// Re-export main types
pub use provider::*;
pub use adapter::*;
pub use storage::*; 
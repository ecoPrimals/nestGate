// Filesystem Storage Backend - Modular Implementation
//! Module definitions and exports.
// This module provides a complete filesystem-based storage implementation with
//! features like atomic operations, metadata tracking, and directory watching.
//! Module definitions and exports.
// The implementation is broken down into focused modules for maintainability:
//! - `types`: Core types and configuration structures
//! - `backend`: Main backend implementation  
//! - `operations`: File and directory operations
//! - `metadata`: Metadata handling and utilities
//! - `tests`: Comprehensive test suite

pub mod backend;
pub mod metadata;
pub mod operations;
pub mod types;

#[cfg(test)]
mod tests;

// Re-export public types for convenience
pub use backend::FilesystemBackend;
pub use types::{EntryType, FileMetadata, FilesystemConfig, ResponseMetadata};

// Re-export common type aliases
pub use types::{DataChunk, StorageProtocolInfo};

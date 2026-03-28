// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Remote ZFS Backend
//!
//! Connects to remote ZFS services over HTTP/HTTPS.
//! This backend can be used to manage ZFS on remote systems.

/// HTTP/HTTPS client for remote ZFS communication
pub mod client;
/// Connection management and pooling
pub mod connection;
/// Core implementation of remote ZFS operations
pub mod implementation;
/// High-level service interface for remote ZFS
pub mod service;

// #[cfg(test)]
#[cfg(test)]
mod tests;

// Re-export main types
pub use connection::{ConnectionError, ConnectionStats};
pub use service::RemoteZfsService;

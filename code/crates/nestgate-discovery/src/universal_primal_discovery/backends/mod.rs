// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Discovery backend implementations
//!
//! mDNS/Consul/K8s backends have been removed — peer discovery is delegated
//! to orchestration-provider IPC. The in-memory backend remains for testing and local dev.

/// In-memory backend for testing and local development
pub mod memory;

pub use memory::InMemoryDiscoveryBackend;

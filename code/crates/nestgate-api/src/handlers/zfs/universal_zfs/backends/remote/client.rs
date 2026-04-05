// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Remote ZFS HTTP client — **DEPRECATED**.
//!
//! HTTP transport was removed per Concentrated Gap Architecture.
//! Remote ZFS operations now flow through Unix sockets via the orchestration gateway.
//! This module is retained only so existing `use` paths compile; all methods
//! return `RemoteError` directing callers to the Unix transport.

use std::time::Duration;

use crate::handlers::zfs::universal_zfs::config::RemoteConfig;
use crate::handlers::zfs::universal_zfs_types::{UniversalZfsError, UniversalZfsResult};

/// Deprecated HTTP client stub — all operations return `RemoteError`.
#[derive(Debug, Clone)]
pub struct HttpClient {
    _endpoint: String,
    _timeout: Duration,
}

impl HttpClient {
    #[must_use]
    pub fn new(config: &RemoteConfig) -> Self {
        Self {
            _endpoint: config.endpoint.clone(),
            _timeout: config.timeout,
        }
    }

    fn removed() -> UniversalZfsResult<serde_json::Value> {
        Err(UniversalZfsError::RemoteError(
            "HTTP removed — use Unix sockets via the orchestration gateway".to_string(),
        ))
    }

    pub async fn health_check(&self) -> UniversalZfsResult<()> {
        Err(UniversalZfsError::RemoteError(
            "HTTP removed — use Unix sockets via the orchestration gateway".to_string(),
        ))
    }

    pub async fn get(&self, _path: &str) -> UniversalZfsResult<serde_json::Value> {
        Self::removed()
    }

    pub async fn post(
        &self,
        _path: &str,
        _body: serde_json::Value,
    ) -> UniversalZfsResult<serde_json::Value> {
        Self::removed()
    }

    pub async fn put(
        &self,
        _path: &str,
        _body: serde_json::Value,
    ) -> UniversalZfsResult<serde_json::Value> {
        Self::removed()
    }

    pub async fn delete(&self, _path: &str) -> UniversalZfsResult<serde_json::Value> {
        Self::removed()
    }
}

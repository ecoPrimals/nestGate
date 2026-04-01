// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Shared router state: ZFS manager, communication counters, and event log.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

// Production: Use real ZFS manager and config
#[cfg(not(feature = "dev-stubs"))]
use nestgate_zfs::ProductionZfsManager;

// Development: Use stub manager and config
#[cfg(feature = "dev-stubs")]
use crate::dev_stubs::zfs::{ProductionZfsManager, ZfsConfig};

/// Production ZFS manager type alias
///
/// Defines the production ZFS manager implementation used throughout
/// the application for consistent ZFS operations and management.
pub type ZfsManager = ProductionZfsManager;

/// Atomic counters backing `GET /api/v1/communication/stats`.
///
/// Updated by live handlers when those code paths run (e.g. WebSocket lifecycle, SSE JSON
/// snapshot routes). Unobserved layers remain at zero rather than invented values.
#[derive(Debug)]
pub struct CommunicationCounters {
    /// Current WebSocket connections served by this process (`streaming-rpc`).
    pub websocket_active: AtomicU64,
    /// WebSocket messages handled after upgrade (`streaming-rpc`).
    pub websocket_messages_total: AtomicU64,
    /// Reserved for long-lived SSE subscribers when that transport is instrumented.
    pub sse_active: AtomicU64,
    /// Snapshots returned by the SSE JSON endpoints under `/api/v1/sse/*` (`streaming-rpc`).
    pub sse_events_sent: AtomicU64,
    /// Reserved for MCP streaming when wired.
    pub mcp_active_streams: AtomicU64,
    /// Reserved for MCP message totals when wired.
    pub mcp_messages_total: AtomicU64,
}

impl CommunicationCounters {
    #[must_use]
    /// Build a fresh counter set (all zeros).
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            websocket_active: AtomicU64::new(0),
            websocket_messages_total: AtomicU64::new(0),
            sse_active: AtomicU64::new(0),
            sse_events_sent: AtomicU64::new(0),
            mcp_active_streams: AtomicU64::new(0),
            mcp_messages_total: AtomicU64::new(0),
        })
    }

    /// Serialize current counter values for the REST response.
    #[must_use]
    pub fn to_json_snapshot(&self) -> serde_json::Value {
        let websocket_active = self.websocket_active.load(Ordering::Relaxed);
        let websocket_messages = self.websocket_messages_total.load(Ordering::Relaxed);
        let sse_active = self.sse_active.load(Ordering::Relaxed);
        let sse_events = self.sse_events_sent.load(Ordering::Relaxed);
        let mcp_streams = self.mcp_active_streams.load(Ordering::Relaxed);
        let mcp_messages = self.mcp_messages_total.load(Ordering::Relaxed);
        let total_active = websocket_active
            .saturating_add(sse_active)
            .saturating_add(mcp_streams);
        let total_messages = websocket_messages
            .saturating_add(sse_events)
            .saturating_add(mcp_messages);
        serde_json::json!({
            "websocket": {
                "active_connections": websocket_active,
                "total_messages": websocket_messages,
            },
            "sse": {
                "active_connections": sse_active,
                "events_sent": sse_events,
            },
            "mcp_streaming": {
                "active_streams": mcp_streams,
                "total_messages": mcp_messages,
            },
            "total_active_connections": total_active,
            "total_messages_processed": total_messages,
        })
    }
}

/// Application state shared across all route handlers
///
/// Contains shared resources and services that route handlers need
/// to access, including ZFS management and configuration.
#[derive(Clone)]
pub struct AppState {
    /// ZFS manager instance for storage operations
    pub zfs_manager: Arc<ZfsManager>,
    /// Live communication counters for `/api/v1/communication/stats`.
    pub communication_counters: Arc<CommunicationCounters>,
    /// Operational events for `GET /api/v1/events`; empty until producers record entries.
    pub event_log: Arc<tokio::sync::RwLock<Vec<serde_json::Value>>>,
    /// Phantom data for future extensibility
    #[cfg(feature = "streaming-rpc")]
    pub _phantom: std::marker::PhantomData<()>,
}

impl Default for AppState {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    /// Create `AppState` with ZFS support
    #[cfg(feature = "streaming-rpc")]
    #[must_use]
    pub fn with_zfs_and_streaming() -> Self {
        Self {
            #[cfg(feature = "dev-stubs")]
            zfs_manager: Arc::new(ZfsManager::new(ZfsConfig::default())),
            #[cfg(not(feature = "dev-stubs"))]
            zfs_manager: Arc::new(ZfsManager::new()),
            communication_counters: CommunicationCounters::new(),
            event_log: Arc::new(tokio::sync::RwLock::new(Vec::new())),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Create `AppState` without streaming features
    #[must_use]
    pub fn without_streaming() -> Self {
        Self {
            #[cfg(feature = "dev-stubs")]
            zfs_manager: Arc::new(ZfsManager::new(ZfsConfig::default())),
            #[cfg(not(feature = "dev-stubs"))]
            zfs_manager: Arc::new(ZfsManager::new()),
            communication_counters: CommunicationCounters::new(),
            event_log: Arc::new(tokio::sync::RwLock::new(Vec::new())),
            #[cfg(feature = "streaming-rpc")]
            _phantom: std::marker::PhantomData,
        }
    }

    /// Create `AppState` with optional streaming components based on feature flags
    #[must_use]
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "dev-stubs")]
            zfs_manager: Arc::new(ZfsManager::new(ZfsConfig::default())),
            #[cfg(not(feature = "dev-stubs"))]
            zfs_manager: Arc::new(ZfsManager::new()),
            communication_counters: CommunicationCounters::new(),
            event_log: Arc::new(tokio::sync::RwLock::new(Vec::new())),
            #[cfg(feature = "streaming-rpc")]
            _phantom: std::marker::PhantomData,
        }
    }

    /// Get ZFS manager reference
    #[must_use]
    pub fn get_zfs_manager(&self) -> Option<Arc<ZfsManager>> {
        Some(self.zfs_manager.clone())
    }

    /// Initialize storage systems - ZFS manager and Universal Storage Bridge
    #[must_use]
    pub const fn with_zfs_manager(self) -> Self {
        // ZFS manager already initialized in constructor
        self
    }
}

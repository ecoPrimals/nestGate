// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Storage protocol request and response types
//!
//! Defines the protocol for communicating with storage backends,
//! including all request and response types.

#![expect(deprecated)]

use super::config::StorageResourceConfig;
use super::{
    events::StorageEventType,
    items::{StorageItem, StorageMetadata},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Universal storage request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UniversalStorageRequest {
    /// Read data from storage
    Read {
        /// Optional byte range for partial reads
        range: Option<std::ops::Range<u64>>,
    },
    /// Write data to storage
    Write {
        /// Data to write
        data: Vec<u8>,
        /// Whether to overwrite existing data
        overwrite: bool,
    },
    /// Delete from storage
    Delete {
        /// Whether to delete recursively
        recursive: bool,
    },
    /// List resources in storage
    List {
        /// Whether to list recursively
        recursive: bool,
        /// Optional filter pattern
        filter: Option<String>,
    },
    /// Create a new storage resource
    CreateResource {
        /// Resource configuration
        config: Box<StorageResourceConfig>,
    },
    /// Get resource metadata
    GetMetadata {},
    /// Set resource metadata
    SetMetadata {
        /// Metadata key-value pairs
        metadata: HashMap<String, serde_json::Value>,
    },
    /// Create a snapshot
    Snapshot {
        /// Snapshot name
        name: String,
    },
    /// Restore from snapshot
    Restore {},
    /// Stream data
    Stream {
        /// Optional byte range for streaming
        range: Option<std::ops::Range<u64>>,
    },
    /// Monitor storage events
    Monitor {
        /// Event types to monitor
        events: Vec<StorageEventType>,
    },
}

/// Universal storage response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UniversalStorageResponse {
    /// Read operation response
    ReadResponse {
        /// Data read from storage
        data: Vec<u8>,
        /// Optional metadata about the read operation
        metadata: Option<StorageMetadata>,
    },
    /// Write operation response
    WriteResponse {
        /// Number of bytes written
        bytes_written: u64,
        /// Optional checksum of written data
        checksum: Option<String>,
    },
    /// Delete operation response
    DeleteResponse {
        /// Number of items deleted
        deleted_items: u64,
    },
    /// List response with storage items
    ListResponse {
        /// List of storage items
        items: Vec<StorageItem>,
    },
    /// Create response
    CreateResponse {},
    /// Metadata response
    MetadataResponse {
        /// Storage metadata
        metadata: StorageMetadata,
    },
    /// Snapshot response
    SnapshotResponse {
        /// Unique snapshot identifier
        snapshot_id: String,
        /// Timestamp when snapshot was created
        created_at: DateTime<Utc>,
    },
    /// Restore response
    RestoreResponse {
        /// Number of bytes restored
        restored_bytes: u64,
        /// Number of items restored
        restored_items: u64,
    },
    /// Stream response
    StreamResponse {
        /// Unique stream identifier
        stream_id: String,
        /// Size of data chunks in stream
        chunk_size: usize,
    },
    /// Monitor response with event data
    MonitorResponse {
        /// Unique monitor session identifier
        monitor_id: String,
        /// Storage events that occurred
        events: Vec<super::events::StorageEvent>,
    },
    /// Error response
    Error {
        /// Error message
        error: String,
        /// Error code for categorization
        error_code: String,
    },
}

/// Legacy storage request (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageRequest {
    /// Operation type
    pub operation: String,
    /// Target path or resource
    pub path: String,
    /// Optional data payload
    pub data: Option<Vec<u8>>,
    /// Optional metadata
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Legacy storage response (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResponse {
    /// Operation status
    pub success: bool,
    /// Optional result data
    pub data: Option<Vec<u8>>,
    /// Optional metadata
    pub metadata: Option<StorageMetadata>,
    /// Optional error message
    pub error: Option<String>,
}

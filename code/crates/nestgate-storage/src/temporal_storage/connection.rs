// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **Connectivity & Data Source Connections**
//!
//! Domain: Connection management for universal data sources
//!
//! This module handles:
//! - Universal data source trait (native async)
//! - Connection handles and lifecycle
//! - Connection status tracking
//! - Capability discovery for data sources

use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};
use std::future::Future;

use super::data_types::DataDescriptor;
use super::ingestion::IngestedData;
use super::sources::{DataSourceType, DataStream};

/// Metadata type (re-exported for convenience)
pub type Metadata = std::collections::HashMap<String, String>;

/// Universal data source trait
///
/// **CANONICAL MODERNIZATION**: Native async trait without `async_trait` overhead
///
/// Represents any data source that can be connected to and queried.
/// Supports universal ingestion across all storage types and eras.
pub trait UniversalDataSource: Send + Sync {
    /// Connect to the data source
    ///
    /// Establishes connection and returns handle for subsequent operations.
    ///
    /// # Returns
    ///
    /// Connection handle if successful
    ///
    /// # Errors
    ///
    /// Returns error if connection fails
    fn connect(&self) -> impl Future<Output = Result<ConnectionHandle>> + Send;

    /// Discover available data
    ///
    /// Scans the data source for available datasets.
    ///
    /// # Returns
    ///
    /// Vector of data descriptors for discovered data
    ///
    /// # Errors
    ///
    /// Returns error if discovery fails
    fn discover_data(&self) -> impl Future<Output = Result<Vec<DataDescriptor>>> + Send;

    /// Ingest data from source
    ///
    /// Reads and ingests data described by the descriptor.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - Description of data to ingest
    ///
    /// # Returns
    ///
    /// Ingested data with metadata
    ///
    /// # Errors
    ///
    /// Returns error if ingestion fails
    fn ingest_data(
        &self,
        descriptor: &DataDescriptor,
    ) -> impl Future<Output = Result<IngestedData>> + Send;

    /// Get metadata for data
    ///
    /// Retrieves metadata without ingesting the full dataset.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - Data to get metadata for
    ///
    /// # Returns
    ///
    /// Metadata key-value pairs
    ///
    /// # Errors
    ///
    /// Returns error if metadata retrieval fails
    fn get_metadata(
        &self,
        descriptor: &DataDescriptor,
    ) -> impl Future<Output = Result<Metadata>> + Send;

    /// Stream data from source
    ///
    /// Opens a streaming connection for large datasets.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - Data to stream
    ///
    /// # Returns
    ///
    /// Data stream for reading
    ///
    /// # Errors
    ///
    /// Returns error if streaming fails
    fn stream_data(
        &self,
        descriptor: &DataDescriptor,
    ) -> impl Future<Output = Result<Box<dyn DataStream>>> + Send;
}

/// Connection handle for data sources
///
/// Represents an established connection to a data source.
/// Used to perform operations and track connection state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionHandle {
    /// Unique connection identifier
    pub connection_id: String,
    /// Type of data source connected to
    pub source_type: DataSourceType,
    /// Current connection status
    pub status: ConnectionStatus,
    /// Capabilities supported by this connection
    pub capabilities: Vec<String>,
}

/// Connection status
///
/// Tracks the current state of a data source connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionStatus {
    /// Successfully connected and ready for operations
    Connected,
    /// Not connected or connection closed
    Disconnected,
    /// Error state with description
    Error(String),
    /// Connection establishment in progress
    Connecting,
}

impl ConnectionHandle {
    /// Create a new connection handle
    ///
    /// # Arguments
    ///
    /// * `connection_id` - Unique identifier for this connection
    /// * `source_type` - Type of data source
    ///
    /// # Returns
    ///
    /// New connection handle in Connecting state
    #[must_use]
    pub const fn new(connection_id: String, source_type: DataSourceType) -> Self {
        Self {
            connection_id,
            source_type,
            status: ConnectionStatus::Connecting,
            capabilities: Vec::new(),
        }
    }

    /// Mark connection as successfully established
    ///
    /// # Arguments
    ///
    /// * `capabilities` - Capabilities discovered from the source
    pub fn mark_connected(&mut self, capabilities: Vec<String>) {
        self.status = ConnectionStatus::Connected;
        self.capabilities = capabilities;
    }

    /// Mark connection as failed with error
    ///
    /// # Arguments
    ///
    /// * `error` - Error description
    pub fn mark_error(&mut self, error: String) {
        self.status = ConnectionStatus::Error(error);
    }

    /// Check if connection is active
    ///
    /// # Returns
    ///
    /// `true` if connected, `false` otherwise
    #[must_use]
    pub const fn is_connected(&self) -> bool {
        matches!(self.status, ConnectionStatus::Connected)
    }

    /// Check if connection has specific capability
    ///
    /// # Arguments
    ///
    /// * `capability` - Capability to check for
    ///
    /// # Returns
    ///
    /// `true` if capability is supported
    #[must_use]
    pub fn has_capability(&self, capability: &str) -> bool {
        self.capabilities.iter().any(|c| c == capability)
    }
}

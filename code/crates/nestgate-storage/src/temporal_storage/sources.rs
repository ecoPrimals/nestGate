// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **Data Sources & Streaming**
//!
//! Domain: Data source types, streaming interfaces, and source classification
//!
//! This module handles:
//! - Data source type classification
//! - Streaming data interface
//! - API type specifications
//! - Capability-based source types
//! - Legacy and future storage technologies

use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

// Re-export CloudProvider from canonical location
pub use crate::universal_storage::consolidated_types::CloudProvider;

/// Data stream trait
///
/// Interface for streaming data from sources.
///
/// **NOTE**: Uses `Pin<Box<dyn Future>>` for object safety (dyn compatibility).
/// Cannot use `impl Future` as this trait needs to be dyn-compatible for trait objects.
pub trait DataStream: Send + Sync {
    /// Read a chunk of data
    ///
    /// # Arguments
    ///
    /// * `size` - Maximum size to read in bytes
    ///
    /// # Returns
    ///
    /// Vector of bytes read (may be less than requested size)
    ///
    /// # Errors
    ///
    /// Returns error if read fails
    fn read_chunk(
        &mut self,
        size: usize,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<u8>>> + Send + '_>>;

    /// Seek to position in stream
    ///
    /// # Arguments
    ///
    /// * `position` - Byte offset to seek to
    ///
    /// # Errors
    ///
    /// Returns error if seek fails or position is invalid
    fn seek(&mut self, position: u64) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>>;
}

/// Data source types (capability-based, not provider-specific)
///
/// Classification of data sources by capability rather than specific provider.
/// This enables discovery-based access without hardcoding providers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSourceType {
    /// Local device storage
    LocalDevice {},
    /// Remote API source
    RemoteAPI {
        /// Type of API
        api_type: APIType,
        /// API endpoint URL
        endpoint: String,
    },
    /// Capability-based data source
    DataCapability {
        /// Capability type identifier
        capability_type: String,
        /// Provider-specific metadata
        provider_metadata: HashMap<String, String>,
    },
    /// Cloud storage provider
    CloudStorage {
        /// Cloud provider
        provider: CloudProvider,
    },
    /// Legacy media source
    LegacyMedia {
        /// Media type
        media_type: LegacyMediaType,
    },
    /// Future storage technology
    FutureStorage {
        /// Technology type
        technology: FutureTechnology,
    },
}

/// API types
///
/// Classification of API protocols and interfaces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum APIType {
    /// REST API
    Rest,
    /// GraphQL API
    GraphQL,
    /// gRPC API
    GRpc,
    /// WebSocket API
    WebSocket,
    /// Custom API type
    Custom(String),
}

/// Universal data capability types
///
/// **Capability-based design**: What we can do, not who provides it.
/// Enables runtime discovery without hardcoding providers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataCapabilityType {
    /// Genome data capability
    GenomeData {
        /// Optional organism filter
        organism_filter: Option<String>,
    },
    /// Model data capability (AI/ML models)
    ModelData {
        /// Optional model type filter
        model_type_filter: Option<String>,
    },
    /// Research data capability
    ResearchData {
        /// Optional domain filter
        domain_filter: Option<String>,
    },
    /// Time series data capability
    TimeSeriesData {
        /// Optional frequency specification
        frequency: Option<String>,
    },
    /// Image data capability
    ImageData {
        /// Optional format filter
        format_filter: Option<String>,
    },
    /// Custom capability
    Custom {
        /// Capability name
        capability_name: String,
    },
}

/// Legacy media types
///
/// Historical storage media types for compatibility.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LegacyMediaType {
    /// Punch cards (1890s-1960s)
    PunchCard,
    /// Magnetic tape (1950s-present)
    MagneticTape,
    /// Floppy disk (1970s-1990s)
    FloppyDisk,
    /// Optical disc (CD, DVD, Blu-ray)
    OpticalDisc,
    /// Zip disk (1990s)
    ZipDisk,
}

/// Future storage technologies
///
/// Experimental and future storage technologies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FutureTechnology {
    /// DNA-based storage
    DnaStorage,
    /// Quantum storage
    QuantumStorage,
    /// Holographic storage
    HolographicStorage,
    /// Molecular storage
    MolecularStorage,
}

impl DataSourceType {
    /// Check if source is local
    ///
    /// # Returns
    ///
    /// `true` if source is local device
    #[must_use]
    pub const fn is_local(&self) -> bool {
        matches!(self, Self::LocalDevice {})
    }

    /// Check if source is remote
    ///
    /// # Returns
    ///
    /// `true` if source is remote (API or cloud)
    #[must_use]
    pub const fn is_remote(&self) -> bool {
        matches!(self, Self::RemoteAPI { .. } | Self::CloudStorage { .. })
    }

    /// Check if source is capability-based
    ///
    /// # Returns
    ///
    /// `true` if source uses capability-based discovery
    #[must_use]
    pub const fn is_capability_based(&self) -> bool {
        matches!(self, Self::DataCapability { .. })
    }

    /// Get endpoint URL if available
    ///
    /// # Returns
    ///
    /// Optional endpoint URL for remote APIs
    #[must_use]
    pub fn endpoint_url(&self) -> Option<&str> {
        match self {
            Self::RemoteAPI { endpoint, .. } => Some(endpoint),
            _ => None,
        }
    }
}

impl APIType {
    /// Check if API type is REST
    ///
    /// # Returns
    ///
    /// `true` if REST API
    #[must_use]
    pub const fn is_rest(&self) -> bool {
        matches!(self, Self::Rest)
    }

    /// Get protocol string
    ///
    /// # Returns
    ///
    /// Protocol name as string
    #[must_use]
    pub fn protocol_name(&self) -> &str {
        match self {
            Self::Rest => "REST",
            Self::GraphQL => "GraphQL",
            Self::GRpc => "gRPC",
            Self::WebSocket => "WebSocket",
            Self::Custom(name) => name,
        }
    }
}

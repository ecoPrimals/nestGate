// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **Canonical Network Trait**
//!
//! Comprehensive network trait for messaging, connections, and streaming.
//!
//! **Extracted**: November 19, 2025 - From canonical_unified_traits.rs
//! **Lines**: ~205 (from original 1,100-line file)

use super::service::CanonicalService;
use super::types::{ConnectionHandle, ConnectionStatus};
use std::future::Future;

// ==================== CANONICAL NETWORK TRAIT ====================

/// **THE** canonical network trait that replaces ALL network service traits
///
/// This trait consolidates and replaces:
/// - Former `NetworkProvider` trait (removed)
/// - Various network service traits
/// - Other network-related provider traits
///
/// **ENHANCED**: November 9, 2025 - Comprehensive network interface (9 methods)
/// **PERFORMANCE**: Native async throughout - zero `async_trait` overhead
/// **COMPLETENESS**: Covers all network operations (messaging, connections, streaming)
/// **UNIFICATION**: Single source of truth for all network implementations
///
/// # Consolidated Methods (9 total)
/// - **Messaging (3)**: send, receive, handle_request
/// - **Connection management (4)**: connect, disconnect, connection_status, list_connections
/// - **Stream support (2)**: open_stream, close_stream
///
/// # Type Safety
/// Generic over `Request` and `Response` types:
/// - Flexible type system supports any network protocol
/// - Type-safe request/response handling
/// - Zero-cost abstractions with compile-time optimization
///
/// # Performance Characteristics
/// - **Native async (RPITIT)**: Zero `async_trait` overhead
/// - **Connection pooling**: Efficient connection reuse
/// - **Stream support**: Large data transfer without memory bloat
/// - **Protocol abstraction**: Zero-cost dispatch for different protocols
///
/// # Example Implementation
/// ```ignore
/// impl CanonicalNetwork for MyNetworkBackend {
///     type Request = Vec<u8>;
///     type Response = Vec<u8>;
///     
///     async fn send(&self, destination: &str, data: &[u8]) -> Result<(), Self::Error> {
///         // Send data over network
///         Ok(())
///     }
///     
///     async fn connect(&self, endpoint: &str) -> Result<ConnectionHandle, Self::Error> {
///         // Establish connection
///         Ok(ConnectionHandle::new())
///     }
///     
///     // ... implement remaining methods
/// }
/// ```
pub trait CanonicalNetwork: CanonicalService {
    /// Request type
    type Request: Clone + Send + Sync + 'static;

    /// Response type
    type Response: Clone + Send + Sync + 'static;

    // ==================== MESSAGING OPERATIONS ====================

    /// Send data to destination - native async
    ///
    /// **Consolidated from**: NetworkProvider::send
    ///
    /// # Arguments
    /// * `destination` - Target endpoint/address
    /// * `data` - Data to send
    ///
    /// # Returns
    /// * `Ok(())` - Data sent successfully
    /// * `Err(Self::Error)` - Send failure
    fn send(
        &self,
        destination: &str,
        data: &[u8],
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Receive data with timeout - native async
    ///
    /// **Consolidated from**: NetworkProvider::receive
    ///
    /// # Arguments
    /// * `timeout_ms` - Timeout in milliseconds (0 = no timeout)
    ///
    /// # Returns
    /// * `Ok(Some(data))` - Data received
    /// * `Ok(None)` - Timeout or no data available
    /// * `Err(Self::Error)` - Receive failure
    fn receive(
        &self,
        timeout_ms: u64,
    ) -> impl Future<Output = std::result::Result<Option<Vec<u8>>, Self::Error>> + Send;

    /// Handle network request - native async
    ///
    /// High-level request/response pattern for typed network operations
    ///
    /// # Arguments
    /// * `request` - Network request to handle
    ///
    /// # Returns
    /// * `Ok(response)` - Request handled successfully
    /// * `Err(Self::Error)` - Request handling failure
    fn handle_request(
        &self,
        request: Self::Request,
    ) -> impl Future<Output = std::result::Result<Self::Response, Self::Error>> + Send;

    // ==================== CONNECTION MANAGEMENT ====================

    /// Establish connection to endpoint - native async
    ///
    /// **Consolidated from**: NetworkProvider::connect
    ///
    /// # Arguments
    /// * `endpoint` - Endpoint to connect to (URL, address, etc.)
    ///
    /// # Returns
    /// * `Ok(handle)` - Connection established
    /// * `Err(Self::Error)` - Connection failure
    fn connect(
        &self,
        endpoint: &str,
    ) -> impl Future<Output = std::result::Result<ConnectionHandle, Self::Error>> + Send;

    /// Close connection - native async
    ///
    /// **Consolidated from**: NetworkProvider::disconnect
    ///
    /// # Arguments
    /// * `handle` - Connection handle to close
    ///
    /// # Returns
    /// * `Ok(())` - Connection closed successfully
    /// * `Err(Self::Error)` - Disconnect failure
    fn disconnect(
        &self,
        handle: ConnectionHandle,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Get connection status - native async
    ///
    /// # Arguments
    /// * `handle` - Connection handle to query
    ///
    /// # Returns
    /// * `Ok(status)` - Connection status
    /// * `Err(Self::Error)` - Query failure
    fn connection_status(
        &self,
        handle: ConnectionHandle,
    ) -> impl Future<Output = std::result::Result<ConnectionStatus, Self::Error>> + Send;

    /// List all active connections - native async
    ///
    /// # Returns
    /// * `Ok(handles)` - List of active connection handles
    /// * `Err(Self::Error)` - Query failure
    fn list_connections(
        &self,
    ) -> impl Future<Output = std::result::Result<Vec<ConnectionHandle>, Self::Error>> + Send;

    // ==================== OPTIONAL STREAM SUPPORT ====================

    /// Open bidirectional stream (if supported) - native async
    ///
    /// # Arguments
    /// * `endpoint` - Endpoint to stream to/from
    ///
    /// # Returns
    /// * `Ok(Some(handle))` - Stream opened
    /// * `Ok(None)` - Streams not supported
    /// * `Err(Self::Error)` - Stream open failure
    fn open_stream(
        &self,
        endpoint: &str,
    ) -> impl Future<Output = std::result::Result<Option<ConnectionHandle>, Self::Error>> + Send
    {
        async move {
            let _ = endpoint;
            Ok(None) // Default: not supported
        }
    }

    /// Close stream (if supported) - native async
    ///
    /// # Arguments
    /// * `handle` - Stream handle to close
    ///
    /// # Returns
    /// * `Ok(())` - Stream closed
    /// * `Err(Self::Error)` - Stream close failure
    fn close_stream(
        &self,
        handle: ConnectionHandle,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send {
        async move {
            let _ = handle;
            Ok(()) // Default: no-op
        }
    }
}

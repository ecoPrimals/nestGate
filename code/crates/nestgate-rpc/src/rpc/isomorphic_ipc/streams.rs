// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # Polymorphic IPC Streams
//!
//! **UNIVERSAL**: Unified interface for Unix socket and TCP streams\
//! **TRANSPARENT**: Same API regardless of transport\
//! **ZERO OVERHEAD**: Direct trait implementation (no boxing in hot path)
//!
//! ## Philosophy
//!
//! Transport is an implementation detail:
//! - Clients work with `AsyncStream` trait
//! - Implementation (Unix or TCP) is transparent
//! - No performance penalty for abstraction
//!
//! This enables **isomorphic networking** where the same code works
//! over different transports without changes.
//!
//! ## Usage Pattern
//!
//! ```rust,ignore
//! use nestgate_core::rpc::isomorphic_ipc::{discover_ipc_endpoint, connect_endpoint};
//!
//! # async fn example() -> anyhow::Result<()> {
//! // Discover endpoint (Unix or TCP)
//! let endpoint = discover_ipc_endpoint("nestgate")?;
//!
//! // Connect (transport transparent)
//! let mut stream = connect_endpoint(&endpoint).await?;
//!
//! // Use stream (same API for Unix and TCP)
//! // stream.write_all(...).await?;
//! // stream.read(...).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Reference
//!
//! Pattern validated in orchestration provider v3.33.0

use anyhow::Result;
use tokio::net::TcpStream;
#[cfg(unix)]
use tokio::net::UnixStream;
use tracing::debug;

use super::discovery::IpcEndpoint;

/// Backward-compatible alias — the canonical type is now
/// [`TransportStream`](super::transport_stream::TransportStream).
pub type IpcStream = super::transport_stream::TransportStream;

/// Connect to IPC endpoint (polymorphic)
///
/// **UNIVERSAL**: Works with Unix socket or TCP endpoints\
/// **TRANSPARENT**: Returns unified stream interface\
/// **AUTOMATIC**: Transport determined by endpoint type
///
/// # Arguments
///
/// * `endpoint` - IPC endpoint (Unix or TCP)
///
/// # Returns
///
/// * `Ok(IpcStream)` - Connected stream (transport transparent)
/// * `Err(_)` - Connection failed
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::rpc::isomorphic_ipc::{discover_ipc_endpoint, connect_endpoint};
///
/// # async fn example() -> anyhow::Result<()> {
/// let endpoint = discover_ipc_endpoint("nestgate")?;
/// let stream = connect_endpoint(&endpoint).await?;
/// println!("Connected via: {}", stream.stream_type());
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns [`anyhow::Error`] when connecting to the Unix socket path or TCP address fails (for
/// example connection refused, missing socket file, or I/O errors).
pub async fn connect_endpoint(endpoint: &IpcEndpoint) -> Result<IpcStream> {
    debug!("Connecting to: {}", endpoint.description());

    match endpoint {
        #[cfg(unix)]
        IpcEndpoint::UnixSocket(path) => {
            debug!("   Using Unix socket transport");
            let stream = UnixStream::connect(path)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to connect to Unix socket: {e}"))?;

            debug!("Connected via Unix socket");
            Ok(IpcStream::Unix(stream))
        }
        #[cfg(not(unix))]
        IpcEndpoint::UnixSocket(_) => Err(anyhow::anyhow!(
            "Unix sockets not available on this platform — use TCP"
        )),
        IpcEndpoint::TcpLocal(addr) => {
            debug!("   Using TCP transport (localhost)");
            let stream = TcpStream::connect(addr)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to connect to TCP endpoint: {e}"))?;

            debug!("Connected via TCP (localhost)");
            Ok(IpcStream::Tcp(stream))
        }
    }
}

/// Delegate to the canonical [`connect_transport`](super::transport_stream::connect_transport).
///
/// # Errors
///
/// Returns [`anyhow::Error`] when the connection fails or the transport variant
/// is not supported for direct connection.
pub async fn connect_transport(endpoint: &nestgate_types::TransportEndpoint) -> Result<IpcStream> {
    super::transport_stream::connect_transport(endpoint).await
}

/// Convert a [`nestgate_types::TransportEndpoint`] to the legacy [`IpcEndpoint`] for backward-compatible
/// code paths that still accept `IpcEndpoint`.
///
/// # Errors
///
/// Returns an error for `MeshRelay` endpoints which have no `IpcEndpoint` equivalent.
pub fn transport_to_ipc_endpoint(
    endpoint: &nestgate_types::TransportEndpoint,
) -> Result<IpcEndpoint> {
    use nestgate_types::TransportEndpoint;

    match endpoint {
        #[cfg(unix)]
        TransportEndpoint::Uds { path } => Ok(IpcEndpoint::UnixSocket(path.clone())),
        #[cfg(not(unix))]
        TransportEndpoint::Uds { path } => Err(anyhow::anyhow!(
            "UDS transport not available on this platform: {}",
            path.display()
        )),
        TransportEndpoint::Tcp { host, port } => {
            let addr: std::net::SocketAddr = format!("{host}:{port}")
                .parse()
                .map_err(|e| anyhow::anyhow!("Invalid TCP address {host}:{port}: {e}"))?;
            Ok(IpcEndpoint::TcpLocal(addr))
        }
        TransportEndpoint::MeshRelay { .. } => Err(anyhow::anyhow!(
            "MeshRelay transport has no IpcEndpoint equivalent"
        )),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use tokio::net::TcpListener;

    #[test]
    fn test_stream_type_description() {
        // Create mock streams (won't actually connect)
        // Just testing the type system and descriptions

        let unix_path = std::path::PathBuf::from("/tmp/test.sock");
        let tcp_addr: std::net::SocketAddr = "127.0.0.1:12345".parse().unwrap();

        let unix_endpoint = IpcEndpoint::UnixSocket(unix_path);
        let tcp_endpoint = IpcEndpoint::TcpLocal(tcp_addr);

        assert!(unix_endpoint.description().contains("Unix"));
        assert!(tcp_endpoint.description().contains("TCP"));
    }

    #[test]
    fn test_ipc_endpoint_is_unix_is_tcp() {
        let unix_ep = IpcEndpoint::UnixSocket(std::path::PathBuf::from("/tmp/x.sock"));
        let tcp_ep = IpcEndpoint::TcpLocal("127.0.0.1:99".parse().unwrap());
        assert!(unix_ep.is_unix_socket());
        assert!(!unix_ep.is_tcp());
        assert!(tcp_ep.is_tcp());
        assert!(!tcp_ep.is_unix_socket());
    }

    #[tokio::test]
    async fn test_stream_type_tcp() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let endpoint = IpcEndpoint::TcpLocal(addr);
        let stream = connect_endpoint(&endpoint).await.unwrap();
        assert_eq!(stream.transport_type(), "TCP");
    }

    #[tokio::test]
    async fn test_connect_tcp_stream_operations() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            drop(stream);
        });
        let endpoint = IpcEndpoint::TcpLocal(addr);
        let mut stream = connect_endpoint(&endpoint).await.unwrap();
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        // Stream may be closed by server - exercise write/read without strict assertions
        let _ = stream.write_all(b"test").await;
        let mut buf = [0u8; 4];
        let _ = stream.read(&mut buf).await;
    }

    #[tokio::test]
    async fn test_connect_unix_nonexistent_fails() {
        let endpoint = IpcEndpoint::UnixSocket(std::path::PathBuf::from("/nonexistent/path.sock"));
        let result = connect_endpoint(&endpoint).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn connect_transport_tcp_succeeds() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let ep = nestgate_types::TransportEndpoint::tcp("127.0.0.1", addr.port());
        let stream = connect_transport(&ep).await.unwrap();
        assert_eq!(stream.transport_type(), "TCP");
    }

    #[tokio::test]
    async fn connect_transport_uds_nonexistent_fails() {
        let ep = nestgate_types::TransportEndpoint::uds("/nonexistent/test.sock");
        let result = connect_transport(&ep).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn connect_transport_mesh_relay_not_supported() {
        let ep = nestgate_types::TransportEndpoint::mesh_relay("peer1", "security");
        let result = connect_transport(&ep).await;
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("MeshRelay"));
        assert!(err_msg.contains("relay negotiation"));
    }

    #[test]
    fn transport_to_ipc_uds() {
        let ep = nestgate_types::TransportEndpoint::uds("/tmp/test.sock");
        let ipc = transport_to_ipc_endpoint(&ep).unwrap();
        assert!(
            matches!(ipc, IpcEndpoint::UnixSocket(ref p) if p.to_str().unwrap() == "/tmp/test.sock")
        );
    }

    #[test]
    fn transport_to_ipc_tcp() {
        let ep = nestgate_types::TransportEndpoint::tcp("127.0.0.1", 9100);
        let ipc = transport_to_ipc_endpoint(&ep).unwrap();
        match ipc {
            IpcEndpoint::TcpLocal(addr) => {
                assert_eq!(addr.port(), 9100);
                assert_eq!(addr.ip().to_string(), "127.0.0.1");
            }
            _ => panic!("expected TcpLocal"),
        }
    }

    #[test]
    fn transport_to_ipc_mesh_relay_errors() {
        let ep = nestgate_types::TransportEndpoint::mesh_relay("peer", "cap");
        assert!(transport_to_ipc_endpoint(&ep).is_err());
    }
}

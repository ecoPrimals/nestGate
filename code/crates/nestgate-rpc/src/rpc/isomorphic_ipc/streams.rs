// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # 🔌 Polymorphic IPC Streams
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
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::net::{TcpStream, UnixStream};
use tracing::debug;

use super::discovery::IpcEndpoint;

/// Polymorphic async stream trait
///
/// **Unified interface** for Unix socket and TCP streams:
/// - Implements `AsyncRead` for reading
/// - Implements `AsyncWrite` for writing
/// - Send + Unpin for safe concurrent usage
///
/// Both `UnixStream` and `TcpStream` implement this trait,
/// allowing transparent usage regardless of transport.
pub trait AsyncStream: AsyncRead + AsyncWrite + Send + Unpin {}

/// Implement `AsyncStream` for Unix sockets
impl AsyncStream for UnixStream {}

/// Implement `AsyncStream` for TCP sockets
impl AsyncStream for TcpStream {}

/// Polymorphic stream wrapper
///
/// Wraps either Unix or TCP stream with a unified interface.
/// Uses enum dispatch for zero-cost abstraction.
pub enum IpcStream {
    /// Unix socket stream
    Unix(UnixStream),
    /// TCP stream (localhost only)
    Tcp(TcpStream),
}

impl IpcStream {
    /// Get stream type description (for logging)
    pub const fn stream_type(&self) -> &str {
        match self {
            Self::Unix(_) => "Unix socket",
            Self::Tcp(_) => "TCP (localhost)",
        }
    }
}

/// Implement `AsyncRead` for `IpcStream` (delegates to inner stream)
impl AsyncRead for IpcStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        match &mut *self {
            Self::Unix(stream) => Pin::new(stream).poll_read(cx, buf),
            Self::Tcp(stream) => Pin::new(stream).poll_read(cx, buf),
        }
    }
}

/// Implement `AsyncWrite` for `IpcStream` (delegates to inner stream)
impl AsyncWrite for IpcStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        match &mut *self {
            Self::Unix(stream) => Pin::new(stream).poll_write(cx, buf),
            Self::Tcp(stream) => Pin::new(stream).poll_write(cx, buf),
        }
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        match &mut *self {
            Self::Unix(stream) => Pin::new(stream).poll_flush(cx),
            Self::Tcp(stream) => Pin::new(stream).poll_flush(cx),
        }
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        match &mut *self {
            Self::Unix(stream) => Pin::new(stream).poll_shutdown(cx),
            Self::Tcp(stream) => Pin::new(stream).poll_shutdown(cx),
        }
    }
}

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
    debug!("🔌 Connecting to: {}", endpoint.description());

    match endpoint {
        IpcEndpoint::UnixSocket(path) => {
            debug!("   Using Unix socket transport");
            let stream = UnixStream::connect(path)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to connect to Unix socket: {e}"))?;

            debug!("✅ Connected via Unix socket");
            Ok(IpcStream::Unix(stream))
        }
        IpcEndpoint::TcpLocal(addr) => {
            debug!("   Using TCP transport (localhost)");
            let stream = TcpStream::connect(addr)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to connect to TCP endpoint: {e}"))?;

            debug!("✅ Connected via TCP (localhost)");
            Ok(IpcStream::Tcp(stream))
        }
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
        assert_eq!(stream.stream_type(), "TCP (localhost)");
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
}

// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Phase 2 transport abstraction — unified stream and listener types.
//!
//! `TransportStream` wraps Unix or TCP async streams with enum dispatch.
//! `TransportListener` unifies bind/accept for server-side transport.
//!
//! These are the ecosystem-standard types that all Phase 2 primals share.
//! nestGate's existing `IpcStream` is re-exported as a type alias.

use std::fmt;
use std::net::SocketAddr;
use std::path::Path;
#[cfg(unix)]
use std::path::PathBuf;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::net::TcpStream;
#[cfg(unix)]
use tokio::net::UnixStream;

/// Ecosystem-standard polymorphic async stream.
///
/// Wraps Unix domain socket or TCP stream with zero-cost enum dispatch.
/// All Phase 2 primals use this type (or an equivalent) for transport-agnostic I/O.
#[derive(Debug)]
pub enum TransportStream {
    /// Unix domain socket stream.
    #[cfg(unix)]
    Unix(UnixStream),
    /// TCP stream.
    Tcp(TcpStream),
}

impl TransportStream {
    /// Human-readable transport type (for logging).
    #[must_use]
    pub const fn transport_type(&self) -> &str {
        match self {
            #[cfg(unix)]
            Self::Unix(_) => "UDS",
            Self::Tcp(_) => "TCP",
        }
    }

    /// Non-destructive peek at the first bytes without consuming them.
    ///
    /// Uses the OS-level `MSG_PEEK` flag so the data remains in the kernel
    /// receive buffer for subsequent reads. Used by G65 protocol negotiation
    /// to detect a `PROTOCOLS:` line without committing to a read.
    ///
    /// # Errors
    ///
    /// Forwards the I/O error from the underlying socket.
    pub async fn peek(&self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            #[cfg(unix)]
            Self::Unix(s) => {
                use std::os::fd::AsFd;
                loop {
                    s.readable().await?;
                    match rustix::net::recv(
                        s.as_fd(),
                        &mut *buf,
                        rustix::net::RecvFlags::PEEK,
                    ) {
                        Ok((n, _trunc)) => return Ok(n),
                        Err(rustix::io::Errno::AGAIN) => {}
                        Err(e) => return Err(std::io::Error::from(e)),
                    }
                }
            }
            Self::Tcp(s) => s.peek(buf).await,
        }
    }
}

impl AsyncRead for TransportStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        match &mut *self {
            #[cfg(unix)]
            Self::Unix(s) => Pin::new(s).poll_read(cx, buf),
            Self::Tcp(s) => Pin::new(s).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for TransportStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        match &mut *self {
            #[cfg(unix)]
            Self::Unix(s) => Pin::new(s).poll_write(cx, buf),
            Self::Tcp(s) => Pin::new(s).poll_write(cx, buf),
        }
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        match &mut *self {
            #[cfg(unix)]
            Self::Unix(s) => Pin::new(s).poll_flush(cx),
            Self::Tcp(s) => Pin::new(s).poll_flush(cx),
        }
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        match &mut *self {
            #[cfg(unix)]
            Self::Unix(s) => Pin::new(s).poll_shutdown(cx),
            Self::Tcp(s) => Pin::new(s).poll_shutdown(cx),
        }
    }
}

/// Ecosystem-standard transport listener for server-side accept loops.
///
/// Wraps Unix or TCP listeners. The server binds once and then calls
/// `accept()` in a loop to get `TransportStream` connections.
#[derive(Debug)]
pub enum TransportListener {
    /// Unix domain socket listener.
    #[cfg(unix)]
    Unix {
        /// The underlying listener.
        listener: tokio::net::UnixListener,
        /// Socket path (retained for cleanup on drop).
        path: PathBuf,
    },
    /// TCP listener.
    Tcp(tokio::net::TcpListener),
}

impl TransportListener {
    /// Bind a Unix domain socket listener.
    ///
    /// Removes any stale socket file before binding. The socket file is
    /// **not** automatically cleaned up on drop — the caller (or OS) is
    /// responsible for removal.
    ///
    /// # Errors
    ///
    /// Returns `io::Error` if the bind fails.
    #[cfg(unix)]
    pub fn bind_unix(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        if path.exists() {
            let _ = std::fs::remove_file(&path);
        }
        let listener = tokio::net::UnixListener::bind(&path)?;
        Ok(Self::Unix { listener, path })
    }

    /// Wrap an already-bound TCP listener.
    #[must_use]
    pub const fn from_tcp(listener: tokio::net::TcpListener) -> Self {
        Self::Tcp(listener)
    }

    /// Bind a new TCP listener.
    ///
    /// # Errors
    ///
    /// Returns `io::Error` if the bind fails.
    pub async fn bind_tcp(addr: SocketAddr) -> std::io::Result<Self> {
        let listener = tokio::net::TcpListener::bind(addr).await?;
        Ok(Self::Tcp(listener))
    }

    /// Accept a new connection, returning the stream and a peer label.
    ///
    /// The peer label is:
    /// - `"unix"` for UDS connections
    /// - `"{addr}"` for TCP connections (e.g. `"127.0.0.1:54321"`)
    ///
    /// # Errors
    ///
    /// Returns `io::Error` on accept failure.
    pub async fn accept(&self) -> std::io::Result<(TransportStream, String)> {
        match self {
            #[cfg(unix)]
            Self::Unix { listener, .. } => {
                let (stream, _addr) = listener.accept().await?;
                Ok((TransportStream::Unix(stream), "unix".into()))
            }
            Self::Tcp(listener) => {
                let (stream, addr) = listener.accept().await?;
                Ok((TransportStream::Tcp(stream), addr.to_string()))
            }
        }
    }

    /// Human-readable address for logging.
    #[must_use]
    pub fn display_address(&self) -> String {
        match self {
            #[cfg(unix)]
            Self::Unix { path, .. } => path.display().to_string(),
            Self::Tcp(l) => l
                .local_addr()
                .map_or_else(|_| "TCP (unknown)".into(), |a| a.to_string()),
        }
    }

    /// The Unix socket path, if this is a UDS listener.
    #[must_use]
    pub fn unix_path(&self) -> Option<&Path> {
        match self {
            #[cfg(unix)]
            Self::Unix { path, .. } => Some(path),
            Self::Tcp(_) => None,
        }
    }
}

impl fmt::Display for TransportListener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(unix)]
            Self::Unix { path, .. } => write!(f, "UDS:{}", path.display()),
            Self::Tcp(l) => {
                let addr = l
                    .local_addr()
                    .map_or_else(|_| "?".into(), |a| a.to_string());
                write!(f, "TCP:{addr}")
            }
        }
    }
}

/// Connect to a [`TransportEndpoint`](nestgate_types::TransportEndpoint),
/// returning a [`TransportStream`].
///
/// This is the ecosystem-standard outbound connect function.
///
/// For `MeshRelay` endpoints, the coordinator socket is resolved via standard
/// discovery env vars and the connection is established to the coordinator,
/// which proxies the mesh-relayed traffic. Callers should send an `ipc.relay`
/// handshake after connecting to bind the stream to the target peer/capability.
///
/// # Errors
///
/// Returns an error when the connection fails or when the coordinator cannot
/// be discovered for `MeshRelay` transport.
pub async fn connect_transport(
    endpoint: &nestgate_types::TransportEndpoint,
) -> anyhow::Result<TransportStream> {
    use nestgate_types::TransportEndpoint as EP;
    use tracing::debug;

    debug!("connect_transport: {endpoint}");

    match endpoint {
        #[cfg(unix)]
        EP::Uds { path } => {
            let stream = UnixStream::connect(path)
                .await
                .map_err(|e| anyhow::anyhow!("UDS connect to {}: {e}", path.display()))?;
            Ok(TransportStream::Unix(stream))
        }
        #[cfg(not(unix))]
        EP::Uds { path } => Err(anyhow::anyhow!(
            "UDS transport not available on this platform: {}",
            path.display()
        )),
        EP::Tcp { host, port } => {
            let addr = format!("{host}:{port}");
            let stream = TcpStream::connect(&addr)
                .await
                .map_err(|e| anyhow::anyhow!("TCP connect to {addr}: {e}"))?;
            Ok(TransportStream::Tcp(stream))
        }
        EP::MeshRelay {
            peer_id,
            capability,
        } => {
            debug!("MeshRelay connect: resolving coordinator for {peer_id}/{capability}");
            let coordinator = discover_coordinator_socket_for_relay()
                .ok_or_else(|| anyhow::anyhow!(
                    "MeshRelay transport ({peer_id}/{capability}) requires the ecosystem \
                     coordinator, but no coordinator socket was discovered"
                ))?;

            debug!(
                "MeshRelay: connecting to coordinator at {} for relay to {peer_id}/{capability}",
                coordinator.display()
            );

            #[cfg(unix)]
            {
                let stream = UnixStream::connect(&coordinator)
                    .await
                    .map_err(|e| anyhow::anyhow!(
                        "MeshRelay coordinator connect to {}: {e}",
                        coordinator.display()
                    ))?;
                Ok(TransportStream::Unix(stream))
            }
            #[cfg(not(unix))]
            {
                Err(anyhow::anyhow!(
                    "MeshRelay coordinator connection requires UDS (not available on this platform)"
                ))
            }
        }
    }
}

/// Discover the ecosystem coordinator socket for mesh relay connections.
///
/// Same discovery logic as `primal_announce` and the capability router, adapted
/// for the transport layer.
fn discover_coordinator_socket_for_relay() -> Option<std::path::PathBuf> {
    use std::path::PathBuf;

    let eco = nestgate_config::constants::system::ecosystem_name(&nestgate_types::ProcessEnv);

    for var in ["ECOSYSTEM_IPC_SOCKET", "BIOMEOS_IPC_SOCKET"] {
        if let Ok(val) = std::env::var(var) {
            let p = PathBuf::from(val);
            if p.exists() {
                return Some(p);
            }
        }
    }
    if let Ok(dir) = std::env::var("ECOSYSTEM_SOCKET_DIR")
        .or_else(|_| std::env::var("BIOMEOS_SOCKET_DIR"))
    {
        let p = PathBuf::from(dir).join(format!("{eco}.sock"));
        if p.exists() {
            return Some(p);
        }
    }
    if let Ok(xdg) = std::env::var("XDG_RUNTIME_DIR") {
        for name in [format!("{eco}.sock"), format!("{eco}-coordinator.sock")] {
            let p = PathBuf::from(&xdg).join(&eco).join(&name);
            if p.exists() {
                return Some(p);
            }
        }
    }
    let tmp = std::env::temp_dir().join(format!("{eco}.sock"));
    if tmp.exists() {
        return Some(tmp);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn transport_stream_tcp_roundtrip() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let handle = tokio::spawn(async move {
            let (mut conn, _) = listener.accept().await.unwrap();
            let mut buf = [0u8; 5];
            conn.read_exact(&mut buf).await.unwrap();
            conn.write_all(&buf).await.unwrap();
        });

        let ep = nestgate_types::TransportEndpoint::tcp("127.0.0.1", addr.port());
        let mut stream = connect_transport(&ep).await.unwrap();
        assert_eq!(stream.transport_type(), "TCP");

        stream.write_all(b"hello").await.unwrap();
        let mut buf = [0u8; 5];
        stream.read_exact(&mut buf).await.unwrap();
        assert_eq!(&buf, b"hello");

        handle.await.unwrap();
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn transport_stream_uds_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let sock = dir.path().join("test.sock");

        let listener = TransportListener::bind_unix(&sock).unwrap();
        assert!(listener.unix_path().is_some());
        assert!(listener.display_address().contains("test.sock"));

        let ep = nestgate_types::TransportEndpoint::uds(&sock);
        let handle = tokio::spawn(async move {
            let (mut conn, peer) = listener.accept().await.unwrap();
            assert_eq!(peer, "unix");
            let mut buf = [0u8; 3];
            conn.read_exact(&mut buf).await.unwrap();
            conn.write_all(&buf).await.unwrap();
        });

        let mut stream = connect_transport(&ep).await.unwrap();
        assert_eq!(stream.transport_type(), "UDS");
        stream.write_all(b"hi!").await.unwrap();
        let mut buf = [0u8; 3];
        stream.read_exact(&mut buf).await.unwrap();
        assert_eq!(&buf, b"hi!");

        handle.await.unwrap();
    }

    #[tokio::test]
    async fn transport_listener_tcp_accept() {
        let listener = TransportListener::bind_tcp("127.0.0.1:0".parse().unwrap())
            .await
            .unwrap();
        let addr_str = listener.display_address();
        assert!(addr_str.contains("127.0.0.1"));

        let addr: SocketAddr = addr_str.parse().unwrap();
        let handle = tokio::spawn(async move {
            let (stream, peer) = listener.accept().await.unwrap();
            assert_eq!(stream.transport_type(), "TCP");
            assert!(peer.contains("127.0.0.1"));
        });

        let _client = TcpStream::connect(addr).await.unwrap();
        handle.await.unwrap();
    }

    #[tokio::test]
    async fn connect_transport_mesh_relay_no_coordinator() {
        temp_env::async_with_vars(
            [
                ("ECOSYSTEM_IPC_SOCKET", None::<&str>),
                ("BIOMEOS_IPC_SOCKET", None::<&str>),
                ("ECOSYSTEM_SOCKET_DIR", None::<&str>),
                ("BIOMEOS_SOCKET_DIR", None::<&str>),
            ],
            async {
                let ep = nestgate_types::TransportEndpoint::mesh_relay("peer1", "security");
                let err = connect_transport(&ep).await.unwrap_err().to_string();
                assert!(
                    err.contains("coordinator"),
                    "expected coordinator discovery error, got: {err}"
                );
            },
        )
        .await;
    }

    #[tokio::test]
    async fn connect_transport_uds_nonexistent_fails() {
        let ep = nestgate_types::TransportEndpoint::uds("/nonexistent/test.sock");
        assert!(connect_transport(&ep).await.is_err());
    }

    #[test]
    fn transport_listener_display() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let listener = TransportListener::bind_tcp("127.0.0.1:0".parse().unwrap())
                .await
                .unwrap();
            let display = format!("{listener}");
            assert!(display.starts_with("TCP:"));
        });
    }
}

//! **UNIX SOCKET LISTENER**
//!
//! Unix socket transport for TRUE PRIMAL IPC.

use nestgate_core::error::{NestGateError, Result};
use std::path::{Path, PathBuf};
use tokio::net::{UnixListener, UnixStream};
use tracing::{error, info, warn};

/// **UNIX SOCKET LISTENER**
///
/// Listens for connections on a Unix socket and handles JSON-RPC requests.
pub struct UnixSocketListener {
    socket_path: PathBuf,
    listener: Option<UnixListener>,
}

impl UnixSocketListener {
    /// Create new Unix socket listener
    ///
    /// # Errors
    ///
    /// Returns error if socket cannot be created
    pub fn new(socket_path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            socket_path: socket_path.as_ref().to_path_buf(),
            listener: None,
        })
    }

    /// Bind to the Unix socket
    ///
    /// # Errors
    ///
    /// Returns error if binding fails
    pub fn bind(&mut self) -> Result<()> {
        // Remove old socket if it exists
        if self.socket_path.exists() {
            info!("Removing old socket: {}", self.socket_path.display());
            std::fs::remove_file(&self.socket_path).map_err(|e| {
                NestGateError::network_error(&format!("Failed to remove old socket: {}", e))
            })?;
        }

        // Create parent directory if it doesn't exist
        if let Some(parent) = self.socket_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    NestGateError::network_error(&format!(
                        "Failed to create socket directory: {}",
                        e
                    ))
                })?;
            }
        }

        // Bind to socket
        let listener = UnixListener::bind(&self.socket_path).map_err(|e| {
            NestGateError::network_error(&format!("Failed to bind Unix socket: {}", e))
        })?;

        info!(
            "✅ NestGate listening on Unix socket: {}",
            self.socket_path.display()
        );

        self.listener = Some(listener);
        Ok(())
    }

    /// Accept a connection
    ///
    /// # Errors
    ///
    /// Returns error if connection acceptance fails
    pub async fn accept(&self) -> Result<UnixStream> {
        let listener = self
            .listener
            .as_ref()
            .ok_or_else(|| NestGateError::network_error("Listener not bound"))?;

        let (stream, _addr) = listener.accept().await.map_err(|e| {
            NestGateError::network_error(&format!("Failed to accept connection: {}", e))
        })?;

        Ok(stream)
    }

    /// Get socket path
    #[must_use]
    pub fn socket_path(&self) -> &Path {
        &self.socket_path
    }
}

impl Drop for UnixSocketListener {
    fn drop(&mut self) {
        // Clean up socket file
        if self.socket_path.exists() {
            if let Err(e) = std::fs::remove_file(&self.socket_path) {
                warn!("Failed to remove socket on cleanup: {}", e);
            } else {
                info!("Cleaned up Unix socket: {}", self.socket_path.display());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_listener_creation() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");
        let listener = UnixSocketListener::new(&socket_path).unwrap();
        assert_eq!(listener.socket_path(), socket_path);
    }

    #[tokio::test]
    async fn test_listener_bind() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");
        let mut listener = UnixSocketListener::new(&socket_path).unwrap();
        assert!(listener.bind().is_ok());
        assert!(socket_path.exists());
    }
}

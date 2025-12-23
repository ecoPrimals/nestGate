//! # Dynamic Port Allocation for Tests
//!
//! Eliminates port conflicts in concurrent tests by using OS-assigned ports.
//!
//! ## Anti-Pattern: Hardcoded Ports
//!
//! ```rust,ignore
//! // ❌ BAD: Port conflicts in parallel tests
//! let server = Server::bind("127.0.0.1:8080").await?; // Conflict!
//! ```
//!
//! ## Modern Pattern: Dynamic Ports
//!
//! ```rust,ignore
//! // ✅ GOOD: OS assigns unique port
//! let port = DynamicPort::new();
//! let server = Server::bind(format!("127.0.0.1:{}", port.get())).await?;
//! ```

use std::net::TcpListener;

/// **Dynamic Port** - OS-assigned port for testing
///
/// Uses port 0 to let the OS assign an available port, eliminating conflicts.
///
/// # Example
///
/// ```rust,ignore
/// #[tokio::test]
/// async fn test_server() {
///     let port = DynamicPort::new();
///     let addr = format!("127.0.0.1:{}", port.get());
///     let server = Server::bind(&addr).await?;
///     // Test server...
/// }
/// ```
pub struct DynamicPort {
    port: u16,
}

impl DynamicPort {
    /// Allocate a new dynamic port
    ///
    /// Binds to port 0, gets OS-assigned port, then releases the socket.
    /// There's a tiny race window, but it's acceptable for tests.
    pub fn new() -> Self {
        let port = Self::allocate_port();
        Self { port }
    }

    /// Get the allocated port number
    pub fn get(&self) -> u16 {
        self.port
    }

    /// Allocate a port by binding and immediately releasing
    fn allocate_port() -> u16 {
        // Bind to port 0 to let OS assign
        let listener = TcpListener::bind("127.0.0.1:0")
            .expect("Failed to bind to ephemeral port");
        
        let addr = listener.local_addr()
            .expect("Failed to get local address");
        
        // Port is released when listener drops
        addr.port()
    }

    /// Create a bind address string
    pub fn bind_addr(&self) -> String {
        format!("127.0.0.1:{}", self.port)
    }

    /// Create a full URL
    pub fn url(&self, path: &str) -> String {
        format!("http://127.0.0.1:{}{}", self.port, path)
    }
}

impl Default for DynamicPort {
    fn default() -> Self {
        Self::new()
    }
}

/// Allocate multiple dynamic ports at once
///
/// # Example
///
/// ```rust,ignore
/// let [api_port, ws_port, metrics_port] = allocate_ports::<3>();
/// ```
pub fn allocate_ports<const N: usize>() -> [u16; N] {
    let mut ports = [0u16; N];
    for port in &mut ports {
        *port = DynamicPort::allocate_port();
    }
    ports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_port_allocation() {
        let port = DynamicPort::new();
        assert!(port.get() > 0);
        assert!(port.get() <= 65535);
    }

    #[test]
    fn test_unique_ports() {
        let port1 = DynamicPort::new();
        let port2 = DynamicPort::new();
        assert_ne!(port1.get(), port2.get(), "Ports should be unique");
    }

    #[test]
    fn test_bind_addr() {
        let port = DynamicPort::new();
        let addr = port.bind_addr();
        assert!(addr.starts_with("127.0.0.1:"));
    }

    #[test]
    fn test_url_generation() {
        let port = DynamicPort::new();
        let url = port.url("/api/v1/health");
        assert!(url.starts_with("http://127.0.0.1:"));
        assert!(url.ends_with("/api/v1/health"));
    }

    #[test]
    fn test_allocate_multiple() {
        let [p1, p2, p3] = allocate_ports::<3>();
        assert_ne!(p1, p2);
        assert_ne!(p2, p3);
        assert_ne!(p1, p3);
    }
}


//
// Connects to remote ZFS services over HTTP/HTTPS.
// This backend can be used to manage ZFS on remote systems.

pub mod client;
pub mod connection;
pub mod implementation;
pub mod service;

// #[cfg(test)]
#[cfg(test)]
mod tests;

// Re-export main types
pub use connection::{ConnectionError, ConnectionStats};
pub use service::RemoteZfsService;

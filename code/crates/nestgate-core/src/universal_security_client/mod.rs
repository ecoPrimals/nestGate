// **UNIVERSAL SECURITY CLIENT**
// Modular implementation of capability-based decentralized authentication client.
// Split from a single 1660+ line file for better maintainability.
// Core client implementation
//! Universal Security Client module

pub mod client;
// Removed discovery module - using unified NestGateError
// Re-export public API
pub use client::UniversalSecurityClient;
// Removed discovery import - using unified NestGateError

// Test module (only in test builds)
#[cfg(test)]
mod tests;

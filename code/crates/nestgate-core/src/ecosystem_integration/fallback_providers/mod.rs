// Fallback Providers
// Local implementations that provide capabilities when external primals are unavailable
//
// **SECURITY NOTE** (Nov 20, 2025):
// The security fallback provider has been REMOVED due to critical security vulnerability.
// It was using base64 encoding instead of real encryption.
// Production systems MUST use a real security capability provider (discovered dynamically).

//! Fallback Providers module

pub mod ai;
pub mod orchestration;
// pub mod security; // REMOVED: Insecure implementation (base64 "encryption")
pub mod zfs;

pub use ai::AiFallbackProvider;
pub use orchestration::OrchestrationFallbackProvider;
// pub use security::SecurityFallbackProvider; // REMOVED: Security vulnerability
pub use zfs::ZfsFallbackProvider;

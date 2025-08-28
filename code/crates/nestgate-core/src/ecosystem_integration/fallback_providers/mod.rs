//! Fallback Providers
//! Local implementations that provide capabilities when external primals are unavailable

pub mod ai;
pub mod orchestration;
pub mod security;
pub mod zfs;

pub use ai::AiFallbackProvider;
pub use orchestration::OrchestrationFallbackProvider;
pub use security::SecurityFallbackProvider;
pub use zfs::ZfsFallbackProvider;

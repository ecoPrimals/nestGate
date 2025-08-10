//! Fallback Providers
//! Local implementations that provide capabilities when external primals are unavailable

pub mod ai;
pub mod orchestration;
pub mod security;
// Temporarily commented out due to compilation issues
// pub mod zfs;

pub use ai::AiFallbackProvider;
pub use orchestration::OrchestrationFallbackProvider;
pub use security::SecurityFallbackProvider;
// Temporarily commented out due to compilation issues
// pub use zfs::ZfsFallbackProvider;

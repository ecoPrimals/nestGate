//
// **Unified configuration and types for NestGate File System Orchestrator**
// 
// This library provides the **single source of truth** for all NestGate configurations,
// error types, and core abstractions, based on the proven success patterns from
// Songbird Universal Orchestrator.
//
// ## 🏆 Architecture Benefits
// 
// - **Unified Configuration**: Single `NestGateConfig` replaces fragmented configs
// - **Consistent Error Handling**: Single `NestGateError` with AI automation hints
// - **Zero-Cost Patterns**: Direct composition eliminating Arc<dyn> overhead
// - **Performance Monitoring**: Comprehensive regression testing integration
//
// ## 🚀 Usage
//
// ```rust
// use nestgate_canonical::{NestGateConfig, NestGateError, NestGateResult};
//
// // Unified configuration loading
// let config = NestGateConfig::from_env()?;
// 
// // Consistent error handling with AI hints
// fn file_operation() -> NestGateResult<String> {
//     // File system operations with unified error handling
// }
// ```

pub mod config;
pub mod error;
pub mod types;
pub mod traits;

// Re-export core types for easy access (Songbird pattern)
pub use config::NestGateConfig;
pub use error::{NestGateError, NestGateResult};
pub use types::*;
pub use traits::*;

// Version and metadata
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Initialize the canonical NestGate system with default configuration
pub fn init() -> NestGateResult<NestGateConfig> {
    NestGateConfig::from_env()
}

/// Initialize with custom configuration path
pub fn init_with_config(config_path: &str) -> NestGateResult<NestGateConfig> {
    NestGateConfig::from_file(config_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_initialization() {
        // Test that canonical system initializes correctly
        let config = NestGateConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test] 
    fn test_error_system_integration() {
        // Test unified error handling
        let error = NestGateError::file_system("Test error", None);
        assert!(error.to_string().contains("File System Error"));
    }
} 
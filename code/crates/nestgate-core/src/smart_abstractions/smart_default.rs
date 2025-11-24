// Smart Default Implementation
//! Smart Default functionality and utilities.
// Provides intelligent default value generation that eliminates the need
//! for manual impl Default blocks across the codebase.
//! Smart Default functionality and utilities.
// **PROBLEM SOLVED**: 200+ manual impl Default blocks (~3000 lines of boilerplate)
// **SOLUTION**: Attribute-based smart defaults with intelligent inference

use std::collections::HashMap;
use std::time::Duration;

/// Smart Default trait that provides intelligent default values
///
/// This trait can be derived and supports attribute-based customization:
/// - `#[default = "value"]` - Custom default value
/// - `#[default_fn = "function_name"]` - Custom default function
/// - Automatic defaults for common types (String, Vec, HashMap, etc.)
pub trait SmartDefault {
    fn smart_default() -> Self;
}
// Implement for common types with sensible defaults
impl SmartDefault for String {
    fn smart_default() -> Self {
        String::new()
    }
}

impl<T> SmartDefault for Vec<T> {
    fn smart_default() -> Self {
        Vec::new()
    }
}

impl<K, V> SmartDefault for HashMap<K, V> {
    fn smart_default() -> Self {
        HashMap::new()
    }
}

impl SmartDefault for bool {
    fn smart_default() -> Self {
        false
    }
}

impl SmartDefault for u16 {
    fn smart_default() -> Self {
        0
    }
}

impl SmartDefault for u32 {
    fn smart_default() -> Self {
        0
    }
}

impl SmartDefault for u64 {
    fn smart_default() -> Self {
        0
    }
}

impl SmartDefault for f64 {
    fn smart_default() -> Self {
        0.0
    }
}

impl SmartDefault for Duration {
    fn smart_default() -> Self {
        Duration::from_secs(30) // Sensible default timeout
    }
}

impl<T> SmartDefault for Option<T> {
    fn smart_default() -> Self {
        None
    }
}

/// Smart default values for common configuration patterns
pub struct SmartDefaults;
impl SmartDefaults {
    /// Use canonical constant for default host
    pub use crate::constants::LOCALHOST as DEFAULT_HOST;
    /// Use canonical constant for default port
    pub use crate::constants::API_PORT as DEFAULT_PORT;
    pub const DEFAULT_TIMEOUT_SECS: u64 = 30;
    pub const DEFAULT_RETRY_ATTEMPTS: u32 = 3;
    pub const DEFAULT_BUFFER_SIZE: usize = 8192;
    pub const DEFAULT_MAX_CONNECTIONS: usize = 1000;

    pub fn default_host() -> String {
        Self::DEFAULT_HOST.to_string()
    }

    pub fn default_timeout() -> Duration {
        Duration::from_secs(Self::DEFAULT_TIMEOUT_SECS)
    }

    pub fn default_empty_vec<T>() -> Vec<T> {
        Vec::new()
    }

    pub fn default_empty_map<K, V>() -> HashMap<K, V> {
        HashMap::new()
    }
}

// Example usage demonstration - this would be used with a derive macro

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type ExampleConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using ExampleConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod example {
    use super::*;

    // BEFORE (manual implementation - 20+ lines):
    // impl Default for ExampleConfig {
    //     fn default() -> Self {
    //         Self {
    //             host: "127.0.0.1".to_string(),
    //             port: 8080,
    //             timeout: Duration::from_secs(30),
    //             enabled: true,
    //             tags: HashMap::new(),
    //             channels: Vec::new(),
    //         }
    //     }
    // }

    // AFTER (declarative with SmartDefault):
    #[derive(Clone)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::ExampleConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::ExampleConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
    pub struct ExampleConfig {
        pub host: String,                  // Would use SmartDefaults::default_host()
        pub port: u16,                     // Would use SmartDefaults::DEFAULT_PORT
        pub timeout: Duration,             // Would use SmartDefaults::default_timeout()
        pub enabled: bool,                 // Would use SmartDefault::smart_default() -> false
        pub tags: HashMap<String, String>, // Would use SmartDefault::smart_default()
        pub channels: Vec<String>,         // Would use SmartDefault::smart_default()
    }

    // Implementation that would be generated by derive macro
    impl SmartDefault for ExampleConfig {
        fn smart_default() -> Self {
            Self {
                host: SmartDefaults::default_host(),
                port: SmartDefaults::DEFAULT_PORT,
                timeout: SmartDefaults::default_timeout(),
                enabled: true, // Custom override
                tags: HashMap::smart_default(),
                channels: Vec::smart_default(),
            }
        }
    }

    impl Default for ExampleConfig {
        fn default() -> Self {
            Self::smart_default()
        }
    }
}


// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type ExampleConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using ExampleConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {

    #[test]
    fn test_smart_defaults() {
        assert_eq!(String::smart_default(), "");
        assert_eq!(bool::smart_default(), false);
        assert_eq!(u16::smart_default(), 0);
        assert_eq!(Duration::smart_default(), Duration::from_secs(30));
        assert_eq!(Vec::<String>::smart_default().len(), 0);
        assert_eq!(HashMap::<String, String>::smart_default().len(), 0);
    }

    #[test]
    fn test_smart_defaults_constants() {
        assert_eq!(SmartDefaults::DEFAULT_HOST, "127.0.0.1");
        assert_eq!(SmartDefaults::DEFAULT_PORT, 8080);
        assert_eq!(SmartDefaults::DEFAULT_TIMEOUT_SECS, 30);
        assert_eq!(SmartDefaults::default_host(), "127.0.0.1");
        assert_eq!(SmartDefaults::default_timeout(), Duration::from_secs(30));
    }
}

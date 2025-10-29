//! **FINAL CONSTANTS CONSOLIDATION**
//! 
//! The ultimate consolidation of all constants across the NestGate ecosystem

use std::time::Duration;

// ==================== NETWORK CONSTANTS ====================

pub mod network {
    use std::time::Duration;
    
    /// Default API server port
    pub const DEFAULT_API_PORT: u16 = 8080;
    
    /// Default WebSocket port  
    pub const DEFAULT_WEBSOCKET_PORT: u16 = 8081;
    
    /// Default metrics port
    pub const DEFAULT_METRICS_PORT: u16 = 9090;
    
    /// Default host address
    pub const DEFAULT_HOST: &str = "127.0.0.1";
    
    /// Localhost address
    pub const LOCALHOST: &str = "localhost";
    
    /// Default connection timeout
    pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
    
    /// Default read timeout
    pub const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(60);
    
    /// Default write timeout
    pub const DEFAULT_WRITE_TIMEOUT: Duration = Duration::from_secs(60);
    
    /// Maximum concurrent connections
    pub const MAX_CONNECTIONS: usize = 1000;
    
    /// Connection pool size
    pub const CONNECTION_POOL_SIZE: usize = 100;
}

// ==================== STORAGE CONSTANTS ====================

pub mod storage {
    use std::time::Duration;
    
    /// Default buffer size for I/O operations
    pub const DEFAULT_BUFFER_SIZE: usize = 4096;
    
    /// Large buffer size for bulk operations
    pub const LARGE_BUFFER_SIZE: usize = 65536;
    
    /// Small buffer size for metadata
    pub const SMALL_BUFFER_SIZE: usize = 1024;
    
    /// Page size for memory mapping
    pub const PAGE_SIZE: usize = 4096;
    
    /// Cache line size for optimization
    pub const CACHE_LINE_SIZE: usize = 64;
    
    /// Maximum file size for single operations
    pub const MAX_FILE_SIZE: usize = 1_073_741_824; // 1GB
    
    /// Default storage timeout
    pub const DEFAULT_STORAGE_TIMEOUT: Duration = Duration::from_secs(300);
    
    /// Batch operation size
    pub const BATCH_SIZE: usize = 1000;
}

// ==================== PERFORMANCE CONSTANTS ====================

pub mod performance {
    use std::time::Duration;
    
    /// Target performance improvement percentage
    pub const TARGET_IMPROVEMENT_PERCENT: f64 = 20.0;
    
    /// Maximum retry attempts
    pub const MAX_RETRIES: u32 = 3;
    
    /// Retry backoff base duration
    pub const RETRY_BACKOFF_BASE: Duration = Duration::from_millis(100);
    
    /// Circuit breaker failure threshold
    pub const CIRCUIT_BREAKER_THRESHOLD: u32 = 5;
    
    /// Rate limiting window
    pub const RATE_LIMIT_WINDOW: Duration = Duration::from_secs(60);
    
    /// Default rate limit
    pub const DEFAULT_RATE_LIMIT: u32 = 1000;
    
    /// Thread pool size
    pub const THREAD_POOL_SIZE: usize = 8;
    
    /// Worker queue size
    pub const WORKER_QUEUE_SIZE: usize = 10000;
}

// ==================== SECURITY CONSTANTS ====================

pub mod security {
    use std::time::Duration;
    
    /// Token expiration time
    pub const TOKEN_EXPIRATION: Duration = Duration::from_secs(3600); // 1 hour
    
    /// Session timeout
    pub const SESSION_TIMEOUT: Duration = Duration::from_secs(1800); // 30 minutes
    
    /// Maximum login attempts
    pub const MAX_LOGIN_ATTEMPTS: u32 = 3;
    
    /// Account lockout duration
    pub const LOCKOUT_DURATION: Duration = Duration::from_secs(300); // 5 minutes
    
    /// Password minimum length
    pub const MIN_PASSWORD_LENGTH: usize = 8;
    
    /// Salt length for hashing
    pub const SALT_LENGTH: usize = 32;
    
    /// Key rotation interval
    pub const KEY_ROTATION_INTERVAL: Duration = Duration::from_secs(86400); // 24 hours
}

// ==================== API CONSTANTS ====================

pub mod api {
    use std::time::Duration;
    
    /// Request timeout
    pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
    
    /// Maximum request body size
    pub const MAX_REQUEST_SIZE: usize = 10_485_760; // 10MB
    
    /// Maximum response size
    pub const MAX_RESPONSE_SIZE: usize = 104_857_600; // 100MB
    
    /// API version
    pub const API_VERSION: &str = "v1";
    
    /// Default page size for pagination
    pub const DEFAULT_PAGE_SIZE: usize = 50;
    
    /// Maximum page size
    pub const MAX_PAGE_SIZE: usize = 1000;
    
    /// Rate limiting per endpoint
    pub const API_RATE_LIMIT: u32 = 100;
}

// ==================== CONSOLIDATED CONSTANTS STRUCT ====================

/// **THE** consolidated constants structure - single source of truth
#[derive(Debug, Clone)]
pub struct NestGateConsolidatedConstants {
    pub network: NetworkConstants,
    pub storage: StorageConstants,
    pub performance: PerformanceConstants,
    pub security: SecurityConstants,
    pub api: ApiConstants,
}

impl Default for NestGateConsolidatedConstants {
    fn default() -> Self {
        Self {
            network: NetworkConstants::default(),
            storage: StorageConstants::default(),
            performance: PerformanceConstants::default(),
            security: SecurityConstants::default(),
            api: ApiConstants::default(),
        }
    }
}

// Supporting structs for type safety
#[derive(Debug, Clone, Default)]
pub struct NetworkConstants;

#[derive(Debug, Clone, Default)]
pub struct StorageConstants;

#[derive(Debug, Clone, Default)]
pub struct PerformanceConstants;

#[derive(Debug, Clone, Default)]
pub struct SecurityConstants;

#[derive(Debug, Clone, Default)]
pub struct ApiConstants;

/// Macro for easy constant access
#[macro_export]
macro_rules! nestgate_const {
    (network::$const:ident) => {
        $crate::constants::final_consolidation::network::$const
    };
    (storage::$const:ident) => {
        $crate::constants::final_consolidation::storage::$const
    };
    (performance::$const:ident) => {
        $crate::constants::final_consolidation::performance::$const
    };
    (security::$const:ident) => {
        $crate::constants::final_consolidation::security::$const
    };
    (api::$const:ident) => {
        $crate::constants::final_consolidation::api::$const
    };
}

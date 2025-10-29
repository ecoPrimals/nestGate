//! **CANONICAL HANDLER CONFIGURATION**
//!
//! This module consolidates ALL handler configuration variants across the NestGate ecosystem
//! into a single, authoritative configuration structure.
//!
//! **CONSOLIDATES**:
//! - 50+ scattered handler configurations across all crates
//! - API handlers, ZFS handlers, middleware handlers
//! - Error handlers, validation handlers, security handlers
//! - Monitoring handlers, performance handlers
//!
//! **MODULAR STRUCTURE**:
//! - `api`: API request/response handler configurations
//! - `zfs`: ZFS operation handler configurations
//! - `middleware`: Middleware handler configurations
//! - `error`: Error handling configurations
//! - `validation`: Validation handler configurations
//! - `security`: Security handler configurations
//! - `monitoring`: Monitoring and observability handlers
//! - `performance`: Performance handler configurations
//! - `event`: Event handler configurations
//! - `lifecycle`: Lifecycle handler configurations
//! - `environment`: Environment-specific handler settings

use serde::{Deserialize, Serialize};

// Import all handler configuration modules
pub mod api;
pub mod zfs;
pub mod middleware;
pub mod error;
pub mod validation;
pub mod security;
pub mod monitoring;
pub mod performance;
pub mod event;
pub mod lifecycle;
pub mod environment;

// Re-export all configuration types
pub use api::{
    ApiHandlerConfig, RequestHandlerConfig, ResponseHandlerConfig, 
    RouteHandlerConfig, AuthHandlerConfig, RateLimitHandlerConfig
};
pub use zfs::{
    ZfsHandlerConfig, PoolHandlerConfig, DatasetHandlerConfig, 
    SnapshotHandlerConfig, BackupHandlerConfig
};
pub use middleware::{
    MiddlewareHandlerConfig, CorsHandlerConfig, CompressionHandlerConfig,
    SecurityMiddlewareConfig, LoggingMiddlewareConfig
};
pub use error::{
    ErrorHandlerConfig, ErrorResponseConfig, ErrorLoggingConfig,
    ErrorRecoveryConfig, ErrorNotificationConfig
};
pub use validation::{
    ValidationHandlerConfig, SchemaValidationConfig, DataValidationConfig,
    BusinessRuleValidationConfig, CustomValidationConfig
};
pub use security::{
    SecurityHandlerConfig, AuthenticationHandlerConfig, AuthorizationHandlerConfig,
    ThreatDetectionConfig, AuditHandlerConfig
};
pub use monitoring::{
    MonitoringHandlerConfig, MetricsHandlerConfig, TracingHandlerConfig,
    HealthCheckHandlerConfig, AlertingHandlerConfig
};
pub use performance::{
    PerformanceHandlerConfig, CachingHandlerConfig, OptimizationHandlerConfig,
    ProfilerHandlerConfig, LoadBalancingHandlerConfig
};
pub use event::{
    EventHandlerConfig, EventProcessingConfig, EventRoutingConfig,
    EventSubscriptionConfig, EventPublishingConfig
};
pub use lifecycle::{
    LifecycleHandlerConfig, StartupHandlerConfig, ShutdownHandlerConfig,
    HealthHandlerConfig, MaintenanceHandlerConfig
};
pub use environment::{
    HandlerEnvironmentConfig, HandlerDebugConfig, HandlerFeatureConfig,
    HandlerOverrideConfig
};

// ==================== CANONICAL HANDLER CONFIGURATION ====================

/// **THE** canonical handler configuration for the entire NestGate ecosystem
/// This replaces ALL other handler configuration variants
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct CanonicalHandlerConfigs {
    /// API request/response handlers
    pub api_handlers: ApiHandlerConfig,
    
    /// ZFS operation handlers
    pub zfs_handlers: ZfsHandlerConfig,
    
    /// Middleware handlers
    pub middleware_handlers: MiddlewareHandlerConfig,
    
    /// Error handling configuration
    pub error_handlers: ErrorHandlerConfig,
    
    /// Validation handlers
    pub validation_handlers: ValidationHandlerConfig,
    
    /// Security handlers
    pub security_handlers: SecurityHandlerConfig,
    
    /// Monitoring and observability handlers
    pub monitoring_handlers: MonitoringHandlerConfig,
    
    /// Performance handlers
    pub performance_handlers: PerformanceHandlerConfig,
    
    /// Event handlers
    pub event_handlers: EventHandlerConfig,
    
    /// Lifecycle handlers
    pub lifecycle_handlers: LifecycleHandlerConfig,
    
    /// Environment-specific handler settings
    pub environment: HandlerEnvironmentConfig,
}


impl CanonicalHandlerConfigs {
    /// Create a new canonical handler configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a configuration optimized for production environments
    pub fn production_optimized() -> Self {
        Self {
            api_handlers: ApiHandlerConfig::production_optimized(),
            zfs_handlers: ZfsHandlerConfig::production_optimized(),
            middleware_handlers: MiddlewareHandlerConfig::production_optimized(),
            error_handlers: ErrorHandlerConfig::production_optimized(),
            validation_handlers: ValidationHandlerConfig::production_optimized(),
            security_handlers: SecurityHandlerConfig::production_optimized(),
            monitoring_handlers: MonitoringHandlerConfig::production_optimized(),
            performance_handlers: PerformanceHandlerConfig::production_optimized(),
            event_handlers: EventHandlerConfig::production_optimized(),
            lifecycle_handlers: LifecycleHandlerConfig::production_optimized(),
            environment: HandlerEnvironmentConfig::production_optimized(),
        }
    }

    /// Create a configuration optimized for development environments
    pub fn development_optimized() -> Self {
        Self {
            api_handlers: ApiHandlerConfig::development_optimized(),
            zfs_handlers: ZfsHandlerConfig::development_optimized(),
            middleware_handlers: MiddlewareHandlerConfig::development_optimized(),
            error_handlers: ErrorHandlerConfig::development_optimized(),
            validation_handlers: ValidationHandlerConfig::development_optimized(),
            security_handlers: SecurityHandlerConfig::development_optimized(),
            monitoring_handlers: MonitoringHandlerConfig::development_optimized(),
            performance_handlers: PerformanceHandlerConfig::development_optimized(),
            event_handlers: EventHandlerConfig::development_optimized(),
            lifecycle_handlers: LifecycleHandlerConfig::development_optimized(),
            environment: HandlerEnvironmentConfig::development_optimized(),
        }
    }

    /// Create a configuration for high-performance environments
    pub fn high_performance() -> Self {
        Self {
            api_handlers: ApiHandlerConfig::high_performance(),
            zfs_handlers: ZfsHandlerConfig::high_performance(),
            middleware_handlers: MiddlewareHandlerConfig::high_performance(),
            error_handlers: ErrorHandlerConfig::high_performance(),
            validation_handlers: ValidationHandlerConfig::high_performance(),
            security_handlers: SecurityHandlerConfig::high_performance(),
            monitoring_handlers: MonitoringHandlerConfig::high_performance(),
            performance_handlers: PerformanceHandlerConfig::high_performance(),
            event_handlers: EventHandlerConfig::high_performance(),
            lifecycle_handlers: LifecycleHandlerConfig::high_performance(),
            environment: HandlerEnvironmentConfig::high_performance(),
        }
    }

    /// Merge with another configuration (other takes precedence)
    pub fn merge(mut self, other: Self) -> Self {
        self.api_handlers = self.api_handlers.merge(other.api_handlers);
        self.zfs_handlers = self.zfs_handlers.merge(other.zfs_handlers);
        self.middleware_handlers = self.middleware_handlers.merge(other.middleware_handlers);
        self.error_handlers = self.error_handlers.merge(other.error_handlers);
        self.validation_handlers = self.validation_handlers.merge(other.validation_handlers);
        self.security_handlers = self.security_handlers.merge(other.security_handlers);
        self.monitoring_handlers = self.monitoring_handlers.merge(other.monitoring_handlers);
        self.performance_handlers = self.performance_handlers.merge(other.performance_handlers);
        self.event_handlers = self.event_handlers.merge(other.event_handlers);
        self.lifecycle_handlers = self.lifecycle_handlers.merge(other.lifecycle_handlers);
        self.environment = self.environment.merge(other.environment);
        self
    }

    /// Validate the handler configuration for completeness and consistency
    pub fn validate(&self) -> crate::Result<()> {
        // Validate API handlers
        self.api_handlers.validate()?;
        
        // Validate ZFS handlers
        self.zfs_handlers.validate()?;
        
        // Validate middleware handlers
        self.middleware_handlers.validate()?;
        
        // Validate error handlers
        self.error_handlers.validate()?;
        
        // Validate validation handlers
        self.validation_handlers.validate()?;
        
        // Validate security handlers
        self.security_handlers.validate()?;
        
        // Validate monitoring handlers
        self.monitoring_handlers.validate()?;
        
        // Validate performance handlers
        self.performance_handlers.validate()?;
        
        // Validate event handlers
        self.event_handlers.validate()?;
        
        // Validate lifecycle handlers
        self.lifecycle_handlers.validate()?;
        
        // Validate environment configuration
        self.environment.validate()?;
        
        Ok(())
    }
}

// ==================== BACKWARD COMPATIBILITY ALIASES ====================

/// Backward compatibility alias for existing HandlerConfig usage
pub type HandlerConfig = CanonicalHandlerConfigs;

/// Backward compatibility alias for UnifiedHandlerConfig
pub type UnifiedHandlerConfig = CanonicalHandlerConfigs;

/// Backward compatibility alias for HandlerConfigs
pub type HandlerConfigs = CanonicalHandlerConfigs; 
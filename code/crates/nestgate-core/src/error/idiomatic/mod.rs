// **IDIOMATIC ERROR EVOLUTION - MODULARIZED**
//! Module definitions and exports.
// Evolution from Result<T> to idiomatic Result<T, E> while preserving ALL benefits 
//! of our sophisticated unified error system. Organized into focused modules.

// ==================== IDIOMATIC ERROR MODULES ====================

// Core result types and patterns
pub mod result_types;
// Domain-specific error types
pub mod domain_errors;
// Error conversion and context traits
pub mod traits;
// Error creation macros
pub mod macros;
// Extension methods for Result types
pub mod extensions;
// ==================== RE-EXPORTS ====================

pub use result_types::{IdioResult, ValidationResult, NetworkResult, StorageResult, SecurityResult, ZfsResult, ApiResult, McpResult,
        };
pub use domain_errors::{ValidationError, NetworkError, StorageError, SecurityError, ZfsError, ApiError, McpError, TestingError, PerformanceError, HandlerError, SerializationError, DatabaseError, CacheError, WorkflowError, MonitoringError,
        };
pub use traits::{IntoNestGateError, WithContext, IdioResultExt,
        };

// ==================== BACKWARD COMPATIBILITY ====================

// **BACKWARD COMPATIBLE RESULT**
// 
// Maintains compatibility with existing Result<T> usage while encouraging
// migration to IdioResult<T, E> for new code.
pub type Result<T> = IdioResult<T>; 
// **ERROR VARIANTS - MODULAR ARCHITECTURE**
//! Domain-specific error variant modules for maintainable error handling.
//! Module definitions and exports.
//! This module organizes error variants into focused domain modules, replacing
//! the monolithic variants.rs with a more maintainable modular structure.

// Core error types
pub mod api_errors;
pub mod automation_errors;
pub mod core_errors;
pub mod network_errors;
pub mod security_errors;
pub mod storage_errors;
pub mod system_errors;

// Re-export the main unified error type
pub use core_errors::NestGateUnifiedError;

// Domain-specific error implementations are available through the unified type

// **THE** primary error type - canonical across all NestGate
pub type NestGateError = NestGateUnifiedError;

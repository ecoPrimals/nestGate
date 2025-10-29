//! Compliance module for enterprise storage systems
//!
//! This module implements comprehensive compliance features including:
//! - Data retention policies
//! - Access control compliance
//! - Audit logging
//! - Regulatory compliance (GDPR, HIPAA, SOX, etc.)

pub mod types;
pub mod handlers;

// TEMP_DISABLED: #[cfg(test)]
// TEMP_DISABLED: mod tests;
// TEMP_DISABLED: 
// TEMP_DISABLED: // Re-export commonly used types
// TEMP_DISABLED: pub use types::*;
// TEMP_DISABLED: pub use handlers::{create_compliance_routes, initialize_compliance_manager};
// TEMP_DISABLED: pub use handlers::{
// TEMP_DISABLED:     get_compliance_dashboard,
// TEMP_DISABLED:     get_retention_policies,
// TEMP_DISABLED:     create_retention_policy,
// TEMP_DISABLED:     get_audit_logs,
// TEMP_DISABLED:     get_violations,
// TEMP_DISABLED: };

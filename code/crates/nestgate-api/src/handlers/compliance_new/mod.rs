//! Compliance module for enterprise storage systems
//!
//! This module implements comprehensive compliance features including:
//! - Data retention policies
//! - Access control compliance
//! - Audit logging
//! - Regulatory compliance (GDPR, HIPAA, SOX, etc.)

pub mod types;
pub mod handlers;

// Re-export commonly used types
pub use types::*;
pub use handlers::{create_compliance_routes, initialize_compliance_manager};
pub use handlers::{
    get_compliance_dashboard,
    get_retention_policies,
    create_retention_policy,
    get_audit_logs,
    get_violations,
};

// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Regulatory frameworks, controls, and compliance status enums.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Regulatory framework
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryFramework {
    /// Framework ID
    pub id: String,
    /// Framework name
    pub name: String,
    /// Framework type
    pub framework_type: RegulatoryType,
    /// Required controls
    pub required_controls: Vec<ComplianceControl>,
    /// Audit frequency
    pub audit_frequency_days: u32,
    /// Last audit date
    pub last_audit: Option<DateTime<Utc>>,
    /// Compliance status
    pub compliance_status: ComplianceStatus,
}

/// Regulatory framework types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegulatoryType {
    /// GDPR (General Data Protection Regulation)
    GDPR,
    /// HIPAA (Health Insurance Portability and Accountability Act)
    HIPAA,
    /// SOX (Sarbanes-Oxley Act)
    SOX,
    /// PCI DSS (Payment Card Industry Data Security Standard)
    PCIDSS,
    /// ISO 27001
    ISO27001,
    /// `FedRAMP`
    FedRAMP,
    /// Custom framework
    Custom(String),
}

/// Compliance control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceControl {
    /// Control ID
    pub id: String,
    /// Control name
    pub name: String,
    /// Control description
    pub description: String,
    /// Control type
    pub control_type: ControlType,
    /// Implementation status
    pub implementation_status: ImplementationStatus,
    /// Last assessment date
    pub last_assessment: Option<DateTime<Utc>>,
    /// Next assessment due
    pub next_assessment_due: Option<DateTime<Utc>>,
}

/// Control types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlType {
    /// Preventive control
    Preventive,
    /// Detective control
    Detective,
    /// Corrective control
    Corrective,
    /// Compensating control
    Compensating,
}

/// Implementation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImplementationStatus {
    /// Not implemented
    NotImplemented,
    /// Partially implemented
    PartiallyImplemented,
    /// Fully implemented
    FullyImplemented,
    /// Under review
    UnderReview,
    /// Non-compliant
    NonCompliant,
}

/// Compliance status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceStatus {
    /// Compliant
    Compliant,
    /// Non-compliant
    NonCompliant,
    /// Partially compliant
    PartiallyCompliant,
    /// Under assessment
    UnderAssessment,
    /// Unknown
    Unknown,
}

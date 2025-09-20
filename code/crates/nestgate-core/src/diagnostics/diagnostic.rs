use super::types::{ComponentType, DiagnosticLevel};
/// Individual Diagnostic Entries
/// This module contains the core Diagnostic struct and its functionality.
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Individual diagnostic entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    /// Diagnostic ID
    pub id: String,
    /// Diagnostic level
    pub level: DiagnosticLevel,
    /// Component type
    pub component: ComponentType,
    /// Diagnostic message
    pub message: String,
    /// Timestamp when the diagnostic was created
    pub timestamp: SystemTime,
    /// Optional details
    pub details: Option<String>,
    /// Optional associated resource
    pub path: Option<String>,
    /// Whether the diagnostic is resolved
    pub resolved: bool,
    /// Timestamp when the diagnostic was resolved (if resolved)
    pub resolved_at: Option<SystemTime>,
}
impl Diagnostic {
    /// Create a new diagnostic using standardized builder
    #[must_use]
    pub const fn new(level: DiagnosticLevel, component: ComponentType, message: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            level,
            component,
            message,
            timestamp: std::time::SystemTime::now(),
            details: None,
            path: None,
            resolved: false,
            resolved_at: None,
        }
    }

    /// Create a new info diagnostic
    #[must_use]
    pub const fn info(component: ComponentType, message: String) -> Self {
        Self::new(DiagnosticLevel::Info, component, message)
    }

    /// Create a new warning diagnostic
    #[must_use]
    pub const fn warning(component: ComponentType, message: String) -> Self {
        Self::new(DiagnosticLevel::Warning, component, message)
    }

    /// Create a new error diagnostic
    #[must_use]
    pub const fn error(component: ComponentType, message: String) -> Self {
        Self::new(DiagnosticLevel::Error, component, message)
    }

    /// Create a new critical diagnostic
    #[must_use]
    pub const fn critical(component: ComponentType, message: String) -> Self {
        Self::new(DiagnosticLevel::Critical, component, message)
    }

    /// Set the details for the diagnostic
    #[must_use]
    pub fn with_details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }

    /// Set the resource for the diagnostic
    #[must_use]
    pub fn with_resource(mut self, path: &str) -> Self {
        self.path = Some(path.to_string());
        self
    }

    /// Mark the diagnostic as resolved
    pub fn resolve(&mut self) {
        self.resolved = true;
        self.resolved_at = Some(SystemTime::now());
    }

    /// Check if diagnostic is critical or error level
    #[must_use]
    pub const fn is_severe(&self) -> bool {
        matches!(
            self.level,
            DiagnosticLevel::Critical | DiagnosticLevel::Error
        )
    }

    /// Check if diagnostic is unresolved
    #[must_use]
    pub const fn is_unresolved(&self) -> bool {
        !self.resolved
    }

    /// Get age of diagnostic in seconds
    #[must_use]
    pub const fn age_seconds(&self) -> u64 {
        SystemTime::now()
            .duration_since(self.timestamp)
            .unwrap_or_default()
            .as_secs()
    }
}

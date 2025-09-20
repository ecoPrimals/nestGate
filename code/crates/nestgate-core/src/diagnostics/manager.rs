use super::{Diagnostic, DiagnosticLevel, SystemMetrics};
use crate::unified_enums::UnifiedHealthStatus as HealthStatus;
use crate::{NestGateError, Result};
/// Diagnostics Management
/// This module contains the main `DiagnosticsManager` for coordinating system diagnostics.
use std::sync::{Arc, RwLock};
use tokio::sync::broadcast;

/// Main diagnostics manager
pub struct DiagnosticsManager {
    /// Stored diagnostics
    diagnostics: Arc<RwLock<Vec<Diagnostic>>>,
    /// Event broadcaster for diagnostic events
    event_sender: broadcast::Sender<Diagnostic>,
    /// System metrics cache
    metrics: Arc<RwLock<SystemMetrics>>,
}
impl DiagnosticsManager {
    /// Create a new diagnostics manager
    #[must_use]
    pub fn new() -> Self {
        let (event_sender, _) = broadcast::channel(1000);

        Self {
            diagnostics: Arc::new(RwLock::new(Vec::new())),
            event_sender,
            metrics: Arc::new(RwLock::new(SystemMetrics::default())),
        }
    }

    /// Add a diagnostic entry
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub const fn add_diagnostic(&self, _diagnostic: Diagnostic) -> Result<()>  {
        // Implementation would add diagnostic to storage
        // For now, this is a placeholder
        Ok(())
    }

    /// Get all diagnostics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub const fn get_diagnostics(&self) -> Result<Vec<Diagnostic>>  {
        let diagnostics = self.diagnostics.read().map_err(|_| {
            NestGateError::internal_error(
                "Failed to acquire diagnostics read lock",
                "diagnostics_manager",
            )
        })?;

        Ok(diagnostics.clone())
    }

    /// Get unresolved diagnostics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub const fn get_unresolved_diagnostics(&self) -> Result<Vec<Diagnostic>>  {
        let diagnostics = self.get_diagnostics()?;
        Ok(diagnostics
            .into_iter()
            .filter(|d| d.is_unresolved())
            .collect())
    }

    /// Calculate overall health status
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub const fn calculate_health_status(&self) -> Result<HealthStatus>  {
        let diagnostics = self.get_unresolved_diagnostics()?;

        if diagnostics.is_empty() {
            return Ok(HealthStatus::Healthy);
        }

        // Find the highest severity level
        let mut has_critical = false;
        let mut has_error = false;
        let mut has_warning = false;

        for diagnostic in &diagnostics {
            match diagnostic.level {
                DiagnosticLevel::Critical => has_critical = true,
                DiagnosticLevel::Error => has_error = true,
                DiagnosticLevel::Warning => has_warning = true,
                DiagnosticLevel::Info => {}
            }
        }

        if has_critical {
            Ok(HealthStatus::Critical)
        } else if has_error {
            Ok(HealthStatus::Error)
        } else if has_warning {
            Ok(HealthStatus::Warning)
        } else {
            Ok(HealthStatus::Healthy)
        }
    }

    /// Subscribe to diagnostic events
    #[must_use]
    pub const fn subscribe(&self) -> broadcast::Receiver<Diagnostic> {
        self.event_sender.subscribe()
    }

    /// Update system metrics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub const fn update_metrics(&self, _metrics: SystemMetrics) -> Result<()>  {
        // Implementation would update internal metrics storage
        // For now, this is a placeholder that accepts metrics
        Ok(())
    }

    /// Get current system metrics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub const fn get_metrics(&self) -> Result<SystemMetrics>  {
        let metrics = self.metrics.read().map_err(|_| {
            NestGateError::internal_error(
                "Failed to acquire metrics read lock",
                "diagnostics_manager",
            )
        })?;

        Ok(metrics.clone())
    }

    /// Clear all resolved diagnostics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn clear_resolved(&self) -> Result<usize>  {
        let mut diagnostics = self.diagnostics.write().map_err(|_| {
            NestGateError::internal_error(
                "Failed to acquire diagnostics write lock",
                "diagnostics_manager",
            )
        })?;

        let original_count = diagnostics.len();
        diagnostics.retain(|d| d.is_unresolved());
        let cleared_count = original_count - diagnostics.len();

        Ok(cleared_count)
    }
}

impl Default for DiagnosticsManager {
    fn default() -> Self {
        Self::new()
    }
}

// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::{Diagnostic, DiagnosticLevel, SystemMetrics};
use nestgate_types::Result;
use nestgate_types::unified_enums::UnifiedHealthStatus as HealthStatus;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};

/// Main diagnostics manager.
///
/// All lock operations are async-aware (`tokio::sync::RwLock`) so this type
/// is safe to use from Tokio task handlers without blocking worker threads.
pub struct DiagnosticsManager {
    diagnostics: Arc<RwLock<Vec<Diagnostic>>>,
    event_sender: broadcast::Sender<Diagnostic>,
    metrics: Arc<RwLock<SystemMetrics>>,
}

impl DiagnosticsManager {
    /// Create a new diagnostics manager.
    #[must_use]
    pub fn new() -> Self {
        let (event_sender, _) = broadcast::channel(1000);

        Self {
            diagnostics: Arc::new(RwLock::new(Vec::new())),
            event_sender,
            metrics: Arc::new(RwLock::new(SystemMetrics::default())),
        }
    }

    /// Add a diagnostic entry.
    ///
    /// # Errors
    ///
    /// Returns error if internal state is corrupted.
    pub async fn add_diagnostic(&self, diagnostic: Diagnostic) -> Result<()> {
        let mut diagnostics = self.diagnostics.write().await;
        diagnostics.push(diagnostic);
        Ok(())
    }

    /// Get all diagnostics.
    ///
    /// # Errors
    ///
    /// Returns error if internal state is corrupted.
    pub async fn get_diagnostics(&self) -> Result<Vec<Diagnostic>> {
        let diagnostics = self.diagnostics.read().await;
        Ok(diagnostics.clone())
    }

    /// Get unresolved diagnostics.
    ///
    /// # Errors
    ///
    /// Returns error if internal state is corrupted.
    pub async fn get_unresolved_diagnostics(&self) -> Result<Vec<Diagnostic>> {
        let diagnostics = self.get_diagnostics().await?;
        Ok(diagnostics
            .into_iter()
            .filter(super::diagnostic::Diagnostic::is_unresolved)
            .collect())
    }

    /// Calculate overall health status from unresolved diagnostics.
    ///
    /// # Errors
    ///
    /// Returns error if diagnostics cannot be read.
    pub async fn calculate_health_status(&self) -> Result<HealthStatus> {
        let diagnostics = self.get_unresolved_diagnostics().await?;

        if diagnostics.is_empty() {
            return Ok(HealthStatus::Healthy);
        }

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

    /// Subscribe to diagnostic events.
    #[must_use]
    pub fn subscribe(&self) -> broadcast::Receiver<Diagnostic> {
        self.event_sender.subscribe()
    }

    /// Update system metrics.
    ///
    /// # Errors
    ///
    /// Returns error if internal state is corrupted.
    pub async fn update_metrics(&self, metrics: SystemMetrics) -> Result<()> {
        let mut m = self.metrics.write().await;
        *m = metrics;
        Ok(())
    }

    /// Get current system metrics.
    ///
    /// # Errors
    ///
    /// Returns error if internal state is corrupted.
    pub async fn get_metrics(&self) -> Result<SystemMetrics> {
        let metrics = self.metrics.read().await;
        Ok(metrics.clone())
    }

    /// Clear all resolved diagnostics, returning how many were cleared.
    ///
    /// # Errors
    ///
    /// Returns error if internal state is corrupted.
    pub async fn clear_resolved(&self) -> Result<usize> {
        let mut diagnostics = self.diagnostics.write().await;
        let original_count = diagnostics.len();
        diagnostics.retain(super::diagnostic::Diagnostic::is_unresolved);
        let cleared_count = original_count - diagnostics.len();
        Ok(cleared_count)
    }
}

impl Default for DiagnosticsManager {
    fn default() -> Self {
        Self::new()
    }
}

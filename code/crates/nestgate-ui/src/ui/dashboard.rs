//! # NestGate Dashboard UI Component
//!
//! **System overview and metrics dashboard for NestGate storage management**
//!
//! This module provides the main dashboard interface showing system status,
//! storage pool health, performance metrics, and key operational indicators.
//!
//! ## Dashboard Features
//!
//! - **System Overview**: High-level status and health indicators
//! - **Storage Pools**: ZFS pool status and capacity visualization
//! - **Performance Metrics**: Real-time CPU, memory, disk I/O charts
//! - **Alert Summary**: Critical notifications and warnings
//! - **Quick Actions**: Common operations accessible from dashboard
//!
//! ## Real-Time Updates
//!
//! The dashboard continuously updates with:
//! - Live performance data every second
//! - Storage pool status monitoring
//! - Network activity and throughput
//! - System resource utilization
//!
//! ## Visual Components
//!
//! - Progress bars for capacity and health
//! - Time-series charts for performance trends
//! - Status indicators with color coding
//! - Interactive widgets for system control

use crate::types::*;
use eframe::egui::{self, RichText};
use std::time::Duration;

impl NestGateApp {
    pub fn render_enhanced_dashboard(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("📊 System Overview").size(16.0));
        ui.add_space(10.0);

        // Real-time system metrics cards
        ui.horizontal(|ui| {
            self.render_metric_card(
                ui,
                "Data Source",
                &format!("{:?}", self.system_status.mode),
                self.get_status_color(&self.system_status.mode),
                "🔄",
            );
            ui.add_space(10.0);
            self.render_metric_card(
                ui,
                "ZFS Status",
                if self.system_status.zfs_available {
                    "Available"
                } else {
                    "Not Available"
                },
                if self.system_status.zfs_available {
                    self.theme.success_color
                } else {
                    self.theme.error_color
                },
                "💾",
            );
            ui.add_space(10.0);
            self.render_metric_card(
                ui,
                "Pools Online",
                &self.system_status.pools_online.to_string(),
                self.theme.accent_color,
                "🗂️",
            );
            ui.add_space(10.0);
            self.render_metric_card(
                ui,
                "Total Capacity",
                &self.system_status.total_capacity,
                self.theme.accent_color,
                "📊",
            );
        });

        ui.add_space(15.0);

        // Performance overview with mini charts
        ui.horizontal(|ui| {
            self.render_performance_card(
                ui,
                "CPU Usage",
                self.system_status.cpu_usage,
                100.0,
                self.theme.accent_color,
                "🖥️",
            );
            ui.add_space(10.0);
            self.render_performance_card(
                ui,
                "Memory Usage",
                self.system_status.memory_usage,
                100.0,
                self.theme.warning_color,
                "🔧",
            );
            ui.add_space(10.0);
            self.render_performance_card(
                ui,
                "Network I/O",
                self.system_status.network_io,
                50.0,
                self.theme.success_color,
                "🌐",
            );
        });

        ui.add_space(20.0);

        // Enhanced tier overview with activity indicators
        ui.heading(RichText::new("🗂️ Storage Tiers Activity").size(14.0));
        ui.add_space(10.0);

        for (tier, stats) in &self.tier_stats {
            self.render_enhanced_tier_summary(ui, tier, stats);
            ui.add_space(8.0);
        }

        ui.add_space(15.0);

        // Quick actions
        ui.heading(RichText::new("⚡ Quick Actions").size(14.0));
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if ui
                .button(RichText::new("🤖 Run AI Optimization").size(12.0))
                .clicked()
            {
                self.add_notification(
                    "AI optimization started".to_string(),
                    NotificationLevel::Info,
                    Duration::from_secs(5),
                );
            }
            ui.add_space(8.0);
            if ui
                .button(RichText::new("📁 Browse Files").size(12.0))
                .clicked()
            {
                self.current_view = AppView::FileBrowser;
            }
            ui.add_space(8.0);
            if ui
                .button(RichText::new("📊 View Performance").size(12.0))
                .clicked()
            {
                self.current_view = AppView::Performance;
            }
        });
    }
}

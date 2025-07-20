pub mod browser;
pub mod components;
pub mod dashboard;
pub mod performance;
pub mod settings;
pub mod storage;

use crate::types::*;
use eframe::egui::{self, Color32, RichText, Stroke};
use nestgate_core::types::StorageTier;
use std::time::Duration;

impl NestGateApp {
    pub fn render_sidebar(&mut self, ui: &mut egui::Ui) {
        ui.heading(
            RichText::new("🏠 NestGate")
                .size(18.0)
                .color(Color32::WHITE),
        );
        ui.add_space(5.0);
        ui.label(
            RichText::new("Production ZFS Storage")
                .size(11.0)
                .color(Color32::LIGHT_GRAY),
        );
        ui.separator();

        let views = [
            (AppView::Dashboard, "📊", "Dashboard"),
            (AppView::TieredStorage, "🗂️", "Tiered Storage"),
            (AppView::ZfsManagement, "💾", "ZFS Pools"),
            (AppView::FileBrowser, "📁", "File Browser"),
            (AppView::AIOptimization, "🤖", "AI Optimization"),
            (AppView::Security, "🔒", "Security"),
            (AppView::Performance, "⚡", "Performance"),
            (AppView::Settings, "⚙️", "Settings"),
        ];

        for (view, icon, label) in views {
            let is_selected = self.current_view == view;
            let response = ui.selectable_label(
                is_selected,
                RichText::new(format!("{icon} {label}"))
                    .size(14.0)
                    .color(if is_selected {
                        self.theme.accent_color
                    } else {
                        Color32::WHITE
                    }),
            );

            if response.clicked() {
                self.current_view = view;
            }

            // Add hover effect
            if response.hovered() {
                ui.painter().rect_filled(
                    response.rect,
                    4.0,
                    Color32::from_rgba_premultiplied(255, 255, 255, 20),
                );
            }
        }

        ui.separator();
        ui.add_space(10.0);

        // Enhanced system status footer
        self.render_sidebar_status(ui);
    }

    fn render_sidebar_status(&self, ui: &mut egui::Ui) {
        ui.label(
            RichText::new("System Status")
                .size(12.0)
                .color(Color32::GRAY),
        );
        ui.add_space(5.0);

        let status_color = match self.system_status.mode {
            DataSource::Live => self.theme.success_color,
            DataSource::Mock => self.theme.warning_color,
            DataSource::FallbackMock => self.theme.error_color,
        };

        ui.label(
            RichText::new(format!("🟢 Mode: {:?}", self.system_status.mode))
                .size(10.0)
                .color(status_color),
        );
        ui.label(
            RichText::new(format!("💾 Pools: {}", self.system_status.pools_online))
                .size(10.0)
                .color(Color32::LIGHT_GRAY),
        );
        ui.label(
            RichText::new(format!(
                "📊 Capacity: {}",
                self.system_status.total_capacity
            ))
            .size(10.0)
            .color(Color32::LIGHT_GRAY),
        );
        ui.label(
            RichText::new(format!("🖥️ CPU: {:.1}%", self.system_status.cpu_usage))
                .size(10.0)
                .color(Color32::LIGHT_GRAY),
        );
        ui.label(
            RichText::new(format!("🔧 RAM: {:.1}%", self.system_status.memory_usage))
                .size(10.0)
                .color(Color32::LIGHT_GRAY),
        );

        if self.system_status.ai_ml_ready {
            ui.label(
                RichText::new("🤖 AI/ML: Ready")
                    .size(10.0)
                    .color(self.theme.success_color),
            );
        }

        // Uptime display
        let uptime_days =
            self.system_status.uptime.as_secs() / nestgate_core::constants::time::DAY.as_secs();
        ui.label(
            RichText::new(format!("⏱️ Uptime: {uptime_days}d"))
                .size(10.0)
                .color(Color32::LIGHT_GRAY),
        );
    }

    pub fn render_content(&mut self, ui: &mut egui::Ui) {
        // Enhanced header with animations
        self.render_header(ui);
        ui.separator();
        ui.add_space(10.0);

        // Render notifications
        self.render_notifications(ui);

        match self.current_view {
            AppView::Dashboard => self.render_enhanced_dashboard(ui),
            AppView::TieredStorage => self.render_enhanced_tiered_storage(ui),
            AppView::ZfsManagement => self.render_enhanced_zfs_management(ui),
            AppView::FileBrowser => self.render_file_browser(ui),
            AppView::AIOptimization => self.render_ai_optimization(ui),
            AppView::Security => self.render_security(ui),
            AppView::Performance => self.render_enhanced_performance(ui),
            AppView::Settings => self.render_settings(ui),
        }
    }

    fn render_notifications(&mut self, ui: &mut egui::Ui) {
        let now = std::time::Instant::now();

        // Remove expired notifications
        self.notifications
            .retain(|n| now.duration_since(n.timestamp) < n.duration);

        if !self.notifications.is_empty() {
            ui.add_space(5.0);
            for notification in &self.notifications {
                let color = match notification.level {
                    NotificationLevel::Info => self.theme.accent_color,
                    NotificationLevel::Success => self.theme.success_color,
                    NotificationLevel::Warning => self.theme.warning_color,
                    NotificationLevel::Error => self.theme.error_color,
                };

                egui::Frame::none()
                    .fill(Color32::from_rgba_premultiplied(
                        color.r(),
                        color.g(),
                        color.b(),
                        30,
                    ))
                    .stroke(Stroke::new(1.0, color))
                    .rounding(6.0)
                    .inner_margin(egui::style::Margin::same(8.0))
                    .show(ui, |ui| {
                        ui.label(RichText::new(&notification.message).size(12.0).color(color));
                    });
            }
            ui.add_space(10.0);
        }
    }

    fn render_header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Animated title with pulse effect
            let pulse_color = Color32::from_rgba_premultiplied(
                self.theme.accent_color.r(),
                self.theme.accent_color.g(),
                self.theme.accent_color.b(),
                (128.0 + 64.0 * (self.animations.pulse_time * 2.0).sin()) as u8,
            );

            ui.heading(
                RichText::new("NestGate Production Storage")
                    .size(20.0)
                    .color(pulse_color),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Enhanced status badges with animations
                let badges = [
                    ("🤖 AI/ML Ready", self.theme.success_color),
                    ("🔒 Production Security", self.theme.success_color),
                    ("✅ 100% Compilation", self.theme.success_color),
                ];

                for (text, color) in badges {
                    egui::Frame::none()
                        .fill(Color32::from_rgba_premultiplied(
                            color.r(),
                            color.g(),
                            color.b(),
                            20,
                        ))
                        .stroke(Stroke::new(1.0, color))
                        .rounding(12.0)
                        .inner_margin(egui::style::Margin::symmetric(8.0, 4.0))
                        .show(ui, |ui| {
                            ui.label(RichText::new(text).size(11.0).color(color));
                        });
                    ui.add_space(5.0);
                }
            });
        });
    }

    pub fn get_tier_color(&self, tier: &StorageTier) -> Color32 {
        match tier {
            StorageTier::Hot => Color32::from_rgb(245, 34, 45), // Red for hot
            StorageTier::Warm => Color32::from_rgb(250, 140, 22), // Orange for warm
            StorageTier::Cold => Color32::from_rgb(24, 144, 255), // Blue for cold
            StorageTier::Cache => Color32::from_rgb(138, 43, 226), // Purple for cache
        }
    }

    pub fn get_status_color(&self, mode: &DataSource) -> Color32 {
        match mode {
            DataSource::Live => self.theme.success_color,
            DataSource::Mock => self.theme.warning_color,
            DataSource::FallbackMock => self.theme.error_color,
        }
    }

    pub fn get_health_color(&self, health: &TierHealth) -> Color32 {
        match health {
            TierHealth::Optimal => self.theme.success_color,
            TierHealth::Good => self.theme.success_color,
            TierHealth::Warning => self.theme.warning_color,
            TierHealth::Critical => self.theme.error_color,
        }
    }

    pub fn add_notification(
        &mut self,
        message: String,
        level: NotificationLevel,
        duration: Duration,
    ) {
        self.notifications.push_back(Notification {
            message,
            level,
            timestamp: std::time::Instant::now(),
            duration,
        });
    }
}

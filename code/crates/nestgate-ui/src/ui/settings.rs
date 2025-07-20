use crate::types::*;
use eframe::egui::{self, RichText};
use std::time::Duration;

impl NestGateApp {
    pub fn render_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("⚙️ System Configuration").size(16.0));
        ui.add_space(10.0);

        ui.label("Pure Rust local-only configuration");
        ui.add_space(15.0);

        // Theme selection
        ui.horizontal(|ui| {
            ui.label("Theme:");
            if ui.button("🌙 Dark").clicked() {
                self.add_notification(
                    "Dark theme applied".to_string(),
                    NotificationLevel::Info,
                    Duration::from_secs(3),
                );
            }
            if ui.button("☀️ Light").clicked() {
                self.add_notification(
                    "Light theme applied".to_string(),
                    NotificationLevel::Info,
                    Duration::from_secs(3),
                );
            }
        });

        ui.add_space(10.0);

        // System settings
        ui.label("System Settings:");
        ui.label("• Remote access: Use universal orchestration module");
        ui.label("• Security: Universal security module integration available");
        ui.label("• Mode: Pure Rust native with zero web dependencies");

        ui.add_space(15.0);

        // Advanced settings
        ui.collapsing("Advanced Settings", |ui| {
            ui.checkbox(
                &mut self.file_browser.show_hidden,
                "Show hidden files by default",
            );
            ui.horizontal(|ui| {
                ui.label("Update interval:");
                ui.label("1 second");
            });
            ui.horizontal(|ui| {
                ui.label("Chart history:");
                ui.label("60 points");
            });
        });
    }
}

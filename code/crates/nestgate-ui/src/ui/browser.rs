use crate::types::*;
use eframe::egui::{self, Color32, RichText, Stroke};
use std::time::Duration;

impl NestGateApp {
    pub fn render_file_browser(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("📁 File Browser").size(16.0));
        ui.add_space(10.0);

        // Path navigation
        ui.horizontal(|ui| {
            ui.label("Path:");
            ui.text_edit_singleline(&mut self.file_browser.current_path);
            if ui.button("📂 Browse").clicked() {
                self.add_notification(
                    "Path updated".to_string(),
                    NotificationLevel::Info,
                    Duration::from_secs(3),
                );
            }
        });

        ui.add_space(10.0);

        // File browser controls
        ui.horizontal(|ui| {
            ui.label("View:");
            ui.selectable_value(&mut self.file_browser.view_mode, ViewMode::List, "📋 List");
            ui.selectable_value(&mut self.file_browser.view_mode, ViewMode::Grid, "🗂️ Grid");
            ui.selectable_value(
                &mut self.file_browser.view_mode,
                ViewMode::Details,
                "📊 Details",
            );

            ui.separator();

            ui.label("Sort:");
            ui.selectable_value(&mut self.file_browser.sort_by, SortBy::Name, "📝 Name");
            ui.selectable_value(&mut self.file_browser.sort_by, SortBy::Size, "📏 Size");
            ui.selectable_value(
                &mut self.file_browser.sort_by,
                SortBy::Modified,
                "📅 Modified",
            );
            ui.selectable_value(&mut self.file_browser.sort_by, SortBy::Type, "🏷️ Type");

            ui.separator();

            ui.checkbox(&mut self.file_browser.show_hidden, "👁️ Show Hidden");
        });

        ui.add_space(15.0);

        // File listing placeholder
        egui::Frame::none()
            .fill(self.theme.card_background)
            .stroke(Stroke::new(1.0, Color32::from_rgb(220, 220, 220)))
            .rounding(8.0)
            .inner_margin(egui::style::Margin::same(15.0))
            .show(ui, |ui| {
                ui.set_min_height(300.0);
                ui.vertical_centered(|ui| {
                    ui.add_space(50.0);
                    ui.heading("🗂️ File Browser");
                    ui.add_space(10.0);
                    ui.label("File browser integration with ZFS datasets");
                    ui.add_space(10.0);
                    ui.label(format!("Current path: {}", self.file_browser.current_path));
                    ui.add_space(20.0);
                    if ui.button("🔄 Refresh Directory").clicked() {
                        self.add_notification(
                            "Directory refreshed".to_string(),
                            NotificationLevel::Info,
                            Duration::from_secs(3),
                        );
                    }
                });
            });
    }
}

use crate::types::*;
use eframe::egui::{self, Color32, RichText, Stroke, Vec2};
use nestgate_core::types::StorageTier;

impl NestGateApp {
    pub fn render_metric_card(
        &self,
        ui: &mut egui::Ui,
        title: &str,
        value: &str,
        color: Color32,
        icon: &str,
    ) {
        egui::Frame::none()
            .fill(self.theme.card_background)
            .stroke(Stroke::new(1.5, Color32::from_rgb(240, 240, 240)))
            .rounding(8.0)
            .inner_margin(egui::style::Margin::same(15.0))
            .show(ui, |ui| {
                ui.set_min_width(140.0);
                ui.vertical_centered(|ui| {
                    ui.label(
                        RichText::new(format!("{icon} {title}"))
                            .size(11.0)
                            .color(self.theme.muted_text),
                    );
                    ui.add_space(6.0);
                    ui.label(RichText::new(value).size(16.0).color(color).strong());
                });
            });
    }

    pub fn render_performance_card(
        &self,
        ui: &mut egui::Ui,
        title: &str,
        value: f32,
        max_value: f32,
        color: Color32,
        icon: &str,
    ) {
        egui::Frame::none()
            .fill(self.theme.card_background)
            .stroke(Stroke::new(1.5, Color32::from_rgb(240, 240, 240)))
            .rounding(8.0)
            .inner_margin(egui::style::Margin::same(15.0))
            .show(ui, |ui| {
                ui.set_min_width(140.0);
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("{icon} {title}"))
                                .size(11.0)
                                .color(self.theme.muted_text),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new(format!("{value:.1}%"))
                                    .size(14.0)
                                    .color(color)
                                    .strong(),
                            );
                        });
                    });

                    ui.add_space(6.0);

                    // Progress bar with gradient effect
                    let progress = value / max_value;
                    let progress_bar = egui::ProgressBar::new(progress).fill(color).animate(true);
                    ui.add(progress_bar);
                });
            });
    }

    pub fn render_enhanced_tier_summary(
        &self,
        ui: &mut egui::Ui,
        tier: &StorageTier,
        stats: &TierStats,
    ) {
        let tier_color = self.get_tier_color(tier);
        let activity = self
            .tier_activity
            .get(tier)
            .map(|a| a.back().unwrap_or(&0.0))
            .unwrap_or(&0.0);

        egui::Frame::none()
            .fill(self.theme.card_background)
            .stroke(Stroke::new(2.0, tier_color))
            .rounding(10.0)
            .inner_margin(egui::style::Margin::same(15.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Tier info
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new(&stats.name)
                                .size(14.0)
                                .color(tier_color)
                                .strong(),
                        );
                        ui.add_space(4.0);

                        // Usage bar with better styling
                        let usage_percent =
                            (stats.used_space as f32 / stats.total_space as f32) * 100.0;
                        let progress_bar = egui::ProgressBar::new(usage_percent / 100.0)
                            .text(format!("{usage_percent:.1}% used"))
                            .fill(tier_color)
                            .animate(true);
                        ui.add_sized([200.0, 12.0], progress_bar);

                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(format!("{} files", stats.file_count))
                                    .size(10.0)
                                    .color(self.theme.muted_text),
                            );
                            ui.label(
                                RichText::new(&stats.compression)
                                    .size(10.0)
                                    .color(self.theme.muted_text),
                            );
                        });
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Activity indicator
                        ui.vertical(|ui| {
                            let health_color = self.get_health_color(&stats.health);
                            ui.label(
                                RichText::new(format!("{:?}", stats.health))
                                    .size(12.0)
                                    .color(health_color),
                            );
                            ui.label(
                                RichText::new(&stats.performance)
                                    .size(10.0)
                                    .color(self.theme.muted_text),
                            );

                            // Temperature indicator
                            let temp_color = Color32::from_rgb(
                                (255.0 * stats.temperature) as u8,
                                (255.0 * (1.0 - stats.temperature)) as u8,
                                50,
                            );
                            ui.label(
                                RichText::new(format!("🌡️ {:.1}°", stats.temperature * 100.0))
                                    .size(10.0)
                                    .color(temp_color),
                            );

                            // Activity pulse
                            let pulse_size = 6.0 + activity * 4.0;
                            let (rect, _response) = ui
                                .allocate_exact_size(Vec2::splat(pulse_size), egui::Sense::hover());
                            let activity_color = Color32::from_rgba_premultiplied(
                                tier_color.r(),
                                tier_color.g(),
                                tier_color.b(),
                                (activity * 255.0) as u8,
                            );
                            ui.painter().circle_filled(
                                rect.center(),
                                pulse_size / 2.0,
                                activity_color,
                            );
                        });
                    });
                });
            });
    }

    pub fn render_detailed_tier_panel(
        &self,
        ui: &mut egui::Ui,
        tier: &StorageTier,
        stats: &TierStats,
    ) {
        let tier_color = self.get_tier_color(tier);

        egui::Frame::none()
            .fill(self.theme.card_background)
            .stroke(Stroke::new(1.0, tier_color))
            .rounding(10.0)
            .inner_margin(egui::style::Margin::same(18.0))
            .show(ui, |ui| {
                ui.heading(RichText::new(&stats.name).size(15.0).color(tier_color));
                ui.add_space(12.0);

                ui.columns(4, |columns| {
                    // Storage Usage
                    columns[0].label(RichText::new("Storage Usage").strong());
                    columns[0].add_space(4.0);
                    let usage_gb = stats.used_space as f64 / (1024.0 * 1024.0 * 1024.0);
                    let total_gb = stats.total_space as f64 / (1024.0 * 1024.0 * 1024.0);
                    columns[0].label(format!("{usage_gb:.1} GB / {total_gb:.1} GB"));
                    let usage_percent =
                        (stats.used_space as f32 / stats.total_space as f32) * 100.0;
                    let progress = egui::ProgressBar::new(usage_percent / 100.0).fill(tier_color);
                    columns[0].add(progress);

                    // Performance Metrics
                    columns[1].label(RichText::new("Performance").strong());
                    columns[1].add_space(4.0);
                    columns[1].label(&stats.performance);
                    columns[1].label(format!("{:.0} MB/s I/O", stats.io_rate));

                    // Activity Stats
                    columns[2].label(RichText::new("Activity").strong());
                    columns[2].add_space(4.0);
                    columns[2].label(format!("{:.0} access/hour", stats.access_frequency));
                    columns[2].label(format!("Temp: {:.0}%", stats.temperature * 100.0));

                    // Compression & Health
                    columns[3].label(RichText::new("Status").strong());
                    columns[3].add_space(4.0);
                    columns[3].label(&stats.compression);
                    let health_color = self.get_health_color(&stats.health);
                    columns[3]
                        .label(RichText::new(format!("{:?}", stats.health)).color(health_color));
                });
            });
    }
}

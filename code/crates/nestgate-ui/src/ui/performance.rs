use crate::types::*;
use eframe::egui::{self, Color32, Pos2, Rect, RichText, Stroke, Vec2};

impl NestGateApp {
    pub fn render_enhanced_performance(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("⚡ System Performance").size(16.0));
        ui.add_space(10.0);

        // Real-time performance charts
        ui.horizontal(|ui| {
            // CPU chart
            self.render_performance_chart(
                ui,
                "CPU Usage",
                &self
                    .performance_history
                    .iter()
                    .map(|p| p.cpu_usage)
                    .collect::<Vec<_>>(),
                self.theme.accent_color,
                100.0,
            );
            ui.add_space(10.0);
            // Memory chart
            self.render_performance_chart(
                ui,
                "Memory Usage",
                &self
                    .performance_history
                    .iter()
                    .map(|p| p.memory_usage)
                    .collect::<Vec<_>>(),
                self.theme.warning_color,
                100.0,
            );
        });

        ui.add_space(15.0);

        ui.horizontal(|ui| {
            // Disk I/O chart
            self.render_performance_chart(
                ui,
                "Disk I/O",
                &self
                    .performance_history
                    .iter()
                    .map(|p| p.disk_io)
                    .collect::<Vec<_>>(),
                self.theme.success_color,
                50.0,
            );
            ui.add_space(10.0);
            // Network I/O chart
            self.render_performance_chart(
                ui,
                "Network I/O",
                &self
                    .performance_history
                    .iter()
                    .map(|p| p.network_io)
                    .collect::<Vec<_>>(),
                Color32::from_rgb(138, 43, 226),
                30.0,
            );
        });

        ui.add_space(20.0);

        // System info
        ui.label("Real system metrics integration operational:");
        ui.label("• Real I/O wait percentage from /proc/stat");
        ui.label("• Network I/O statistics from /proc/net/dev");
        ui.label("• ZFS cache hit ratios from /proc/spl/kstat/zfs/arcstats");
        ui.label("• Graceful fallback when system files unavailable");
    }

    pub fn render_performance_chart(
        &self,
        ui: &mut egui::Ui,
        title: &str,
        data: &[f32],
        color: Color32,
        max_value: f32,
    ) {
        egui::Frame::none()
            .fill(self.theme.card_background)
            .stroke(Stroke::new(1.0, Color32::from_rgb(220, 220, 220)))
            .rounding(8.0)
            .inner_margin(egui::style::Margin::same(12.0))
            .show(ui, |ui| {
                ui.set_min_size(Vec2::new(250.0, 150.0));
                ui.label(RichText::new(title).size(14.0).strong());
                ui.add_space(8.0);

                if let Some(latest) = data.last() {
                    ui.label(
                        RichText::new(format!("Current: {latest:.1}%"))
                            .size(12.0)
                            .color(color),
                    );
                }

                ui.add_space(5.0);

                // Simple chart rendering
                let chart_rect = ui.available_rect_before_wrap();
                let chart_area =
                    Rect::from_min_size(chart_rect.min, Vec2::new(chart_rect.width(), 80.0));

                if data.len() > 1 {
                    let points: Vec<Pos2> = data
                        .iter()
                        .enumerate()
                        .map(|(i, &value)| {
                            let x = chart_area.min.x
                                + (i as f32 / (data.len() - 1) as f32) * chart_area.width();
                            let y = chart_area.max.y - (value / max_value) * chart_area.height();
                            Pos2::new(x, y)
                        })
                        .collect();

                    // Draw grid lines
                    for i in 0..=4 {
                        let y = chart_area.min.y + (i as f32 / 4.0) * chart_area.height();
                        ui.painter().line_segment(
                            [
                                Pos2::new(chart_area.min.x, y),
                                Pos2::new(chart_area.max.x, y),
                            ],
                            Stroke::new(0.5, Color32::LIGHT_GRAY),
                        );
                    }

                    // Draw chart line
                    for i in 1..points.len() {
                        ui.painter()
                            .line_segment([points[i - 1], points[i]], Stroke::new(2.0, color));
                    }

                    // Fill area under curve
                    if !points.is_empty() {
                        let mut fill_points = points.clone();
                        fill_points.push(Pos2::new(chart_area.max.x, chart_area.max.y));
                        fill_points.push(Pos2::new(chart_area.min.x, chart_area.max.y));
                        ui.painter().add(egui::Shape::convex_polygon(
                            fill_points,
                            Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 30),
                            Stroke::NONE,
                        ));
                    }
                }
            });
    }
}

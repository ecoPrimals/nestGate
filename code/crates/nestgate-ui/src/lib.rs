use eframe::egui;
use nestgate_core::Result;
use tracing::info;

/// Native Rust UI for NestGate using egui
pub struct NestGateApp {
    current_view: AppView,
    system_status: SystemStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppView {
    Dashboard,
    Storage,
    Performance,
    Settings,
}

#[derive(Debug, Clone)]
pub struct SystemStatus {
    pub pools_online: usize,
    pub total_capacity: String,
    pub health_status: String,
}

impl Default for NestGateApp {
    fn default() -> Self {
        Self {
            current_view: AppView::Dashboard,
            system_status: SystemStatus {
                pools_online: 1,
                total_capacity: "1TB".to_string(),
                health_status: "Healthy".to_string(),
            },
        }
    }
}

impl NestGateApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        info!("NestGate native UI initialized");
        Self::default()
    }

    fn render_sidebar(&mut self, ui: &mut egui::Ui) {
        ui.heading("🏠 NestGate");
        ui.separator();

        let views = [
            (AppView::Dashboard, "📊 Dashboard"),
            (AppView::Storage, "💾 Storage"),
            (AppView::Performance, "⚡ Performance"),
            (AppView::Settings, "⚙️ Settings"),
        ];

        for (view, label) in views {
            if ui.selectable_label(self.current_view == view, label).clicked() {
                self.current_view = view;
            }
        }
    }

    fn render_content(&mut self, ui: &mut egui::Ui) {
        match self.current_view {
            AppView::Dashboard => {
                ui.heading("📊 System Dashboard");
                ui.separator();
                ui.label("Welcome to NestGate - Pure Rust Storage Management");
                ui.label(format!("Pools Online: {}", self.system_status.pools_online));
                ui.label(format!("Total Capacity: {}", self.system_status.total_capacity));
                ui.label(format!("Health: {}", self.system_status.health_status));
            }
            AppView::Storage => {
                ui.heading("�� Storage Management");
                ui.separator();
                ui.label("ZFS pool management interface");
            }
            AppView::Performance => {
                ui.heading("⚡ Performance Analytics");
                ui.separator();
                ui.label("AI-powered performance optimization");
            }
            AppView::Settings => {
                ui.heading("⚙️ System Settings");
                ui.separator();
                ui.label("Configuration management");
            }
        }
    }
}

impl eframe::App for NestGateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("sidebar")
            .default_width(200.0)
            .show(ctx, |ui| {
                self.render_sidebar(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_content(ui);
        });
    }
}

/// Run the native NestGate application
pub fn run_app() -> Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "NestGate - Pure Rust Storage Management",
        options,
        Box::new(|cc| Box::new(NestGateApp::new(cc))),
    ).map_err(|e| nestgate_core::NestGateError::Storage(format!("Failed to run UI: {}", e)))
}

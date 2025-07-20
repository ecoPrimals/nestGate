use eframe::egui::Color32;
use nestgate_core::types::StorageTier;
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub enum AppView {
    Dashboard,
    TieredStorage,
    ZfsManagement,
    AIOptimization,
    Security,
    Performance,
    Settings,
    FileBrowser,
}

#[derive(Debug, Clone)]
pub struct SystemStatus {
    pub mode: DataSource,
    pub pools_online: usize,
    pub total_capacity: String,
    pub health_status: String,
    pub zfs_available: bool,
    pub compilation_status: String,
    pub security_level: String,
    pub ai_ml_ready: bool,
    pub uptime: Duration,
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub network_io: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataSource {
    Live,
    Mock,
    FallbackMock,
}

#[derive(Debug, Clone)]
pub struct TierStats {
    pub name: String,
    pub used_space: u64,
    pub total_space: u64,
    pub file_count: u32,
    pub performance: String,
    pub compression: String,
    pub health: TierHealth,
    pub io_rate: f32,          // MB/s
    pub access_frequency: f32, // Accesses per hour
    pub temperature: f32,      // Normalized 0-1 hotness
}

#[derive(Debug, Clone, PartialEq)]
pub enum TierHealth {
    Optimal,
    Good,
    Warning,
    Critical,
}

#[derive(Debug, Clone)]
pub struct PerformancePoint {
    pub timestamp: Instant,
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub disk_io: f32,
    pub network_io: f32,
}

#[derive(Debug, Clone)]
pub struct AnimationState {
    pub pulse_time: f32,
    pub loading_dots: usize,
    pub tier_hover: Option<StorageTier>,
}

#[derive(Debug, Clone)]
pub struct FileBrowserState {
    pub current_path: String,
    pub selected_files: Vec<String>,
    pub show_hidden: bool,
    pub sort_by: SortBy,
    pub view_mode: ViewMode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SortBy {
    Name,
    Size,
    Modified,
    Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ViewMode {
    List,
    Grid,
    Details,
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub message: String,
    pub level: NotificationLevel,
    pub timestamp: Instant,
    pub duration: Duration,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NotificationLevel {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct UITheme {
    pub accent_color: Color32,
    pub success_color: Color32,
    pub warning_color: Color32,
    pub error_color: Color32,
    pub background_color: Color32,
    pub card_background: Color32,
    pub text_color: Color32,
    pub muted_text: Color32,
}

impl Default for UITheme {
    fn default() -> Self {
        Self {
            accent_color: Color32::from_rgb(24, 144, 255),
            success_color: Color32::from_rgb(82, 196, 26),
            warning_color: Color32::from_rgb(250, 140, 22),
            error_color: Color32::from_rgb(245, 34, 45),
            background_color: Color32::from_rgb(248, 250, 252),
            card_background: Color32::WHITE,
            text_color: Color32::from_rgb(20, 20, 20),
            muted_text: Color32::from_rgb(128, 128, 128),
        }
    }
}

/// Main NestGate application structure
pub struct NestGateApp {
    pub current_view: AppView,
    pub system_status: SystemStatus,
    pub tier_stats: HashMap<StorageTier, TierStats>,
    pub _real_data_available: bool,
    pub last_update: std::time::Instant,
    pub performance_history: VecDeque<PerformancePoint>,
    pub tier_activity: HashMap<StorageTier, VecDeque<f32>>,
    pub animations: AnimationState,
    pub file_browser: FileBrowserState,
    pub notifications: VecDeque<Notification>,
    pub theme: UITheme,
}

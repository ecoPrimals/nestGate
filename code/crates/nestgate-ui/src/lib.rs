use eframe::egui;
use nestgate_core::error::{RecoveryStrategy, SystemResource};
use nestgate_core::Result;

pub mod app;
pub mod types;
pub mod ui;

pub use types::*;

/// Run the enhanced NestGate application
pub fn run_app() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([1200.0, 800.0])
            .with_title("NestGate v2.0 - Production ZFS Storage Management"),
        ..Default::default()
    };

    eframe::run_native(
        "NestGate - Production ZFS Storage Management",
        options,
        Box::new(|cc| Box::new(NestGateApp::new(cc))),
    )
    .map_err(|e| nestgate_core::NestGateError::System {
        message: format!("Failed to run enhanced UI: {e}"),
        resource: SystemResource::Memory,
        utilization: Some(85.0), // Low memory utilization
        recovery: RecoveryStrategy::Retry,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use eframe::egui::Color32;
    use nestgate_core::types::StorageTier;
    use std::collections::VecDeque;
    use std::time::{Duration, Instant};

    #[test]
    fn test_app_view_variants() {
        // Test all AppView variants
        let views = [
            AppView::Dashboard,
            AppView::TieredStorage,
            AppView::ZfsManagement,
            AppView::AIOptimization,
            AppView::Security,
            AppView::Performance,
            AppView::Settings,
            AppView::FileBrowser,
        ];

        assert_eq!(views.len(), 8);

        // Test equality and cloning
        assert_eq!(AppView::Dashboard, AppView::Dashboard);
        assert_ne!(AppView::Dashboard, AppView::TieredStorage);

        let cloned_view = AppView::Dashboard.clone();
        assert_eq!(cloned_view, AppView::Dashboard);
    }

    #[test]
    fn test_data_source_variants() {
        // Test all DataSource variants
        let sources = [DataSource::Live, DataSource::Mock, DataSource::FallbackMock];

        assert_eq!(sources.len(), 3);

        // Test equality and cloning
        assert_eq!(DataSource::Live, DataSource::Live);
        assert_ne!(DataSource::Live, DataSource::Mock);

        let cloned_source = DataSource::Live.clone();
        assert_eq!(cloned_source, DataSource::Live);
    }

    #[test]
    fn test_tier_health_variants() {
        // Test all TierHealth variants
        let health_states = [
            TierHealth::Optimal,
            TierHealth::Good,
            TierHealth::Warning,
            TierHealth::Critical,
        ];

        assert_eq!(health_states.len(), 4);

        // Test equality and cloning
        assert_eq!(TierHealth::Optimal, TierHealth::Optimal);
        assert_ne!(TierHealth::Optimal, TierHealth::Warning);

        let cloned_health = TierHealth::Good.clone();
        assert_eq!(cloned_health, TierHealth::Good);
    }

    #[test]
    fn test_sort_by_variants() {
        // Test all SortBy variants
        let sort_options = [SortBy::Name, SortBy::Size, SortBy::Modified, SortBy::Type];

        assert_eq!(sort_options.len(), 4);

        // Test equality and cloning
        assert_eq!(SortBy::Name, SortBy::Name);
        assert_ne!(SortBy::Name, SortBy::Size);

        let cloned_sort = SortBy::Modified.clone();
        assert_eq!(cloned_sort, SortBy::Modified);
    }

    #[test]
    fn test_view_mode_variants() {
        // Test all ViewMode variants
        let view_modes = [ViewMode::List, ViewMode::Grid, ViewMode::Details];

        assert_eq!(view_modes.len(), 3);

        // Test equality and cloning
        assert_eq!(ViewMode::List, ViewMode::List);
        assert_ne!(ViewMode::List, ViewMode::Grid);

        let cloned_mode = ViewMode::Details.clone();
        assert_eq!(cloned_mode, ViewMode::Details);
    }

    #[test]
    fn test_notification_level_variants() {
        // Test all NotificationLevel variants
        let levels = [
            NotificationLevel::Info,
            NotificationLevel::Success,
            NotificationLevel::Warning,
            NotificationLevel::Error,
        ];

        assert_eq!(levels.len(), 4);

        // Test equality and cloning
        assert_eq!(NotificationLevel::Info, NotificationLevel::Info);
        assert_ne!(NotificationLevel::Info, NotificationLevel::Error);

        let cloned_level = NotificationLevel::Warning.clone();
        assert_eq!(cloned_level, NotificationLevel::Warning);
    }

    #[test]
    fn test_ui_theme_default() {
        let theme = UITheme::default();

        // Test that default colors are set correctly
        assert_eq!(theme.accent_color, Color32::from_rgb(24, 144, 255));
        assert_eq!(theme.success_color, Color32::from_rgb(82, 196, 26));
        assert_eq!(theme.warning_color, Color32::from_rgb(250, 140, 22));
        assert_eq!(theme.error_color, Color32::from_rgb(245, 34, 45));
        assert_eq!(theme.background_color, Color32::from_rgb(248, 250, 252));
        assert_eq!(theme.card_background, Color32::WHITE);
        assert_eq!(theme.text_color, Color32::from_rgb(20, 20, 20));
        assert_eq!(theme.muted_text, Color32::from_rgb(128, 128, 128));
    }

    #[test]
    fn test_ui_theme_creation() {
        let custom_theme = UITheme {
            accent_color: Color32::RED,
            success_color: Color32::GREEN,
            warning_color: Color32::YELLOW,
            error_color: Color32::DARK_RED,
            background_color: Color32::DARK_GRAY,
            card_background: Color32::GRAY,
            text_color: Color32::WHITE,
            muted_text: Color32::LIGHT_GRAY,
        };

        assert_eq!(custom_theme.accent_color, Color32::RED);
        assert_eq!(custom_theme.success_color, Color32::GREEN);
        assert_eq!(custom_theme.warning_color, Color32::YELLOW);
        assert_eq!(custom_theme.error_color, Color32::DARK_RED);
        assert_eq!(custom_theme.background_color, Color32::DARK_GRAY);
        assert_eq!(custom_theme.card_background, Color32::GRAY);
        assert_eq!(custom_theme.text_color, Color32::WHITE);
        assert_eq!(custom_theme.muted_text, Color32::LIGHT_GRAY);
    }

    #[test]
    fn test_system_status_creation() {
        let status = SystemStatus {
            mode: DataSource::Live,
            pools_online: 3,
            total_capacity: "2.5TB".to_string(),
            health_status: "Healthy".to_string(),
            zfs_available: true,
            compilation_status: "Ready".to_string(),
            security_level: "High".to_string(),
            ai_ml_ready: true,
            uptime: nestgate_core::constants::time::DAY, // 1 day
            cpu_usage: 25.5,
            memory_usage: 60.2,
            network_io: 12.8,
        };

        assert_eq!(status.mode, DataSource::Live);
        assert_eq!(status.pools_online, 3);
        assert_eq!(status.total_capacity, "2.5TB");
        assert_eq!(status.health_status, "Healthy");
        assert!(status.zfs_available);
        assert_eq!(status.compilation_status, "Ready");
        assert_eq!(status.security_level, "High");
        assert!(status.ai_ml_ready);
        assert_eq!(status.uptime, nestgate_core::constants::time::DAY);
        assert_eq!(status.cpu_usage, 25.5);
        assert_eq!(status.memory_usage, 60.2);
        assert_eq!(status.network_io, 12.8);
    }

    #[test]
    fn test_tier_stats_creation() {
        let stats = TierStats {
            name: "Hot Tier".to_string(),
            used_space: 1024 * 1024 * 1024,       // 1GB
            total_space: 10 * 1024 * 1024 * 1024, // 10GB
            file_count: 1500,
            performance: "fast".to_string(),
            compression: "lz4".to_string(),
            health: TierHealth::Optimal,
            io_rate: 500.0,
            access_frequency: 120.0,
            temperature: 0.8,
        };

        assert_eq!(stats.name, "Hot Tier");
        assert_eq!(stats.used_space, 1024 * 1024 * 1024);
        assert_eq!(stats.total_space, 10 * 1024 * 1024 * 1024);
        assert_eq!(stats.file_count, 1500);
        assert_eq!(stats.performance, "fast");
        assert_eq!(stats.compression, "lz4");
        assert_eq!(stats.health, TierHealth::Optimal);
        assert_eq!(stats.io_rate, 500.0);
        assert_eq!(stats.access_frequency, 120.0);
        assert_eq!(stats.temperature, 0.8);
    }

    #[test]
    fn test_performance_point_creation() {
        let now = Instant::now();
        let point = PerformancePoint {
            timestamp: now,
            cpu_usage: 45.2,
            memory_usage: 72.8,
            disk_io: 15.6,
            network_io: 8.3,
        };

        assert_eq!(point.timestamp, now);
        assert_eq!(point.cpu_usage, 45.2);
        assert_eq!(point.memory_usage, 72.8);
        assert_eq!(point.disk_io, 15.6);
        assert_eq!(point.network_io, 8.3);
    }

    #[test]
    fn test_animation_state_creation() {
        let animation = AnimationState {
            pulse_time: 1.5,
            loading_dots: 3,
            tier_hover: Some(StorageTier::Hot),
        };

        assert_eq!(animation.pulse_time, 1.5);
        assert_eq!(animation.loading_dots, 3);
        assert_eq!(animation.tier_hover, Some(StorageTier::Hot));

        // Test with None hover
        let animation_none = AnimationState {
            pulse_time: 0.0,
            loading_dots: 0,
            tier_hover: None,
        };

        assert_eq!(animation_none.tier_hover, None);
    }

    #[test]
    fn test_file_browser_state_creation() {
        let browser = FileBrowserState {
            current_path: "/home/user/documents".to_string(),
            selected_files: vec!["file1.txt".to_string(), "file2.pdf".to_string()],
            show_hidden: false,
            sort_by: SortBy::Name,
            view_mode: ViewMode::List,
        };

        assert_eq!(browser.current_path, "/home/user/documents");
        assert_eq!(browser.selected_files.len(), 2);
        assert_eq!(browser.selected_files[0], "file1.txt");
        assert_eq!(browser.selected_files[1], "file2.pdf");
        assert!(!browser.show_hidden);
        assert_eq!(browser.sort_by, SortBy::Name);
        assert_eq!(browser.view_mode, ViewMode::List);
    }

    #[test]
    fn test_file_browser_state_with_hidden_files() {
        let browser = FileBrowserState {
            current_path: "/etc".to_string(),
            selected_files: vec![],
            show_hidden: true,
            sort_by: SortBy::Modified,
            view_mode: ViewMode::Details,
        };

        assert_eq!(browser.current_path, "/etc");
        assert!(browser.selected_files.is_empty());
        assert!(browser.show_hidden);
        assert_eq!(browser.sort_by, SortBy::Modified);
        assert_eq!(browser.view_mode, ViewMode::Details);
    }

    #[test]
    fn test_notification_creation() {
        let now = Instant::now();
        let notification = Notification {
            message: "System backup completed".to_string(),
            level: NotificationLevel::Success,
            timestamp: now,
            duration: Duration::from_secs(5),
        };

        assert_eq!(notification.message, "System backup completed");
        assert_eq!(notification.level, NotificationLevel::Success);
        assert_eq!(notification.timestamp, now);
        assert_eq!(notification.duration, Duration::from_secs(5));
    }

    #[test]
    fn test_notification_different_levels() {
        let now = Instant::now();

        let info_notification = Notification {
            message: "Info message".to_string(),
            level: NotificationLevel::Info,
            timestamp: now,
            duration: Duration::from_secs(3),
        };

        let error_notification = Notification {
            message: "Error occurred".to_string(),
            level: NotificationLevel::Error,
            timestamp: now,
            duration: Duration::from_secs(10),
        };

        assert_eq!(info_notification.level, NotificationLevel::Info);
        assert_eq!(error_notification.level, NotificationLevel::Error);
        assert_ne!(info_notification.level, error_notification.level);

        // Error notifications should have longer duration
        assert!(error_notification.duration > info_notification.duration);
    }

    #[test]
    fn test_nestgate_app_default() {
        let app = NestGateApp::default();

        // Test initial state
        assert_eq!(app.current_view, AppView::Dashboard);
        assert!(app._real_data_available); // Based on handoff - real data is available
        assert!(!app.performance_history.is_empty()); // Should have initial performance data
        assert!(!app.tier_stats.is_empty()); // Should have initial tier stats

        // Test that all storage tiers are initialized
        assert!(app.tier_stats.contains_key(&StorageTier::Hot));
        assert!(app.tier_stats.contains_key(&StorageTier::Warm));
        assert!(app.tier_stats.contains_key(&StorageTier::Cold));

        // Test theme is default
        let default_theme = UITheme::default();
        assert_eq!(app.theme.accent_color, default_theme.accent_color);
    }

    #[test]
    fn test_tier_stats_space_calculations() {
        let stats = TierStats {
            name: "Test Tier".to_string(),
            used_space: 5 * 1024 * 1024 * 1024,   // 5GB
            total_space: 10 * 1024 * 1024 * 1024, // 10GB
            file_count: 1000,
            performance: "fast".to_string(),
            compression: "gzip".to_string(),
            health: TierHealth::Good,
            io_rate: 100.0,
            access_frequency: 50.0,
            temperature: 0.5,
        };

        // Test space usage calculation
        let usage_ratio = stats.used_space as f64 / stats.total_space as f64;
        assert_eq!(usage_ratio, 0.5); // 50% usage

        let free_space = stats.total_space - stats.used_space;
        assert_eq!(free_space, 5 * 1024 * 1024 * 1024); // 5GB free
    }

    #[test]
    fn test_performance_history_management() {
        let mut history = VecDeque::new();
        let now = Instant::now();

        // Add performance points
        for i in 0..5 {
            history.push_back(PerformancePoint {
                timestamp: now + Duration::from_secs(i),
                cpu_usage: i as f32 * 10.0,
                memory_usage: i as f32 * 15.0,
                disk_io: i as f32 * 5.0,
                network_io: i as f32 * 2.0,
            });
        }

        assert_eq!(history.len(), 5);

        // Test that we can access first and last points
        let first_point = history.front().unwrap();
        let last_point = history.back().unwrap();

        assert_eq!(first_point.cpu_usage, 0.0);
        assert_eq!(last_point.cpu_usage, 40.0);

        // Test removing old points (simulate rolling window)
        if history.len() > 3 {
            history.pop_front();
        }

        assert_eq!(history.len(), 4);
        assert_eq!(history.front().unwrap().cpu_usage, 10.0);
    }
}

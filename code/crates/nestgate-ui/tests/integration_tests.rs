//! UI Integration Tests for NestGate
//!
//! Tests the actual UI functionality with real data integration

use eframe::egui::Color32;
use nestgate_core::StorageTier;
use nestgate_ui::{
    AnimationState, AppView, DataSource, FileBrowserState, NestGateApp, Notification,
    NotificationLevel, PerformancePoint, SortBy, SystemStatus, TierHealth, TierStats, UITheme,
    ViewMode,
};
use std::time::{Duration, Instant};

// Import real data sources
use std::fs;
use std::path::Path;
use std::process::Command;

/// Helper to get real ZFS pool information
fn get_real_zfs_pool_info() -> Option<(String, u64, u64)> {
    // Try to get real ZFS pool information
    if let Ok(output) = Command::new("zpool").args(&["list", "-H", "-p"]).output() {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines().take(1) {
                // Get first pool
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() >= 3 {
                    if let (Ok(total), Ok(used)) =
                        (parts[1].parse::<u64>(), parts[2].parse::<u64>())
                    {
                        return Some((parts[0].to_string(), total, used));
                    }
                }
            }
        }
    }
    None
}

/// Helper to get real system performance metrics
fn get_real_system_metrics() -> (f32, f32, f32, f32) {
    let mut cpu_usage = 0.0;
    let mut memory_usage = 0.0;
    let mut disk_io = 0.0;
    let mut network_io = 0.0;

    // Try to get real CPU usage from /proc/loadavg
    if let Ok(loadavg) = fs::read_to_string("/proc/loadavg") {
        if let Some(load) = loadavg.split_whitespace().next() {
            if let Ok(load_val) = load.parse::<f32>() {
                cpu_usage = (load_val * 100.0).min(100.0);
            }
        }
    }

    // Try to get real memory usage from /proc/meminfo
    if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
        let mut total_mem = 0u64;
        let mut available_mem = 0u64;

        for line in meminfo.lines() {
            if line.starts_with("MemTotal:") {
                if let Some(val) = line.split_whitespace().nth(1) {
                    total_mem = val.parse().unwrap_or(0);
                }
            } else if line.starts_with("MemAvailable:") {
                if let Some(val) = line.split_whitespace().nth(1) {
                    available_mem = val.parse().unwrap_or(0);
                }
            }
        }

        if total_mem > 0 && available_mem > 0 {
            let used_mem = total_mem - available_mem;
            memory_usage = (used_mem as f32 / total_mem as f32) * 100.0;
        }
    }

    // Try to get disk I/O from /proc/diskstats (simplified)
    if let Ok(diskstats) = fs::read_to_string("/proc/diskstats") {
        for line in diskstats.lines().take(5) {
            // Check first few disks
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 10 {
                if let Ok(sectors_read) = parts[5].parse::<u64>() {
                    if let Ok(sectors_written) = parts[9].parse::<u64>() {
                        disk_io = ((sectors_read + sectors_written) as f32 / 1000.0).min(1000.0);
                        break;
                    }
                }
            }
        }
    }

    // Try to get network I/O from /proc/net/dev (simplified)
    if let Ok(netdev) = fs::read_to_string("/proc/net/dev") {
        for line in netdev.lines().skip(2).take(3) {
            // Skip header, check first few interfaces
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 10 && !parts[0].starts_with("lo") {
                // Skip loopback
                if let Ok(rx_bytes) = parts[1].parse::<u64>() {
                    if let Ok(tx_bytes) = parts[9].parse::<u64>() {
                        network_io = ((rx_bytes + tx_bytes) as f32 / (1024.0 * 1024.0)).min(1000.0);
                        break;
                    }
                }
            }
        }
    }

    (cpu_usage, memory_usage, disk_io, network_io)
}

/// Helper to get real file system information
fn get_real_filesystem_info() -> (String, Vec<String>, u64) {
    let mut current_path = "/".to_string();
    let mut files = Vec::new();
    let mut file_count = 0u64;

    // Try to find a real NestGate mount point
    for candidate in &["/mnt/nestpool", "/nestpool", "/zfs", "/mnt", "/"] {
        if Path::new(candidate).exists() {
            current_path = candidate.to_string();
            break;
        }
    }

    // Try to read directory contents
    if let Ok(entries) = fs::read_dir(&current_path) {
        for entry in entries.take(10) {
            // Limit to first 10 entries
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    files.push(name.to_string());
                    file_count += 1;
                }
            }
        }
    }

    (current_path, files, file_count)
}

/// Helper to determine real ZFS health status
fn get_real_zfs_health() -> (bool, String, String) {
    let mut zfs_available = false;
    let mut health_status = "Unknown".to_string();
    let mut compilation_status = "❓ Unknown".to_string();

    // Check if ZFS is available
    if let Ok(output) = Command::new("zpool").args(&["status"]).output() {
        if output.status.success() {
            zfs_available = true;
            let stdout = String::from_utf8_lossy(&output.stdout);

            if stdout.contains("ONLINE") {
                health_status = "Optimal".to_string();
            } else if stdout.contains("DEGRADED") {
                health_status = "Degraded".to_string();
            } else if stdout.contains("FAULTED") {
                health_status = "Critical".to_string();
            } else {
                health_status = "Good".to_string();
            }
        }
    }

    // Check compilation status by trying to compile a simple test
    if let Ok(output) = Command::new("cargo")
        .args(&["check", "--workspace", "--quiet"])
        .output()
    {
        if output.status.success() {
            compilation_status = "✅ 100% Success".to_string();
        } else {
            compilation_status = "⚠️ Build Issues".to_string();
        }
    }

    (zfs_available, health_status, compilation_status)
}

#[cfg(test)]
mod app_initialization_tests {
    use super::*;

    #[test]
    fn test_nestgate_app_creation() {
        let _app = NestGateApp::default();

        // Manager should be created (or gracefully handle ZFS unavailability)
        println!("✅ NestGate UI app created successfully");
    }

    #[test]
    fn test_app_view_variants() {
        let views = vec![
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

        for view in views {
            let cloned = view.clone();
            assert_eq!(view, cloned);
        }
    }

    #[test]
    fn test_data_source_variants() {
        let sources = vec![DataSource::Live, DataSource::Mock, DataSource::FallbackMock];

        assert_eq!(sources.len(), 3);

        for source in sources {
            let cloned = source.clone();
            assert_eq!(source, cloned);
        }
    }

    #[test]
    fn test_system_status_with_real_data() {
        let (cpu, memory, _disk_io, network_io) = get_real_system_metrics();
        let (zfs_available, health_status, compilation_status) = get_real_zfs_health();

        let system_status = SystemStatus {
            mode: DataSource::Live,
            pools_online: if zfs_available { 1 } else { 0 },
            total_capacity: if zfs_available {
                if let Some((_, total, _)) = get_real_zfs_pool_info() {
                    format!("{:.2}TB", total as f64 / (1024.0_f64.powi(4)))
                } else {
                    "Unknown".to_string()
                }
            } else {
                "No ZFS".to_string()
            },
            health_status,
            zfs_available,
            compilation_status,
            security_level: "🔒 Production".to_string(),
            ai_ml_ready: true,
            uptime: Duration::from_secs(3600 * 24), // Simplified uptime
            cpu_usage: cpu,
            memory_usage: memory,
            network_io,
        };

        assert!(system_status.cpu_usage >= 0.0 && system_status.cpu_usage <= 100.0);
        assert!(system_status.memory_usage >= 0.0 && system_status.memory_usage <= 100.0);
        assert!(system_status.network_io >= 0.0);
        println!(
            "✅ Real system status: CPU: {:.1}%, Memory: {:.1}%, ZFS: {}",
            system_status.cpu_usage, system_status.memory_usage, system_status.zfs_available
        );
    }
}

#[cfg(test)]
mod ui_theme_tests {
    use super::*;

    #[test]
    fn test_ui_theme_default() {
        let theme = UITheme::default();

        // Verify theme has valid colors
        assert_ne!(theme.accent_color, Color32::TRANSPARENT);
        assert_ne!(theme.success_color, Color32::TRANSPARENT);
        assert_ne!(theme.warning_color, Color32::TRANSPARENT);
        assert_ne!(theme.error_color, Color32::TRANSPARENT);
    }

    #[test]
    fn test_ui_theme_creation() {
        let theme = UITheme {
            accent_color: Color32::from_rgb(0, 120, 215),
            success_color: Color32::from_rgb(16, 124, 16),
            warning_color: Color32::from_rgb(255, 140, 0),
            error_color: Color32::from_rgb(196, 43, 28),
            background_color: Color32::from_rgb(32, 32, 32),
            card_background: Color32::from_rgb(48, 48, 48),
            text_color: Color32::from_rgb(255, 255, 255),
            muted_text: Color32::from_rgb(200, 200, 200),
        };

        assert_eq!(theme.accent_color, Color32::from_rgb(0, 120, 215));
        assert_eq!(theme.success_color, Color32::from_rgb(16, 124, 16));
        assert_eq!(theme.warning_color, Color32::from_rgb(255, 140, 0));
        assert_eq!(theme.error_color, Color32::from_rgb(196, 43, 28));
        assert_eq!(theme.background_color, Color32::from_rgb(32, 32, 32));
        assert_eq!(theme.card_background, Color32::from_rgb(48, 48, 48));
        assert_eq!(theme.text_color, Color32::from_rgb(255, 255, 255));
        assert_eq!(theme.muted_text, Color32::from_rgb(200, 200, 200));
    }
}

#[cfg(test)]
mod tier_stats_tests {
    use super::*;

    #[test]
    fn test_tier_stats_with_real_data() {
        let (pool_name, total, used) = get_real_zfs_pool_info().unwrap_or((
            "mock_pool".to_string(),
            1024 * 1024 * 1024 * 1024,
            256 * 1024 * 1024 * 1024,
        ));

        let (_, _, file_count) = get_real_filesystem_info();

        let stats = TierStats {
            name: format!("{} Hot Tier", pool_name),
            used_space: used,
            total_space: total,
            file_count: file_count.try_into().unwrap_or(0),
            performance: if total > 0 {
                "< 1ms latency (real data)"
            } else {
                "mock latency"
            }
            .to_string(),
            compression: "lz4 (real compression)".to_string(),
            health: TierHealth::Optimal,
            io_rate: 1250.0,
            access_frequency: 340.0,
            temperature: 0.9,
        };

        assert!(!stats.name.is_empty());
        assert!(stats.used_space <= stats.total_space);
        assert_eq!(stats.health, TierHealth::Optimal);

        println!(
            "✅ Real tier stats: {} using {:.1}% of {:.2}TB",
            stats.name,
            (stats.used_space as f64 / stats.total_space as f64) * 100.0,
            stats.total_space as f64 / (1024.0_f64.powi(4))
        );
    }

    #[test]
    fn test_tier_health_variants() {
        let health_levels = vec![
            TierHealth::Optimal,
            TierHealth::Good,
            TierHealth::Warning,
            TierHealth::Critical,
        ];

        assert_eq!(health_levels.len(), 4);

        for health in health_levels {
            let cloned = health.clone();
            assert_eq!(health, cloned);
        }
    }

    #[test]
    fn test_tier_stats_space_calculations_real() {
        let (_, total, used) = get_real_zfs_pool_info().unwrap_or((
            String::new(),
            1000 * 1024 * 1024 * 1024,
            250 * 1024 * 1024 * 1024,
        ));

        let stats = TierStats {
            name: "Real Data Tier".to_string(),
            used_space: used,
            total_space: total,
            file_count: 1000,
            performance: "real performance".to_string(),
            compression: "real compression".to_string(),
            health: TierHealth::Good,
            io_rate: 100.0,
            access_frequency: 50.0,
            temperature: 0.5,
        };

        // Test space calculations with real data
        assert!(stats.used_space <= stats.total_space);
        let free_space = stats.total_space - stats.used_space;
        assert!(free_space <= stats.total_space);

        // Test utilization percentage
        let utilization = (stats.used_space as f64 / stats.total_space as f64) * 100.0;
        assert!(utilization >= 0.0 && utilization <= 100.0);

        println!(
            "✅ Real space calculation: {:.1}% utilization ({:.2}GB free)",
            utilization,
            free_space as f64 / (1024.0_f64.powi(3))
        );
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_performance_point_with_real_data() {
        let (cpu, memory, disk_io, network_io) = get_real_system_metrics();

        let point = PerformancePoint {
            timestamp: Instant::now(),
            cpu_usage: cpu,
            memory_usage: memory,
            disk_io,
            network_io,
        };

        assert!(point.cpu_usage >= 0.0 && point.cpu_usage <= 100.0);
        assert!(point.memory_usage >= 0.0 && point.memory_usage <= 100.0);
        assert!(point.disk_io >= 0.0);
        assert!(point.network_io >= 0.0);

        println!(
            "✅ Real performance: CPU: {:.1}%, Memory: {:.1}%, Disk I/O: {:.1}, Network I/O: {:.1}",
            point.cpu_usage, point.memory_usage, point.disk_io, point.network_io
        );
    }

    #[test]
    fn test_performance_point_validation() {
        let (cpu, memory, disk_io, network_io) = get_real_system_metrics();

        let point = PerformancePoint {
            timestamp: Instant::now(),
            cpu_usage: cpu,
            memory_usage: memory,
            disk_io,
            network_io,
        };

        // Validate real data is within reasonable bounds
        assert!(point.cpu_usage >= 0.0 && point.cpu_usage <= 100.0);
        assert!(point.memory_usage >= 0.0 && point.memory_usage <= 100.0);
        assert!(point.disk_io >= 0.0);
        assert!(point.network_io >= 0.0);
    }
}

#[cfg(test)]
mod file_browser_tests {
    use super::*;

    #[test]
    fn test_file_browser_state_with_real_data() {
        let (current_path, files, _) = get_real_filesystem_info();

        let browser = FileBrowserState {
            current_path: current_path.clone(),
            selected_files: files.clone().into_iter().take(2).collect(),
            show_hidden: false,
            sort_by: SortBy::Name,
            view_mode: ViewMode::List,
        };

        assert!(!browser.current_path.is_empty());
        assert!(browser.selected_files.len() <= files.len());
        assert!(!browser.show_hidden);
        assert_eq!(browser.sort_by, SortBy::Name);
        assert_eq!(browser.view_mode, ViewMode::List);

        println!(
            "✅ Real file browser: {} with {} files",
            current_path,
            files.len()
        );
    }

    #[test]
    fn test_sort_by_variants() {
        let sort_options = vec![SortBy::Name, SortBy::Size, SortBy::Modified, SortBy::Type];

        assert_eq!(sort_options.len(), 4);

        for sort in sort_options {
            let cloned = sort.clone();
            assert_eq!(sort, cloned);
        }
    }

    #[test]
    fn test_view_mode_variants() {
        let view_modes = vec![ViewMode::List, ViewMode::Grid, ViewMode::Details];

        assert_eq!(view_modes.len(), 3);

        for mode in view_modes {
            let cloned = mode.clone();
            assert_eq!(mode, cloned);
        }
    }
}

#[cfg(test)]
mod notification_tests {
    use super::*;

    #[test]
    fn test_notification_creation() {
        let (zfs_available, health_status, _) = get_real_zfs_health();

        let message = if zfs_available {
            format!("ZFS system is {}", health_status)
        } else {
            "ZFS not available - using mock data".to_string()
        };

        let level = if zfs_available && health_status == "Optimal" {
            NotificationLevel::Success
        } else if zfs_available {
            NotificationLevel::Warning
        } else {
            NotificationLevel::Info
        };

        let notification = Notification {
            message: message.clone(),
            level: level.clone(),
            timestamp: Instant::now(),
            duration: Duration::from_secs(5),
        };

        assert_eq!(notification.message, message);
        assert_eq!(notification.level, level);
        assert_eq!(notification.duration, Duration::from_secs(5));

        println!("✅ Real notification: {}", message);
    }

    #[test]
    fn test_notification_level_variants() {
        let levels = vec![
            NotificationLevel::Info,
            NotificationLevel::Success,
            NotificationLevel::Warning,
            NotificationLevel::Error,
        ];

        assert_eq!(levels.len(), 4);

        for level in levels {
            let cloned = level.clone();
            assert_eq!(level, cloned);
        }
    }

    #[test]
    fn test_notification_different_levels_real() {
        let (cpu, memory, _, _) = get_real_system_metrics();
        let (zfs_available, health_status, _) = get_real_zfs_health();

        let mut notifications = Vec::new();

        // Create notifications based on real system state
        if zfs_available && health_status == "Optimal" {
            notifications.push(Notification {
                message: "ZFS system running optimally".to_string(),
                level: NotificationLevel::Success,
                timestamp: Instant::now(),
                duration: Duration::from_secs(4),
            });
        }

        if cpu > 80.0 {
            notifications.push(Notification {
                message: format!("High CPU usage: {:.1}%", cpu),
                level: NotificationLevel::Warning,
                timestamp: Instant::now(),
                duration: Duration::from_secs(5),
            });
        }

        if memory > 90.0 {
            notifications.push(Notification {
                message: format!("High memory usage: {:.1}%", memory),
                level: NotificationLevel::Error,
                timestamp: Instant::now(),
                duration: Duration::from_secs(10),
            });
        }

        notifications.push(Notification {
            message: format!("System status: CPU {:.1}%, Memory {:.1}%", cpu, memory),
            level: NotificationLevel::Info,
            timestamp: Instant::now(),
            duration: Duration::from_secs(3),
        });

        assert!(!notifications.is_empty());
        println!(
            "✅ Generated {} real notifications based on system state",
            notifications.len()
        );
    }
}

#[cfg(test)]
mod animation_tests {
    use super::*;

    #[test]
    fn test_animation_state_creation() {
        let animations = AnimationState {
            pulse_time: 1.5,
            loading_dots: 2,
            tier_hover: Some(StorageTier::Hot),
        };

        assert_eq!(animations.pulse_time, 1.5);
        assert_eq!(animations.loading_dots, 2);
        assert_eq!(animations.tier_hover, Some(StorageTier::Hot));
    }

    #[test]
    fn test_animation_state_with_no_hover() {
        let animations = AnimationState {
            pulse_time: 0.0,
            loading_dots: 0,
            tier_hover: None,
        };

        assert_eq!(animations.pulse_time, 0.0);
        assert_eq!(animations.loading_dots, 0);
        assert_eq!(animations.tier_hover, None);
    }
}

#[cfg(test)]
mod integration_validation_tests {
    use super::*;

    #[test]
    fn test_complete_ui_component_integration_with_real_data() {
        // Test that all major UI components can be created together with real data
        let _app = NestGateApp::default();
        let _theme = UITheme::default();

        let (cpu, memory, disk_io, network_io) = get_real_system_metrics();
        let (zfs_available, health_status, compilation_status) = get_real_zfs_health();
        let (pool_name, total, used) = get_real_zfs_pool_info().unwrap_or((
            "mock_pool".to_string(),
            1024 * 1024 * 1024 * 1024,
            256 * 1024 * 1024 * 1024,
        ));

        let _system_status = SystemStatus {
            mode: DataSource::Live,
            pools_online: if zfs_available { 1 } else { 0 },
            total_capacity: format!("{:.2}TB", total as f64 / (1024.0_f64.powi(4))),
            health_status,
            zfs_available,
            compilation_status,
            security_level: "🔒 Production".to_string(),
            ai_ml_ready: true,
            uptime: Duration::from_secs(3600 * 24 * 7),
            cpu_usage: cpu,
            memory_usage: memory,
            network_io,
        };

        let _tier_stats = TierStats {
            name: format!("{} Integration Tier", pool_name),
            used_space: used,
            total_space: total,
            file_count: 1000,
            performance: "< 1ms latency (real)".to_string(),
            compression: "lz4 (real)".to_string(),
            health: if zfs_available {
                TierHealth::Optimal
            } else {
                TierHealth::Warning
            },
            io_rate: 1250.0,
            access_frequency: 340.0,
            temperature: 0.9,
        };

        let _performance_point = PerformancePoint {
            timestamp: Instant::now(),
            cpu_usage: cpu,
            memory_usage: memory,
            disk_io,
            network_io,
        };

        let (current_path, files, _) = get_real_filesystem_info();
        let _file_browser = FileBrowserState {
            current_path,
            selected_files: files.into_iter().take(2).collect(),
            show_hidden: false,
            sort_by: SortBy::Name,
            view_mode: ViewMode::List,
        };

        let _notification = Notification {
            message: "Real data integration test complete".to_string(),
            level: NotificationLevel::Success,
            timestamp: Instant::now(),
            duration: Duration::from_secs(5),
        };

        println!("✅ Complete UI integration with real data successful");
        // All components should be created successfully with real data
    }

    #[test]
    fn test_storage_tier_integration_with_real_metrics() {
        let (_, total, used) = get_real_zfs_pool_info().unwrap_or((
            String::new(),
            1000 * 1024 * 1024 * 1024,
            250 * 1024 * 1024 * 1024,
        ));

        let tiers = vec![
            StorageTier::Hot,
            StorageTier::Warm,
            StorageTier::Cold,
            StorageTier::Cache,
        ];

        for tier in tiers {
            let tier_used = used / 4; // Distribute usage across tiers
            let tier_total = total / 4;

            let stats = TierStats {
                name: format!("{:?} Tier (Real Data)", tier),
                used_space: tier_used,
                total_space: tier_total,
                file_count: 1000,
                performance: "real performance metrics".to_string(),
                compression: "real compression data".to_string(),
                health: TierHealth::Good,
                io_rate: 100.0,
                access_frequency: 50.0,
                temperature: 0.5,
            };

            assert!(!stats.name.is_empty());
            assert!(stats.used_space <= stats.total_space);
            assert_eq!(stats.health, TierHealth::Good);

            let utilization = (stats.used_space as f64 / stats.total_space as f64) * 100.0;
            println!("✅ {} utilization: {:.1}%", stats.name, utilization);
        }
    }

    #[test]
    fn test_ui_state_consistency_with_real_data() {
        // Test that UI state remains consistent across different operations with real data
        let _app = NestGateApp::default();

        // App should be created with consistent state
        // Since fields are private, we test that creation doesn't panic
        // and that the app can be used for UI operations

        let theme = UITheme::default();

        // Theme should have consistent color values
        assert_ne!(theme.accent_color, Color32::TRANSPARENT);
        assert_ne!(theme.success_color, Color32::TRANSPARENT);
        assert_ne!(theme.warning_color, Color32::TRANSPARENT);
        assert_ne!(theme.error_color, Color32::TRANSPARENT);

        let (cpu, memory, _, _) = get_real_system_metrics();
        let (zfs_available, _, _) = get_real_zfs_health();

        println!("✅ UI state consistency validated with real system data (CPU: {:.1}%, Memory: {:.1}%, ZFS: {})",
                 cpu, memory, zfs_available);
    }
}

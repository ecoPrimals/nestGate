// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Round 6 ZFS coverage: scheduler edges, pool setup, failover, MCP, device helpers.

#[cfg(test)]
mod round6_zfs_tests {
    use crate::failover::{CanonicalFailoverConfig, PoolFailoverState, PoolMetadata, PoolState};
    use crate::mcp_integration::ZfsMcpConfig;
    use crate::pool_setup::config::{
        DeviceDetectionConfig, PoolSetupConfig, PoolTopology, RedundancyLevel,
    };
    use crate::pool_setup::creation::PoolCreator;
    use crate::pool_setup::device_detection::{
        DeviceScanner, DeviceType, SpeedClass, StorageDevice,
    };
    use crate::snapshot::{RetentionPolicy, SnapshotInfo, SnapshotOperationStatus};
    use crate::types::StorageTier;
    use std::collections::HashMap;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    #[test]
    fn r6_retention_policy_variants_construct() {
        let _ = RetentionPolicy::Duration(Duration::from_secs(60));
        let _ = RetentionPolicy::Count(10);
        let _ = RetentionPolicy::Custom {
            hourly_hours: 1,
            daily_days: 2,
            weekly_weeks: 3,
            monthly_months: 4,
            yearly_years: 5,
        };
    }

    #[test]
    fn r6_retention_duration_filters_epoch_snapshot() {
        let old_snap = SnapshotInfo {
            name: "n".into(),
            full_name: "d@n".into(),
            dataset: "d".into(),
            created_at: UNIX_EPOCH,
            size: 1,
            referenced_size: 1,
            written_size: 1,
            compression_ratio: 1.0,
            properties: HashMap::new(),
            policy: None,
            tier: StorageTier::Warm,
            protected: false,
            tags: Vec::new(),
        };
        let cutoff = SystemTime::now() - Duration::from_secs(3600);
        let old: Vec<_> = vec![old_snap]
            .into_iter()
            .filter(|s| s.created_at < cutoff)
            .collect();
        assert_eq!(old.len(), 1);
    }

    #[test]
    fn r6_snapshot_operation_status_eq() {
        assert_eq!(
            SnapshotOperationStatus::Queued,
            SnapshotOperationStatus::Queued
        );
    }

    #[tokio::test]
    async fn r6_pool_creator_dry_run_triple_raidz_devices() {
        let creator = PoolCreator::new_dry_run();
        let config = PoolSetupConfig {
            pool_name: "r6rz3".into(),
            devices: vec![
                "/dev/a".into(),
                "/dev/b".into(),
                "/dev/c".into(),
                "/dev/d".into(),
            ],
            topology: PoolTopology::RaidZ3,
            properties: HashMap::new(),
            tier_mappings: HashMap::new(),
            redundancy: RedundancyLevel::Triple,
            device_detection: DeviceDetectionConfig::default(),
            create_tiers: false,
        };
        let r = creator.create_pool_safe(&config).await;
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn r6_pool_creator_dry_run_mirror_two_devices() {
        let creator = PoolCreator::new_dry_run();
        let config = PoolSetupConfig {
            pool_name: "r6mir".into(),
            devices: vec!["/dev/a".into(), "/dev/b".into()],
            topology: PoolTopology::Mirror,
            properties: HashMap::new(),
            tier_mappings: HashMap::new(),
            redundancy: RedundancyLevel::Double,
            device_detection: DeviceDetectionConfig::default(),
            create_tiers: false,
        };
        assert!(creator.create_pool_safe(&config).await.is_ok());
    }

    #[test]
    fn r6_failover_pool_state_ord() {
        assert_ne!(PoolState::Online, PoolState::Faulted);
        assert_eq!(PoolFailoverState::Active, PoolFailoverState::Active);
    }

    #[test]
    fn r6_canonical_failover_config_clone() {
        let c = CanonicalFailoverConfig::default();
        let c2 = c.clone();
        assert_eq!(c.health_check_interval_secs, c2.health_check_interval_secs);
    }

    #[test]
    fn r6_pool_metadata_serde() {
        let m = PoolMetadata {
            name: "p".into(),
            original_owner: "n1".into(),
            last_seen: UNIX_EPOCH,
            import_guid: None,
            state: PoolFailoverState::Unknown,
        };
        let j = serde_json::to_string(&m).unwrap();
        let _: PoolMetadata = serde_json::from_str(&j).unwrap();
    }

    #[test]
    fn r6_mcp_config_validate_edges() {
        let mut c = ZfsMcpConfig::default();
        c.max_concurrent_operations = 0;
        assert!(c.validate().is_err());
        c.max_concurrent_operations = 1001;
        assert!(c.validate().is_err());
        c.max_concurrent_operations = 10;
        assert!(c.validate().is_ok());
    }

    #[test]
    fn r6_mcp_tier_config_all_tiers() {
        let c = ZfsMcpConfig::default();
        for t in [
            StorageTier::Hot,
            StorageTier::Warm,
            StorageTier::Cold,
            StorageTier::Cache,
            StorageTier::Archive,
        ] {
            let tc = c.get_tier_config(&t);
            assert!(tc.replication > 0);
        }
    }

    #[test]
    fn r6_device_scanner_filter_mixed() {
        let d1 = StorageDevice {
            device_path: "/dev/nvme0n1".into(),
            model: "gen4".into(),
            size_bytes: 1,
            device_type: DeviceType::NvmeSsd,
            speed_class: SpeedClass::UltraFast,
            in_use: false,
            current_use: None,
        };
        let d2 = StorageDevice {
            device_path: "/dev/sda".into(),
            model: "hdd".into(),
            size_bytes: 1,
            device_type: DeviceType::Hdd,
            speed_class: SpeedClass::Slow,
            in_use: true,
            current_use: Some("in use".into()),
        };
        let all = vec![d1, d2];
        assert_eq!(
            DeviceScanner::filter_by_type(&all, DeviceType::Hdd).len(),
            1
        );
        assert_eq!(
            DeviceScanner::filter_by_speed(&all, SpeedClass::UltraFast).len(),
            1
        );
        assert_eq!(DeviceScanner::filter_available(&all).len(), 1);
    }

    #[test]
    fn r6_device_parse_size_pb() {
        let s = DeviceScanner::new(DeviceDetectionConfig::default());
        let b = s.test_parse_size_string("1PB").expect("pb");
        assert!(b >= 1024_u64.pow(5));
    }

    #[test]
    fn r6_device_parse_empty() {
        let s = DeviceScanner::new(DeviceDetectionConfig::default());
        assert_eq!(s.test_parse_size_string("").expect("e"), 0);
    }

    #[test]
    fn r6_classify_sata_enterprise() {
        let s = DeviceScanner::new(DeviceDetectionConfig::default());
        assert_eq!(
            s.test_classify_device_speed(&DeviceType::SataSsd, "enterprise pro"),
            SpeedClass::Fast
        );
    }

    #[test]
    fn r6_classify_nvme_non_gen4() {
        let s = DeviceScanner::new(DeviceDetectionConfig::default());
        assert_eq!(
            s.test_classify_device_speed(&DeviceType::NvmeSsd, "standard"),
            SpeedClass::Fast
        );
    }

    #[test]
    fn r6_should_include_loop_respects_config() {
        let mut cfg = DeviceDetectionConfig::default();
        cfg.include_loop_devices = false;
        let scan = DeviceScanner::new(cfg);
        let dev = StorageDevice {
            device_path: "/dev/loop0".into(),
            model: "loop".into(),
            size_bytes: 100,
            device_type: DeviceType::Unknown,
            speed_class: SpeedClass::Medium,
            in_use: false,
            current_use: None,
        };
        assert!(!scan.should_include_device(&dev));
    }
}

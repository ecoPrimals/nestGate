// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::{
    ConfigStorageTier, DetectionDeviceType, PoolSetupError, PoolSetupResult, PoolTopology,
    SpeedClass, StorageDevice, ZfsPoolSetup,
};

fn sample_nvme(path: &str) -> StorageDevice {
    StorageDevice {
        device_path: path.to_string(),
        model: "test-model".to_string(),
        size_bytes: 2 * 1024 * 1024 * 1024,
        device_type: DetectionDeviceType::NvmeSsd,
        speed_class: SpeedClass::UltraFast,
        in_use: false,
        current_use: None,
    }
}

fn sample_sata_ssd(path: &str) -> StorageDevice {
    StorageDevice {
        device_path: path.to_string(),
        model: "sata-ssd".to_string(),
        size_bytes: 1024 * 1024 * 1024,
        device_type: DetectionDeviceType::SataSsd,
        speed_class: SpeedClass::Fast,
        in_use: false,
        current_use: None,
    }
}

fn sample_hdd(path: &str) -> StorageDevice {
    StorageDevice {
        device_path: path.to_string(),
        model: "spinning".to_string(),
        size_bytes: 4 * 1024 * 1024 * 1024,
        device_type: DetectionDeviceType::Hdd,
        speed_class: SpeedClass::Slow,
        in_use: false,
        current_use: None,
    }
}

#[test]
fn recommend_pool_config_rejects_empty_name() {
    let setup = ZfsPoolSetup::test_fixture(vec![sample_nvme("/dev/nvme0n1")], vec![]);
    assert!(setup.recommend_pool_config("").is_err());
}

#[test]
fn recommend_pool_config_rejects_no_available_devices() {
    let busy = StorageDevice {
        device_path: "/dev/busy".to_string(),
        model: "busy".to_string(),
        size_bytes: 1,
        device_type: DetectionDeviceType::NvmeSsd,
        speed_class: SpeedClass::UltraFast,
        in_use: true,
        current_use: Some("zfs".to_string()),
    };
    let setup = ZfsPoolSetup::test_fixture(vec![busy], vec![]);
    assert!(setup.recommend_pool_config("tank").is_err());
}

#[test]
fn recommend_pool_config_with_one_device() {
    let setup = ZfsPoolSetup::test_fixture(vec![sample_nvme("/dev/nvme0n1")], vec![]);
    let cfg = setup
        .recommend_pool_config("tank")
        .expect("recommended config");
    assert_eq!(cfg.pool_name, "tank");
    assert_eq!(cfg.topology, PoolTopology::Single);
    assert_eq!(cfg.devices.len(), 1);
}

#[test]
fn recommend_pool_config_two_devices_mirror() {
    let setup = ZfsPoolSetup::test_fixture(
        vec![sample_nvme("/dev/nvme0n1"), sample_nvme("/dev/nvme1n1")],
        vec![],
    );
    let cfg = setup
        .recommend_pool_config("mirror-tank")
        .expect("recommended config");
    assert_eq!(cfg.topology, PoolTopology::Mirror);
    assert_eq!(cfg.devices.len(), 2);
}

#[test]
fn recommend_pool_config_three_devices_raidz1() {
    let setup = ZfsPoolSetup::test_fixture(
        vec![
            sample_nvme("/dev/nvme0n1"),
            sample_nvme("/dev/nvme1n1"),
            sample_nvme("/dev/nvme2n1"),
        ],
        vec![],
    );
    let cfg = setup
        .recommend_pool_config("rz1")
        .expect("recommended config");
    assert_eq!(cfg.topology, PoolTopology::RaidZ1);
    assert_eq!(cfg.devices.len(), 3);
}

#[test]
fn recommend_pool_config_six_devices_raidz2() {
    let devices: Vec<StorageDevice> = (0..6)
        .map(|i| sample_nvme(&format!("/dev/nvme{i}n1")))
        .collect();
    let setup = ZfsPoolSetup::test_fixture(devices, vec![]);
    let cfg = setup
        .recommend_pool_config("rz2-pool")
        .expect("recommended config");
    assert_eq!(cfg.topology, PoolTopology::RaidZ2);
    assert_eq!(cfg.devices.len(), 6);
}

#[test]
fn recommend_pool_config_twelve_devices_raidz3() {
    let devices: Vec<StorageDevice> = (0..12)
        .map(|i| sample_nvme(&format!("/dev/nvme{i}n1")))
        .collect();
    let setup = ZfsPoolSetup::test_fixture(devices, vec![]);
    let cfg = setup
        .recommend_pool_config("rz3-pool")
        .expect("recommended config");
    assert_eq!(cfg.topology, PoolTopology::RaidZ3);
    assert_eq!(cfg.devices.len(), 12);
}

#[test]
fn recommend_pool_config_mixed_device_types_sets_tier_mappings() {
    let setup = ZfsPoolSetup::test_fixture(
        vec![
            sample_nvme("/dev/nvme0n1"),
            sample_sata_ssd("/dev/sda"),
            sample_hdd("/dev/sdb"),
        ],
        vec![],
    );
    let cfg = setup
        .recommend_pool_config("mixed")
        .expect("recommended config");
    assert!(cfg.create_tiers);
    assert!(!cfg.tier_mappings.is_empty());
    assert!(cfg.tier_mappings.contains_key(&ConfigStorageTier::Hot));
    assert!(cfg.tier_mappings.contains_key(&ConfigStorageTier::Warm));
    assert!(cfg.tier_mappings.contains_key(&ConfigStorageTier::Cold));
}

#[test]
fn recommend_pool_config_single_device_type_replicates_across_tiers() {
    let setup = ZfsPoolSetup::test_fixture(
        vec![sample_nvme("/dev/nvme0n1"), sample_nvme("/dev/nvme1n1")],
        vec![],
    );
    let cfg = setup
        .recommend_pool_config("uniform")
        .expect("recommended config");
    let hot = cfg
        .tier_mappings
        .get(&ConfigStorageTier::Hot)
        .expect("hot tier");
    let cold = cfg
        .tier_mappings
        .get(&ConfigStorageTier::Cold)
        .expect("cold tier");
    assert_eq!(hot, cold);
}

#[test]
fn get_system_report_without_devices() {
    let setup = ZfsPoolSetup::test_fixture(vec![], vec![]);
    let report = setup.get_system_report();
    assert_eq!(report.total_devices, 0);
    assert!(
        report
            .recommendations
            .iter()
            .any(|r| r.contains("No available devices"))
    );
}

#[test]
fn pool_setup_error_display_contains_message() {
    let e = PoolSetupError::Configuration("bad".to_string());
    assert!(e.to_string().contains("Configuration"));
}

#[test]
fn pool_setup_result_json_roundtrip() {
    let r = PoolSetupResult {
        pool_name: "p".to_string(),
        success: true,
        message: "done".to_string(),
        devices_used: vec!["/dev/sda".to_string()],
        topology: PoolTopology::Mirror,
    };
    let json = serde_json::to_string(&r).expect("serialize");
    let back: PoolSetupResult = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.pool_name, "p");
    assert_eq!(back.topology, PoolTopology::Mirror);
}

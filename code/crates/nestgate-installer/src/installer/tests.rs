// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::NestGateInstaller;
use super::types::InstallationInfo;
use crate::config::InstallerConfig;
use anyhow::Result;
use chrono::Utc;
use std::fs;
use std::path::PathBuf;
use tokio::sync::Mutex;

/// Serializes tests that repoint `HOME` so parallel runs do not race.
static INSTALLER_HOME_MUTEX: Mutex<()> = Mutex::const_new(());

/// Creates  Test Installation Info
fn create_test_installation_info() -> InstallationInfo {
    InstallationInfo {
        version: "1.0.0".to_string(),
        install_date: Utc::now(),
        install_path: PathBuf::from("/opt/nestgate"),
        config_path: PathBuf::from("/etc/nestgate"),
        data_path: PathBuf::from("/var/lib/nestgate"),
        service_installed: true,
        features: vec!["zfs".to_string(), "nfs".to_string()],
    }
}

#[test]
fn test_installation_info_creation() {
    let info = create_test_installation_info();
    assert_eq!(info.version, "1.0.0");
    assert_eq!(info.install_path, PathBuf::from("/opt/nestgate"));
    assert!(info.service_installed);
    assert_eq!(info.features.len(), 2);
}

#[test]
fn test_installation_info_clone() {
    let info = create_test_installation_info();
    let cloned = info.clone();
    assert_eq!(info.version, cloned.version);
    assert_eq!(info.install_path, cloned.install_path);
}

#[test]
fn test_installation_info_serialization() {
    let info = create_test_installation_info();
    let serialized =
        serde_json::to_string(&info).expect("Test: installation info serialization should succeed");
    assert!(serialized.contains("1.0.0"));
    assert!(serialized.contains("/opt/nestgate"));
}

#[test]
fn test_installation_info_deserialization() {
    let json = r#"{
            "version": "2.0.0",
            "install_date": "2025-01-01T00:00:00Z",
            "install_path": "/usr/local/nestgate",
            "config_path": "/etc/nestgate",
            "data_path": "/var/lib/nestgate",
            "service_installed": false,
            "features": ["zfs"]
        }"#;
    let info: InstallationInfo =
        serde_json::from_str(json).expect("Test: installation info deserialization should succeed");
    assert_eq!(info.version, "2.0.0");
    assert_eq!(info.install_path, PathBuf::from("/usr/local/nestgate"));
    assert!(!info.service_installed);
}

#[test]
fn test_installation_info_empty_features() {
    let info = InstallationInfo {
        version: "1.0.0".to_string(),
        install_date: Utc::now(),
        install_path: PathBuf::from("/opt/nestgate"),
        config_path: PathBuf::from("/etc/nestgate"),
        data_path: PathBuf::from("/var/lib/nestgate"),
        service_installed: false,
        features: vec![],
    };
    assert!(info.features.is_empty());
}

#[test]
fn test_installation_info_many_features() {
    let features = vec![
        "zfs".to_string(),
        "nfs".to_string(),
        "smb".to_string(),
        "monitoring".to_string(),
        "backup".to_string(),
    ];
    let info = InstallationInfo {
        version: "1.0.0".to_string(),
        install_date: Utc::now(),
        install_path: PathBuf::from("/opt/nestgate"),
        config_path: PathBuf::from("/etc/nestgate"),
        data_path: PathBuf::from("/var/lib/nestgate"),
        service_installed: true,
        features,
    };
    assert_eq!(info.features.len(), 5);
    assert!(info.features.contains(&"monitoring".to_string()));
}

#[test]
fn test_installer_new() {
    let installer = NestGateInstaller::new(None);
    assert!(installer.is_ok());
}

#[test]
fn test_installer_new_with_path() {
    let install_dir = Some(PathBuf::from("/custom/install"));
    let installer = NestGateInstaller::new(install_dir);
    assert!(installer.is_ok());
}

#[test]
fn test_installer_new_with_different_paths() {
    let paths = vec![
        "/usr/local/nestgate",
        "/opt/nestgate",
        "/home/user/nestgate",
        "C:\\Program Files\\NestGate",
    ];

    for path in paths {
        let installer = NestGateInstaller::new(Some(PathBuf::from(path)));
        assert!(
            installer.is_ok(),
            "Failed to create installer with path: {}",
            path
        );
    }
}

#[test]
fn test_installation_info_debug() {
    let info = create_test_installation_info();
    let debug_str = format!("{info:?}");
    assert!(debug_str.contains("InstallationInfo"));
    assert!(debug_str.contains("1.0.0"));
}

#[test]
fn test_installation_info_version_formats() {
    let versions = vec!["1.0.0", "2.1.3", "0.9.0-beta", "3.0.0-rc1"];

    for version in versions {
        let info = InstallationInfo {
            version: version.to_string(),
            install_date: Utc::now(),
            install_path: PathBuf::from("/opt/nestgate"),
            config_path: PathBuf::from("/etc/nestgate"),
            data_path: PathBuf::from("/var/lib/nestgate"),
            service_installed: true,
            features: vec![],
        };
        assert_eq!(info.version, version);
    }
}

#[test]
fn test_installation_info_path_types() {
    let install_paths = vec![
        PathBuf::from("/"),
        PathBuf::from("/opt/nestgate"),
        PathBuf::from("C:\\Program Files\\NestGate"),
        PathBuf::from("/usr/local/bin"),
    ];

    for path in install_paths {
        let info = InstallationInfo {
            version: "1.0.0".to_string(),
            install_date: Utc::now(),
            install_path: path.clone(),
            config_path: PathBuf::from("/etc/nestgate"),
            data_path: PathBuf::from("/var/lib/nestgate"),
            service_installed: true,
            features: vec![],
        };
        assert_eq!(info.install_path, path);
    }
}

#[test]
fn test_installer_multiple_instances() {
    let installer1 = NestGateInstaller::new(None);
    let installer2 = NestGateInstaller::new(Some(PathBuf::from("/opt/nestgate")));

    // Both should be valid instances
    assert!(installer1.is_ok());
    assert!(installer2.is_ok());
}

#[test]
fn test_installation_info_service_states() {
    let installed = InstallationInfo {
        version: "1.0.0".to_string(),
        install_date: Utc::now(),
        install_path: PathBuf::from("/opt/nestgate"),
        config_path: PathBuf::from("/etc/nestgate"),
        data_path: PathBuf::from("/var/lib/nestgate"),
        service_installed: true,
        features: vec![],
    };
    assert!(installed.service_installed);

    let not_installed = InstallationInfo {
        version: "1.0.0".to_string(),
        install_date: Utc::now(),
        install_path: PathBuf::from("/opt/nestgate"),
        config_path: PathBuf::from("/etc/nestgate"),
        data_path: PathBuf::from("/var/lib/nestgate"),
        service_installed: false,
        features: vec![],
    };
    assert!(!not_installed.service_installed);
}

#[test]
fn install_with_default_config_ok() {
    let installer = NestGateInstaller::new(None).expect("installer");
    let cfg = InstallerConfig::default();
    installer.install(&cfg).expect("install noop");
}

#[test]
fn configure_errors_when_install_metadata_missing() {
    let _guard = INSTALLER_HOME_MUTEX.blocking_lock();
    let tmp = tempfile::tempdir().expect("tempdir");
    let xdg_data = tmp.path().join("xdg_data_empty");
    fs::create_dir_all(&xdg_data).expect("mkdir xdg");
    temp_env::with_vars(
        [
            ("HOME", Some(tmp.path().to_str().expect("utf8"))),
            ("XDG_DATA_HOME", Some(xdg_data.to_str().expect("utf8"))),
        ],
        || {
            let res = (|| -> Result<()> {
                let installer = NestGateInstaller::new(None)?;
                installer.configure(None)?;
                Ok(())
            })();
            assert!(
                res.is_err(),
                "expected error when install-info is absent: {res:?}"
            );
        },
    );
}

#[test]
fn configure_prints_when_config_file_missing_but_installed() {
    let _guard = INSTALLER_HOME_MUTEX.blocking_lock();
    let tmp = tempfile::tempdir().expect("tempdir");
    let xdg_data = tmp.path().join("xdg_data");

    let install_root = tmp.path().join("opt/nestgate");
    let config_path = tmp.path().join("etc/nestgate/nestgate.toml");
    let data_path = tmp.path().join("var/nestgate");
    fs::create_dir_all(install_root.join("placeholder")).expect("mkdir");
    fs::create_dir_all(config_path.parent().expect("parent")).expect("mkdir");
    fs::create_dir_all(&data_path).expect("mkdir");

    let info = InstallationInfo {
        version: "1.0.0".into(),
        install_date: Utc::now(),
        install_path: install_root,
        config_path: config_path.clone(),
        data_path,
        service_installed: false,
        features: vec![],
    };

    let meta = xdg_data.join("nestgate").join("install-info.json");
    fs::create_dir_all(meta.parent().expect("meta parent")).expect("mkdir meta");
    fs::write(&meta, serde_json::to_string(&info).expect("json")).expect("write meta");

    temp_env::with_vars(
        [
            ("HOME", Some(tmp.path().to_str().expect("utf8"))),
            ("XDG_DATA_HOME", Some(xdg_data.to_str().expect("utf8"))),
        ],
        || {
            let installer = NestGateInstaller::new(None).expect("installer");
            let res = installer.configure(None);
            assert!(
                res.is_ok(),
                "configure should succeed listing missing file: {res:?}"
            );
        },
    );
    assert!(!config_path.exists());
}

#[test]
fn uninstall_with_force_removes_paths_and_metadata() {
    let _guard = INSTALLER_HOME_MUTEX.blocking_lock();
    let tmp = tempfile::tempdir().expect("tempdir");
    let xdg_data = tmp.path().join("xdg_data");

    let install_root = tmp.path().join("opt/nestgate");
    let config_path = tmp.path().join("state/nestgate-config");
    let data_path = tmp.path().join("var/nestgate-data");
    fs::create_dir_all(install_root.join("bin")).expect("mkdir");
    fs::create_dir_all(&config_path).expect("mkdir");
    fs::create_dir_all(&data_path).expect("mkdir");

    let info = InstallationInfo {
        version: "0.0.1".into(),
        install_date: Utc::now(),
        install_path: install_root.clone(),
        config_path: config_path.clone(),
        data_path: data_path.clone(),
        service_installed: false,
        features: vec![],
    };

    let meta = xdg_data.join("nestgate").join("install-info.json");
    fs::create_dir_all(meta.parent().expect("meta parent")).expect("mkdir meta");
    fs::write(&meta, serde_json::to_string(&info).expect("json")).expect("write meta");

    temp_env::with_vars(
        [
            ("HOME", Some(tmp.path().to_str().expect("utf8"))),
            ("XDG_DATA_HOME", Some(xdg_data.to_str().expect("utf8"))),
        ],
        || {
            let installer = NestGateInstaller::new(None).expect("installer");
            installer.uninstall(true, true, true).expect("uninstall");
        },
    );
    assert!(!install_root.exists());
    assert!(!config_path.exists());
    assert!(!data_path.exists());
    assert!(!meta.exists());
}

#[test]
fn doctor_runs_with_isolated_home() {
    let _guard = INSTALLER_HOME_MUTEX.blocking_lock();
    let tmp = tempfile::tempdir().expect("tempdir");
    let xdg_data = tmp.path().join("xdg_data_doc");
    fs::create_dir_all(&xdg_data).expect("mkdir xdg");
    temp_env::with_vars(
        [
            ("HOME", Some(tmp.path().to_str().expect("utf8"))),
            ("XDG_DATA_HOME", Some(xdg_data.to_str().expect("utf8"))),
        ],
        || {
            let installer = NestGateInstaller::new(None).expect("installer");
            let res = installer.doctor();
            assert!(res.is_ok());
        },
    );
}

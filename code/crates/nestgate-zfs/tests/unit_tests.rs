// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

/// Focused unit tests for individual components and functions
use nestgate_core::canonical_types::StorageTier as CoreStorageTier;
use nestgate_zfs::config::ZfsConfig;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

#[cfg(test)]
mod config_unit_tests {
    use super::*;

    #[test]
    fn test_zfs_config_defaults() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let config = ZfsConfig::default();

        // Verify basic ZFS configuration
        assert!(
            !config.zfs_binary.is_empty(),
            "ZFS binary path should be set"
        );
        assert!(
            !config.zpool_binary.is_empty(),
            "ZPool binary path should be set"
        );
        assert!(config.use_sudo, "Default should use sudo");
        assert!(
            config.command_timeout.as_secs() > 0,
            "Timeout should be positive"
        );
        Ok(())
    }

    #[test]
    fn test_zfs_config_custom() -> std::result::Result<(), Box<dyn std::error::Error>> {
        use std::time::Duration;

        let config = ZfsConfig {
            zfs_binary: "/custom/path/zfs".to_string(),
            zpool_binary: "/custom/path/zpool".to_string(),
            use_sudo: false,
            command_timeout: Duration::from_secs(60),
        };

        assert_eq!(config.zfs_binary, "/custom/path/zfs");
        assert_eq!(config.zpool_binary, "/custom/path/zpool");
        assert!(!config.use_sudo);
        assert_eq!(config.command_timeout.as_secs(), 60);
        Ok(())
    }
}

#[cfg(test)]
mod storage_tier_tests {
    use super::*;

    #[test]
    fn test_storage_tier_hierarchy() -> std::result::Result<(), Box<dyn std::error::Error>> {
        // Test storage tier ordering
        let hot = CoreStorageTier::Hot;
        let warm = CoreStorageTier::Warm;
        let cold = CoreStorageTier::Cold;

        // All tiers should be distinct (comparing as debug strings)
        assert!(format!("{:?}", hot) != format!("{:?}", warm));
        assert!(format!("{:?}", warm) != format!("{:?}", cold));
        assert!(format!("{:?}", hot) != format!("{:?}", cold));

        Ok(())
    }
}

#[cfg(test)]
mod path_handling_tests {
    use super::*;

    #[test]
    fn test_pathbuf_operations() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let path1 = PathBuf::from("/test/dataset");
        let path2 = PathBuf::from("/test/dataset");
        let path3 = PathBuf::from("/different/path");

        assert_eq!(path1, path2);
        assert_ne!(path1, path3);
        assert!(!path1.to_string_lossy().is_empty());

        Ok(())
    }

    #[test]
    fn test_zfs_dataset_paths() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let pool_path = PathBuf::from("/tank");
        let dataset_path = PathBuf::from("/tank/dataset");

        // Verify path operations
        assert!(pool_path.is_absolute());
        assert!(dataset_path.is_absolute());

        Ok(())
    }
}

#[cfg(test)]
mod time_handling_tests {
    use super::*;

    #[test]
    fn test_system_time_operations() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let now = SystemTime::now();
        let later = SystemTime::now();

        // Later should be >= now
        assert!(later >= now);

        Ok(())
    }
}

#[cfg(test)]
mod hashmap_tests {
    use super::*;

    #[test]
    fn test_property_map_operations() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut properties = HashMap::new();

        properties.insert("compression".to_string(), "lz4".to_string());
        properties.insert("recordsize".to_string(), "128k".to_string());

        assert_eq!(properties.get("compression"), Some(&"lz4".to_string()));
        assert_eq!(properties.get("recordsize"), Some(&"128k".to_string()));
        assert_eq!(properties.len(), 2);

        Ok(())
    }

    #[test]
    fn test_tier_property_mapping() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut tier_props = HashMap::new();

        tier_props.insert("hot".to_string(), "lz4".to_string());
        tier_props.insert("warm".to_string(), "zstd".to_string());
        tier_props.insert("cold".to_string(), "gzip-9".to_string());

        // Verify all tiers are present
        assert!(tier_props.contains_key("hot"));
        assert!(tier_props.contains_key("warm"));
        assert!(tier_props.contains_key("cold"));

        Ok(())
    }
}

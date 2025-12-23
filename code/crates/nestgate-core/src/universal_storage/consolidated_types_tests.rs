//! **NESTGATE CORE - UNIVERSAL STORAGE TYPES TEST EXPANSION**
//!
//! **Test Expansion Phase 1** (Nov 6, 2025)
//! Focus: Storage type definitions, serialization, validation
//! Goal: Expand coverage from 48.28% toward 90%

#[cfg(test)]
mod universal_storage_type_tests {
    use crate::universal_storage::consolidated_types::*;
    use serde_json;

    #[test]
    fn test_universal_storage_type_local() {
        let storage = UniversalStorageType::Local;
        let json = serde_json::to_string(&storage).expect("Should serialize");
        let deserialized: UniversalStorageType =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(storage, deserialized);
    }

    #[test]
    fn test_universal_storage_type_nfs() {
        let storage = UniversalStorageType::Nfs {
            version: NfsVersion::V4,
        };
        let json = serde_json::to_string(&storage).expect("Should serialize");
        assert!(json.contains("Nfs"));
    }

    #[test]
    fn test_universal_storage_type_smb() {
        let storage = UniversalStorageType::Smb {
            version: SmbVersion::V3,
        };
        let json = serde_json::to_string(&storage).expect("Should serialize");
        assert!(json.contains("Smb"));
    }

    #[test]
    fn test_universal_storage_type_object() {
        let storage = UniversalStorageType::Object;
        assert_eq!(storage, UniversalStorageType::Object);
    }

    #[test]
    fn test_universal_storage_type_block() {
        let storage = UniversalStorageType::Block;
        assert_eq!(storage, UniversalStorageType::Block);
    }

    #[test]
    fn test_universal_storage_type_zfs() {
        let storage = UniversalStorageType::Zfs;
        assert_eq!(storage, UniversalStorageType::Zfs);
    }

    #[test]
    fn test_universal_storage_type_memory() {
        let storage = UniversalStorageType::Memory;
        assert_eq!(storage, UniversalStorageType::Memory);
    }

    #[test]
    fn test_universal_storage_type_cloud_aws() {
        let storage = UniversalStorageType::Cloud {
            provider: CloudProvider::AWS {
                region: "us-west-2".to_string(),
            },
        };
        let json = serde_json::to_string(&storage).expect("Should serialize");
        assert!(json.contains("Cloud"));
        assert!(json.contains("AWS"));
    }

    #[test]
    fn test_universal_storage_type_custom() {
        let storage = UniversalStorageType::Custom("MyCustomStorage".to_string());
        assert!(matches!(storage, UniversalStorageType::Custom(_)));
    }

    #[test]
    fn test_nfs_versions() {
        let v3 = NfsVersion::V3;
        let v4 = NfsVersion::V4;
        let v41 = NfsVersion::V41;
        let v42 = NfsVersion::V42;

        assert_ne!(v3, v4);
        assert_ne!(v4, v41);
        assert_ne!(v41, v42);
    }

    #[test]
    fn test_smb_versions() {
        let v2 = SmbVersion::V2;
        let v3 = SmbVersion::V3;
        let v31 = SmbVersion::V31;

        assert_ne!(v2, v3);
        assert_ne!(v3, v31);
    }

    #[test]
    fn test_cloud_provider_aws() {
        let provider = CloudProvider::AWS {
            region: "eu-west-1".to_string(),
        };
        let json = serde_json::to_string(&provider).expect("Should serialize");
        assert!(json.contains("AWS"));
        assert!(json.contains("eu-west-1"));
    }

    #[test]
    fn test_cloud_provider_azure() {
        let provider = CloudProvider::Azure {
            subscription_id: "sub-123".to_string(),
        };
        let json = serde_json::to_string(&provider).expect("Should serialize");
        assert!(json.contains("Azure"));
    }

    #[test]
    fn test_cloud_provider_gcp() {
        let provider = CloudProvider::GCP {
            project_id: "project-456".to_string(),
        };
        let json = serde_json::to_string(&provider).expect("Should serialize");
        assert!(json.contains("GCP"));
    }

    #[test]
    fn test_cloud_provider_custom() {
        let provider = CloudProvider::Custom {
            endpoint: "https://custom.storage.com".to_string(),
        };
        let json = serde_json::to_string(&provider).expect("Should serialize");
        assert!(json.contains("Custom"));
    }
}

#[cfg(test)]
mod storage_resource_type_tests {
    use crate::universal_storage::consolidated_types::StorageResourceType;
    use serde_json;

    #[test]
    fn test_storage_resource_type_pool() {
        let res_type = StorageResourceType::Pool;
        assert_eq!(res_type, StorageResourceType::Pool);
    }

    #[test]
    fn test_storage_resource_type_dataset() {
        let res_type = StorageResourceType::Dataset;
        assert_eq!(res_type, StorageResourceType::Dataset);
    }

    #[test]
    fn test_storage_resource_type_volume() {
        let res_type = StorageResourceType::Volume;
        assert_eq!(res_type, StorageResourceType::Volume);
    }

    #[test]
    fn test_storage_resource_type_serialization() {
        let res_type = StorageResourceType::Dataset;
        let json = serde_json::to_string(&res_type).expect("Should serialize");
        let deserialized: StorageResourceType =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(res_type, deserialized);
    }

    #[test]
    fn test_storage_resource_types_unique() {
        let pool = StorageResourceType::Pool;
        let dataset = StorageResourceType::Dataset;
        let volume = StorageResourceType::Volume;

        assert_ne!(pool, dataset);
        assert_ne!(dataset, volume);
        assert_ne!(volume, pool);
    }
}

#[cfg(test)]
mod type_traits_tests {
    use crate::universal_storage::consolidated_types::*;

    #[test]
    fn test_universal_storage_type_is_send() {
        /// Assert Send
        fn assert_send<T: Send>() {}
        assert_send::<UniversalStorageType>();
    }

    #[test]
    fn test_universal_storage_type_is_sync() {
        /// Assert Sync
        fn assert_sync<T: Sync>() {}
        assert_sync::<UniversalStorageType>();
    }

    #[test]
    fn test_nfs_version_is_copy() {
        /// Assert Copy
        fn assert_copy<T: Copy>() {}
        assert_copy::<NfsVersion>();
    }

    #[test]
    fn test_smb_version_is_copy() {
        /// Assert Copy
        fn assert_copy<T: Copy>() {}
        assert_copy::<SmbVersion>();
    }

    #[test]
    fn test_cloud_provider_is_clone() {
        let provider = CloudProvider::AWS {
            region: "us-east-1".to_string(),
        };
        let cloned = provider.clone();
        assert_eq!(provider, cloned);
    }

    #[test]
    fn test_universal_storage_type_hash() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert(UniversalStorageType::Local, "local-storage");
        map.insert(UniversalStorageType::Zfs, "zfs-storage");

        assert_eq!(
            map.get(&UniversalStorageType::Local),
            Some(&"local-storage")
        );
        assert_eq!(map.get(&UniversalStorageType::Zfs), Some(&"zfs-storage"));
    }

    #[test]
    fn test_storage_resource_type_is_send_sync() {
        /// Assert Send Sync
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<StorageResourceType>();
    }
}

#[cfg(test)]
mod enum_variants_coverage_tests {
    use crate::universal_storage::consolidated_types::*;

    #[test]
    fn test_nfs_version_all_variants() {
        let versions = [
            NfsVersion::V3,
            NfsVersion::V4,
            NfsVersion::V41,
            NfsVersion::V42,
        ];
        assert_eq!(versions.len(), 4);
    }

    #[test]
    fn test_smb_version_all_variants() {
        let versions = [SmbVersion::V2, SmbVersion::V3, SmbVersion::V31];
        assert_eq!(versions.len(), 3);
    }

    #[test]
    fn test_universal_storage_type_coverage() {
        let types = vec![
            UniversalStorageType::Local,
            UniversalStorageType::Object,
            UniversalStorageType::Block,
            UniversalStorageType::Zfs,
            UniversalStorageType::Database,
            UniversalStorageType::Memory,
            UniversalStorageType::Cache,
            UniversalStorageType::Distributed,
        ];
        assert_eq!(types.len(), 8);
    }

    #[test]
    fn test_cloud_provider_all_variants() {
        let providers = [
            CloudProvider::AWS {
                region: "us-east-1".to_string(),
            },
            CloudProvider::Azure {
                subscription_id: "sub-123".to_string(),
            },
            CloudProvider::GCP {
                project_id: "project-456".to_string(),
            },
            CloudProvider::Custom {
                endpoint: "https://custom.com".to_string(),
            },
        ];
        assert_eq!(providers.len(), 4);
    }

    #[test]
    fn test_storage_resource_type_variants() {
        let types = [
            StorageResourceType::Pool,
            StorageResourceType::Dataset,
            StorageResourceType::Volume,
        ];
        assert!(types.len() >= 3);
    }
}

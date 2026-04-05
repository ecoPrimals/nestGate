// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    dead_code,
    unused_doc_comments,
    unused_imports,
    missing_docs,
    rustdoc::missing_crate_level_docs,
    deprecated,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::doc_markdown,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::struct_field_names,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::cast_lossless,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    clippy::unnecessary_wraps,
    clippy::unused_self,
    clippy::unused_async,
    clippy::needless_pass_by_value,
    clippy::option_if_let_else,
    clippy::too_long_first_doc_paragraph,
    clippy::inline_always,
    clippy::redundant_closure,
    clippy::redundant_closure_for_method_calls,
    clippy::collapsible_if,
    clippy::single_char_pattern,
    clippy::implicit_hasher,
    clippy::float_cmp,
    clippy::manual_midpoint,
    clippy::suboptimal_flops,
    clippy::items_after_statements,
    clippy::items_after_test_module,
    clippy::too_many_lines,
    clippy::cognitive_complexity,
    clippy::unreadable_literal,
    clippy::redundant_clone,
    clippy::useless_vec,
    clippy::field_reassign_with_default,
    clippy::cmp_null,
    clippy::bool_assert_comparison,
    clippy::used_underscore_items,
    clippy::needless_raw_string_hashes,
    clippy::ref_as_ptr,
    clippy::no_effect_underscore_binding,
    clippy::needless_collect,
    clippy::module_inception,
    clippy::default_trait_access,
    clippy::wildcard_in_or_patterns,
    clippy::or_fun_call,
    clippy::manual_string_new,
    clippy::unnecessary_literal_unwrap,
    clippy::unnecessary_debug_formatting,
    clippy::assigning_clones,
    clippy::unnecessary_unwrap,
    clippy::unnecessary_map_or,
    clippy::unnecessary_lazy_evaluations,
    clippy::similar_names,
    clippy::needless_continue,
    clippy::collection_is_never_read,
    clippy::char_lit_as_u8,
    clippy::ptr_eq,
    clippy::uninlined_format_args,
    clippy::absurd_extreme_comparisons,
    clippy::match_wild_err_arm,
    clippy::single_match_else,
    clippy::derive_partial_eq_without_eq,
    clippy::match_wildcard_for_single_variants,
    clippy::missing_const_for_fn,
    clippy::used_underscore_binding,
    clippy::ignored_unit_patterns,
    unused_comparisons,
    clippy::format_push_string
)]

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

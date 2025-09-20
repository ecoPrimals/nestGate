// **CONSTANTS MIGRATION HELPERS**
//! Migration Helpers functionality and utilities.
//! This module provides systematic migration utilities for replacing hardcoded values
//! with canonical constants throughout the `NestGate` codebase.

use std::collections::HashMap;

/// Comprehensive hardcoded value replacement mapping
pub struct ConstantsMigrationHelper {
    replacements: HashMap<String, String>,
}

impl ConstantsMigrationHelper {
    /// Create new migration helper with all standard replacements
    #[must_use]
    pub fn new() -> Self {
        let mut replacements = HashMap::new();

        // Network constants
        replacements.insert(
            "127.0.0.1".to_string(),
            "crate::constants::LOCALHOST".to_string(),
        );
        replacements.insert(
            "\"127.0.0.1\"".to_string(),
            "crate::constants::LOCALHOST".to_string(),
        );
        replacements.insert(
            "8080".to_string(),
            "crate::constants::DEFAULT_API_PORT".to_string(),
        );
        replacements.insert(
            "0.0.0.0".to_string(),
            "crate::constants::DEFAULT_BIND_ADDRESS".to_string(),
        );

        // Timeout constants
        replacements.insert(
            "30".to_string(),
            "crate::constants::DEFAULT_TIMEOUT_SECS".to_string(),
        );
        replacements.insert(
            "Duration::from_secs(30)".to_string(),
            "crate::constants::canonical::system::DEFAULT_TIMEOUT".to_string(),
        );

        // Size constants
        replacements.insert(
            "65536".to_string(),
            "crate::constants::DEFAULT_BUFFER_SIZE".to_string(),
        );
        replacements.insert("1024".to_string(), "crate::constants::KB".to_string());
        replacements.insert("1048576".to_string(), "crate::constants::MB".to_string());

        // Performance constants
        replacements.insert(
            "1000".to_string(),
            "crate::constants::MAX_CONNECTIONS".to_string(),
        );
        replacements.insert(
            "10000".to_string(),
            "crate::constants::MAX_CONCURRENT_REQUESTS".to_string(),
        );

        Self { replacements }
    }

    /// Get replacement for a hardcoded value
    #[must_use]
    pub const fn get_replacement(&self, hardcoded: &str) -> Option<&String> {
        self.replacements.get(hardcoded)
    }

    /// Get all replacements for reporting
    #[must_use]
    pub const fn get_all_replacements(&self) -> &HashMap<String, String> {
        &self.replacements
    }

    /// Check if a value should be replaced
    #[must_use]
    pub const fn should_replace(&self, value: &str) -> bool {
        self.replacements.contains_key(value)
    }
}

impl Default for ConstantsMigrationHelper {
    fn default() -> Self {
        Self::new()
    }
}

/// Migration statistics
#[derive(Debug, Clone, Default)]
pub struct MigrationStats {
    pub total_files_scanned: usize,
    pub files_with_replacements: usize,
    pub total_replacements: usize,
    pub replacements_by_type: HashMap<String, usize>,
}

/// Generate migration report
#[must_use]
pub const fn generate_migration_report(stats: &MigrationStats) -> String {
    format!(
        r"
🔄 **CONSTANTS MIGRATION REPORT**

📊 **Summary**:
- Files Scanned: {}
- Files Modified: {}
- Total Replacements: {}

📋 **Replacements by Type**:
{}

✅ **Status**: Migration framework ready for systematic hardcoded value replacement
",
        stats.total_files_scanned,
        stats.files_with_replacements,
        stats.total_replacements,
        stats
            .replacements_by_type
            .iter()
            .map(|(k, v)| format!("- {k}: {v}"))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_helper() {
        let helper = ConstantsMigrationHelper::new();

        assert_eq!(
            helper.get_replacement("127.0.0.1"),
            Some(&"crate::constants::LOCALHOST".to_string())
        );

        assert_eq!(
            helper.get_replacement("8080"),
            Some(&"crate::constants::DEFAULT_API_PORT".to_string())
        );

        assert!(helper.should_replace("127.0.0.1"));
        assert!(!helper.should_replace("192.168.1.1"));
    }
}

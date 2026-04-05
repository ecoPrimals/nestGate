// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **WORKING NAS INTEGRATION TESTS**
//!
//! Tests the actual NAS functionality with current API

use nestgate_nas::{NasConfig, NasService, NasStatus};

#[cfg(test)]
mod nas_config_tests {
    use super::*;

    #[test]
    fn test_nas_config_default() {
        let config = NasConfig::default();
        assert!(!config.enabled); // Default should be disabled
    }

    #[test]
    fn test_nas_service_creation() {
        let config = NasConfig {
            enabled: true,
            ..Default::default()
        };

        let _service = NasService::new(config);
        // Service should be created successfully
    }

    #[test]
    fn test_nas_service_lifecycle() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let config = NasConfig {
            enabled: true,
            ..Default::default()
        };

        let service = NasService::new(config);

        // Test service lifecycle
        service.start()?;
        let status = service.status()?;
        matches!(status, NasStatus::Running);
        service.stop()?;

        Ok(())
    }
}

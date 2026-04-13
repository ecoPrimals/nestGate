// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

impl ZfsPoolSetup {
    /// Validate device
    #[must_use]
    pub fn validate_device(&self, device: &StorageDevice) -> ValidationResult {
        self.validator.validate_device(device)
    }

    /// Validate pool configuration
    #[must_use]
    pub fn validate_pool_config(&self, config: &PoolSetupConfig) -> ValidationResult {
        self.validator.validate_pool_config(config)
    }

    /// Create pool with safety checks
    pub async fn create_pool_safe(&self, config: &PoolSetupConfig) -> CoreResult<PoolSetupResult> {
        // Pre-flight validation
        let validation = self.validate_pool_config(config);
        if !validation.is_valid {
            return Err(NestGateError::internal_error(
                format!(
                    "Pool configuration validation failed: {:?}",
                    validation.issues
                ),
                "create_pool_safe",
            ));
        }

        self.creator.create_pool_safe(config).await
    }
}

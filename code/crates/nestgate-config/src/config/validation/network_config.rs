// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! [`ConfigValidation`] implementation for canonical network configuration.

use std::collections::HashMap;

use super::types::{
    ConfigValidation, FieldDependency, FieldSchema, ValidationErrorBuilder, ValidationErrorType,
    ValidationResult, ValidationSchema, ValidationWarning, WarningSeverity,
};

/// Network configuration
///
/// **CONSOLIDATED**: Now uses `CanonicalNetworkConfig` from
/// `crate::config::canonical_primary::domains::network::CanonicalNetworkConfig`
pub use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig as NetworkConfig;

impl ConfigValidation for NetworkConfig {
    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let suggestions = Vec::new();

        if self.api.port == 0 {
            errors.push(
                ValidationErrorBuilder::new(
                    "api.port",
                    "API port cannot be 0",
                    ValidationErrorType::Required,
                )
                .build(),
            );
        }

        if self.performance.buffer_size == 0 {
            errors.push(
                ValidationErrorBuilder::new(
                    "performance.buffer_size",
                    "Buffer size cannot be 0",
                    ValidationErrorType::Required,
                )
                .build(),
            );
        }

        if self.security.firewall_enabled && self.security.allowed_ips.is_empty() {
            warnings.push(ValidationWarning {
                field: "security.firewall_enabled".to_string(),
                message:
                    "Firewall is enabled but no allowed IPs configured - this may block all traffic"
                        .to_string(),
                severity: WarningSeverity::High,
            });
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            suggestions,
        }
    }

    fn schema() -> ValidationSchema {
        let mut fields = HashMap::new();

        fields.insert(
            "bind_address".to_string(),
            FieldSchema {
                field_type: "string".to_string(),
                required: true,
                default_value: Some("127.0.0.1".to_string()),
                constraints: vec!["Valid IPv4 or IPv6 address".to_string()],
                description: "IP address to bind the server to".to_string(),
            },
        );

        fields.insert(
            "port".to_string(),
            FieldSchema {
                field_type: "u16".to_string(),
                required: true,
                default_value: Some("8080".to_string()),
                constraints: vec!["1-65535".to_string()],
                description: "Port number to listen on".to_string(),
            },
        );

        fields.insert(
            "timeout_ms".to_string(),
            FieldSchema {
                field_type: "u64".to_string(),
                required: true,
                default_value: Some("30000".to_string()),
                constraints: vec!["> 0".to_string()],
                description: "Request timeout in milliseconds".to_string(),
            },
        );

        ValidationSchema {
            fields,
            dependencies: vec![
                FieldDependency {
                    field: "tls_cert_path".to_string(),
                    depends_on: "enable_tls".to_string(),
                    condition: "required when enable_tls is true".to_string(),
                },
                FieldDependency {
                    field: "tls_key_path".to_string(),
                    depends_on: "enable_tls".to_string(),
                    condition: "required when enable_tls is true".to_string(),
                },
            ],
        }
    }
}

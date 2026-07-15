// SPDX-License-Identifier: AGPL-3.0-or-later
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
                field: "security.firewall_enabled".into(),
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
            "bind_address".into(),
            FieldSchema {
                field_type: "string".into(),
                required: true,
                default_value: Some("127.0.0.1".into()),
                constraints: vec!["Valid IPv4 or IPv6 address".into()],
                description: "IP address to bind the server to".into(),
            },
        );

        fields.insert(
            "port".into(),
            FieldSchema {
                field_type: "u16".into(),
                required: true,
                default_value: Some("8080".into()),
                constraints: vec!["1-65535".into()],
                description: "Port number to listen on".into(),
            },
        );

        fields.insert(
            "timeout_ms".into(),
            FieldSchema {
                field_type: "u64".into(),
                required: true,
                default_value: Some("30000".into()),
                constraints: vec!["> 0".into()],
                description: "Request timeout in milliseconds".into(),
            },
        );

        ValidationSchema {
            fields,
            dependencies: vec![
                FieldDependency {
                    field: "tls_cert_path".into(),
                    depends_on: "enable_tls".into(),
                    condition: "required when enable_tls is true".into(),
                },
                FieldDependency {
                    field: "tls_key_path".into(),
                    depends_on: "enable_tls".into(),
                    condition: "required when enable_tls is true".into(),
                },
            ],
        }
    }
}

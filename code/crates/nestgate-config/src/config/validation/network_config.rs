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
                field: String::from("security.firewall_enabled"),
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
            String::from("bind_address"),
            FieldSchema {
                field_type: String::from("string"),
                required: true,
                default_value: Some(String::from("127.0.0.1")),
                constraints: vec![String::from("Valid IPv4 or IPv6 address")],
                description: String::from("IP address to bind the server to"),
            },
        );

        fields.insert(
            String::from("port"),
            FieldSchema {
                field_type: String::from("u16"),
                required: true,
                default_value: Some(String::from("8080")),
                constraints: vec![String::from("1-65535")],
                description: String::from("Port number to listen on"),
            },
        );

        fields.insert(
            String::from("timeout_ms"),
            FieldSchema {
                field_type: String::from("u64"),
                required: true,
                default_value: Some(String::from("30000")),
                constraints: vec![String::from("> 0")],
                description: String::from("Request timeout in milliseconds"),
            },
        );

        ValidationSchema {
            fields,
            dependencies: vec![
                FieldDependency {
                    field: String::from("tls_cert_path"),
                    depends_on: String::from("enable_tls"),
                    condition: String::from("required when enable_tls is true"),
                },
                FieldDependency {
                    field: String::from("tls_key_path"),
                    depends_on: String::from("enable_tls"),
                    condition: String::from("required when enable_tls is true"),
                },
            ],
        }
    }
}

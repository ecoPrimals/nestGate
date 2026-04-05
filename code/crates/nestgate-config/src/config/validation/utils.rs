// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Shared validators for ports, timeouts, paths, and ranges.

use std::net::{IpAddr, SocketAddr};
use std::path::Path;
use std::time::Duration;

use super::types::{ValidationError, ValidationErrorBuilder, ValidationErrorType};

/// Validation utilities for common patterns
pub struct ValidationUtils;

impl ValidationUtils {
    /// Validate a port number
    #[must_use]
    pub fn validate_port(field: &str, port: u16) -> Option<ValidationError> {
        if port == 0 {
            Some(
                ValidationErrorBuilder::new(
                    field,
                    "Port cannot be 0",
                    ValidationErrorType::InvalidValue,
                )
                .current_value(&port.to_string())
                .expected_format("1-65535")
                .build(),
            )
        } else {
            None
        }
    }

    /// Validate a timeout duration
    #[must_use]
    pub fn validate_timeout(field: &str, timeout: Duration) -> Option<ValidationError> {
        if timeout.is_zero() {
            Some(
                ValidationErrorBuilder::new(
                    field,
                    "Timeout cannot be zero",
                    ValidationErrorType::InvalidValue,
                )
                .current_value(&format!("{timeout:?}"))
                .expected_format("> 0ms")
                .build(),
            )
        } else if timeout > Duration::from_secs(3600) {
            Some(
                ValidationErrorBuilder::new(
                    field,
                    "Timeout is unusually long (>1 hour)",
                    ValidationErrorType::OutOfRange,
                )
                .current_value(&format!("{timeout:?}"))
                .expected_format("1ms - 1 hour")
                .build(),
            )
        } else {
            None
        }
    }

    /// Validate an IP address string
    #[must_use]
    pub fn validate_ip_address(field: &str, ip: &str) -> Option<ValidationError> {
        if ip.parse::<IpAddr>().is_err() {
            Some(
                ValidationErrorBuilder::new(
                    field,
                    "Invalid IP address format",
                    ValidationErrorType::InvalidFormat,
                )
                .current_value(ip)
                .expected_format("IPv4 (e.g., 192.168.1.1) or IPv6 (e.g., ::1)")
                .build(),
            )
        } else {
            None
        }
    }

    /// Validate a socket address string
    #[must_use]
    pub fn validate_socket_address(field: &str, addr: &str) -> Option<ValidationError> {
        if addr.parse::<SocketAddr>().is_err() {
            Some(
                ValidationErrorBuilder::new(
                    field,
                    "Invalid socket address format",
                    ValidationErrorType::InvalidFormat,
                )
                .current_value(addr)
                .expected_format("IP:PORT (e.g., 127.0.0.1:8080)")
                .build(),
            )
        } else {
            None
        }
    }

    /// Validate a file path exists
    #[must_use]
    pub fn validate_file_path(field: &str, path: &Path) -> Option<ValidationError> {
        if !path.exists() {
            Some(
                ValidationErrorBuilder::new(
                    field,
                    "File does not exist",
                    ValidationErrorType::InvalidValue,
                )
                .current_value(&path.display().to_string())
                .build(),
            )
        } else if !path.is_file() {
            Some(
                ValidationErrorBuilder::new(
                    field,
                    "Path is not a file",
                    ValidationErrorType::InvalidValue,
                )
                .current_value(&path.display().to_string())
                .build(),
            )
        } else {
            None
        }
    }

    /// Validate a directory path exists
    #[must_use]
    pub fn validate_directory_path(field: &str, path: &Path) -> Option<ValidationError> {
        if !path.exists() {
            Some(
                ValidationErrorBuilder::new(
                    field,
                    "Directory does not exist",
                    ValidationErrorType::InvalidValue,
                )
                .current_value(&path.display().to_string())
                .build(),
            )
        } else if !path.is_dir() {
            Some(
                ValidationErrorBuilder::new(
                    field,
                    "Path is not a directory",
                    ValidationErrorType::InvalidValue,
                )
                .current_value(&path.display().to_string())
                .build(),
            )
        } else {
            None
        }
    }

    /// Validate a string is not empty
    #[must_use]
    pub fn validate_non_empty_string(field: &str, value: &str) -> Option<ValidationError> {
        if value.trim().is_empty() {
            Some(
                ValidationErrorBuilder::new(
                    field,
                    "Value cannot be empty",
                    ValidationErrorType::Required,
                )
                .current_value(value)
                .build(),
            )
        } else {
            None
        }
    }

    /// Validate a numeric range
    pub fn validate_range<T>(field: &str, value: T, min: T, max: T) -> Option<ValidationError>
    where
        T: PartialOrd + std::fmt::Display + Copy,
    {
        if value < min || value > max {
            Some(
                ValidationErrorBuilder::new(
                    field,
                    &format!("Value {value} is outside valid range"),
                    ValidationErrorType::OutOfRange,
                )
                .current_value(&value.to_string())
                .expected_format(&format!("{min} - {max}"))
                .build(),
            )
        } else {
            None
        }
    }
}

// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Error context and retry information
//! Error handling types and utilities.
//! This module provides context information for errors, including retry logic
//! and additional metadata.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::time::{Duration, SystemTime};

/// Error context information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Errorcontext
pub struct ErrorContext {
    /// When the error occurred
    pub timestamp: SystemTime,
    /// Operation that was being performed
    pub operation: Cow<'static, str>,
    /// Component that generated the error
    pub component: Cow<'static, str>,
    /// Request ID for tracing
    pub request_id: Option<Cow<'static, str>>,
    /// User ID if available
    pub user_id: Option<Cow<'static, str>>,
    /// Additional context data
    pub metadata: std::collections::HashMap<String, String>,
}

impl Default for ErrorContext {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            timestamp: SystemTime::now(),
            operation: Cow::Borrowed("unknown"),
            component: Cow::Borrowed("unknown"),
            request_id: None,
            user_id: None,
            metadata: std::collections::HashMap::new(),
        }
    }
}

/// Retry information for recoverable errors
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Retryinfo
pub struct RetryInfo {
    /// Current retry attempt
    pub attempt: u32,
    /// Maximum retry attempts
    pub max_attempts: u32,
    /// Delay between retries
    pub delay: Duration,
    /// Backoff multiplier
    pub backoff_multiplier: f64,
    /// Maximum delay
    pub max_delay: Duration,
    /// Next retry time
    pub next_retry: SystemTime,
    /// Jitter amount (0.0 to 1.0)
    pub jitter: f64,
}

impl Default for RetryInfo {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            attempt: 0,
            max_attempts: 3,
            delay: Duration::from_millis(100),
            backoff_multiplier: 2.0,
            max_delay: Duration::from_secs(30),
            next_retry: SystemTime::now(),
            jitter: 0.1,
        }
    }
}

#[cfg(test)]
mod context_tests {
    use super::*;

    #[test]
    fn error_context_default_serde_roundtrip() {
        let ctx = ErrorContext::default();
        let json = serde_json::to_string(&ctx).expect("serialize");
        let back: ErrorContext = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(ctx.operation, back.operation);
        assert_eq!(ctx.component, back.component);
    }

    #[test]
    fn retry_info_default_serde_roundtrip() {
        let r = RetryInfo::default();
        let json = serde_json::to_string(&r).expect("serialize");
        let back: RetryInfo = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(r.attempt, back.attempt);
        assert_eq!(r.max_attempts, back.max_attempts);
    }
}

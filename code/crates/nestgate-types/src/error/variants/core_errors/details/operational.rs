// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Testing, performance, handler, load balancer, and not-implemented error details.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::error::context::ErrorContext;

/// Testing error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Testing error: {message}")]
/// Testingerrordetails
pub struct TestingErrorDetails {
    /// Error message
    pub message: Cow<'static, str>,
    /// Test name that failed
    pub test_name: Option<Cow<'static, str>>,
    /// Type of test
    pub test_type: Option<TestType>,
    /// Assertion failure details
    pub assertion_failure: Option<Cow<'static, str>>,
    /// Expected value
    pub expected: Option<Cow<'static, str>>,
    /// Actual value
    pub actual: Option<Cow<'static, str>>,
    /// Test-specific error data
    pub test_data: Option<Box<Self>>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// Performance error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Performance error: {message}")]
/// Performanceerrordetails
pub struct PerformanceErrorDetails {
    /// Error message
    pub message: Cow<'static, str>,
    /// Performance operation that failed
    pub operation: Cow<'static, str>,
    /// Metric that failed
    pub metric: Option<Cow<'static, str>>,
    /// Expected performance value
    pub expected: Option<f64>,
    /// Actual performance value
    pub actual: Option<f64>,
    /// Performance unit (ms, MB/s, etc.)
    pub unit: Option<Cow<'static, str>>,
    /// Performance-specific error data
    pub performance_data: Option<Box<Self>>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// Handler error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Handler error in {handler_name}: {message}")]
/// Handlererrordetails
pub struct HandlerErrorDetails {
    /// Error message
    pub message: Cow<'static, str>,
    /// Handler name that failed
    pub handler_name: Cow<'static, str>,
    /// Request that was being handled
    pub request_info: Option<Cow<'static, str>>,
    /// Handler-specific error data
    pub handler_data: Option<Box<Self>>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// Load balancer error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Load balancer error: {message}")]
/// Loadbalancererrordetails
pub struct LoadBalancerErrorDetails {
    /// Error message
    pub message: Cow<'static, str>,
    /// Number of available services
    pub available_services: Option<usize>,
    /// Algorithm being used
    pub algorithm: Option<Cow<'static, str>>,
}

/// Not implemented error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Not implemented: {feature}")]
/// Notimplementederrordetails
pub struct NotImplementedErrorDetails {
    /// Feature that is not implemented
    pub feature: Cow<'static, str>,
    /// Additional context
    pub message: Option<Cow<'static, str>>,
    /// Planned version for implementation
    pub planned_version: Option<Cow<'static, str>>,
}

/// Test type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Test
pub enum TestType {
    /// Unit
    Unit,
    /// Integration
    Integration,
    /// E2E
    E2E,
    /// Performance
    Performance,
    /// Security
    Security,
    /// Chaos
    Chaos,
}

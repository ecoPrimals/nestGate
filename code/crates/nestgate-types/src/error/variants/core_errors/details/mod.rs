// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Domain-specific error detail structs for the unified error type.

mod automation_system;
mod config_api;
mod operational;
mod storage_network_security;
mod validation_io_resource;

pub use automation_system::{
    AutomationErrorDetails, ExternalErrorDetails, InternalErrorDetails, SystemErrorDetails,
};
pub use config_api::{ApiErrorDetails, ConfigurationErrorDetails};
pub use operational::{
    HandlerErrorDetails, LoadBalancerErrorDetails, NotImplementedErrorDetails,
    PerformanceErrorDetails, TestType, TestingErrorDetails,
};
pub use storage_network_security::{
    NetworkErrorDetails, SecurityErrorDetails, StorageErrorDetails,
};
pub use validation_io_resource::{
    IoErrorDetails, ResourceExhaustedErrorDetails, TimeoutErrorDetails, ValidationErrorDetails,
};

// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// **AUTOMATION ERROR UTILITIES**
//! Automation-specific error types and handling for the `NestGate` system.
// Automation system-specific error handling utilities.

use std::borrow::Cow;

use super::core_errors::NestGateUnifiedError;

impl NestGateUnifiedError {
    /// Create a new automation error
    pub fn automation(message: impl Into<Cow<'static, str>>) -> Self {
        Self::Automation(Box::new(
            crate::error::variants::core_errors::AutomationErrorDetails {
                message: message.into(),
                operation: Some(Cow::Borrowed("automation")),
                target: None,
                automation_data: None,
                context: None,
            },
        ))
    }

    /// Create an automation operation error
    pub fn automation_operation(
        message: impl Into<Cow<'static, str>>,
        target: Option<Cow<'static, str>>,
    ) -> Self {
        Self::Automation(Box::new(
            crate::error::variants::core_errors::AutomationErrorDetails {
                message: message.into(),
                operation: Some(Cow::Borrowed("automation_operation")),
                target,
                automation_data: None,
                context: None,
            },
        ))
    }
}

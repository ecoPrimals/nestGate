// **AUTOMATION ERROR UTILITIES**
//! Automation-specific error types and handling for the `NestGate` system.
// Automation system-specific error handling utilities.

use super::core_errors::NestGateUnifiedError;

impl NestGateUnifiedError {
    /// Create a new automation error
    pub fn automation(message: impl Into<String>) -> Self {
        Self::Automation(Box::new(
            crate::error::variants::core_errors::AutomationErrorDetails {
                message: message.into(),
                operation: Some("automation".to_string()),
                target: None,
                automation_data: None,
                context: None,
            },
        ))
    }

    /// Create an automation operation error
    pub fn automation_operation(message: impl Into<String>, target: Option<String>) -> Self {
        Self::Automation(Box::new(
            crate::error::variants::core_errors::AutomationErrorDetails {
                message: message.into(),
                operation: Some("automation_operation".to_string()),
                target,
                automation_data: None,
                context: None,
            },
        ))
    }
}

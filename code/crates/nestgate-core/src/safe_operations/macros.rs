// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

/// Safety macros for easy migration from `unwrap()` and `expect()` patterns
/// **SAFE UNWRAP MACRO**
/// Easy replacement for .`map_err` patterns
#[macro_export]
macro_rules! safe_unwrap {
    ($expr:expr, $context:expr) => {
        $expr.map_err(|e| $crate::NestGateError::Internal {
            message: format!("Safe unwrap failed: {} - Context: {}", e, $context),
            component: "safe_operations".to_string(),
        })?
    };
}
/// **SAFE EXPECT MACRO**
/// Easy replacement for .`expect()` calls
#[macro_export]
macro_rules! safe_expect {
    ($expr:expr, $msg:expr) => {
        $expr.map_err(|_| $crate::NestGateError::Internal {
            message: format!("Safe expect failed: {$msg}"),
            component: "safe_operations".to_string(),
        })?
    };
}

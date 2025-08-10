/// Safety macros for easy migration from unwrap() and expect() patterns
/// **SAFE UNWRAP MACRO**
/// Easy replacement for .map_err patterns
#[macro_export]
macro_rules! safe_unwrap {
    ($expr:expr, $context:expr) => {
        $expr.map_err(|e| $crate::NestGateError::Internal {
            message: format!("Operation failed in context '{}': {:?}", $context, e),
            location: Some(format!("{}:{}", file!(), line!())),
            debug_info: Some(format!("Context: {}", $context)),
            is_bug: false,
        })?
    };
}

/// **SAFE EXPECT MACRO**
/// Easy replacement for .expect() calls
#[macro_export]
macro_rules! safe_expect {
    ($expr:expr, $msg:expr) => {
        $expr.map_err(|_| $crate::NestGateError::Internal {
            message: format!("Expectation failed: {}", $msg),
            location: Some(format!("{}:{}", file!(), line!())),
            debug_info: Some(format!("Message: {}", $msg)),
            is_bug: false,
        })?
    };
}


/// Validation error types
#[derive(Debug, Clone)]
pub enum ValidationError {
    /// Invalid format
    InvalidFormat { field: String, reason: String },
    /// Missing required field
    MissingField { field: String },
    /// Value out of range
    OutOfRange {
        field: String,
        min: Option<i64>,
        max: Option<i64>,
    },
}
impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InvalidFormat { field, reason } => {
                write!(f, "Invalid format for field '{field}': {reason}")
            }
            ValidationError::MissingField { field } => {
                write!(f, "Missing required field '{field}'")
            }
            ValidationError::OutOfRange { field, min, max } => {
                write!(
                    f,
                    "Field '{field}' out of range (min: {min:?}, max: {max:?})"
                )
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// Validation result type - specialized for ValidationError
/// 
/// **Note**: This is a domain-specific Result type, not the deprecated ValidationResult
/// from the old unified_result_system. This uses the local ValidationError.
type ValidationResult<T> = Result<T, ValidationError>;

/// Basic validation utilities
pub mod utils {
    use super::{ValidationError, ValidationResult};
    /// Validate that a string is not empty
    pub fn validate_non_empty(field: &str, value: &str) -> ValidationResult<()> {
        if value.trim().is_empty() {
            Err(ValidationError::MissingField {
                field: field.to_string(),
            })
        } else {
            Ok(())
        }
    }

    /// Validate that a number is within range
    pub fn validate_range(
        field: &str,
        value: i64,
        min: Option<i64>,
        max: Option<i64>,
    ) -> ValidationResult<()> {
        if let Some(min_val) = min {
            if value < min_val {
                return Err(ValidationError::OutOfRange {
                    field: field.to_string(),
                    min,
                    max,
                );
            }
        }
        if let Some(max_val) = max {
            if value > max_val {
                return Err(ValidationError::OutOfRange {
                    field: field.to_string(),
                    min,
                    max,
                );
            }
        }
        Ok(())
    }
}

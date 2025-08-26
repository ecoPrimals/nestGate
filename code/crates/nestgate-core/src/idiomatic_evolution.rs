// Provides idiomatic patterns and utilities

/// Smart default trait for intelligent defaults
pub trait SmartDefault {
    /// Create a smart default instance
    fn smart_default() -> Self;

    /// Check if this type can derive default
    fn can_derive_default() -> bool {
        true
    }
}

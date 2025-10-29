//! Optimization suggestions and zero-copy strategies

/// Optimization suggestion
#[derive(Debug, Clone)]
pub struct OptimizationSuggestion {
    /// Suggestion description
    pub description: String,
}

/// Suggestion engine
#[derive(Debug, Clone)]
pub struct SuggestionEngine;

/// Zero-copy strategy
#[derive(Debug, Clone)]
pub enum ZeroCopyStrategy {
    /// Use borrowing
    Borrowing,
    /// Use Cow
    Cow,
    /// Use Arc
    Arc,
} 
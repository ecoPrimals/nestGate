//
// **CANONICAL MODERNIZATION**: Idiomatic Rust patterns and evolution traits
// that provide smart defaults and modern Rust idioms.

/// **SMART DEFAULT TRAIT**
/// 
/// Provides intelligent default implementations that go beyond std::default::Default
/// by considering context and providing more meaningful defaults.
pub trait SmartDefault {
    /// Create a smart default instance with context awareness
    fn smart_default() -> Self;
    
    /// Create a smart default with additional context
    fn smart_default_with_context(context: &str) -> Self
    where 
        Self: Sized,
    {
        // Default implementation ignores context and falls back to smart_default
        let _ = context;
        Self::smart_default()
    }
    
    /// Check if the type can derive default automatically
    fn can_derive_default() -> bool {
        false // Default conservative implementation
    }
}

/// **IDIOMATIC BUILDER TRAIT**
/// 
/// Provides idiomatic builder patterns for complex types
pub trait IdiomaticBuilder<T> {
    /// Create a new builder instance
    fn builder() -> Self;
    
    /// Build the final instance
    fn build(self) -> T;
}

/// **SMART CLONE TRAIT**
/// 
/// Provides context-aware cloning that can optimize based on usage patterns
pub trait SmartClone {
    /// Clone with optimization hints
    fn smart_clone(&self) -> Self;
    
    /// Clone with specific context for optimization
    fn smart_clone_with_context(&self, context: &str) -> Self
    where 
        Self: Sized,
    {
        // Default implementation ignores context
        let _ = context;
        self.smart_clone()
    }
}

/// **EVOLUTION METADATA**
/// 
/// Tracks evolution and modernization metadata for types
#[derive(Debug, Clone)]
pub struct EvolutionMetadata {
    /// Version of the evolution
    pub version: String,
    /// Migration path information
    pub migration_path: String,
    /// Compatibility notes
    pub compatibility_notes: Vec<String>,
}

impl Default for EvolutionMetadata {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            migration_path: "canonical_modernization".to_string(),
            compatibility_notes: vec!["Canonical modernization compatible".to_string()],
        }
    }
}

/// **IDIOMATIC PATTERNS**
/// 
/// Common idiomatic Rust patterns for the canonical modernization
pub mod patterns {
    use super::*;
    
    /// Create a smart default with error handling
    pub fn safe_smart_default<T: SmartDefault>() -> Result<T, &'static str> {
        Ok(T::smart_default())
    }
    
    /// Apply evolution metadata to a type
    pub fn with_evolution_metadata<T>(value: T, metadata: EvolutionMetadata) -> (T, EvolutionMetadata) {
        (value, metadata)
    }
}

// Implement SmartDefault for common types
impl SmartDefault for String {
    fn smart_default() -> Self {
        String::new()
    }
}

impl SmartDefault for u16 {
    fn smart_default() -> Self {
        8080 // Smart default for port numbers
    }
}

impl SmartDefault for u32 {
    fn smart_default() -> Self {
        0
    }
}

impl SmartDefault for u64 {
    fn smart_default() -> Self {
        0
    }
}

impl SmartDefault for usize {
    fn smart_default() -> Self {
        0
    }
}

impl SmartDefault for bool {
    fn smart_default() -> Self {
        false
    }
} 
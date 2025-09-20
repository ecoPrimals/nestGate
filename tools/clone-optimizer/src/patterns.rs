//! Clone detection patterns and optimization strategies
//!
//! This module defines the patterns used to detect clone operations and
//! the strategies for optimizing them. It forms the core of the clone
//! optimization engine.

pub mod builtin;
pub mod custom;
pub mod registry;

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use builtin::*;
pub use custom::*;
pub use registry::*;

/// A pattern for detecting and optimizing clone operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClonePattern {
    /// Unique identifier for this pattern
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Pattern type classification
    pub pattern_type: ClonePatternType,
    /// Regex pattern for detection
    pub regex: String,
    /// Compiled regex (not serialized)
    #[serde(skip)]
    pub compiled_regex: Option<Regex>,
    /// Optimization strategy to apply
    pub strategy: OptimizationStrategy,
    /// Safety level of this optimization
    pub safety_level: crate::SafetyLevel,
    /// Performance impact assessment
    pub performance_impact: PerformanceImpact,
    /// Context requirements for safe application
    pub context_requirements: Vec<ContextRequirement>,
    /// Description of what this pattern does
    pub description: String,
    /// Examples of before/after code
    pub examples: Vec<OptimizationExample>,
}

impl ClonePattern {
    /// Create a new clone pattern
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        pattern_type: ClonePatternType,
        regex: impl Into<String>,
        strategy: OptimizationStrategy,
    ) -> crate::Result<Self> {
        let regex_str = regex.into();
        let compiled_regex = Regex::new(&regex_str)?;

        Ok(Self {
            id: id.into(),
            name: name.into(),
            pattern_type,
            regex: regex_str,
            compiled_regex: Some(compiled_regex),
            strategy,
            safety_level: crate::SafetyLevel::Safe,
            performance_impact: PerformanceImpact::Medium,
            context_requirements: Vec::new(),
            description: String::new(),
            examples: Vec::new(),
        })
    }

    /// Compile the regex pattern
    ///
    /// # Errors
    ///
    /// Returns an error if the regex is invalid.
    pub fn compile(&mut self) -> crate::Result<()> {
        self.compiled_regex = Some(Regex::new(&self.regex)?);
        Ok(())
    }

    /// Check if this pattern matches the given text
    pub fn matches(&self, text: &str) -> bool {
        if let Some(ref regex) = self.compiled_regex {
            regex.is_match(text)
        } else {
            false
        }
    }

    /// Find all matches in the given text
    pub fn find_matches<'a>(&self, text: &'a str) -> Vec<regex::Match<'a>> {
        if let Some(ref regex) = self.compiled_regex {
            regex.find_iter(text).collect()
        } else {
            Vec::new()
        }
    }

    /// Set safety level
    pub fn with_safety_level(mut self, level: crate::SafetyLevel) -> Self {
        self.safety_level = level;
        self
    }

    /// Set performance impact
    pub fn with_performance_impact(mut self, impact: PerformanceImpact) -> Self {
        self.performance_impact = impact;
        self
    }

    /// Add context requirement
    pub fn with_context_requirement(mut self, requirement: ContextRequirement) -> Self {
        self.context_requirements.push(requirement);
        self
    }

    /// Set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Add example
    pub fn with_example(mut self, example: OptimizationExample) -> Self {
        self.examples.push(example);
        self
    }
}

/// Types of clone patterns
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClonePatternType {
    /// String cloning patterns
    String,
    /// Vector/collection cloning patterns
    Collection,
    /// Arc/Rc reference cloning patterns
    Reference,
    /// Custom type cloning patterns
    CustomType,
    /// Function parameter cloning patterns
    Parameter,
    /// Return value cloning patterns
    ReturnValue,
    /// Loop iteration cloning patterns
    Loop,
    /// Field access cloning patterns
    FieldAccess,
    /// Method chaining cloning patterns
    MethodChain,
    /// Serialization cloning patterns
    Serialization,
}

impl std::fmt::Display for ClonePatternType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String => write!(f, "string"),
            Self::Collection => write!(f, "collection"),
            Self::Reference => write!(f, "reference"),
            Self::CustomType => write!(f, "custom_type"),
            Self::Parameter => write!(f, "parameter"),
            Self::ReturnValue => write!(f, "return_value"),
            Self::Loop => write!(f, "loop"),
            Self::FieldAccess => write!(f, "field_access"),
            Self::MethodChain => write!(f, "method_chain"),
            Self::Serialization => write!(f, "serialization"),
        }
    }
}

/// Optimization strategies for clone operations
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptimizationStrategy {
    /// Replace with borrowing (&T)
    UseBorrowing {
        /// Suggested lifetime parameter if needed
        lifetime: Option<String>,
    },
    /// Replace with Arc/Rc sharing
    UseSharedReference {
        /// Whether to use Arc (true) or Rc (false)
        use_arc: bool,
    },
    /// Replace with Cow (Clone on Write)
    UseCow {
        /// The type to use for Cow
        cow_type: String,
    },
    /// Move ownership instead of cloning
    MoveOwnership,
    /// Use slice references
    UseSliceReference,
    /// Use string slices
    UseStringSlice,
    /// Lazy evaluation with closures
    UseLazyEvaluation,
    /// Zero-copy deserialization
    UseZeroCopyDeserialization,
    /// Remove unnecessary clone
    RemoveClone,
    /// Replace with more efficient alternative
    UseAlternative {
        /// The alternative approach
        alternative: String,
    },
}

impl std::fmt::Display for OptimizationStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UseBorrowing { .. } => write!(f, "use_borrowing"),
            Self::UseSharedReference { use_arc, .. } => {
                write!(f, "use_{}", if *use_arc { "arc" } else { "rc" })
            }
            Self::UseCow { .. } => write!(f, "use_cow"),
            Self::MoveOwnership => write!(f, "move_ownership"),
            Self::UseSliceReference => write!(f, "use_slice_reference"),
            Self::UseStringSlice => write!(f, "use_string_slice"),
            Self::UseLazyEvaluation => write!(f, "use_lazy_evaluation"),
            Self::UseZeroCopyDeserialization => write!(f, "use_zerocopy_deserialization"),
            Self::RemoveClone => write!(f, "remove_clone"),
            Self::UseAlternative { alternative } => write!(f, "use_alternative_{alternative}"),
        }
    }
}

/// Performance impact levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PerformanceImpact {
    /// Minimal impact (< 100 bytes)
    Minimal,
    /// Low impact (100 bytes - 1KB)
    Low,
    /// Medium impact (1KB - 10KB)
    Medium,
    /// High impact (10KB - 100KB)
    High,
    /// Critical impact (> 100KB)
    Critical,
}

impl PerformanceImpact {
    /// Get the byte threshold for this impact level
    pub fn byte_threshold(self) -> u64 {
        match self {
            Self::Minimal => 100,
            Self::Low => 1_000,
            Self::Medium => 10_000,
            Self::High => 100_000,
            Self::Critical => u64::MAX,
        }
    }

    /// Create from byte size
    pub fn from_bytes(bytes: u64) -> Self {
        match bytes {
            0..=99 => Self::Minimal,
            100..=999 => Self::Low,
            1_000..=9_999 => Self::Medium,
            10_000..=99_999 => Self::High,
            _ => Self::Critical,
        }
    }
}

impl std::fmt::Display for PerformanceImpact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Minimal => write!(f, "minimal"),
            Self::Low => write!(f, "low"),
            Self::Medium => write!(f, "medium"),
            Self::High => write!(f, "high"),
            Self::Critical => write!(f, "critical"),
        }
    }
}

/// Context requirements for safe optimization
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContextRequirement {
    /// Must be in a function context
    InFunction,
    /// Must be in a loop context
    InLoop,
    /// Must be in an async context
    InAsync,
    /// Must have specific return type
    HasReturnType(String),
    /// Must have specific parameter type
    HasParameterType(String),
    /// Must not be in unsafe block
    NotInUnsafe,
    /// Must have specific trait bounds
    HasTraitBounds(Vec<String>),
    /// Must be in test code
    InTestCode,
    /// Must not be in test code
    NotInTestCode,
    /// Must have specific lifetime constraints
    HasLifetimeConstraints(Vec<String>),
    /// Must be thread-safe
    ThreadSafe,
    /// Must not cross thread boundaries
    SingleThreaded,
    /// Custom requirement
    Custom(String),
}

impl std::fmt::Display for ContextRequirement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InFunction => write!(f, "in_function"),
            Self::InLoop => write!(f, "in_loop"),
            Self::InAsync => write!(f, "in_async"),
            Self::HasReturnType(t) => write!(f, "has_return_type_{t}"),
            Self::HasParameterType(t) => write!(f, "has_parameter_type_{t}"),
            Self::NotInUnsafe => write!(f, "not_in_unsafe"),
            Self::HasTraitBounds(bounds) => write!(f, "has_trait_bounds_{}", bounds.join("_")),
            Self::InTestCode => write!(f, "in_test_code"),
            Self::NotInTestCode => write!(f, "not_in_test_code"),
            Self::HasLifetimeConstraints(lifetimes) => {
                write!(f, "has_lifetime_constraints_{}", lifetimes.join("_"))
            }
            Self::ThreadSafe => write!(f, "thread_safe"),
            Self::SingleThreaded => write!(f, "single_threaded"),
            Self::Custom(name) => write!(f, "custom_{name}"),
        }
    }
}

/// Example of an optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationExample {
    /// Description of the example
    pub description: String,
    /// Original code
    pub before: String,
    /// Optimized code
    pub after: String,
    /// Explanation of the optimization
    pub explanation: String,
    /// Performance benefit description
    pub benefit: String,
}

impl OptimizationExample {
    /// Create a new optimization example
    pub fn new(
        description: impl Into<String>,
        before: impl Into<String>,
        after: impl Into<String>,
    ) -> Self {
        Self {
            description: description.into(),
            before: before.into(),
            after: after.into(),
            explanation: String::new(),
            benefit: String::new(),
        }
    }

    /// Set explanation
    pub fn with_explanation(mut self, explanation: impl Into<String>) -> Self {
        self.explanation = explanation.into();
        self
    }

    /// Set benefit description
    pub fn with_benefit(mut self, benefit: impl Into<String>) -> Self {
        self.benefit = benefit.into();
        self
    }
}

/// Collection of clone patterns
#[derive(Debug, Clone, Default)]
pub struct PatternCollection {
    /// All patterns indexed by ID
    patterns: HashMap<String, ClonePattern>,
    /// Patterns grouped by type
    by_type: HashMap<ClonePatternType, Vec<String>>,
    /// Patterns grouped by strategy
    by_strategy: HashMap<OptimizationStrategy, Vec<String>>,
}

impl PatternCollection {
    /// Create a new pattern collection
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a pattern to the collection
    ///
    /// # Errors
    ///
    /// Returns an error if the pattern ID already exists.
    pub fn add_pattern(&mut self, mut pattern: ClonePattern) -> crate::Result<()> {
        if self.patterns.contains_key(&pattern.id) {
            return Err(crate::CloneOptimizerError::pattern(
                format!("Pattern ID '{}' already exists", pattern.id),
                pattern.id,
            ));
        }

        // Ensure regex is compiled
        if pattern.compiled_regex.is_none() {
            pattern.compile()?;
        }

        let id = pattern.id.clone();
        let pattern_type = pattern.pattern_type;
        let strategy = pattern.strategy.clone();

        self.patterns.insert(id.clone(), pattern);
        self.by_type.entry(pattern_type).or_default().push(id.clone());
        self.by_strategy.entry(strategy).or_default().push(id);

        Ok(())
    }

    /// Get a pattern by ID
    pub fn get_pattern(&self, id: &str) -> Option<&ClonePattern> {
        self.patterns.get(id)
    }

    /// Get all patterns of a specific type
    pub fn get_patterns_by_type(&self, pattern_type: ClonePatternType) -> Vec<&ClonePattern> {
        self.by_type
            .get(&pattern_type)
            .map(|ids| ids.iter().filter_map(|id| self.patterns.get(id)).collect())
            .unwrap_or_default()
    }

    /// Get all patterns using a specific strategy
    pub fn get_patterns_by_strategy(
        &self,
        strategy: &OptimizationStrategy,
    ) -> Vec<&ClonePattern> {
        self.by_strategy
            .get(strategy)
            .map(|ids| ids.iter().filter_map(|id| self.patterns.get(id)).collect())
            .unwrap_or_default()
    }

    /// Get all patterns
    pub fn get_all_patterns(&self) -> Vec<&ClonePattern> {
        self.patterns.values().collect()
    }

    /// Remove a pattern
    pub fn remove_pattern(&mut self, id: &str) -> Option<ClonePattern> {
        if let Some(pattern) = self.patterns.remove(id) {
            // Remove from type index
            if let Some(ids) = self.by_type.get_mut(&pattern.pattern_type) {
                ids.retain(|x| x != id);
            }

            // Remove from strategy index
            if let Some(ids) = self.by_strategy.get_mut(&pattern.strategy) {
                ids.retain(|x| x != id);
            }

            Some(pattern)
        } else {
            None
        }
    }

    /// Get number of patterns
    pub fn len(&self) -> usize {
        self.patterns.len()
    }

    /// Check if collection is empty
    pub fn is_empty(&self) -> bool {
        self.patterns.is_empty()
    }

    /// Clear all patterns
    pub fn clear(&mut self) {
        self.patterns.clear();
        self.by_type.clear();
        self.by_strategy.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone_pattern_creation() {
        let pattern = ClonePattern::new(
            "test_pattern",
            "Test Pattern",
            ClonePatternType::String,
            r"\.clone\(\)",
            OptimizationStrategy::UseBorrowing { lifetime: None },
        );
        
        assert!(pattern.is_ok());
        let pattern = pattern.unwrap();
        assert_eq!(pattern.id, "test_pattern");
        assert_eq!(pattern.pattern_type, ClonePatternType::String);
    }

    #[test]
    fn test_pattern_matching() {
        let mut pattern = ClonePattern::new(
            "string_clone",
            "String Clone",
            ClonePatternType::String,
            r"(\w+)\.clone\(\)",
            OptimizationStrategy::UseStringSlice,
        ).unwrap();

        assert!(pattern.matches("name.clone()"));
        assert!(pattern.matches("data.clone()"));
        assert!(!pattern.matches("name.len()"));
    }

    #[test]
    fn test_pattern_collection() {
        let mut collection = PatternCollection::new();
        
        let pattern = ClonePattern::new(
            "test1",
            "Test 1",
            ClonePatternType::String,
            r"\.clone\(\)",
            OptimizationStrategy::UseBorrowing { lifetime: None },
        ).unwrap();

        assert!(collection.add_pattern(pattern).is_ok());
        assert_eq!(collection.len(), 1);
        assert!(collection.get_pattern("test1").is_some());
    }

    #[test]
    fn test_performance_impact_from_bytes() {
        assert_eq!(PerformanceImpact::from_bytes(50), PerformanceImpact::Minimal);
        assert_eq!(PerformanceImpact::from_bytes(500), PerformanceImpact::Low);
        assert_eq!(PerformanceImpact::from_bytes(5000), PerformanceImpact::Medium);
        assert_eq!(PerformanceImpact::from_bytes(50000), PerformanceImpact::High);
        assert_eq!(PerformanceImpact::from_bytes(500000), PerformanceImpact::Critical);
    }
} 
//! Built-in clone patterns for common optimization scenarios
//!
//! This module contains a comprehensive set of pre-defined patterns that
//! cover the most common clone optimization opportunities in Rust code.

use super::*;

/// Create all built-in clone patterns
pub fn create_builtin_patterns() -> crate::Result<PatternCollection> {
    let mut collection = PatternCollection::new();

    // Add string patterns
    for pattern in create_string_patterns()? {
        collection.add_pattern(pattern)?;
    }

    // Add collection patterns
    for pattern in create_collection_patterns()? {
        collection.add_pattern(pattern)?;
    }

    // Add reference patterns
    for pattern in create_reference_patterns()? {
        collection.add_pattern(pattern)?;
    }

    // Add parameter patterns
    for pattern in create_parameter_patterns()? {
        collection.add_pattern(pattern)?;
    }

    // Add loop patterns
    for pattern in create_loop_patterns()? {
        collection.add_pattern(pattern)?;
    }

    // Add method chain patterns
    for pattern in create_method_chain_patterns()? {
        collection.add_pattern(pattern)?;
    }

    // Add serialization patterns
    for pattern in create_serialization_patterns()? {
        collection.add_pattern(pattern)?;
    }

    Ok(collection)
}

/// Create string-related clone patterns
fn create_string_patterns() -> crate::Result<Vec<ClonePattern>> {
    let mut patterns = Vec::new();

    // Basic string clone pattern
    patterns.push(
        ClonePattern::new(
            "string_clone_basic",
            "Basic String Clone",
            ClonePatternType::String,
            r"(\w+)\.clone\(\)",
            OptimizationStrategy::UseStringSlice,
        )?
        .with_safety_level(crate::SafetyLevel::Safe)
        .with_performance_impact(PerformanceImpact::Low)
        .with_context_requirement(ContextRequirement::NotInUnsafe)
        .with_description("Replace String::clone() with &str borrowing when possible")
        .with_example(
            OptimizationExample::new(
                "Basic string clone optimization",
                "fn process(data: String) { println!(\"{}\", data.clone()); }",
                "fn process(data: &str) { println!(\"{}\", data); }",
            )
            .with_explanation("Avoid unnecessary string allocation by using string slices")
            .with_benefit("Eliminates heap allocation and improves performance"),
        ),
    );

    // String clone in function parameters
    patterns.push(
        ClonePattern::new(
            "string_parameter_clone",
            "String Parameter Clone",
            ClonePatternType::Parameter,
            r"fn\s+\w+\([^)]*(\w+):\s*String[^)]*\)[^{]*\{[^}]*\1\.clone\(\)",
            OptimizationStrategy::UseBorrowing { lifetime: None },
        )?
        .with_safety_level(crate::SafetyLevel::Safe)
        .with_performance_impact(PerformanceImpact::Medium)
        .with_context_requirement(ContextRequirement::InFunction)
        .with_description("Optimize string parameters that are immediately cloned")
        .with_example(
            OptimizationExample::new(
                "String parameter optimization",
                "fn process(name: String) -> String { format!(\"Hello {}\", name.clone()) }",
                "fn process(name: &str) -> String { format!(\"Hello {}\", name) }",
            )
            .with_explanation("Use &str parameter instead of String when cloning")
            .with_benefit("Reduces function call overhead and memory allocation"),
        ),
    );

    // String concatenation with clone
    patterns.push(
        ClonePattern::new(
            "string_concat_clone",
            "String Concatenation Clone",
            ClonePatternType::String,
            r"(\w+)\.clone\(\)\s*\+\s*",
            OptimizationStrategy::UseAlternative {
                alternative: "format_macro".to_string(),
            },
        )?
        .with_safety_level(crate::SafetyLevel::Safe)
        .with_performance_impact(PerformanceImpact::Medium)
        .with_description("Optimize string concatenation that uses clone")
        .with_example(
            OptimizationExample::new(
                "String concatenation optimization",
                "let result = name.clone() + \" suffix\";",
                "let result = format!(\"{} suffix\", name);",
            )
            .with_explanation("Use format! macro instead of clone + concatenation")
            .with_benefit("More efficient string building, avoids intermediate allocation"),
        ),
    );

    Ok(patterns)
}

/// Create collection-related clone patterns
fn create_collection_patterns() -> crate::Result<Vec<ClonePattern>> {
    let mut patterns = Vec::new();

    // Vector clone in loops
    patterns.push(
        ClonePattern::new(
            "vector_loop_clone",
            "Vector Clone in Loop",
            ClonePatternType::Loop,
            r"for\s+\w+\s+in\s+(\w+)\.clone\(\)",
            OptimizationStrategy::UseSliceReference,
        )?
        .with_safety_level(crate::SafetyLevel::Safe)
        .with_performance_impact(PerformanceImpact::High)
        .with_context_requirement(ContextRequirement::InLoop)
        .with_description("Replace Vec::clone() with slice reference in loops")
        .with_example(
            OptimizationExample::new(
                "Vector loop optimization",
                "for item in vec.clone() { println!(\"{}\", item); }",
                "for item in &vec { println!(\"{}\", item); }",
            )
            .with_explanation("Iterate over reference instead of cloning entire vector")
            .with_benefit("Eliminates O(n) memory allocation and copying"),
        ),
    );

    // HashMap clone for iteration
    patterns.push(
        ClonePattern::new(
            "hashmap_iter_clone",
            "HashMap Clone for Iteration",
            ClonePatternType::Collection,
            r"(\w+)\.clone\(\)\.iter\(\)",
            OptimizationStrategy::UseSliceReference,
        )?
        .with_safety_level(crate::SafetyLevel::Safe)
        .with_performance_impact(PerformanceImpact::High)
        .with_context_requirement(ContextRequirement::NotInUnsafe)
        .with_description("Avoid cloning HashMap just for iteration")
        .with_example(
            OptimizationExample::new(
                "HashMap iteration optimization",
                "for (k, v) in map.clone().iter() { println!(\"{}: {}\", k, v); }",
                "for (k, v) in &map { println!(\"{}: {}\", k, v); }",
            )
            .with_explanation("Iterate over reference instead of cloning HashMap")
            .with_benefit("Avoids expensive HashMap cloning operation"),
        ),
    );

    // Vector clone for single access
    patterns.push(
        ClonePattern::new(
            "vector_single_access_clone",
            "Vector Clone for Single Access",
            ClonePatternType::Collection,
            r"(\w+)\.clone\(\)\[(\d+|\w+)\]",
            OptimizationStrategy::UseBorrowing { lifetime: None },
        )?
        .with_safety_level(crate::SafetyLevel::Safe)
        .with_performance_impact(PerformanceImpact::Medium)
        .with_description("Avoid cloning vector for single element access")
        .with_example(
            OptimizationExample::new(
                "Vector access optimization",
                "let item = vec.clone()[0];",
                "let item = &vec[0];",
            )
            .with_explanation("Access element by reference instead of cloning vector")
            .with_benefit("Eliminates unnecessary vector allocation"),
        ),
    );

    Ok(patterns)
}

/// Create reference counting patterns
fn create_reference_patterns() -> crate::Result<Vec<ClonePattern>> {
    let mut patterns = Vec::new();

    // Unnecessary Arc::clone
    patterns.push(
        ClonePattern::new(
            "arc_unnecessary_clone",
            "Unnecessary Arc Clone",
            ClonePatternType::Reference,
            r"Arc::clone\(&(\w+)\)",
            OptimizationStrategy::UseSharedReference { use_arc: true },
        )?
        .with_safety_level(crate::SafetyLevel::SafeWithReview)
        .with_performance_impact(PerformanceImpact::Low)
        .with_context_requirement(ContextRequirement::ThreadSafe)
        .with_description("Optimize unnecessary Arc cloning")
        .with_example(
            OptimizationExample::new(
                "Arc optimization",
                "let data_copy = Arc::clone(&data); process(data_copy);",
                "process(Arc::clone(&data));",
            )
            .with_explanation("Inline Arc::clone when intermediate variable isn't needed")
            .with_benefit("Reduces variable allocation and improves readability"),
        ),
    );

    // Rc to Arc conversion opportunity
    patterns.push(
        ClonePattern::new(
            "rc_to_arc_opportunity",
            "Rc to Arc Conversion",
            ClonePatternType::Reference,
            r"Rc::clone\(&(\w+)\)",
            OptimizationStrategy::UseSharedReference { use_arc: true },
        )?
        .with_safety_level(crate::SafetyLevel::RequiresAnalysis)
        .with_performance_impact(PerformanceImpact::Low)
        .with_context_requirement(ContextRequirement::SingleThreaded)
        .with_description("Consider Arc instead of Rc for thread safety")
        .with_example(
            OptimizationExample::new(
                "Rc to Arc conversion",
                "let shared = Rc::clone(&data);",
                "let shared = Arc::clone(&data);",
            )
            .with_explanation("Use Arc for thread-safe reference counting")
            .with_benefit("Enables sharing across thread boundaries"),
        ),
    );

    Ok(patterns)
}

/// Create function parameter patterns
fn create_parameter_patterns() -> crate::Result<Vec<ClonePattern>> {
    let mut patterns = Vec::new();

    // Function taking owned value but immediately cloning
    patterns.push(
        ClonePattern::new(
            "owned_parameter_immediate_clone",
            "Owned Parameter Immediate Clone",
            ClonePatternType::Parameter,
            r"fn\s+\w+\([^)]*(\w+):\s*(\w+)[^)]*\)[^{]*\{[^}]*let\s+\w+\s*=\s*\1\.clone\(\)",
            OptimizationStrategy::UseBorrowing { lifetime: None },
        )?
        .with_safety_level(crate::SafetyLevel::Safe)
        .with_performance_impact(PerformanceImpact::Medium)
        .with_context_requirement(ContextRequirement::InFunction)
        .with_description("Optimize functions that take owned values but immediately clone")
        .with_example(
            OptimizationExample::new(
                "Parameter clone optimization",
                "fn process(data: Vec<String>) { let copy = data.clone(); /* use copy */ }",
                "fn process(data: &[String]) { /* use data directly */ }",
            )
            .with_explanation("Accept borrowed parameter instead of owned + clone")
            .with_benefit("Eliminates unnecessary ownership transfer and cloning"),
        ),
    );

    Ok(patterns)
}

/// Create loop-related patterns
fn create_loop_patterns() -> crate::Result<Vec<ClonePattern>> {
    let mut patterns = Vec::new();

    // Clone in loop body
    patterns.push(
        ClonePattern::new(
            "loop_body_clone",
            "Clone in Loop Body",
            ClonePatternType::Loop,
            r"for\s+\w+\s+in[^{]+\{[^}]*(\w+)\.clone\(\)",
            OptimizationStrategy::UseSliceReference,
        )?
        .with_safety_level(crate::SafetyLevel::Safe)
        .with_performance_impact(PerformanceImpact::High)
        .with_context_requirement(ContextRequirement::InLoop)
        .with_description("Optimize cloning within loop bodies")
        .with_example(
            OptimizationExample::new(
                "Loop body clone optimization",
                "for i in 0..10 { let copy = data.clone(); process(copy); }",
                "for i in 0..10 { process(&data); }",
            )
            .with_explanation("Use reference instead of cloning in each iteration")
            .with_benefit("Eliminates O(n) cloning operations in loop"),
        ),
    );

    Ok(patterns)
}

/// Create method chaining patterns
fn create_method_chain_patterns() -> crate::Result<Vec<ClonePattern>> {
    let mut patterns = Vec::new();

    // Clone in method chain
    patterns.push(
        ClonePattern::new(
            "method_chain_clone",
            "Clone in Method Chain",
            ClonePatternType::MethodChain,
            r"(\w+)\.clone\(\)\.(\w+)\(",
            OptimizationStrategy::UseBorrowing { lifetime: None },
        )?
        .with_safety_level(crate::SafetyLevel::Safe)
        .with_performance_impact(PerformanceImpact::Medium)
        .with_description("Optimize cloning in method chains")
        .with_example(
            OptimizationExample::new(
                "Method chain optimization",
                "let result = data.clone().len();",
                "let result = data.len();",
            )
            .with_explanation("Call method on original value instead of clone")
            .with_benefit("Eliminates unnecessary cloning for read-only operations"),
        ),
    );

    Ok(patterns)
}

/// Create serialization patterns
fn create_serialization_patterns() -> crate::Result<Vec<ClonePattern>> {
    let mut patterns = Vec::new();

    // Clone for serialization
    patterns.push(
        ClonePattern::new(
            "serialization_clone",
            "Clone for Serialization",
            ClonePatternType::Serialization,
            r"serde_json::to_string\(&(\w+)\.clone\(\)\)",
            OptimizationStrategy::RemoveClone,
        )?
        .with_safety_level(crate::SafetyLevel::Safe)
        .with_performance_impact(PerformanceImpact::Medium)
        .with_description("Remove unnecessary cloning for serialization")
        .with_example(
            OptimizationExample::new(
                "Serialization optimization",
                "let json = serde_json::to_string(&data.clone())?;",
                "let json = serde_json::to_string(&data)?;",
            )
            .with_explanation("Serialization works with references, no clone needed")
            .with_benefit("Eliminates cloning before serialization"),
        ),
    );

    Ok(patterns)
}

/// Get patterns by optimization impact level
pub fn get_patterns_by_impact(
    collection: &PatternCollection,
    min_impact: PerformanceImpact,
) -> Vec<&ClonePattern> {
    collection
        .get_all_patterns()
        .into_iter()
        .filter(|pattern| pattern.performance_impact >= min_impact)
        .collect()
}

/// Get patterns by safety level
pub fn get_patterns_by_safety(
    collection: &PatternCollection,
    max_safety: crate::SafetyLevel,
) -> Vec<&ClonePattern> {
    collection
        .get_all_patterns()
        .into_iter()
        .filter(|pattern| pattern.safety_level <= max_safety)
        .collect()
}

/// Get high-impact patterns for aggressive optimization
pub fn get_high_impact_patterns(collection: &PatternCollection) -> Vec<&ClonePattern> {
    get_patterns_by_impact(collection, PerformanceImpact::Medium)
}

/// Get safe patterns for conservative optimization
pub fn get_safe_patterns(collection: &PatternCollection) -> Vec<&ClonePattern> {
    get_patterns_by_safety(collection, crate::SafetyLevel::Safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_builtin_patterns() {
        let collection = create_builtin_patterns().unwrap();
        assert!(!collection.is_empty());
        
        // Should have patterns of different types
        assert!(!collection.get_patterns_by_type(ClonePatternType::String).is_empty());
        assert!(!collection.get_patterns_by_type(ClonePatternType::Collection).is_empty());
        assert!(!collection.get_patterns_by_type(ClonePatternType::Reference).is_empty());
    }

    #[test]
    fn test_string_patterns() {
        let patterns = create_string_patterns().unwrap();
        assert!(!patterns.is_empty());
        
        // Test basic string clone pattern
        let basic_pattern = patterns.iter()
            .find(|p| p.id == "string_clone_basic")
            .unwrap();
        
        assert!(basic_pattern.matches("name.clone()"));
        assert!(basic_pattern.matches("data.clone()"));
        assert!(!basic_pattern.matches("name.len()"));
    }

    #[test]
    fn test_collection_patterns() {
        let patterns = create_collection_patterns().unwrap();
        assert!(!patterns.is_empty());
        
        // Test vector loop pattern
        let loop_pattern = patterns.iter()
            .find(|p| p.id == "vector_loop_clone")
            .unwrap();
        
        assert!(loop_pattern.matches("for item in vec.clone()"));
        assert!(!loop_pattern.matches("for item in vec"));
    }

    #[test]
    fn test_pattern_filtering() {
        let collection = create_builtin_patterns().unwrap();
        
        let high_impact = get_high_impact_patterns(&collection);
        let safe_patterns = get_safe_patterns(&collection);
        
        assert!(!high_impact.is_empty());
        assert!(!safe_patterns.is_empty());
        
        // High impact patterns should include medium+ impact
        for pattern in &high_impact {
            assert!(pattern.performance_impact >= PerformanceImpact::Medium);
        }
        
        // Safe patterns should only include safe level
        for pattern in &safe_patterns {
            assert!(pattern.safety_level <= crate::SafetyLevel::Safe);
        }
    }
} 
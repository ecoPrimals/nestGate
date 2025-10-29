//! Custom clone patterns for domain-specific optimization scenarios
//!
//! This module allows users to define their own clone patterns for
//! project-specific optimization opportunities.

use super::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Custom pattern definition that can be loaded from configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomPatternDefinition {
    /// Pattern metadata
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Description of what this pattern does
    pub description: String,
    
    /// Pattern matching
    pub pattern_type: ClonePatternType,
    /// Regex pattern for matching
    pub regex: String,
    /// Optimization strategy to apply
    pub strategy: OptimizationStrategy,
    
    /// Safety and performance
    pub safety_level: crate::SafetyLevel,
    /// Performance impact assessment
    pub performance_impact: PerformanceImpact,
    /// Context requirements for safe application
    pub context_requirements: Vec<ContextRequirement>,
    
    /// Examples and documentation
    pub examples: Vec<OptimizationExample>,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Whether this pattern is enabled
    pub enabled: bool,
}

impl CustomPatternDefinition {
    /// Convert to a ClonePattern
    pub fn to_pattern(&self) -> crate::Result<ClonePattern> {
        let mut pattern = ClonePattern::new(
            &self.id,
            &self.name,
            self.pattern_type,
            &self.regex,
            self.strategy.clone(),
        )?;

        pattern = pattern
            .with_safety_level(self.safety_level)
            .with_performance_impact(self.performance_impact)
            .with_description(&self.description);

        for requirement in &self.context_requirements {
            pattern = pattern.with_context_requirement(requirement.clone());
        }

        for example in &self.examples {
            pattern = pattern.with_example(example.clone());
        }

        Ok(pattern)
    }
}

/// Custom pattern manager
#[derive(Debug, Clone)]
pub struct CustomPatternManager {
    /// Loaded custom patterns
    patterns: Vec<CustomPatternDefinition>,
    /// Pattern collection cache
    collection_cache: Option<PatternCollection>,
}

impl Default for CustomPatternManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomPatternManager {
    /// Create a new custom pattern manager
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
            collection_cache: None,
        }
    }

    /// Load custom patterns from a TOML file
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
    pub fn load_from_file(&mut self, path: &Path) -> crate::Result<()> {
        let content = std::fs::read_to_string(path)?;
        self.load_from_toml(&content)?;
        Ok(())
    }

    /// Load custom patterns from TOML content
    ///
    /// # Errors
    ///
    /// Returns an error if the TOML cannot be parsed.
    pub fn load_from_toml(&mut self, content: &str) -> crate::Result<()> {
        #[derive(Deserialize)]
        struct CustomPatternsFile {
            patterns: Vec<CustomPatternDefinition>,
        }

        let file: CustomPatternsFile = toml::from_str(content)
            .map_err(|e| crate::CloneOptimizerError::config(format!("Invalid TOML: {e}")))?;

        self.patterns.extend(file.patterns);
        self.collection_cache = None; // Invalidate cache
        Ok(())
    }

    /// Add a custom pattern
    pub fn add_pattern(&mut self, pattern: CustomPatternDefinition) {
        self.patterns.push(pattern);
        self.collection_cache = None; // Invalidate cache
    }

    /// Remove a pattern by ID
    pub fn remove_pattern(&mut self, id: &str) -> bool {
        if let Some(pos) = self.patterns.iter().position(|p| p.id == id) {
            self.patterns.remove(pos);
            self.collection_cache = None; // Invalidate cache
            true
        } else {
            false
        }
    }

    /// Get all custom patterns
    pub fn get_patterns(&self) -> &[CustomPatternDefinition] {
        &self.patterns
    }

    /// Get enabled patterns only
    pub fn get_enabled_patterns(&self) -> Vec<&CustomPatternDefinition> {
        self.patterns.iter().filter(|p| p.enabled).collect()
    }

    /// Convert to pattern collection
    ///
    /// # Errors
    ///
    /// Returns an error if any pattern cannot be compiled.
    pub fn to_pattern_collection(&mut self) -> crate::Result<&PatternCollection> {
        if self.collection_cache.is_none() {
            let mut collection = PatternCollection::new();
            
            for pattern_def in self.get_enabled_patterns() {
                let pattern = pattern_def.to_pattern()?;
                collection.add_pattern(pattern)?;
            }
            
            self.collection_cache = Some(collection);
        }

        Ok(self.collection_cache.as_ref().unwrap())
    }

    /// Save patterns to TOML file
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written.
    pub fn save_to_file(&self, path: &Path) -> crate::Result<()> {
        #[derive(Serialize)]
        struct CustomPatternsFile {
            patterns: Vec<CustomPatternDefinition>,
        }

        let file = CustomPatternsFile {
            patterns: self.patterns.clone(),
        };

        let content = toml::to_string_pretty(&file)
            .map_err(|e| crate::CloneOptimizerError::config(format!("Serialization failed: {e}")))?;

        std::fs::write(path, content)?;
        Ok(())
    }

    /// Get patterns by tag
    pub fn get_patterns_by_tag(&self, tag: &str) -> Vec<&CustomPatternDefinition> {
        self.patterns
            .iter()
            .filter(|p| p.tags.contains(&tag.to_string()))
            .collect()
    }

    /// Enable/disable a pattern
    pub fn set_pattern_enabled(&mut self, id: &str, enabled: bool) -> bool {
        if let Some(pattern) = self.patterns.iter_mut().find(|p| p.id == id) {
            pattern.enabled = enabled;
            self.collection_cache = None; // Invalidate cache
            true
        } else {
            false
        }
    }

    /// Get pattern statistics
    pub fn get_statistics(&self) -> CustomPatternStatistics {
        let total = self.patterns.len();
        let enabled = self.patterns.iter().filter(|p| p.enabled).count();
        let by_type = {
            let mut map = std::collections::HashMap::new();
            for pattern in &self.patterns {
                *map.entry(pattern.pattern_type).or_insert(0) += 1;
            }
            map
        };
        let by_safety = {
            let mut map = std::collections::HashMap::new();
            for pattern in &self.patterns {
                *map.entry(pattern.safety_level).or_insert(0) += 1;
            }
            map
        };

        CustomPatternStatistics {
            total_patterns: total,
            enabled_patterns: enabled,
            disabled_patterns: total - enabled,
            patterns_by_type: by_type,
            patterns_by_safety: by_safety,
        }
    }
}

/// Statistics about custom patterns
#[derive(Debug, Clone)]
pub struct CustomPatternStatistics {
    /// Total number of patterns
    pub total_patterns: usize,
    /// Number of enabled patterns
    pub enabled_patterns: usize,
    /// Number of disabled patterns
    pub disabled_patterns: usize,
    /// Patterns grouped by type
    pub patterns_by_type: std::collections::HashMap<ClonePatternType, usize>,
    /// Patterns grouped by safety level
    pub patterns_by_safety: std::collections::HashMap<crate::SafetyLevel, usize>,
}

/// Create domain-specific patterns for common frameworks and libraries
pub fn create_domain_patterns() -> crate::Result<Vec<CustomPatternDefinition>> {
    let mut patterns = Vec::new();

    // Tokio/async patterns
    patterns.push(CustomPatternDefinition {
        id: "tokio_spawn_clone".to_string(),
        name: "Tokio Spawn Clone".to_string(),
        description: "Optimize cloning for tokio::spawn tasks".to_string(),
        pattern_type: ClonePatternType::Reference,
        regex: r"tokio::spawn\(async move \{[^}]*(\w+)\.clone\(\)".to_string(),
        strategy: OptimizationStrategy::UseSharedReference { use_arc: true },
        safety_level: crate::SafetyLevel::Safe,
        performance_impact: PerformanceImpact::Medium,
        context_requirements: vec![
            ContextRequirement::InAsync,
            ContextRequirement::ThreadSafe,
        ],
        examples: vec![
            OptimizationExample::new(
                "Tokio spawn optimization",
                "tokio::spawn(async move { let data_copy = data.clone(); process(data_copy).await; });",
                "let data_arc = Arc::new(data); tokio::spawn(async move { process(&data_arc).await; });",
            )
            .with_explanation("Use Arc for sharing data across async tasks")
            .with_benefit("Reduces cloning overhead in concurrent tasks"),
        ],
        tags: vec!["tokio".to_string(), "async".to_string(), "concurrency".to_string()],
        enabled: true,
    });

    // Serde patterns
    patterns.push(CustomPatternDefinition {
        id: "serde_clone_before_serialize".to_string(),
        name: "Serde Clone Before Serialize".to_string(),
        description: "Remove unnecessary cloning before serialization".to_string(),
        pattern_type: ClonePatternType::Serialization,
        regex: r"serde_json::to_\w+\(&?(\w+)\.clone\(\)\)".to_string(),
        strategy: OptimizationStrategy::RemoveClone,
        safety_level: crate::SafetyLevel::Safe,
        performance_impact: PerformanceImpact::Medium,
        context_requirements: vec![ContextRequirement::NotInUnsafe],
        examples: vec![
            OptimizationExample::new(
                "Serde serialization optimization",
                "let json = serde_json::to_string(&data.clone())?;",
                "let json = serde_json::to_string(&data)?;",
            )
            .with_explanation("Serde works with references, no need to clone")
            .with_benefit("Eliminates unnecessary cloning before serialization"),
        ],
        tags: vec!["serde".to_string(), "serialization".to_string()],
        enabled: true,
    });

    // Diesel/database patterns
    patterns.push(CustomPatternDefinition {
        id: "diesel_insert_clone".to_string(),
        name: "Diesel Insert Clone".to_string(),
        description: "Optimize cloning for database insertions".to_string(),
        pattern_type: ClonePatternType::CustomType,
        regex: r"diesel::insert_into\([^)]+\)\.values\(&?(\w+)\.clone\(\)\)".to_string(),
        strategy: OptimizationStrategy::UseBorrowing { lifetime: None },
        safety_level: crate::SafetyLevel::Safe,
        performance_impact: PerformanceImpact::Medium,
        context_requirements: vec![ContextRequirement::NotInUnsafe],
        examples: vec![
            OptimizationExample::new(
                "Diesel insert optimization",
                "diesel::insert_into(users).values(&user.clone()).execute(&conn)?;",
                "diesel::insert_into(users).values(&user).execute(&conn)?;",
            )
            .with_explanation("Diesel accepts references, no cloning needed")
            .with_benefit("Reduces memory allocation for database operations"),
        ],
        tags: vec!["diesel".to_string(), "database".to_string()],
        enabled: true,
    });

    // Actix-web patterns
    patterns.push(CustomPatternDefinition {
        id: "actix_state_clone".to_string(),
        name: "Actix State Clone".to_string(),
        description: "Optimize cloning of shared application state".to_string(),
        pattern_type: ClonePatternType::Reference,
        regex: r"(\w+)\.clone\(\)\.into\(\)".to_string(),
        strategy: OptimizationStrategy::UseSharedReference { use_arc: true },
        safety_level: crate::SafetyLevel::Safe,
        performance_impact: PerformanceImpact::Low,
        context_requirements: vec![ContextRequirement::ThreadSafe],
        examples: vec![
            OptimizationExample::new(
                "Actix state optimization",
                "HttpResponse::Ok().json(state.clone().into())",
                "HttpResponse::Ok().json(Arc::clone(&state))",
            )
            .with_explanation("Use Arc::clone for shared state access")
            .with_benefit("More efficient state sharing across handlers"),
        ],
        tags: vec!["actix".to_string(), "web".to_string()],
        enabled: true,
    });

    Ok(patterns)
}

/// Load default custom patterns for common use cases
pub fn load_default_custom_patterns() -> crate::Result<CustomPatternManager> {
    let mut manager = CustomPatternManager::new();
    
    for pattern in create_domain_patterns()? {
        manager.add_pattern(pattern);
    }
    
    Ok(manager)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_custom_pattern_creation() {
        let pattern_def = CustomPatternDefinition {
            id: "test_pattern".to_string(),
            name: "Test Pattern".to_string(),
            description: "A test pattern".to_string(),
            pattern_type: ClonePatternType::String,
            regex: r"\.clone\(\)".to_string(),
            strategy: OptimizationStrategy::UseBorrowing { lifetime: None },
            safety_level: crate::SafetyLevel::Safe,
            performance_impact: PerformanceImpact::Low,
            context_requirements: vec![],
            examples: vec![],
            tags: vec!["test".to_string()],
            enabled: true,
        };

        let pattern = pattern_def.to_pattern().unwrap();
        assert_eq!(pattern.id, "test_pattern");
        assert_eq!(pattern.pattern_type, ClonePatternType::String);
    }

    #[test]
    fn test_custom_pattern_manager() {
        let mut manager = CustomPatternManager::new();
        
        let pattern = CustomPatternDefinition {
            id: "test1".to_string(),
            name: "Test 1".to_string(),
            description: "Test pattern".to_string(),
            pattern_type: ClonePatternType::String,
            regex: r"\.clone\(\)".to_string(),
            strategy: OptimizationStrategy::UseBorrowing { lifetime: None },
            safety_level: crate::SafetyLevel::Safe,
            performance_impact: PerformanceImpact::Low,
            context_requirements: vec![],
            examples: vec![],
            tags: vec!["test".to_string()],
            enabled: true,
        };

        manager.add_pattern(pattern);
        assert_eq!(manager.get_patterns().len(), 1);
        assert_eq!(manager.get_enabled_patterns().len(), 1);

        let stats = manager.get_statistics();
        assert_eq!(stats.total_patterns, 1);
        assert_eq!(stats.enabled_patterns, 1);
    }

    #[test]
    fn test_domain_patterns() {
        let patterns = create_domain_patterns().unwrap();
        assert!(!patterns.is_empty());

        // Should have patterns for different domains
        let tokio_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.tags.contains(&"tokio".to_string()))
            .collect();
        assert!(!tokio_patterns.is_empty());

        let serde_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.tags.contains(&"serde".to_string()))
            .collect();
        assert!(!serde_patterns.is_empty());
    }

    #[test]
    fn test_pattern_file_operations() {
        let mut manager = CustomPatternManager::new();
        
        let pattern = CustomPatternDefinition {
            id: "file_test".to_string(),
            name: "File Test".to_string(),
            description: "Test file operations".to_string(),
            pattern_type: ClonePatternType::String,
            regex: r"\.clone\(\)".to_string(),
            strategy: OptimizationStrategy::UseBorrowing { lifetime: None },
            safety_level: crate::SafetyLevel::Safe,
            performance_impact: PerformanceImpact::Low,
            context_requirements: vec![],
            examples: vec![],
            tags: vec![],
            enabled: true,
        };

        manager.add_pattern(pattern);

        // Test saving and loading
        let temp_file = NamedTempFile::new().unwrap();
        manager.save_to_file(temp_file.path()).unwrap();

        let mut new_manager = CustomPatternManager::new();
        new_manager.load_from_file(temp_file.path()).unwrap();
        
        assert_eq!(new_manager.get_patterns().len(), 1);
        assert_eq!(new_manager.get_patterns()[0].id, "file_test");
    }
} 
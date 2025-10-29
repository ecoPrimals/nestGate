//! Pattern registry for managing built-in and custom clone patterns
//!
//! This module provides a unified interface for accessing and managing
//! all available clone patterns, both built-in and custom.

use super::*;
use crate::patterns::{builtin, custom};

/// Central registry for all clone patterns
#[derive(Debug, Clone)]
pub struct PatternRegistry {
    /// Built-in patterns
    builtin_patterns: PatternCollection,
    /// Custom pattern manager
    custom_patterns: custom::CustomPatternManager,
    /// Combined pattern collection cache
    combined_cache: Option<PatternCollection>,
}

impl Default for PatternRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl PatternRegistry {
    /// Create a new pattern registry with built-in patterns
    pub fn new() -> Self {
        let builtin_patterns = builtin::create_builtin_patterns()
            .unwrap_or_else(|_| PatternCollection::new());
        
        let custom_patterns = custom::load_default_custom_patterns()
            .unwrap_or_else(|_| custom::CustomPatternManager::new());

        Self {
            builtin_patterns,
            custom_patterns,
            combined_cache: None,
        }
    }

    /// Create a minimal registry with only essential patterns
    ///
    /// # Errors
    ///
    /// Returns an error if essential patterns cannot be created.
    pub fn minimal() -> crate::Result<Self> {
        let mut builtin_patterns = PatternCollection::new();
        
        // Add only the most essential patterns
        let essential_patterns = vec![
            ClonePattern::new(
                "basic_string_clone",
                "Basic String Clone",
                ClonePatternType::String,
                r"(\w+)\.clone\(\)",
                OptimizationStrategy::UseStringSlice,
            )?
            .with_safety_level(crate::SafetyLevel::Safe)
            .with_performance_impact(PerformanceImpact::Low),
            
            ClonePattern::new(
                "basic_vec_clone",
                "Basic Vector Clone",
                ClonePatternType::Collection,
                r"(\w+)\.clone\(\)\.iter\(\)",
                OptimizationStrategy::UseSliceReference,
            )?
            .with_safety_level(crate::SafetyLevel::Safe)
            .with_performance_impact(PerformanceImpact::Medium),
        ];

        for pattern in essential_patterns {
            builtin_patterns.add_pattern(pattern)?;
        }

        Ok(Self {
            builtin_patterns,
            custom_patterns: custom::CustomPatternManager::new(),
            combined_cache: None,
        })
    }

    /// Get all patterns (built-in + custom)
    ///
    /// # Errors
    ///
    /// Returns an error if custom patterns cannot be compiled.
    pub fn get_all_patterns(&mut self) -> crate::Result<&PatternCollection> {
        if self.combined_cache.is_none() {
            let mut combined = self.builtin_patterns.clone();
            
            // Add custom patterns
            let custom_collection = self.custom_patterns.to_pattern_collection()?;
            for pattern in custom_collection.get_all_patterns() {
                // Clone the pattern and add to combined collection
                let mut cloned_pattern = pattern.clone();
                // Ensure regex is compiled
                if cloned_pattern.compiled_regex.is_none() {
                    cloned_pattern.compile()?;
                }
                combined.add_pattern(cloned_pattern)?;
            }
            
            self.combined_cache = Some(combined);
        }

        Ok(self.combined_cache.as_ref().unwrap())
    }

    /// Get built-in patterns only
    pub fn get_builtin_patterns(&self) -> &PatternCollection {
        &self.builtin_patterns
    }

    /// Get custom patterns manager
    pub fn get_custom_patterns(&mut self) -> &mut custom::CustomPatternManager {
        &mut self.custom_patterns
    }

    /// Add a custom pattern
    pub fn add_custom_pattern(&mut self, pattern: custom::CustomPatternDefinition) {
        self.custom_patterns.add_pattern(pattern);
        self.combined_cache = None; // Invalidate cache
    }

    /// Remove a custom pattern
    pub fn remove_custom_pattern(&mut self, id: &str) -> bool {
        let removed = self.custom_patterns.remove_pattern(id);
        if removed {
            self.combined_cache = None; // Invalidate cache
        }
        removed
    }

    /// Get patterns by type from all sources
    ///
    /// # Errors
    ///
    /// Returns an error if patterns cannot be compiled.
    pub fn get_patterns_by_type(&mut self, pattern_type: ClonePatternType) -> crate::Result<Vec<&ClonePattern>> {
        let all_patterns = self.get_all_patterns()?;
        Ok(all_patterns.get_patterns_by_type(pattern_type))
    }

    /// Get patterns by strategy from all sources
    ///
    /// # Errors
    ///
    /// Returns an error if patterns cannot be compiled.
    pub fn get_patterns_by_strategy(&mut self, strategy: &OptimizationStrategy) -> crate::Result<Vec<&ClonePattern>> {
        let all_patterns = self.get_all_patterns()?;
        Ok(all_patterns.get_patterns_by_strategy(strategy))
    }

    /// Get patterns filtered by safety level
    ///
    /// # Errors
    ///
    /// Returns an error if patterns cannot be compiled.
    pub fn get_patterns_by_safety(&mut self, max_safety: crate::SafetyLevel) -> crate::Result<Vec<&ClonePattern>> {
        let all_patterns = self.get_all_patterns()?;
        Ok(all_patterns
            .get_all_patterns()
            .into_iter()
            .filter(|pattern| pattern.safety_level <= max_safety)
            .collect())
    }

    /// Get patterns filtered by performance impact
    ///
    /// # Errors
    ///
    /// Returns an error if patterns cannot be compiled.
    pub fn get_patterns_by_impact(&mut self, min_impact: PerformanceImpact) -> crate::Result<Vec<&ClonePattern>> {
        let all_patterns = self.get_all_patterns()?;
        Ok(all_patterns
            .get_all_patterns()
            .into_iter()
            .filter(|pattern| pattern.performance_impact >= min_impact)
            .collect())
    }

    /// Find patterns that match the given text
    ///
    /// # Errors
    ///
    /// Returns an error if patterns cannot be compiled.
    pub fn find_matching_patterns(&mut self, text: &str) -> crate::Result<Vec<PatternMatch>> {
        let all_patterns = self.get_all_patterns()?;
        let mut matches = Vec::new();

        for pattern in all_patterns.get_all_patterns() {
            for regex_match in pattern.find_matches(text) {
                matches.push(PatternMatch {
                    pattern_id: pattern.id.clone(),
                    pattern_name: pattern.name.clone(),
                    match_start: regex_match.start(),
                    match_end: regex_match.end(),
                    matched_text: regex_match.as_str().to_string(),
                    strategy: pattern.strategy.clone(),
                    safety_level: pattern.safety_level,
                    performance_impact: pattern.performance_impact,
                });
            }
        }

        // Sort by position in text
        matches.sort_by_key(|m| m.match_start);
        Ok(matches)
    }

    /// Get registry statistics
    ///
    /// # Errors
    ///
    /// Returns an error if patterns cannot be compiled.
    pub fn get_statistics(&mut self) -> crate::Result<RegistryStatistics> {
        // Get counts first to avoid borrowing conflicts
        let builtin_count = self.builtin_patterns.len();
        let custom_stats = self.custom_patterns.get_statistics();
        let custom_count = custom_stats.enabled_patterns;

        // Now get all patterns
        let all_patterns = self.get_all_patterns()?;
        let total_patterns = all_patterns.len();

        let mut by_type = std::collections::HashMap::new();
        let mut by_safety = std::collections::HashMap::new();
        let mut by_impact = std::collections::HashMap::new();

        for pattern in all_patterns.get_all_patterns() {
            *by_type.entry(pattern.pattern_type).or_insert(0) += 1;
            *by_safety.entry(pattern.safety_level).or_insert(0) += 1;
            *by_impact.entry(pattern.performance_impact).or_insert(0) += 1;
        }

        Ok(RegistryStatistics {
            total_patterns,
            builtin_patterns: builtin_count,
            custom_patterns: custom_count,
            patterns_by_type: by_type,
            patterns_by_safety: by_safety,
            patterns_by_impact: by_impact,
        })
    }

    /// Load custom patterns from file
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
    pub fn load_custom_patterns_from_file(&mut self, path: &std::path::Path) -> crate::Result<()> {
        self.custom_patterns.load_from_file(path)?;
        self.combined_cache = None; // Invalidate cache
        Ok(())
    }

    /// Save custom patterns to file
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written.
    pub fn save_custom_patterns_to_file(&self, path: &std::path::Path) -> crate::Result<()> {
        self.custom_patterns.save_to_file(path)
    }

    /// Clear all cached data
    pub fn clear_cache(&mut self) {
        self.combined_cache = None;
    }

    /// Refresh the registry (reload patterns)
    ///
    /// # Errors
    ///
    /// Returns an error if patterns cannot be reloaded.
    pub fn refresh(&mut self) -> crate::Result<()> {
        // Reload built-in patterns
        self.builtin_patterns = builtin::create_builtin_patterns()?;
        
        // Clear cache to force regeneration
        self.clear_cache();
        
        Ok(())
    }
}

/// A pattern match result
#[derive(Debug, Clone)]
pub struct PatternMatch {
    /// ID of the matching pattern
    pub pattern_id: String,
    /// Name of the matching pattern
    pub pattern_name: String,
    /// Start position in the text
    pub match_start: usize,
    /// End position in the text
    pub match_end: usize,
    /// The matched text
    pub matched_text: String,
    /// Optimization strategy for this match
    pub strategy: OptimizationStrategy,
    /// Safety level of the optimization
    pub safety_level: crate::SafetyLevel,
    /// Performance impact of the optimization
    pub performance_impact: PerformanceImpact,
}

impl PatternMatch {
    /// Get the length of the matched text
    pub fn len(&self) -> usize {
        self.match_end - self.match_start
    }

    /// Check if the match is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the matched text as a string slice reference
    pub fn as_str(&self) -> &str {
        &self.matched_text
    }
}

/// Statistics about the pattern registry
#[derive(Debug, Clone)]
pub struct RegistryStatistics {
    /// Total number of patterns
    pub total_patterns: usize,
    /// Number of built-in patterns
    pub builtin_patterns: usize,
    /// Number of custom patterns
    pub custom_patterns: usize,
    /// Patterns grouped by type
    pub patterns_by_type: std::collections::HashMap<ClonePatternType, usize>,
    /// Patterns grouped by safety level
    pub patterns_by_safety: std::collections::HashMap<crate::SafetyLevel, usize>,
    /// Patterns grouped by performance impact
    pub patterns_by_impact: std::collections::HashMap<PerformanceImpact, usize>,
}

impl RegistryStatistics {
    /// Get the percentage of safe patterns
    pub fn safe_pattern_percentage(&self) -> f64 {
        if self.total_patterns == 0 {
            return 0.0;
        }
        
        let safe_count = self.patterns_by_safety
            .get(&crate::SafetyLevel::Safe)
            .unwrap_or(&0);
        
        (*safe_count as f64 / self.total_patterns as f64) * 100.0
    }

    /// Get the percentage of high-impact patterns
    pub fn high_impact_percentage(&self) -> f64 {
        if self.total_patterns == 0 {
            return 0.0;
        }
        
        let high_count = self.patterns_by_impact
            .get(&PerformanceImpact::High)
            .unwrap_or(&0);
        let critical_count = self.patterns_by_impact
            .get(&PerformanceImpact::Critical)
            .unwrap_or(&0);
        
        ((high_count + critical_count) as f64 / self.total_patterns as f64) * 100.0
    }
}

/// Create a registry with patterns optimized for specific use cases
pub fn create_specialized_registry(use_case: RegistryUseCase) -> crate::Result<PatternRegistry> {
    match use_case {
        RegistryUseCase::WebDevelopment => create_web_registry(),
        RegistryUseCase::SystemsProgramming => create_systems_registry(),
        RegistryUseCase::DataProcessing => create_data_registry(),
        RegistryUseCase::GameDevelopment => create_game_registry(),
        RegistryUseCase::Conservative => create_conservative_registry(),
        RegistryUseCase::Aggressive => create_aggressive_registry(),
    }
}

/// Specialized registry use cases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistryUseCase {
    /// Web development (focus on async, serialization)
    WebDevelopment,
    /// Systems programming (focus on performance, safety)
    SystemsProgramming,
    /// Data processing (focus on collections, memory)
    DataProcessing,
    /// Game development (focus on performance, real-time)
    GameDevelopment,
    /// Conservative optimization (only safe patterns)
    Conservative,
    /// Aggressive optimization (all patterns)
    Aggressive,
}

fn create_web_registry() -> crate::Result<PatternRegistry> {
    let mut registry = PatternRegistry::new();
    
    // Add web-specific custom patterns
    let web_patterns = vec![
        custom::CustomPatternDefinition {
            id: "http_response_clone".to_string(),
            name: "HTTP Response Clone".to_string(),
            description: "Optimize cloning in HTTP response handlers".to_string(),
            pattern_type: ClonePatternType::Serialization,
            regex: r"HttpResponse::\w+\(\)\.json\((\w+)\.clone\(\)\)".to_string(),
            strategy: OptimizationStrategy::RemoveClone,
            safety_level: crate::SafetyLevel::Safe,
            performance_impact: PerformanceImpact::Medium,
            context_requirements: vec![ContextRequirement::NotInUnsafe],
            examples: vec![],
            tags: vec!["web".to_string(), "http".to_string()],
            enabled: true,
        },
    ];

    for pattern in web_patterns {
        registry.add_custom_pattern(pattern);
    }

    Ok(registry)
}

fn create_systems_registry() -> crate::Result<PatternRegistry> {
    let registry = PatternRegistry::new();
    
    // Focus on high-performance, low-level patterns
    // Remove patterns that might not be suitable for systems programming
    
    Ok(registry)
}

fn create_data_registry() -> crate::Result<PatternRegistry> {
    let mut registry = PatternRegistry::new();
    
    // Add data processing specific patterns
    let data_patterns = vec![
        custom::CustomPatternDefinition {
            id: "dataframe_clone".to_string(),
            name: "DataFrame Clone".to_string(),
            description: "Optimize DataFrame cloning operations".to_string(),
            pattern_type: ClonePatternType::Collection,
            regex: r"(\w+)\.clone\(\)\.into_iter\(\)".to_string(),
            strategy: OptimizationStrategy::UseSliceReference,
            safety_level: crate::SafetyLevel::Safe,
            performance_impact: PerformanceImpact::High,
            context_requirements: vec![ContextRequirement::NotInUnsafe],
            examples: vec![],
            tags: vec!["data".to_string(), "collections".to_string()],
            enabled: true,
        },
    ];

    for pattern in data_patterns {
        registry.add_custom_pattern(pattern);
    }

    Ok(registry)
}

fn create_game_registry() -> crate::Result<PatternRegistry> {
    let mut registry = PatternRegistry::new();
    
    // Add game-specific patterns focusing on performance
    let game_patterns = vec![
        custom::CustomPatternDefinition {
            id: "entity_component_clone".to_string(),
            name: "Entity Component Clone".to_string(),
            description: "Optimize entity component cloning".to_string(),
            pattern_type: ClonePatternType::CustomType,
            regex: r"entity\.get::<(\w+)>\(\)\.clone\(\)".to_string(),
            strategy: OptimizationStrategy::UseBorrowing { lifetime: None },
            safety_level: crate::SafetyLevel::Safe,
            performance_impact: PerformanceImpact::High,
            context_requirements: vec![ContextRequirement::NotInUnsafe],
            examples: vec![],
            tags: vec!["game".to_string(), "ecs".to_string()],
            enabled: true,
        },
    ];

    for pattern in game_patterns {
        registry.add_custom_pattern(pattern);
    }

    Ok(registry)
}

fn create_conservative_registry() -> crate::Result<PatternRegistry> {
    let registry = PatternRegistry::minimal()?;
    
    // Only include patterns with Safe or Conservative safety levels
    // This is handled by the safety filtering in the analysis phase
    
    Ok(registry)
}

fn create_aggressive_registry() -> crate::Result<PatternRegistry> {
    let mut registry = PatternRegistry::new();
    
    // Include all patterns, even experimental ones
    let aggressive_patterns = vec![
        custom::CustomPatternDefinition {
            id: "experimental_cow_optimization".to_string(),
            name: "Experimental Cow Optimization".to_string(),
            description: "Aggressive Cow optimization (experimental)".to_string(),
            pattern_type: ClonePatternType::CustomType,
            regex: r"(\w+)\.clone\(\)".to_string(),
            strategy: OptimizationStrategy::UseCow { cow_type: "str".to_string() },
            safety_level: crate::SafetyLevel::RequiresAnalysis,
            performance_impact: PerformanceImpact::High,
            context_requirements: vec![ContextRequirement::Custom("experimental".to_string())],
            examples: vec![],
            tags: vec!["experimental".to_string(), "aggressive".to_string()],
            enabled: true,
        },
    ];

    for pattern in aggressive_patterns {
        registry.add_custom_pattern(pattern);
    }

    Ok(registry)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_registry_creation() {
        let registry = PatternRegistry::new();
        assert!(!registry.builtin_patterns.is_empty());
    }

    #[test]
    fn test_minimal_registry() {
        let registry = PatternRegistry::minimal().unwrap();
        assert!(!registry.builtin_patterns.is_empty());
        // Should have fewer patterns than full registry
        let full_registry = PatternRegistry::new();
        assert!(registry.builtin_patterns.len() < full_registry.builtin_patterns.len());
    }

    #[test]
    fn test_pattern_matching() {
        let mut registry = PatternRegistry::minimal().unwrap();
        let matches = registry.find_matching_patterns("let result = data.clone();").unwrap();
        assert!(!matches.is_empty());
        
        let first_match = &matches[0];
        assert!(first_match.matched_text.contains("clone"));
    }

    #[test]
    fn test_registry_statistics() {
        let mut registry = PatternRegistry::new();
        let stats = registry.get_statistics().unwrap();
        
        assert!(stats.total_patterns > 0);
        assert!(stats.builtin_patterns > 0);
        assert!(!stats.patterns_by_type.is_empty());
        assert!(!stats.patterns_by_safety.is_empty());
    }

    #[test]
    fn test_specialized_registries() {
        let web_registry = create_specialized_registry(RegistryUseCase::WebDevelopment).unwrap();
        let conservative_registry = create_specialized_registry(RegistryUseCase::Conservative).unwrap();
        
        // Web registry should have more patterns than conservative
        assert!(web_registry.builtin_patterns.len() >= conservative_registry.builtin_patterns.len());
    }

    #[test]
    fn test_custom_pattern_integration() {
        let mut registry = PatternRegistry::new();
        
        let custom_pattern = custom::CustomPatternDefinition {
            id: "test_custom".to_string(),
            name: "Test Custom".to_string(),
            description: "Test custom pattern".to_string(),
            pattern_type: ClonePatternType::String,
            regex: r"test\.clone\(\)".to_string(),
            strategy: OptimizationStrategy::UseBorrowing { lifetime: None },
            safety_level: crate::SafetyLevel::Safe,
            performance_impact: PerformanceImpact::Low,
            context_requirements: vec![],
            examples: vec![],
            tags: vec!["test".to_string()],
            enabled: true,
        };

        let initial_count = registry.get_all_patterns().unwrap().len();
        registry.add_custom_pattern(custom_pattern);
        let new_count = registry.get_all_patterns().unwrap().len();
        
        assert_eq!(new_count, initial_count + 1);
    }
} 
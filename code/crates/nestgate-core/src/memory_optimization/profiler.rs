//! **MEMORY PROFILING**
//!
//! Memory profiling and reporting utilities.

use std::collections::HashMap;

// ==================== MEMORY PROFILER ====================

/// **MEMORY PROFILER**
///
/// Memory profiling and reporting utilities
pub struct MemoryProfiler {
    categories: HashMap<String, u64>,
}

impl MemoryProfiler {
    /// Create new memory profiler
    #[must_use]
    pub fn new() -> Self {
        Self {
            categories: HashMap::new(),
        }
    }
    
    /// Record memory usage for a category
    pub fn record(&mut self, category: impl Into<String>, bytes: u64) {
        *self.categories.entry(category.into()).or_insert(0) += bytes;
    }
    
    /// Generate memory report
    #[must_use]
    pub fn generate_report(&self) -> MemoryReport {
        let mut categories: Vec<CategoryReport> = self
            .categories
            .iter()
            .map(|(name, &bytes)| CategoryReport {
                name: name.clone(),
                bytes,
            })
            .collect();
        
        categories.sort_by(|a, b| b.bytes.cmp(&a.bytes));
        
        let total_bytes = categories.iter().map(|c| c.bytes).sum();
        
        MemoryReport {
            total_bytes,
            categories,
        }
    }
}

impl Default for MemoryProfiler {
    fn default() -> Self {
        Self::new()
    }
}

// ==================== MEMORY REPORT ====================

/// **MEMORY REPORT**
///
/// Report of memory usage by category
#[derive(Debug)]
pub struct MemoryReport {
    /// Total bytes across all categories
    pub total_bytes: u64,
    /// Memory usage by category (sorted by bytes, descending)
    pub categories: Vec<CategoryReport>,
}

/// **CATEGORY REPORT**
///
/// Memory usage for a specific category
#[derive(Debug)]
pub struct CategoryReport {
    /// Category name
    pub name: String,
    /// Bytes used
    pub bytes: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profiler() {
        let mut profiler = MemoryProfiler::new();
        profiler.record("buffers", 1024);
        profiler.record("caches", 2048);
        
        let report = profiler.generate_report();
        assert_eq!(report.total_bytes, 3072);
        assert_eq!(report.categories.len(), 2);
        assert_eq!(report.categories[0].name, "caches"); // Sorted by size
    }
}

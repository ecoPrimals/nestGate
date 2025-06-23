//! GPU Memory Manager
//!
//! Manages GPU memory allocation for AI models

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

use crate::{Priority, Result};

/// Information about GPU memory allocation
#[derive(Debug, Clone)]
struct AllocationInfo {
    /// Size of the allocation in bytes
    size: usize,
    /// Priority of the allocation
    priority: Priority,
    /// When the allocation was made
    allocated_at: DateTime<Utc>,
}

/// GPU memory manager
#[derive(Debug)]
pub struct GPUMemoryManager {
    /// Total GPU memory available (bytes)
    total_memory: usize,
    /// Current memory allocations
    allocated_memory: Arc<RwLock<HashMap<String, AllocationInfo>>>,
    /// Compute capability of the GPU
    compute_capability: f32,
}

impl GPUMemoryManager {
    /// Creates a new GPU memory manager
    pub fn new(total_memory: usize, compute_capability: f32) -> Self {
        Self {
            total_memory,
            allocated_memory: Arc::new(RwLock::new(HashMap::new())),
            compute_capability,
        }
    }

    /// Allocates memory for a model
    pub async fn allocate_memory(
        &self,
        model_id: &str,
        size: usize,
        priority: Priority,
    ) -> Result<Box<[u8]>> {
        let mut allocations = self.allocated_memory.write().await;
        let available = self.total_memory - self.calculate_allocated_memory(&allocations);
        
        if available >= size {
            // Sufficient memory available
            let allocation = AllocationInfo {
                size,
                priority,
                allocated_at: Utc::now(),
            };
            
            allocations.insert(model_id.to_string(), allocation);
            return Ok(vec![0; size].into_boxed_slice());
        }
        
        // Insufficient memory, check if lower priority models can be unloaded
        let candidates: Vec<_> = allocations
            .iter()
            .filter(|(_, info)| info.priority < priority)
            .map(|(id, info)| (id.clone(), info.clone()))
            .collect();
        
        if candidates.is_empty() {
            return Err(format!(
                "Insufficient GPU memory: requested {}MB, available {}MB",
                size / (1024 * 1024),
                available / (1024 * 1024)
            ).into());
        }
        
        // For now, just return error - in real implementation would unload lower priority models
        Err(format!(
            "Insufficient GPU memory: requested {}MB, available {}MB (would need to unload {} models)",
            size / (1024 * 1024),
            available / (1024 * 1024),
            candidates.len()
        ).into())
    }

    /// Frees memory for a model
    pub async fn free_memory(&self, model_id: &str) -> Result<()> {
        let mut allocations = self.allocated_memory.write().await;
        if allocations.remove(model_id).is_none() {
            return Err(format!("No memory allocation found for model {}", model_id).into());
        }
        Ok(())
    }

    /// Calculate total allocated memory
    fn calculate_allocated_memory(&self, allocations: &HashMap<String, AllocationInfo>) -> usize {
        allocations.values().map(|info| info.size).sum()
    }

    /// Get memory usage statistics
    pub async fn get_memory_stats(&self) -> Result<MemoryStats> {
        let allocations = self.allocated_memory.read().await;
        let allocated = self.calculate_allocated_memory(&allocations);
        
        Ok(MemoryStats {
            total_memory: self.total_memory,
            allocated_memory: allocated,
            available_memory: self.total_memory - allocated,
            num_allocations: allocations.len(),
        })
    }
}

/// Memory usage statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_memory: usize,
    pub allocated_memory: usize,
    pub available_memory: usize,
    pub num_allocations: usize,
} 
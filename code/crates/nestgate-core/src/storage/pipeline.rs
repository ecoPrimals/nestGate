// Storage Pipeline Router - Adaptive strategy selection
//
// Routes data through optimal pipeline based on analysis

use super::analysis::{DataAnalysis, DataFormat};
use super::compression::CompressionAlgorithm;
use anyhow::Result;
use bytes::Bytes;

/// Pipeline router
pub struct PipelineRouter {
    rules: Vec<RoutingRule>,
}

impl PipelineRouter {
    pub fn new() -> Self {
        let mut router = Self {
            rules: Vec::new(),
        };
        
        // Register default routing rules (priority ordered)
        router.register_default_rules();
        
        router
    }
    
    fn register_default_rules(&mut self) {
        // Rule 1: Already compressed → PASSTHROUGH (Priority: 100)
        self.rules.push(RoutingRule {
            name: "already_compressed".into(),
            priority: 100,
            condition: Box::new(|analysis| {
                analysis.format.is_compressed()
            }),
            pipeline: PipelineType::Passthrough,
        });
        
        // Rule 2: High entropy (>7.5) → PASSTHROUGH (Priority: 90)
        self.rules.push(RoutingRule {
            name: "high_entropy".into(),
            priority: 90,
            condition: Box::new(|analysis| {
                analysis.entropy > 7.5
            }),
            pipeline: PipelineType::Passthrough,
        });
        
        // Rule 3: Too small (<256 bytes) → PASSTHROUGH (Priority: 85)
        self.rules.push(RoutingRule {
            name: "too_small".into(),
            priority: 85,
            condition: Box::new(|analysis| {
                analysis.size < 256
            }),
            pipeline: PipelineType::Passthrough,
        });
        
        // Rule 4: Genomic data → MAXIMUM compression (Priority: 80)
        self.rules.push(RoutingRule {
            name: "genomic_max".into(),
            priority: 80,
            condition: Box::new(|analysis| {
                matches!(analysis.format,
                    DataFormat::Fasta |
                    DataFormat::Fastq |
                    DataFormat::Vcf
                )
            }),
            pipeline: PipelineType::MaxCompression,
        });
        
        // Rule 5: High compressibility (>0.7) → BALANCED (Priority: 50)
        self.rules.push(RoutingRule {
            name: "highly_compressible".into(),
            priority: 50,
            condition: Box::new(|analysis| {
                analysis.compressibility > 0.7
            }),
            pipeline: PipelineType::Balanced,
        });
        
        // Rule 6: Text data → FAST compression (Priority: 40)
        self.rules.push(RoutingRule {
            name: "text_fast".into(),
            priority: 40,
            condition: Box::new(|analysis| {
                analysis.is_text
            }),
            pipeline: PipelineType::Fast,
        });
        
        // Rule 7: Default → BALANCED (Priority: 1)
        self.rules.push(RoutingRule {
            name: "default".into(),
            priority: 1,
            condition: Box::new(|_| true),
            pipeline: PipelineType::Balanced,
        });
    }
    
    /// Select optimal pipeline for data
    pub fn select_pipeline(&self, analysis: &DataAnalysis) -> Result<Pipeline> {
        // Find first matching rule (rules are priority-sorted)
        for rule in &self.rules {
            if (rule.condition)(analysis) {
                return Ok(Pipeline::from_type(rule.pipeline.clone()));
            }
        }
        
        // Fallback (should never happen with default rule)
        Ok(Pipeline::from_type(PipelineType::Balanced))
    }
    
    /// Add custom routing rule
    pub fn add_rule(&mut self, rule: RoutingRule) {
        self.rules.push(rule);
        // Re-sort by priority (descending)
        self.rules.sort_by_key(|r| std::cmp::Reverse(r.priority));
    }
}

impl Default for PipelineRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// Routing rule
pub struct RoutingRule {
    pub name: String,
    pub priority: u32,
    pub condition: Box<dyn Fn(&DataAnalysis) -> bool + Send + Sync>,
    pub pipeline: PipelineType,
}

/// Pipeline type
#[derive(Debug, Clone)]
pub enum PipelineType {
    /// No compression or encryption
    Passthrough,
    
    /// Fast compression (LZ4)
    Fast,
    
    /// Balanced compression (Zstd level 6)
    Balanced,
    
    /// Maximum compression (Zstd level 19)
    MaxCompression,
    
    /// Encrypt then compress
    EncryptThenCompress { compression: CompressionAlgorithm },
    
    /// Compress then encrypt
    CompressThenEncrypt { compression: CompressionAlgorithm },
}

/// Storage pipeline
pub struct Pipeline {
    steps: Vec<PipelineStep>,
}

impl Pipeline {
    /// Create pipeline from type
    pub fn from_type(pipeline_type: PipelineType) -> Self {
        let steps = match pipeline_type {
            PipelineType::Passthrough => {
                vec![PipelineStep::Store]
            }
            
            PipelineType::Fast => {
                vec![
                    PipelineStep::Compress(CompressionAlgorithm::Lz4),
                    PipelineStep::Store,
                ]
            }
            
            PipelineType::Balanced => {
                vec![
                    PipelineStep::Compress(CompressionAlgorithm::Zstd { level: 6 }),
                    PipelineStep::Store,
                ]
            }
            
            PipelineType::MaxCompression => {
                vec![
                    PipelineStep::Compress(CompressionAlgorithm::Zstd { level: 19 }),
                    PipelineStep::Store,
                ]
            }
            
            PipelineType::EncryptThenCompress { compression } => {
                vec![
                    PipelineStep::Encrypt,
                    PipelineStep::Compress(compression),
                    PipelineStep::Store,
                ]
            }
            
            PipelineType::CompressThenEncrypt { compression } => {
                vec![
                    PipelineStep::Compress(compression),
                    PipelineStep::Encrypt,
                    PipelineStep::Store,
                ]
            }
        };
        
        Self { steps }
    }
    
    /// Execute pipeline
    pub async fn execute(
        &self,
        data: &[u8],
        hash: &super::ContentHash,
    ) -> Result<PipelineResult> {
        let mut current_data = data.to_vec();
        let original_size = data.len();
        let mut compression_used = None;
        let mut encrypted = false;
        
        for step in &self.steps {
            match step {
                PipelineStep::Compress(algorithm) => {
                    current_data = super::compression::compress(&current_data, *algorithm)?;
                    compression_used = Some(*algorithm);
                }
                
                PipelineStep::Encrypt => {
                    // FUTURE: Integrate with BearDog for encryption (Phase 3 - primal integration)
                    // current_data = beardog.encrypt(&current_data).await?;
                    encrypted = true;
                }
                
                PipelineStep::Store => {
                    // Final step - data is ready
                    break;
                }
            }
        }
        
        let stored_size = current_data.len();
        let ratio = original_size as f64 / stored_size as f64;
        
        Ok(PipelineResult {
            stored_data: Bytes::from(current_data),
            stored_size,
            original_size,
            compression_ratio: ratio,
            compression: compression_used,
            encrypted,
        })
    }
}

/// Pipeline step
#[derive(Debug, Clone)]
enum PipelineStep {
    Compress(CompressionAlgorithm),
    Encrypt,
    Store,
}

/// Pipeline execution result
pub struct PipelineResult {
    pub stored_data: Bytes,
    pub stored_size: usize,
    pub original_size: usize,
    pub compression_ratio: f64,
    pub compression: Option<CompressionAlgorithm>,
    pub encrypted: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::analysis::{DataAnalysis, DataFormat};
    
    #[test]
    fn test_passthrough_for_compressed() {
        let router = PipelineRouter::new();
        
        let analysis = DataAnalysis {
            size: 1024,
            entropy: 7.9,
            format: DataFormat::Gzip,
            compressibility: 0.1,
            is_text: false,
        };
        
        let pipeline = router.select_pipeline(&analysis).unwrap();
        assert_eq!(pipeline.steps.len(), 1);
        assert!(matches!(pipeline.steps[0], PipelineStep::Store));
    }
    
    #[test]
    fn test_max_compression_for_genomic() {
        let router = PipelineRouter::new();
        
        let analysis = DataAnalysis {
            size: 1024 * 1024,
            entropy: 2.0,
            format: DataFormat::Fasta,
            compressibility: 0.95,
            is_text: true,
        };
        
        let pipeline = router.select_pipeline(&analysis).unwrap();
        assert_eq!(pipeline.steps.len(), 2);
        assert!(matches!(pipeline.steps[0], PipelineStep::Compress(CompressionAlgorithm::Zstd { level: 19 })));
    }
    
    #[test]
    fn test_passthrough_for_high_entropy() {
        let router = PipelineRouter::new();
        
        let analysis = DataAnalysis {
            size: 1024 * 1024,
            entropy: 7.99,
            format: DataFormat::Binary,
            compressibility: 0.05,
            is_text: false,
        };
        
        let pipeline = router.select_pipeline(&analysis).unwrap();
        assert_eq!(pipeline.steps.len(), 1);
    }
}


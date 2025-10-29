// Live Provider Implementations
//! Module definitions and exports.
// Real implementations that connect to actual external APIs like NCBI, Ensembl,
// HuggingFace, etc. These demonstrate how external systems can integrate with
// NestGate's universal data capabilities using their real APIs.

pub mod ncbi_live_provider;
pub mod ensembl_live_provider;
pub mod huggingface_live_provider;

// Re-export live providers
pub use ncbi_live_provider::NCBILiveProvider;
pub use ensembl_live_provider::EnsemblLiveProvider;
pub use huggingface_live_provider::HuggingFaceLiveProvider; 
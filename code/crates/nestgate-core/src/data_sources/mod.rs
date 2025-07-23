// Removed unused error imports
/// Universal Data Source Implementations
///
/// Concrete implementations of data sources for research databases and AI platforms.
/// This module provides modular data source implementations, each in their own submodule
/// for better organization and maintainability.
pub mod huggingface;
pub mod ncbi;

// Re-export the main data source implementations for backward compatibility
pub use huggingface::HuggingFaceModelSource;
pub use ncbi::NCBIGenomeSource;

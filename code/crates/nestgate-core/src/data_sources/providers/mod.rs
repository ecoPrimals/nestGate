// Example Data Capability Providers
//! Module definitions and exports.
// These are reference implementations showing how external data sources
//! can implement NestGate's universal data capabilities. Any system that
//! can provide genome data, model data, or research data can implement
//! these interfaces without NestGate knowing their specific identity.

pub mod genome_provider_example;
pub mod model_provider_example;
pub mod research_provider_example;
pub mod universal_http_provider;
pub mod live_providers;

// Re-export the example providers
pub use genome_provider_example::UniversalGenomeProvider;
pub use model_provider_example::UniversalModelProvider;
pub use research_provider_example::UniversalResearchProvider;
pub use universal_http_provider::UniversalHttpProvider;

// Re-export live providers
pub use live_providers::{NCBILiveProvider, EnsemblLiveProvider, HuggingFaceLiveProvider}; 
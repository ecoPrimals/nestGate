//! Universal Data Source Capabilities
//!
//! NestGate doesn't know about specific data providers like NCBI or HuggingFace.
//! Instead, it defines universal capabilities that any data provider can implement.
//! This enables true ecosystem agnosticism - we can leverage any data structure
//! without hardcoding specific providers.

pub mod universal_data_adapter;
pub mod data_capabilities;
pub mod providers;

// Re-export the universal interfaces
pub use universal_data_adapter::UniversalDataAdapter;
pub use data_capabilities::{
    DataCapability, DataRequest, DataResponse, 
    GenomeDataCapability, ModelDataCapability, ResearchDataCapability
};

// Re-export example providers
pub use providers::{
    UniversalGenomeProvider, UniversalHttpProvider,
    // Other providers will be added as they're implemented
};

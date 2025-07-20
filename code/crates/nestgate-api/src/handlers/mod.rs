//! API request handlers

pub mod auth;
pub mod health;
pub mod status;
pub mod zfs;
pub mod hardware_tuning;
pub mod workspace_management;
pub mod performance_analytics;
pub mod load_testing;
pub mod storage;
pub mod byob_management;
pub mod compute_handler;
pub mod ecoprimal_analysis;
// pub mod performance_dashboard; // Temporarily disabled due to compilation issues
pub mod universal_ai_handler;
pub mod universal_storage_handler;
pub mod zfs_management;

pub use health::*;
pub use zfs::*;
pub use hardware_tuning::*;
pub use workspace_management::*;
pub use performance_analytics::*;
pub use load_testing::*;
pub use storage::*;

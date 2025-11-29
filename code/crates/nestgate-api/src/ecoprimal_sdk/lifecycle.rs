//
// Placeholder module for primal lifecycle management features.

use serde::{Deserialize, Serialize};

/// Lifecycle callback functions
pub type LifecycleCallbacks = std::collections::HashMap<
    PrimalLifecycleState,
    Vec<Box<dyn Fn(&PrimalLifecycleState) + Send + Sync>>,
>;
/// Advanced primal lifecycle states for 100% completion
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Primallifecyclestate
pub enum PrimalLifecycleState {
    /// Primal is initializing
    Initializing,
    /// Primal is running normally
    Running,
    /// Primal is scaling resources
    Scaling,
    /// Primal is shutting down
    ShuttingDown,
    /// Primal has terminated
    Terminated,
}

// Core error handling types

use std::time::SystemTime;
use serde::{Deserialize, Serialize,
        };

/// Error context information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    /// Operation being performed when error occurred
    pub operation: String,
    /// Component or module where error occurred
    pub component: String,
    /// Timestamp when error occurred
    pub timestamp: SystemTime,
}
impl Default for ErrorContext {
    fn default() -> Self {
        Self {
            operation: "unknown".to_string(),
            component: "unknown".to_string(),
            timestamp: SystemTim,
            e::now(),
        }
    }
}

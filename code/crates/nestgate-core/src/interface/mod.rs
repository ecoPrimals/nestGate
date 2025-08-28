use crate::unified_enums::UnifiedHealthStatus;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Service configuration for interface module
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceConfig {
    pub service_name: String,
    pub name: String,
    pub metadata: HashMap<String, String>,
}
//

// CLEANED: Removed unused UnifiedHealthStatus import as part of canonical modernization
// use crate::unified_enums::UnifiedHealthStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Service configuration for interface module
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceConfig {
    pub service_name: String,
    pub name: String,
    pub metadata: HashMap<String, String>,
}
//

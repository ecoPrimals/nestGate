//! **API HANDLER CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct ApiHandlerConfig {
    pub request: RequestHandlerConfig,
    pub response: ResponseHandlerConfig,
    pub routes: RouteHandlerConfig,
    pub auth: AuthHandlerConfig,
    pub rate_limiting: RateLimitHandlerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestHandlerConfig {
    pub logging: bool,
    pub timeout: Duration,
    pub max_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseHandlerConfig {
    pub compression: bool,
    pub caching: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteHandlerConfig {
    pub enabled: bool,
    pub routes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthHandlerConfig {
    pub enabled: bool,
    pub methods: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitHandlerConfig {
    pub enabled: bool,
    pub requests_per_minute: u32,
}


impl Default for RequestHandlerConfig {
    fn default() -> Self {
        Self {
            logging: true,
            timeout: Duration::from_secs(30),
            max_size: 1024 * 1024, // 1MB
        }
    }
}

impl Default for ResponseHandlerConfig {
    fn default() -> Self {
        Self {
            compression: true,
            caching: false,
        }
    }
}

impl Default for RouteHandlerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            routes: vec![],
        }
    }
}

impl Default for AuthHandlerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            methods: vec!["bearer".to_string()],
        }
    }
}

impl Default for RateLimitHandlerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            requests_per_minute: 1000,
        }
    }
}

impl ApiHandlerConfig {
    pub fn production_optimized() -> Self { Self::default() }
    pub fn development_optimized() -> Self { Self::default() }
    pub fn high_performance() -> Self { Self::default() }
    pub fn merge(self, _other: Self) -> Self { self }
    pub fn validate(&self) -> crate::Result<()> { Ok(()) }
} 
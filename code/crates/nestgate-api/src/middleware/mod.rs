//! Middleware module
//!
//! This module contains all middleware components for the NestGate API,
//! including authentication and AI-First compliance middleware.

pub mod ai_first_middleware;
pub mod auth_middleware;

// Re-export middleware functions for easy use
pub use ai_first_middleware::ai_first_middleware;
pub use auth_middleware::{auth_middleware, AuthMiddleware};

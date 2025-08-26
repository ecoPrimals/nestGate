//
// This module contains all middleware components for the NestGate API,
// including the AI-First compliance middleware for ecosystem integration.

pub mod ai_first_middleware;

// Re-export the main middleware function for easy use
pub use ai_first_middleware::ai_first_middleware;

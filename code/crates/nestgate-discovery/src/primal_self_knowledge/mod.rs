// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Primal Self-Knowledge System
//!
//! Core implementation of the self-knowledge philosophy: Each primal knows what
//! it can do, announces itself, and discovers others at runtime.
//!
//! # Philosophy
//!
//! - **Self-Knowledge**: Each primal introspects its own capabilities
//! - **Announcement**: Primals announce themselves to the ecosystem
//! - **Discovery**: Primals discover others through runtime mechanisms
//! - **No Hardcoding**: Zero assumptions about other primals' locations
//!
//! # Example
//!
//! ```rust,ignore
//! use nestgate_core::primal_self_knowledge::PrimalSelfKnowledge;
//! use anyhow::Result;
//!
//! # async fn example() -> Result<()> {
//! // Initialize with self-knowledge
//! let mut primal = PrimalSelfKnowledge::initialize().await?;
//!
//! // Announce ourselves to the ecosystem
//! primal.announce_self()?;
//!
//! // Discover another primal at runtime by capability
//! let peer = primal.discover_primal("orchestration_provider").await?;
//! println!("Found peer at: {}", peer.primary_endpoint());
//! # Ok(())
//! # }
//! ```

mod knowledge;
mod types;

pub use knowledge::PrimalSelfKnowledge;
pub use types::{Capability, DiscoveredPrimal, DiscoveryMechanism, Endpoint, PrimalIdentity};

#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_types::MapEnv;
    use std::sync::Arc;

    #[test]
    fn endpoint_url_without_path_suffix() {
        let endpoint = Endpoint {
            protocol: "https".to_string(),
            address: "10.0.0.1".to_string(),
            port: 443,
            path: None,
            health_path: None,
        };
        assert_eq!(endpoint.url(), "https://10.0.0.1:443");
        assert!(endpoint.health_url().is_none());
    }

    #[tokio::test]
    async fn discover_primal_errors_when_not_configured() {
        let mut primal = PrimalSelfKnowledge::initialize_with_env(Arc::new(MapEnv::new()))
            .await
            .expect("initialize");
        let err = primal
            .discover_primal("orchestration_provider")
            .await
            .expect_err("no discovery source");
        assert!(
            err.to_string().contains("not discovered"),
            "unexpected: {err}"
        );
    }

    #[tokio::test]
    async fn discovered_primals_map_starts_empty() {
        let primal = PrimalSelfKnowledge::initialize().await.expect("initialize");
        assert!(primal.discovered_primals().is_empty());
    }

    #[tokio::test]
    async fn test_primal_initialization() {
        let primal = PrimalSelfKnowledge::initialize().await;
        assert!(primal.is_ok());
    }

    #[tokio::test]
    async fn test_primal_has_identity() {
        let primal = PrimalSelfKnowledge::initialize().await.unwrap();
        let identity = primal.identity();

        assert_eq!(identity.primal_type, "nestgate");
        assert!(!identity.id.is_empty());
        assert!(!identity.version.is_empty());
    }

    #[tokio::test]
    async fn test_primal_has_capabilities() {
        let primal = PrimalSelfKnowledge::initialize().await.unwrap();
        let caps = primal.capabilities();

        assert!(!caps.is_empty());
        assert!(caps.iter().any(|c| c.name == "storage"));
    }

    #[tokio::test]
    async fn test_primal_has_endpoints() {
        let primal = PrimalSelfKnowledge::initialize().await.unwrap();
        let endpoints = primal.endpoints();

        assert!(!endpoints.is_empty());
    }

    #[tokio::test]
    async fn test_endpoint_url() {
        let endpoint = Endpoint {
            protocol: "http".to_string(),
            address: "localhost".to_string(),
            port: 8080,
            path: Some("/api".to_string()),
            health_path: Some("/health".to_string()),
        };

        assert_eq!(endpoint.url(), "http://localhost:8080/api");
        assert_eq!(
            endpoint.health_url(),
            Some("http://localhost:8080/health".to_string())
        );
    }

    #[tokio::test]
    async fn test_discovered_primal_has_capability() {
        let discovered = DiscoveredPrimal {
            identity: PrimalIdentity {
                id: "test".to_string(),
                primal_type: "testprimal".to_string(),
                version: "1.0.0".to_string(),
                started_at: std::time::SystemTime::now(),
            },
            capabilities: vec![Capability {
                name: "storage".to_string(),
                description: "Storage".to_string(),
                endpoint: "/storage".to_string(),
                metadata: std::collections::HashMap::new(),
            }],
            primary_endpoint: Endpoint {
                protocol: "http".to_string(),
                address: "localhost".to_string(),
                port: 8080,
                path: None,
                health_path: None,
            },
            discovered_at: std::time::SystemTime::now(),
            discovery_method: DiscoveryMechanism::Environment,
        };

        assert!(discovered.has_capability("storage"));
        assert!(discovered.has_capability("STORAGE")); // Case insensitive
        assert!(!discovered.has_capability("nonexistent"));
    }

    #[tokio::test]
    async fn test_announce_self() {
        let primal = PrimalSelfKnowledge::initialize().await.unwrap();

        // Should succeed (may do nothing if mechanisms not configured)
        let result = primal.announce_self();
        assert!(result.is_ok());
    }
}

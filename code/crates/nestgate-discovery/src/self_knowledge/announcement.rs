// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Announcement system for broadcasting primal presence
//!
//! Handles the serialization and broadcasting of primal information
//! to discovery backends.

use super::{PrimalId, SelfKnowledge};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::SystemTime;

/// Announcement message broadcast to discovery systems
///
/// Contains all information other primals need to discover and connect to this primal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announcement {
    /// Unique identifier of the announcing primal
    pub id: PrimalId,
    /// Human-readable name
    pub name: String,
    /// Version for compatibility checking
    pub version: String,
    /// Capabilities provided
    pub capabilities: Vec<String>,
    /// Endpoints where primal is accessible
    pub endpoints: HashMap<String, SocketAddr>,
    /// When this announcement was created
    pub announced_at: SystemTime,
    /// TTL for this announcement (how long it's valid)
    pub ttl_seconds: u64,
}

impl From<&SelfKnowledge> for Announcement {
    fn from(knowledge: &SelfKnowledge) -> Self {
        Self {
            id: knowledge.id.clone(),
            name: knowledge.name.clone(),
            version: knowledge.version.clone(),
            capabilities: knowledge.capabilities.clone(),
            endpoints: knowledge.endpoints.clone(),
            announced_at: SystemTime::now(),
            ttl_seconds: 300, // 5 minutes default
        }
    }
}

impl Announcement {
    /// Create announcement from self-knowledge
    pub fn from_knowledge(knowledge: &SelfKnowledge) -> Self {
        Self::from(knowledge)
    }
    /// Create announcement with custom TTL
    pub fn with_ttl(mut self, ttl_seconds: u64) -> Self {
        self.ttl_seconds = ttl_seconds;
        self
    }
    /// Check if this announcement has expired
    pub fn is_expired(&self) -> bool {
        match SystemTime::now().duration_since(self.announced_at) {
            Ok(elapsed) => elapsed.as_secs() > self.ttl_seconds,
            Err(_) => true, // Clock went backwards, consider expired
        }
    }

    /// Serialize to JSON
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    /// Deserialize from JSON
    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_announcement_creation() {
        let knowledge = SelfKnowledge::builder()
            .with_id("test")
            .with_name("Test Primal")
            .with_version("1.0.0")
            .with_capability("storage")
            .build()
            .unwrap();

        let announcement = Announcement::from_knowledge(&knowledge);

        assert_eq!(announcement.id.as_str(), "test");
        assert_eq!(announcement.name, "Test Primal");
        assert_eq!(announcement.version, "1.0.0");
        assert_eq!(announcement.capabilities.len(), 1);
        assert_eq!(announcement.ttl_seconds, 300);
    }

    #[test]
    fn test_announcement_with_custom_ttl() {
        let knowledge = SelfKnowledge::builder().with_id("test").build().unwrap();

        let announcement = Announcement::from_knowledge(&knowledge).with_ttl(600);

        assert_eq!(announcement.ttl_seconds, 600);
    }

    #[test]
    fn test_announcement_serialization() {
        let knowledge = SelfKnowledge::builder()
            .with_id("test")
            .with_capability("storage")
            .build()
            .unwrap();

        let announcement = Announcement::from_knowledge(&knowledge);
        let json = announcement.to_json().unwrap();
        let deserialized = Announcement::from_json(&json).unwrap();

        assert_eq!(announcement.id, deserialized.id);
        assert_eq!(announcement.capabilities, deserialized.capabilities);
    }

    #[test]
    fn test_announcement_expiry() {
        let knowledge = SelfKnowledge::builder().with_id("test").build().unwrap();

        let mut announcement = Announcement::from_knowledge(&knowledge).with_ttl(0); // Expire immediately

        // Manually set announced_at to past
        announcement.announced_at = SystemTime::now() - std::time::Duration::from_secs(10);

        assert!(announcement.is_expired());
    }
}

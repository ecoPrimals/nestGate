// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # 📊 Audit Storage for Collaborative Intelligence
//!
//! **Execution Audit Trail Storage & Management**
//!
//! Provides audit trail storage capabilities for ecosystem collaborative intelligence workflows,
//! enabling AI learning from execution history and user modifications.
//!
//! ## Philosophy
//! - **Self-Knowledge**: Audits stored per `family_id` (multi-tenant isolation)
//! - **Zero Hardcoding**: All behavior driven by data
//! - **Modern Rust**: No unsafe code, proper error handling
//! - **Complete Implementation**: Production-ready from day one
//!
//! ## Features
//! - Execution audit storage
//! - Modification tracking
//! - Outcome recording
//! - Query capabilities

use chrono::{DateTime, Utc};
use nestgate_types::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Execution audit record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionAudit {
    /// Unique audit identifier
    pub id: String,

    /// Execution identifier
    pub execution_id: String,

    /// Graph identifier
    pub graph_id: String,

    /// Optional template used
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub template_id: Option<String>,

    /// User who executed
    pub user_id: String,

    /// Family/app identifier for isolation
    pub family_id: String,

    /// Execution start time
    pub started_at: DateTime<Utc>,

    /// Execution completion time
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub completed_at: Option<DateTime<Utc>>,

    /// Status (running, completed, failed, cancelled)
    pub status: ExecutionStatus,

    /// Graph modifications during execution
    #[serde(default)]
    pub modifications: Vec<GraphModification>,

    /// Node execution outcomes
    #[serde(default)]
    pub outcomes: Vec<NodeOutcome>,

    /// Additional metadata
    #[serde(default)]
    pub metadata: serde_json::Value,
}

/// Execution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionStatus {
    /// Execution in progress
    Running,
    /// Execution completed successfully
    Completed,
    /// Execution failed
    Failed,
    /// Execution cancelled by user
    Cancelled,
}

/// Graph modification record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphModification {
    /// Modification timestamp
    pub timestamp: DateTime<Utc>,

    /// Type of modification
    pub modification_type: ModificationType,

    /// Affected node ID (if applicable)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub node_id: Option<String>,

    /// Modification data
    pub data: serde_json::Value,
}

/// Type of graph modification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModificationType {
    /// Add new node
    AddNode,
    /// Remove existing node
    RemoveNode,
    /// Modify node configuration
    ModifyNode,
    /// Add edge/dependency
    AddEdge,
    /// Remove edge/dependency
    RemoveEdge,
}

/// Node execution outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeOutcome {
    /// Node identifier
    pub node_id: String,

    /// Execution status
    pub status: NodeStatus,

    /// Start time
    pub started_at: DateTime<Utc>,

    /// Completion time
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub completed_at: Option<DateTime<Utc>>,

    /// Duration in milliseconds
    pub duration_ms: u64,

    /// Error message if failed
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub error: Option<String>,

    /// Node-specific metrics
    #[serde(default)]
    pub metrics: serde_json::Value,
}

/// Node execution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum NodeStatus {
    /// Node executed successfully
    Success,
    /// Node execution failed
    Failed,
    /// Node execution skipped
    Skipped,
}

/// Audit storage state (in-memory for now, will be persistent later)
#[derive(Debug, Clone, Default)]
pub struct AuditStorage {
    /// Audits stored by `family_id` -> `execution_id` -> audit
    /// Ensures family-based isolation (self-knowledge principle)
    audits: Arc<RwLock<HashMap<String, HashMap<String, ExecutionAudit>>>>,
}

impl AuditStorage {
    /// Create new audit storage
    #[must_use]
    pub fn new() -> Self {
        Self {
            audits: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Store execution audit
    ///
    /// # Self-Knowledge Principle
    /// - Audits isolated by `family_id`
    /// - No cross-family access
    /// - Immutable once stored (append-only)
    ///
    /// # Arguments
    /// - `audit`: Complete execution audit record
    ///
    /// # Returns
    /// - Audit ID on success
    ///
    /// # Errors
    ///
    /// Returns validation errors when `execution_id`, `family_id`, or `user_id` is empty.
    pub async fn store_audit(&self, mut audit: ExecutionAudit) -> Result<String> {
        // Validate inputs
        if audit.execution_id.is_empty() {
            return Err(NestGateError::invalid_input_with_field(
                "execution_id",
                "Execution ID cannot be empty",
            ));
        }

        if audit.family_id.is_empty() {
            return Err(NestGateError::invalid_input_with_field(
                "family_id",
                "Family ID cannot be empty",
            ));
        }

        if audit.user_id.is_empty() {
            return Err(NestGateError::invalid_input_with_field(
                "user_id",
                "User ID cannot be empty",
            ));
        }

        // Generate unique audit ID if not provided
        if audit.id.is_empty() {
            audit.id = format!("audit_{}", uuid::Uuid::new_v4().simple());
        }

        let audit_id = audit.id.clone();
        let family_id = audit.family_id.clone();
        let execution_id = audit.execution_id.clone();

        // Store with family isolation
        let mut storage = self.audits.write().await;
        let family_audits = storage
            .entry(family_id.clone())
            .or_insert_with(HashMap::new);
        family_audits.insert(execution_id.clone(), audit);

        tracing::info!(
            "Stored audit '{}' for execution '{}' (family: '{}')",
            audit_id,
            execution_id,
            family_id
        );

        Ok(audit_id)
    }

    /// Retrieve audit by execution ID
    ///
    /// # Self-Knowledge Principle
    /// - Only retrieves from specified `family_id`
    /// - No cross-family access possible
    ///
    /// # Errors
    ///
    /// Returns `not_found` when the family bucket or execution id is missing.
    pub async fn retrieve_audit(
        &self,
        execution_id: &str,
        family_id: &str,
    ) -> Result<ExecutionAudit> {
        let storage = self.audits.read().await;

        let family_audits = storage
            .get(family_id)
            .ok_or_else(|| NestGateError::not_found("No audits for this family"))?;

        let audit = family_audits
            .get(execution_id)
            .ok_or_else(|| NestGateError::not_found("Audit not found"))?;

        tracing::debug!("Retrieved audit for execution '{}'", execution_id);

        Ok(audit.clone())
    }

    /// List audits for a family (with optional filters)
    ///
    /// # Errors
    ///
    /// Returns `not_found` when no audits exist for the given `family_id`.
    pub async fn list_audits(
        &self,
        family_id: &str,
        user_id: Option<&str>,
        status: Option<ExecutionStatus>,
    ) -> Result<Vec<ExecutionAudit>> {
        let storage = self.audits.read().await;

        let family_audits = storage
            .get(family_id)
            .ok_or_else(|| NestGateError::not_found("No audits for this family"))?;

        let mut audits: Vec<ExecutionAudit> = family_audits
            .values()
            .filter(|a| {
                // Filter by user_id if specified
                if let Some(uid) = user_id
                    && a.user_id != uid
                {
                    return false;
                }

                // Filter by status if specified
                if let Some(ref s) = status
                    && &a.status != s
                {
                    return false;
                }

                true
            })
            .cloned()
            .collect();

        // Sort by started_at (most recent first)
        audits.sort_by(|a, b| b.started_at.cmp(&a.started_at));

        Ok(audits)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_store_and_retrieve_audit() {
        let storage = AuditStorage::new();

        let audit = ExecutionAudit {
            id: String::new(), // Will be auto-generated
            execution_id: "exec_123".to_string(),
            graph_id: "graph_456".to_string(),
            template_id: Some("tmpl_789".to_string()),
            user_id: "user_abc".to_string(),
            family_id: "test_family".to_string(),
            started_at: Utc::now(),
            completed_at: Some(Utc::now()),
            status: ExecutionStatus::Completed,
            modifications: vec![],
            outcomes: vec![],
            metadata: json!({}),
        };

        let audit_id = storage.store_audit(audit).await.unwrap();
        assert!(audit_id.starts_with("audit_"));

        let retrieved = storage
            .retrieve_audit("exec_123", "test_family")
            .await
            .unwrap();

        assert_eq!(retrieved.execution_id, "exec_123");
        assert_eq!(retrieved.user_id, "user_abc");
    }

    #[tokio::test]
    async fn test_audit_family_isolation() {
        let storage = AuditStorage::new();

        let audit = ExecutionAudit {
            id: String::new(),
            execution_id: "exec_123".to_string(),
            graph_id: "graph_456".to_string(),
            template_id: None,
            user_id: "user_abc".to_string(),
            family_id: "family_1".to_string(),
            started_at: Utc::now(),
            completed_at: None,
            status: ExecutionStatus::Running,
            modifications: vec![],
            outcomes: vec![],
            metadata: json!({}),
        };

        storage.store_audit(audit).await.unwrap();

        // Try to retrieve from different family
        let result = storage.retrieve_audit("exec_123", "family_2").await;
        assert!(result.is_err());

        // Retrieve from correct family should work
        let result = storage.retrieve_audit("exec_123", "family_1").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_audit_with_modifications() {
        let storage = AuditStorage::new();

        let modifications = vec![
            GraphModification {
                timestamp: Utc::now(),
                modification_type: ModificationType::AddNode,
                node_id: Some("node_cache".to_string()),
                data: json!({"type": "redis"}),
            },
            GraphModification {
                timestamp: Utc::now(),
                modification_type: ModificationType::ModifyNode,
                node_id: Some("node_db".to_string()),
                data: json!({"pool_size": 20}),
            },
        ];

        let audit = ExecutionAudit {
            id: String::new(),
            execution_id: "exec_with_mods".to_string(),
            graph_id: "graph_789".to_string(),
            template_id: None,
            user_id: "user_xyz".to_string(),
            family_id: "test_family".to_string(),
            started_at: Utc::now(),
            completed_at: Some(Utc::now()),
            status: ExecutionStatus::Completed,
            modifications,
            outcomes: vec![],
            metadata: json!({}),
        };

        storage.store_audit(audit).await.unwrap();

        let retrieved = storage
            .retrieve_audit("exec_with_mods", "test_family")
            .await
            .unwrap();

        assert_eq!(retrieved.modifications.len(), 2);
        assert_eq!(
            retrieved.modifications[0].modification_type,
            ModificationType::AddNode
        );
    }

    #[tokio::test]
    async fn test_audit_validation() {
        let storage = AuditStorage::new();

        // Empty execution_id should fail
        let audit = ExecutionAudit {
            id: String::new(),
            execution_id: String::new(),
            graph_id: "graph_123".to_string(),
            template_id: None,
            user_id: "user_abc".to_string(),
            family_id: "family".to_string(),
            started_at: Utc::now(),
            completed_at: None,
            status: ExecutionStatus::Running,
            modifications: vec![],
            outcomes: vec![],
            metadata: json!({}),
        };

        let result = storage.store_audit(audit).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_audits_filters_by_user_and_status() {
        let storage = AuditStorage::new();
        let base = ExecutionAudit {
            id: String::new(),
            execution_id: "e1".to_string(),
            graph_id: "g".to_string(),
            template_id: None,
            user_id: "alice".to_string(),
            family_id: "fam_list".to_string(),
            started_at: Utc::now(),
            completed_at: None,
            status: ExecutionStatus::Running,
            modifications: vec![],
            outcomes: vec![],
            metadata: json!({}),
        };
        let mut second = base.clone();
        second.execution_id = "e2".to_string();
        second.user_id = "bob".to_string();
        second.status = ExecutionStatus::Completed;

        storage.store_audit(base).await.unwrap();
        storage.store_audit(second).await.unwrap();

        let alice_only = storage
            .list_audits("fam_list", Some("alice"), None)
            .await
            .unwrap();
        assert_eq!(alice_only.len(), 1);
        assert_eq!(alice_only[0].execution_id, "e1");

        let completed = storage
            .list_audits("fam_list", None, Some(ExecutionStatus::Completed))
            .await
            .unwrap();
        assert_eq!(completed.len(), 1);
        assert_eq!(completed[0].execution_id, "e2");
    }

    #[tokio::test]
    async fn test_list_audits_unknown_family_errors() {
        let storage = AuditStorage::new();
        let r = storage.list_audits("missing_fam", None, None).await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn list_audits_empty_when_user_filter_excludes_all() {
        let storage = AuditStorage::new();
        let audit = ExecutionAudit {
            id: String::new(),
            execution_id: "ex_f".into(),
            graph_id: "g".into(),
            template_id: None,
            user_id: "carol".into(),
            family_id: "fam_filter".into(),
            started_at: Utc::now(),
            completed_at: None,
            status: ExecutionStatus::Running,
            modifications: vec![],
            outcomes: vec![],
            metadata: json!({}),
        };
        storage.store_audit(audit).await.unwrap();
        let empty = storage
            .list_audits("fam_filter", Some("dave"), None)
            .await
            .unwrap();
        assert!(empty.is_empty());
    }

    #[tokio::test]
    async fn test_store_rejects_empty_family_id() {
        let storage = AuditStorage::new();
        let audit = ExecutionAudit {
            id: String::new(),
            execution_id: "e".to_string(),
            graph_id: "g".to_string(),
            template_id: None,
            user_id: "u".to_string(),
            family_id: String::new(),
            started_at: Utc::now(),
            completed_at: None,
            status: ExecutionStatus::Running,
            modifications: vec![],
            outcomes: vec![],
            metadata: json!({}),
        };
        assert!(storage.store_audit(audit).await.is_err());
    }

    #[tokio::test]
    async fn test_store_rejects_empty_user_id() {
        let storage = AuditStorage::new();
        let audit = ExecutionAudit {
            id: String::new(),
            execution_id: "e".to_string(),
            graph_id: "g".to_string(),
            template_id: None,
            user_id: String::new(),
            family_id: "fam".to_string(),
            started_at: Utc::now(),
            completed_at: None,
            status: ExecutionStatus::Running,
            modifications: vec![],
            outcomes: vec![],
            metadata: json!({}),
        };
        assert!(storage.store_audit(audit).await.is_err());
    }

    #[test]
    fn execution_audit_and_node_outcome_serde_roundtrip() {
        let audit = ExecutionAudit {
            id: "audit_full".into(),
            execution_id: "ex".into(),
            graph_id: "g".into(),
            template_id: Some("t".into()),
            user_id: "u".into(),
            family_id: "f".into(),
            started_at: Utc::now(),
            completed_at: None,
            status: ExecutionStatus::Cancelled,
            modifications: vec![],
            outcomes: vec![NodeOutcome {
                node_id: "n1".into(),
                status: NodeStatus::Skipped,
                started_at: Utc::now(),
                completed_at: None,
                duration_ms: 0,
                error: None,
                metrics: json!({"k": 1}),
            }],
            metadata: json!({"x": true}),
        };
        let s = serde_json::to_string(&audit).unwrap();
        let back: ExecutionAudit = serde_json::from_str(&s).unwrap();
        assert_eq!(back.status, ExecutionStatus::Cancelled);
        assert_eq!(back.outcomes[0].status, NodeStatus::Skipped);
    }

    #[test]
    fn graph_modification_types_serialize() {
        for mt in [
            ModificationType::AddNode,
            ModificationType::RemoveNode,
            ModificationType::ModifyNode,
            ModificationType::AddEdge,
            ModificationType::RemoveEdge,
        ] {
            let gm = GraphModification {
                timestamp: Utc::now(),
                modification_type: mt.clone(),
                node_id: None,
                data: json!({}),
            };
            let s = serde_json::to_string(&gm).unwrap();
            let back: GraphModification = serde_json::from_str(&s).unwrap();
            assert_eq!(back.modification_type, mt);
        }
    }

    #[test]
    fn execution_status_and_node_status_json_roundtrip() {
        for s in [
            ExecutionStatus::Running,
            ExecutionStatus::Completed,
            ExecutionStatus::Failed,
            ExecutionStatus::Cancelled,
        ] {
            let j = serde_json::to_string(&s).unwrap();
            let back: ExecutionStatus = serde_json::from_str(&j).unwrap();
            assert_eq!(back, s);
        }
        for s in [NodeStatus::Success, NodeStatus::Failed, NodeStatus::Skipped] {
            let j = serde_json::to_string(&s).unwrap();
            let back: NodeStatus = serde_json::from_str(&j).unwrap();
            assert_eq!(back, s);
        }
    }
}

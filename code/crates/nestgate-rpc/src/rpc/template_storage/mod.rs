// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # 🗂️ Template Storage for Collaborative Intelligence
//!
//! **Graph Template Storage & Management**
//!
//! Provides template storage capabilities for ecosystem collaborative intelligence workflows,
//! enabling users to save, share, and discover graph templates for faster bootstrapping.
//!
//! ## Philosophy
//! - **Self-Knowledge**: Templates stored per `family_id` (multi-tenant isolation)
//! - **Zero Hardcoding**: All behavior driven by data and environment
//! - **Modern Rust**: No unsafe code, proper error handling throughout
//! - **Complete Implementation**: No mocks, production-ready from day one
//!
//! ## Features
//! - Template CRUD operations
//! - Version control
//! - Community sharing & ranking
//! - Usage tracking
//! - Success rate calculations

mod operations;
mod types;

pub use operations::TemplateStorage;
pub use types::{GraphTemplate, TemplateMetadata};

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn store_rejects_empty_name() {
        let ts = TemplateStorage::new();
        let r = ts
            .store_template(
                String::new(),
                "d".into(),
                serde_json::json!({}),
                "u".into(),
                "fam".into(),
                TemplateMetadata::default(),
            )
            .await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn store_rejects_empty_family_id() {
        let ts = TemplateStorage::new();
        let r = ts
            .store_template(
                "n".into(),
                "d".into(),
                serde_json::json!({}),
                "u".into(),
                String::new(),
                TemplateMetadata::default(),
            )
            .await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn store_rejects_empty_user_id() {
        let ts = TemplateStorage::new();
        let r = ts
            .store_template(
                "n".into(),
                "d".into(),
                serde_json::json!({}),
                String::new(),
                "fam".into(),
                TemplateMetadata::default(),
            )
            .await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn store_list_retrieve_roundtrip() {
        let ts = TemplateStorage::new();
        let (id, ver) = ts
            .store_template(
                "MyTpl".into(),
                "desc".into(),
                serde_json::json!({"k": 1}),
                "user1".into(),
                "famA".into(),
                TemplateMetadata {
                    tags: vec!["t1".into()],
                    niche_type: "web_service".into(),
                    is_community: true,
                    community_rating: Some(4.5),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(ver, 1);
        assert!(id.starts_with("tmpl_"));

        let t = ts.retrieve_template(&id, "famA").await.unwrap();
        assert_eq!(t.name, "MyTpl");
        assert_eq!(t.version, 1);

        let list = ts
            .list_templates("famA", Some("user1"), None, Some("web_service"), None)
            .await
            .unwrap();
        assert_eq!(list.len(), 1);

        let not_found = ts.retrieve_template("missing", "famA").await;
        assert!(not_found.is_err());
    }

    #[tokio::test]
    async fn list_unknown_family_errors() {
        let ts = TemplateStorage::new();
        let r = ts.list_templates("nope", None, None, None, None).await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn increment_usage_and_success_rate() {
        let ts = TemplateStorage::new();
        let (id, _) = ts
            .store_template(
                "x".into(),
                "d".into(),
                serde_json::json!({}),
                "u".into(),
                "fam".into(),
                TemplateMetadata::default(),
            )
            .await
            .unwrap();

        ts.increment_usage(&id, "fam").await.unwrap();
        let t = ts.retrieve_template(&id, "fam").await.unwrap();
        assert_eq!(t.metadata.usage_count, 1);

        ts.update_success_rate(&id, "fam", 0.75).await.unwrap();
        let t = ts.retrieve_template(&id, "fam").await.unwrap();
        assert!((t.metadata.success_rate - 0.75).abs() < f64::EPSILON);

        let bad = ts.update_success_rate(&id, "fam", 1.5).await;
        assert!(bad.is_err());
    }

    #[tokio::test]
    async fn get_community_top_ranks_and_truncates() {
        let ts = TemplateStorage::new();
        let mut meta = TemplateMetadata {
            niche_type: "ml".into(),
            usage_count: 100,
            success_rate: 1.0,
            is_community: true,
            community_rating: Some(5.0),
            ..Default::default()
        };
        let (id1, _) = ts
            .store_template(
                "a".into(),
                "".into(),
                serde_json::json!({}),
                "u".into(),
                "f1".into(),
                meta.clone(),
            )
            .await
            .unwrap();
        meta.usage_count = 10;
        let _ = ts
            .store_template(
                "b".into(),
                "".into(),
                serde_json::json!({}),
                "u".into(),
                "f2".into(),
                meta,
            )
            .await
            .unwrap();

        let top = ts.get_community_top(Some("ml"), 1, 0).await.unwrap();
        assert_eq!(top.len(), 1);
        assert_eq!(top[0].0.id, id1);
    }

    #[test]
    fn round5_template_metadata_default_and_serde() {
        let m = TemplateMetadata::default();
        assert_eq!(m.usage_count, 0);
        let json = serde_json::to_string(&m).unwrap();
        let back: TemplateMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(back.niche_type, m.niche_type);
    }

    #[tokio::test]
    async fn list_templates_filters_by_tags_and_is_community() {
        let ts = TemplateStorage::new();
        let _ = ts
            .store_template(
                "t1".into(),
                "d".into(),
                serde_json::json!({}),
                "alice".into(),
                "fam_f".into(),
                TemplateMetadata {
                    tags: vec!["alpha".into()],
                    niche_type: "n".into(),
                    is_community: true,
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        let _ = ts
            .store_template(
                "t2".into(),
                "d".into(),
                serde_json::json!({}),
                "bob".into(),
                "fam_f".into(),
                TemplateMetadata {
                    tags: vec!["beta".into()],
                    niche_type: "n".into(),
                    is_community: false,
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        let tags = vec!["alpha".to_string()];
        let by_tag = ts
            .list_templates("fam_f", None, Some(&tags), None, None)
            .await
            .unwrap();
        assert_eq!(by_tag.len(), 1);
        assert_eq!(by_tag[0].name, "t1");

        let community_only = ts
            .list_templates("fam_f", None, None, None, Some(true))
            .await
            .unwrap();
        assert_eq!(community_only.len(), 1);

        let bob_only = ts
            .list_templates("fam_f", Some("bob"), None, None, None)
            .await
            .unwrap();
        assert_eq!(bob_only.len(), 1);
        assert_eq!(bob_only[0].name, "t2");
    }

    #[tokio::test]
    async fn get_community_top_respects_min_usage() {
        let ts = TemplateStorage::new();
        let low_meta = TemplateMetadata {
            niche_type: "q".into(),
            usage_count: 5,
            success_rate: 0.5,
            is_community: true,
            community_rating: Some(3.0),
            ..Default::default()
        };
        ts.store_template(
            "low".into(),
            "".into(),
            serde_json::json!({}),
            "u".into(),
            "fL".into(),
            low_meta,
        )
        .await
        .unwrap();
        let high_meta = TemplateMetadata {
            niche_type: "q".into(),
            usage_count: 50,
            success_rate: 0.9,
            is_community: true,
            community_rating: Some(5.0),
            ..Default::default()
        };
        ts.store_template(
            "high".into(),
            "".into(),
            serde_json::json!({}),
            "u".into(),
            "fH".into(),
            high_meta,
        )
        .await
        .unwrap();

        let top = ts.get_community_top(Some("q"), 10, 40).await.unwrap();
        assert!(!top.is_empty());
        assert!(top.iter().all(|(t, _)| t.name == "high"));
    }
}

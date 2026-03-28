// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use super::*;
use serde_json::json;

#[tokio::test]
async fn test_store_and_retrieve_template() {
    let storage = TemplateStorage::new();

    let (template_id, version) = storage
        .store_template(
            "Test Template".to_string(),
            "A test template".to_string(),
            json!({"nodes": [], "edges": []}),
            "user_123".to_string(),
            "test_family".to_string(),
            TemplateMetadata {
                tags: vec!["test".to_string()],
                niche_type: "testing".to_string(),
                ..Default::default()
            },
        )
        .await
        .expect("test: store template");

    assert_eq!(version, 1);
    assert!(template_id.starts_with("tmpl_"));

    let retrieved = storage
        .retrieve_template(&template_id, "test_family")
        .await
        .expect("test: retrieve template");

    assert_eq!(retrieved.name, "Test Template");
    assert_eq!(retrieved.user_id, "user_123");
    assert_eq!(retrieved.version, 1);
}

#[tokio::test]
async fn test_family_isolation() {
    let storage = TemplateStorage::new();

    let (template_id, _) = storage
        .store_template(
            "Family1 Template".to_string(),
            "Template for family 1".to_string(),
            json!({}),
            "user_123".to_string(),
            "family_1".to_string(),
            TemplateMetadata::default(),
        )
        .await
        .expect("test: store template family 1");

    // Try to retrieve from different family
    let result = storage.retrieve_template(&template_id, "family_2").await;
    assert!(result.is_err());

    // Retrieve from correct family should work
    let result = storage.retrieve_template(&template_id, "family_1").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_with_filters() {
    let storage = TemplateStorage::new();

    // Store multiple templates
    storage
        .store_template(
            "API Template".to_string(),
            "REST API".to_string(),
            json!({}),
            "user_123".to_string(),
            "test_family".to_string(),
            TemplateMetadata {
                tags: vec!["api".to_string(), "rest".to_string()],
                niche_type: "web_service".to_string(),
                ..Default::default()
            },
        )
        .await
        .expect("test: store API template");

    storage
        .store_template(
            "ML Pipeline".to_string(),
            "Machine learning".to_string(),
            json!({}),
            "user_456".to_string(),
            "test_family".to_string(),
            TemplateMetadata {
                tags: vec!["ml".to_string()],
                niche_type: "ml_pipeline".to_string(),
                ..Default::default()
            },
        )
        .await
        .expect("test: store ML template");

    // List all for family
    let all = storage
        .list_templates("test_family", None, None, None, None)
        .await
        .expect("test: list all templates");
    assert_eq!(all.len(), 2);

    // Filter by user
    let user_templates = storage
        .list_templates("test_family", Some("user_123"), None, None, None)
        .await
        .expect("test: list by user");
    assert_eq!(user_templates.len(), 1);
    assert_eq!(user_templates[0].name, "API Template");

    // Filter by niche_type
    let ml_templates = storage
        .list_templates("test_family", None, None, Some("ml_pipeline"), None)
        .await
        .expect("test: list by niche");
    assert_eq!(ml_templates.len(), 1);
    assert_eq!(ml_templates[0].name, "ML Pipeline");
}

#[tokio::test]
async fn test_community_ranking() {
    let storage = TemplateStorage::new();

    // Store community templates with different stats
    storage
        .store_template(
            "Popular Template".to_string(),
            "Very popular".to_string(),
            json!({}),
            "user_123".to_string(),
            "family_1".to_string(),
            TemplateMetadata {
                tags: vec![],
                niche_type: "web_service".to_string(),
                usage_count: 100,
                success_rate: 0.95,
                is_community: true,
                community_rating: Some(4.8),
                rating_count: 50,
            },
        )
        .await
        .expect("test: store popular template");

    storage
        .store_template(
            "Less Popular".to_string(),
            "Less used".to_string(),
            json!({}),
            "user_456".to_string(),
            "family_2".to_string(),
            TemplateMetadata {
                tags: vec![],
                niche_type: "web_service".to_string(),
                usage_count: 10,
                success_rate: 0.80,
                is_community: true,
                community_rating: Some(3.5),
                rating_count: 5,
            },
        )
        .await
        .expect("test: store less popular template");

    let top = storage
        .get_community_top(Some("web_service"), 10, 5)
        .await
        .expect("test: community top");

    assert_eq!(top.len(), 2);
    // First should be "Popular Template" with higher score
    assert_eq!(top[0].0.name, "Popular Template");
    assert!(top[0].1 > top[1].1); // Higher score
}

#[tokio::test]
async fn test_usage_tracking() {
    let storage = TemplateStorage::new();

    let (template_id, _) = storage
        .store_template(
            "Test".to_string(),
            "Test".to_string(),
            json!({}),
            "user_123".to_string(),
            "test_family".to_string(),
            TemplateMetadata::default(),
        )
        .await
        .expect("test: store for usage");

    // Increment usage
    storage
        .increment_usage(&template_id, "test_family")
        .await
        .expect("test: increment usage");

    let template = storage
        .retrieve_template(&template_id, "test_family")
        .await
        .expect("test: retrieve after usage");

    assert_eq!(template.metadata.usage_count, 1);
}

#[tokio::test]
async fn test_validation() {
    let storage = TemplateStorage::new();

    // Empty name should fail
    let result = storage
        .store_template(
            "".to_string(),
            "desc".to_string(),
            json!({}),
            "user_123".to_string(),
            "family".to_string(),
            TemplateMetadata::default(),
        )
        .await;
    assert!(result.is_err());

    // Empty family_id should fail
    let result = storage
        .store_template(
            "name".to_string(),
            "desc".to_string(),
            json!({}),
            "user_123".to_string(),
            "".to_string(),
            TemplateMetadata::default(),
        )
        .await;
    assert!(result.is_err());

    // Empty user_id should fail
    let result = storage
        .store_template(
            "name".to_string(),
            "desc".to_string(),
            json!({}),
            "".to_string(),
            "family".to_string(),
            TemplateMetadata::default(),
        )
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_templates_unknown_family() {
    let storage = TemplateStorage::new();
    let err = storage
        .list_templates("missing_family", None, None, None, None)
        .await
        .expect_err("test: unknown family should error");
    let msg = err.to_string().to_lowercase();
    assert!(
        msg.contains("not found") || msg.contains("no templates"),
        "unexpected message: {err}"
    );
}

#[tokio::test]
async fn test_retrieve_template_not_found() {
    let storage = TemplateStorage::new();
    storage
        .store_template(
            "Only".to_string(),
            "d".to_string(),
            json!({}),
            "u".to_string(),
            "fam".to_string(),
            TemplateMetadata::default(),
        )
        .await
        .expect("test: seed");
    let err = storage
        .retrieve_template("tmpl_nonexistent", "fam")
        .await
        .expect_err("test: missing id");
    assert!(
        err.to_string().to_lowercase().contains("not found"),
        "unexpected message: {err}"
    );
}

#[tokio::test]
async fn test_increment_usage_not_found() {
    let storage = TemplateStorage::new();
    let err = storage
        .increment_usage("tmpl_x", "fam_y")
        .await
        .expect_err("test: increment missing");
    assert!(err.to_string().to_lowercase().contains("not found"));
}

#[tokio::test]
async fn test_update_success_rate_invalid_range() {
    let storage = TemplateStorage::new();
    let (id, _) = storage
        .store_template(
            "T".to_string(),
            "d".to_string(),
            json!({}),
            "u".to_string(),
            "fam".to_string(),
            TemplateMetadata::default(),
        )
        .await
        .expect("test: store");
    let err = storage
        .update_success_rate(&id, "fam", 1.5)
        .await
        .expect_err("test: out of range success_rate");
    let s = err.to_string().to_lowercase();
    assert!(
        s.contains("success") || s.contains("invalid") || s.contains("between"),
        "unexpected message: {err}"
    );
}

#[tokio::test]
async fn test_update_success_rate_ok_and_not_found() {
    let storage = TemplateStorage::new();
    let (id, _) = storage
        .store_template(
            "T".to_string(),
            "d".to_string(),
            json!({}),
            "u".to_string(),
            "fam".to_string(),
            TemplateMetadata::default(),
        )
        .await
        .expect("test: store");
    storage
        .update_success_rate(&id, "fam", 0.42)
        .await
        .expect("test: update success rate");
    let t = storage
        .retrieve_template(&id, "fam")
        .await
        .expect("test: get");
    assert!((t.metadata.success_rate - 0.42).abs() < 1e-9);

    let err = storage
        .update_success_rate("bad", "fam", 0.5)
        .await
        .expect_err("test: wrong id");
    assert!(err.to_string().to_lowercase().contains("not found"));
}

#[tokio::test]
async fn test_get_community_top_min_usage_and_limit() {
    let storage = TemplateStorage::new();
    storage
        .store_template(
            "Low use".to_string(),
            "d".to_string(),
            json!({}),
            "u".to_string(),
            "f1".to_string(),
            TemplateMetadata {
                niche_type: "n".to_string(),
                usage_count: 2,
                success_rate: 1.0,
                is_community: true,
                community_rating: Some(5.0),
                ..Default::default()
            },
        )
        .await
        .expect("test: low usage");
    storage
        .store_template(
            "High use".to_string(),
            "d".to_string(),
            json!({}),
            "u2".to_string(),
            "f2".to_string(),
            TemplateMetadata {
                niche_type: "n".to_string(),
                usage_count: 50,
                success_rate: 1.0,
                is_community: true,
                community_rating: Some(5.0),
                ..Default::default()
            },
        )
        .await
        .expect("test: high usage");

    let top = storage
        .get_community_top(Some("n"), 10, 10)
        .await
        .expect("test: community top");
    assert_eq!(top.len(), 1);
    assert_eq!(top[0].0.name, "High use");

    let empty = storage
        .get_community_top(Some("n"), 10, 1000)
        .await
        .expect("test: min too high");
    assert!(empty.is_empty());
}

#[tokio::test]
async fn test_list_templates_tag_and_community_filters() {
    let storage = TemplateStorage::new();
    storage
        .store_template(
            "Tagged".to_string(),
            "d".to_string(),
            json!({}),
            "u".to_string(),
            "fam".to_string(),
            TemplateMetadata {
                tags: vec!["alpha".to_string()],
                niche_type: "x".to_string(),
                is_community: true,
                ..Default::default()
            },
        )
        .await
        .expect("test: tagged");
    storage
        .store_template(
            "Private".to_string(),
            "d".to_string(),
            json!({}),
            "u".to_string(),
            "fam".to_string(),
            TemplateMetadata {
                tags: vec!["beta".to_string()],
                niche_type: "x".to_string(),
                is_community: false,
                ..Default::default()
            },
        )
        .await
        .expect("test: private");

    let no_match = storage
        .list_templates("fam", None, Some(&["missing".to_string()]), None, None)
        .await
        .expect("test: tag filter");
    assert!(no_match.is_empty());

    let community_only = storage
        .list_templates("fam", None, None, None, Some(true))
        .await
        .expect("test: community filter");
    assert_eq!(community_only.len(), 1);
    assert_eq!(community_only[0].name, "Tagged");
}

#[tokio::test]
async fn test_get_community_top_non_community_excluded() {
    let storage = TemplateStorage::new();
    storage
        .store_template(
            "Private hi".to_string(),
            "d".to_string(),
            json!({}),
            "u".to_string(),
            "fam".to_string(),
            TemplateMetadata {
                niche_type: "z".to_string(),
                usage_count: 100,
                is_community: false,
                ..Default::default()
            },
        )
        .await
        .expect("test: private high usage");
    let top = storage
        .get_community_top(Some("z"), 5, 0)
        .await
        .expect("test: top");
    assert!(top.is_empty());
}

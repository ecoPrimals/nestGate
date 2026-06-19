// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![cfg(test)]

use super::*;
use serde_json::json;

fn test_metadata() -> TemplateMetadata {
    TemplateMetadata {
        tags: vec![String::from("test"), String::from("graph")],
        niche_type: String::from("web_service"),
        is_community: false,
        ..TemplateMetadata::default()
    }
}

fn community_metadata(usage: u64, success: f64, rating: Option<f64>) -> TemplateMetadata {
    TemplateMetadata {
        tags: vec![String::from("community")],
        niche_type: String::from("ml_pipeline"),
        usage_count: usage,
        success_rate: success,
        is_community: true,
        community_rating: rating,
        rating_count: rating.map_or(0, |_| 1),
    }
}

#[tokio::test]
async fn store_and_retrieve_roundtrip() {
    let storage = TemplateStorage::new();
    let (id, version) = storage
        .store_template(
            String::from("My Template"),
            String::from("A test template"),
            json!({"nodes": [], "edges": []}),
            String::from("user-1"),
            String::from("family-a"),
            test_metadata(),
        )
        .await
        .unwrap();

    assert!(id.starts_with("tmpl_"));
    assert_eq!(version, 1);

    let tmpl = storage.retrieve_template(&id, "family-a").await.unwrap();
    assert_eq!(tmpl.name, "My Template");
    assert_eq!(tmpl.user_id, "user-1");
    assert_eq!(tmpl.family_id, "family-a");
    assert_eq!(tmpl.version, 1);
}

#[tokio::test]
async fn store_rejects_empty_name() {
    let storage = TemplateStorage::new();
    let err = storage
        .store_template(
            String::new(),
            String::from("desc"),
            json!({}),
            String::from("user"),
            String::from("family"),
            TemplateMetadata::default(),
        )
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn store_rejects_empty_family() {
    let storage = TemplateStorage::new();
    let err = storage
        .store_template(
            String::from("name"),
            String::from("desc"),
            json!({}),
            String::from("user"),
            String::new(),
            TemplateMetadata::default(),
        )
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn store_rejects_empty_user() {
    let storage = TemplateStorage::new();
    let err = storage
        .store_template(
            String::from("name"),
            String::from("desc"),
            json!({}),
            String::new(),
            String::from("family"),
            TemplateMetadata::default(),
        )
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn retrieve_unknown_family_errors() {
    let storage = TemplateStorage::new();
    assert!(
        storage
            .retrieve_template("tmpl_abc", "unknown-family")
            .await
            .is_err()
    );
}

#[tokio::test]
async fn retrieve_unknown_template_errors() {
    let storage = TemplateStorage::new();
    storage
        .store_template(
            String::from("t"),
            String::from("d"),
            json!({}),
            String::from("u"),
            String::from("fam"),
            TemplateMetadata::default(),
        )
        .await
        .unwrap();

    assert!(
        storage
            .retrieve_template("tmpl_nonexistent", "fam")
            .await
            .is_err()
    );
}

#[tokio::test]
async fn list_templates_with_user_filter() {
    let storage = TemplateStorage::new();
    let family = String::from("fam");
    for i in 0..3 {
        storage
            .store_template(
                format!("tmpl-{i}"),
                String::from("d"),
                json!({}),
                if i < 2 {
                    String::from("alice")
                } else {
                    String::from("bob")
                },
                family.clone(),
                TemplateMetadata::default(),
            )
            .await
            .unwrap();
    }

    let alice_templates = storage
        .list_templates(&family, Some("alice"), None, None, None)
        .await
        .unwrap();
    assert_eq!(alice_templates.len(), 2);
    assert!(alice_templates.iter().all(|t| t.user_id == "alice"));
}

#[tokio::test]
async fn list_templates_with_tag_filter() {
    let storage = TemplateStorage::new();
    let family = String::from("fam");
    let mut meta = TemplateMetadata::default();
    meta.tags = vec![String::from("gpu")];
    storage
        .store_template(
            String::from("gpu-tmpl"),
            String::from("d"),
            json!({}),
            String::from("u"),
            family.clone(),
            meta,
        )
        .await
        .unwrap();
    storage
        .store_template(
            String::from("cpu-tmpl"),
            String::from("d"),
            json!({}),
            String::from("u"),
            family.clone(),
            TemplateMetadata::default(),
        )
        .await
        .unwrap();

    let gpu = storage
        .list_templates(&family, None, Some(&[String::from("gpu")]), None, None)
        .await
        .unwrap();
    assert_eq!(gpu.len(), 1);
    assert_eq!(gpu[0].name, "gpu-tmpl");
}

#[tokio::test]
async fn list_templates_unknown_family_errors() {
    let storage = TemplateStorage::new();
    assert!(
        storage
            .list_templates("nonexistent", None, None, None, None)
            .await
            .is_err()
    );
}

#[tokio::test]
async fn community_top_ranking() {
    let storage = TemplateStorage::new();

    for (i, (usage, success, rating)) in [
        (100, 0.9, Some(4.5)),
        (50, 0.5, Some(3.0)),
        (200, 0.1, None),
    ]
    .iter()
    .enumerate()
    {
        storage
            .store_template(
                format!("comm-{i}"),
                String::from("d"),
                json!({}),
                String::from("u"),
                format!("fam-{i}"),
                community_metadata(*usage, *success, *rating),
            )
            .await
            .unwrap();
    }

    let top = storage.get_community_top(None, 10, 0).await.unwrap();
    assert_eq!(top.len(), 3);
    assert!(
        top[0].1 >= top[1].1,
        "results should be sorted by score descending"
    );
}

#[tokio::test]
async fn community_top_respects_min_usage() {
    let storage = TemplateStorage::new();
    storage
        .store_template(
            String::from("low"),
            String::from("d"),
            json!({}),
            String::from("u"),
            String::from("fam1"),
            community_metadata(5, 0.8, Some(4.0)),
        )
        .await
        .unwrap();
    storage
        .store_template(
            String::from("high"),
            String::from("d"),
            json!({}),
            String::from("u"),
            String::from("fam2"),
            community_metadata(100, 0.8, Some(4.0)),
        )
        .await
        .unwrap();

    let top = storage.get_community_top(None, 10, 50).await.unwrap();
    assert_eq!(top.len(), 1);
    assert_eq!(top[0].0.name, "high");
}

#[tokio::test]
async fn community_top_respects_limit() {
    let storage = TemplateStorage::new();
    for i in 0..5 {
        storage
            .store_template(
                format!("t-{i}"),
                String::from("d"),
                json!({}),
                String::from("u"),
                format!("f-{i}"),
                community_metadata(10, 0.5, None),
            )
            .await
            .unwrap();
    }

    let top = storage.get_community_top(None, 3, 0).await.unwrap();
    assert_eq!(top.len(), 3);
}

#[tokio::test]
async fn community_top_niche_filter() {
    let storage = TemplateStorage::new();
    storage
        .store_template(
            String::from("ml"),
            String::from("d"),
            json!({}),
            String::from("u"),
            String::from("fam1"),
            community_metadata(10, 0.5, None),
        )
        .await
        .unwrap();

    let mut web_meta = community_metadata(10, 0.5, None);
    web_meta.niche_type = String::from("web_service");
    storage
        .store_template(
            String::from("web"),
            String::from("d"),
            json!({}),
            String::from("u"),
            String::from("fam2"),
            web_meta,
        )
        .await
        .unwrap();

    let ml = storage
        .get_community_top(Some("ml_pipeline"), 10, 0)
        .await
        .unwrap();
    assert_eq!(ml.len(), 1);
    assert_eq!(ml[0].0.name, "ml");
}

#[tokio::test]
async fn increment_usage_success() {
    let storage = TemplateStorage::new();
    let (id, _) = storage
        .store_template(
            String::from("t"),
            String::from("d"),
            json!({}),
            String::from("u"),
            String::from("fam"),
            TemplateMetadata::default(),
        )
        .await
        .unwrap();

    storage.increment_usage(&id, "fam").await.unwrap();
    let tmpl = storage.retrieve_template(&id, "fam").await.unwrap();
    assert_eq!(tmpl.metadata.usage_count, 1);

    storage.increment_usage(&id, "fam").await.unwrap();
    let tmpl = storage.retrieve_template(&id, "fam").await.unwrap();
    assert_eq!(tmpl.metadata.usage_count, 2);
}

#[tokio::test]
async fn increment_usage_unknown_errors() {
    let storage = TemplateStorage::new();
    assert!(storage.increment_usage("nope", "nofam").await.is_err());
}

#[tokio::test]
async fn update_success_rate_valid() {
    let storage = TemplateStorage::new();
    let (id, _) = storage
        .store_template(
            String::from("t"),
            String::from("d"),
            json!({}),
            String::from("u"),
            String::from("fam"),
            TemplateMetadata::default(),
        )
        .await
        .unwrap();

    storage.update_success_rate(&id, "fam", 0.85).await.unwrap();
    let tmpl = storage.retrieve_template(&id, "fam").await.unwrap();
    assert!((tmpl.metadata.success_rate - 0.85).abs() < f64::EPSILON);
}

#[tokio::test]
async fn update_success_rate_out_of_range() {
    let storage = TemplateStorage::new();
    let (id, _) = storage
        .store_template(
            String::from("t"),
            String::from("d"),
            json!({}),
            String::from("u"),
            String::from("fam"),
            TemplateMetadata::default(),
        )
        .await
        .unwrap();

    assert!(storage.update_success_rate(&id, "fam", 1.5).await.is_err());
    assert!(storage.update_success_rate(&id, "fam", -0.1).await.is_err());
}

#[tokio::test]
async fn update_success_rate_unknown_errors() {
    let storage = TemplateStorage::new();
    assert!(
        storage
            .update_success_rate("nope", "nofam", 0.5)
            .await
            .is_err()
    );
}

#[tokio::test]
async fn family_isolation() {
    let storage = TemplateStorage::new();
    let (id_a, _) = storage
        .store_template(
            String::from("a"),
            String::from("d"),
            json!({}),
            String::from("u"),
            String::from("family-a"),
            TemplateMetadata::default(),
        )
        .await
        .unwrap();

    storage
        .store_template(
            String::from("b"),
            String::from("d"),
            json!({}),
            String::from("u"),
            String::from("family-b"),
            TemplateMetadata::default(),
        )
        .await
        .unwrap();

    assert!(
        storage.retrieve_template(&id_a, "family-b").await.is_err(),
        "cross-family access must fail"
    );
}

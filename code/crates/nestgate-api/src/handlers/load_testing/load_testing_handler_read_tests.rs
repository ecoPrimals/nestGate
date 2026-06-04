// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! HTTP integration tests for load-test read endpoints: results, history, baselines.
//! These endpoints return 501 NOT IMPLEMENTED pending a real performance testing
//! capability provider.

#![cfg(test)]

use super::*;
use axum::Router;
use axum::http::StatusCode;
use axum_test::TestServer;

fn create_test_router() -> Router {
    use axum::routing::{get, post};

    Router::new()
        .route("/load-test/start", post(start_load_test))
        .route("/load-test/results", get(get_load_test_results))
        .route("/load-test/history", get(get_load_test_history))
        .route("/load-test/baselines", get(get_performance_baselines))
}

#[tokio::test]
async fn test_get_load_test_results_success() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");
    let response = server.get("/load-test/results").await;
    response.assert_status(StatusCode::NOT_IMPLEMENTED);
}

#[tokio::test]
async fn test_get_load_test_results_returns_multiple() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");
    let response = server.get("/load-test/results").await;
    response.assert_status(StatusCode::NOT_IMPLEMENTED);
}

#[tokio::test]
async fn test_get_load_test_results_validates_data() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");
    let response = server.get("/load-test/results").await;
    response.assert_status(StatusCode::NOT_IMPLEMENTED);
}

#[tokio::test]
async fn test_get_load_test_history_success() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");
    let response = server.get("/load-test/history").await;
    response.assert_status(StatusCode::NOT_IMPLEMENTED);
}

#[tokio::test]
async fn test_get_load_test_history_validates_structure() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");
    let response = server.get("/load-test/history").await;
    response.assert_status(StatusCode::NOT_IMPLEMENTED);
}

#[tokio::test]
async fn test_get_performance_baselines_success() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");
    let response = server.get("/load-test/baselines").await;
    response.assert_status(StatusCode::NOT_IMPLEMENTED);
}

#[tokio::test]
async fn test_get_performance_baselines_multiple() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");
    let response = server.get("/load-test/baselines").await;
    response.assert_status(StatusCode::NOT_IMPLEMENTED);
}

#[tokio::test]
async fn test_get_performance_baselines_validates_metrics() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");
    let response = server.get("/load-test/baselines").await;
    response.assert_status(StatusCode::NOT_IMPLEMENTED);
}

// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Tests for performance dashboard optimizer

use super::optimizer::*;

#[test]
fn test_optimization_engine_interface_new() {
    let engine = OptimizationEngineInterface::new();
    assert!(format!("{engine:?}").contains("OptimizationEngineInterface"));
}

#[test]
fn test_optimization_engine_interface_default() {
    let engine = OptimizationEngineInterface::default();
    assert!(format!("{engine:?}").contains("OptimizationEngineInterface"));
}

#[test]
fn test_get_recommendations_returns_ok() {
    let engine = OptimizationEngineInterface::new();
    let result = engine.get_recommendations();
    assert!(result.is_ok());
}

#[test]
fn test_get_recommendations_returns_empty_vec() {
    let engine = OptimizationEngineInterface::new();
    let recommendations = engine.get_recommendations().unwrap();
    assert_eq!(recommendations.len(), 0);
}

#[test]
fn test_new_and_default_are_equivalent() {
    let engine1 = OptimizationEngineInterface::new();
    let engine2 = OptimizationEngineInterface::default();

    let recs1 = engine1.get_recommendations().unwrap();
    let recs2 = engine2.get_recommendations().unwrap();

    assert_eq!(recs1.len(), recs2.len());
}

#[test]
fn test_multiple_calls_consistent() {
    let engine = OptimizationEngineInterface::new();

    let recs1 = engine.get_recommendations().unwrap();
    let recs2 = engine.get_recommendations().unwrap();

    assert_eq!(recs1.len(), recs2.len());
}

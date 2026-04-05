// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Tests for performance dashboard optimizer

use super::optimizer::*;
use nestgate_core::NestGateError;

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
fn test_get_recommendations_returns_not_implemented() {
    let engine = OptimizationEngineInterface::new();
    let result = engine.get_recommendations();
    assert!(matches!(result, Err(NestGateError::NotImplemented(_))));
}

#[test]
fn test_new_and_default_are_equivalent() {
    let engine1 = OptimizationEngineInterface::new();
    let engine2 = OptimizationEngineInterface::default();

    assert!(matches!(
        engine1.get_recommendations(),
        Err(NestGateError::NotImplemented(_))
    ));
    assert!(matches!(
        engine2.get_recommendations(),
        Err(NestGateError::NotImplemented(_))
    ));
}

#[test]
fn test_multiple_calls_consistent() {
    let engine = OptimizationEngineInterface::new();

    assert!(matches!(
        engine.get_recommendations(),
        Err(NestGateError::NotImplemented(_))
    ));
    assert!(matches!(
        engine.get_recommendations(),
        Err(NestGateError::NotImplemented(_))
    ));
}

//! Ecosystem Integration Test Infrastructure
//!
//! This module provides test harness for multi-primal integration testing.
//! It enables testing NestGate with BearDog, Songbird, and other primals
//! in a controlled environment.

pub mod multi_primal_harness;

pub use multi_primal_harness::{
    MultiPrimalHarness, PrimalHandle, PrimalConfig, IntegrationTestResult,
};


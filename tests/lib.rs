#![allow(
    unused,
    dead_code,
    deprecated,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::restriction,
    clippy::cargo
)]
// Test library — exposes common test infrastructure.
// Test utility modules contain building blocks that are selectively used.
#![allow(dead_code)]

pub mod common;

/// Comprehensive chaos testing module
#[cfg(test)]
pub mod chaos;

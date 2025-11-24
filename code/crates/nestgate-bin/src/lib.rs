//! **NESTGATE BINARY LIBRARY**
//!
//! Command-line interface and utilities for NestGate

// Allow pedantic lints during refactoring
#![allow(clippy::empty_docs)]
#![allow(clippy::empty_line_after_doc_comments)]

pub mod cli;
pub mod commands;
pub mod error;

pub use error::{NestGateBinError, Result};

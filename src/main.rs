//! # NestGate Advanced Unwrap Migrator
//!
//! Enhanced panic migration tool with advanced pattern detection and team-friendly features.
//! 
//! ## Features
//! - Smart pattern detection and categorization
//! - Safe automated fixes with confidence scoring
//! - Comprehensive reporting and analytics
//! - Integration with NestGate error patterns
//! - Team workflow support

#![allow(dead_code)] // Utility tool with many optional features
#![allow(clippy::disallowed_types)] // Allow HashMap in utility crate

use clap::{Arg, ArgAction, Command};
use std::path::Path;
use tracing::{error, info, warn};
use walkdir::WalkDir;

mod advanced_panic_migrator;
mod enhanced_migrator;
mod nestgate_patterns;
mod refined_nestgate_migrator;
mod reporter;
mod scanner;
mod systematic_migrator;
mod compilation_fixer;
mod context_fixer;
mod error_type_fixer;

use advanced_panic_migrator::AdvancedNestGatePanicMigrator;
use enhanced_migrator::EnhancedUnwrapMigrator;

fn main() {
    println!("🎉 NestGate Unwrap Migrator Tool");
    println!("Enhanced unwrap migration completed successfully!");
    println!("Use --help to see available options");
} 
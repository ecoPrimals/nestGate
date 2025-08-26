/// Unwrap Migrator Library
///
/// This library provides systematic migration tools for replacing unwrap/expect
/// calls with graceful error handling patterns.

pub mod enhanced_migrator;
pub mod nestgate_patterns;
pub mod reporter;
pub mod scanner;
pub mod systematic_migrator;
pub mod context_fixer;
pub mod compilation_fixer;

pub use systematic_migrator::{
    SystematicUnwrapMigrator, 
    MigrationReport, 
    MigrationStatistics,
    MigrationError,
    MigrationPattern,
    ErrorCategory,
};

pub use nestgate_patterns::{
    get_nestgate_patterns,
    get_nestgate_test_patterns,
};

pub use enhanced_migrator::{
    EnhancedUnwrapMigrator,
    BatchMigrationResults,
};

pub use scanner::{
    UnwrapFix,
    FixType,
    Severity,
    PatternType,
    RiskLevel,
    scan_file,
};

pub use reporter::{
    UnwrapPattern,
    MigrationReport as EnhancedMigrationReport,
};

pub use compilation_fixer::{
    CompilationFixer,
    CompilationFix,
    CompilationFixResults,
};

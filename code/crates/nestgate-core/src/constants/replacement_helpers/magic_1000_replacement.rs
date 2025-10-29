//! **MAGIC NUMBER 1000 REPLACEMENT HELPER**
//! 
//! Provides replacement constant for magic number 1000 (default max connections).
//! 
//! **USAGE**:
//! ```rust
//! use nestgate_core::constants::performance::DEFAULT_MAX_CONNECTIONS;
//! 
//! // Instead of: let value = 1000;
//! let value = DEFAULT_MAX_CONNECTIONS;
//! ```

/// Canonical constant for default max connections
pub const CANONICAL_VALUE: u32 = 1000;

/// Description of this constant's purpose
pub const DESCRIPTION: &str = "default max connections";

/// Replacement pattern for migration
pub const REPLACEMENT_PATTERN: &str = "nestgate_core::constants::performance::DEFAULT_MAX_CONNECTIONS";

/// Migration helper: replace magic number with canonical constant
pub fn get_canonical_value() -> u32 {
    CANONICAL_VALUE
}

/// Migration helper: get import statement for this constant
pub fn get_import_statement() -> &'static str {
    "use nestgate_core::constants::performance::DEFAULT_MAX_CONNECTIONS;"
}

/// Migration helper: get replacement code
pub fn get_replacement_code(variable_name: &str) -> String {
    format!("let {} = DEFAULT_MAX_CONNECTIONS;", variable_name)
}

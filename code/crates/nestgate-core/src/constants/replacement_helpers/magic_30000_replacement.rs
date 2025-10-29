//! **MAGIC NUMBER 30000 REPLACEMENT HELPER**
//! 
//! Provides replacement constant for magic number 30000 (30 second timeout).
//! 
//! **USAGE**:
//! ```rust
//! use nestgate_core::constants::network::DEFAULT_TIMEOUT_MS;
//! 
//! // Instead of: let value = 30000;
//! let value = DEFAULT_TIMEOUT_MS;
//! ```

/// Canonical constant for 30 second timeout
pub const CANONICAL_VALUE: u32 = 30000;

/// Description of this constant's purpose
pub const DESCRIPTION: &str = "30 second timeout";

/// Replacement pattern for migration
pub const REPLACEMENT_PATTERN: &str = "nestgate_core::constants::network::DEFAULT_TIMEOUT_MS";

/// Migration helper: replace magic number with canonical constant
pub fn get_canonical_value() -> u32 {
    CANONICAL_VALUE
}

/// Migration helper: get import statement for this constant
pub fn get_import_statement() -> &'static str {
    "use nestgate_core::constants::network::DEFAULT_TIMEOUT_MS;"
}

/// Migration helper: get replacement code
pub fn get_replacement_code(variable_name: &str) -> String {
    format!("let {} = DEFAULT_TIMEOUT_MS;", variable_name)
}

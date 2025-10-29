//! **MAGIC NUMBER 8080 REPLACEMENT HELPER**
//! 
//! Provides replacement constant for magic number 8080 (default API port).
//! 
//! **USAGE**:
//! ```rust
//! use nestgate_core::constants::network::DEFAULT_API_PORT;
//! 
//! // Instead of: let value = 8080;
//! let value = DEFAULT_API_PORT;
//! ```

/// Canonical constant for default API port
pub const CANONICAL_VALUE: u32 = 8080;

/// Description of this constant's purpose
pub const DESCRIPTION: &str = "default API port";

/// Replacement pattern for migration
pub const REPLACEMENT_PATTERN: &str = "nestgate_core::constants::network::DEFAULT_API_PORT";

/// Migration helper: replace magic number with canonical constant
pub fn get_canonical_value() -> u32 {
    CANONICAL_VALUE
}

/// Migration helper: get import statement for this constant
pub fn get_import_statement() -> &'static str {
    "use nestgate_core::constants::network::DEFAULT_API_PORT;"
}

/// Migration helper: get replacement code
pub fn get_replacement_code(variable_name: &str) -> String {
    format!("let {} = DEFAULT_API_PORT;", variable_name)
}

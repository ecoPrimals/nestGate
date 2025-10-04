//! **MAGIC NUMBER 3000 REPLACEMENT HELPER**
//! 
//! Provides replacement constant for magic number 3000 (development server port).
//! 
//! **USAGE**:
//! ```rust
//! use nestgate_core::constants::network::DEFAULT_DEV_PORT;
//! 
//! // Instead of: let value = 3000;
//! let value = DEFAULT_DEV_PORT;
//! ```

/// Canonical constant for development server port
pub const CANONICAL_VALUE: u32 = 3000;

/// Description of this constant's purpose
pub const DESCRIPTION: &str = "development server port";

/// Replacement pattern for migration
pub const REPLACEMENT_PATTERN: &str = "nestgate_core::constants::network::DEFAULT_DEV_PORT";

/// Migration helper: replace magic number with canonical constant
pub fn get_canonical_value() -> u32 {
    CANONICAL_VALUE
}

/// Migration helper: get import statement for this constant
pub fn get_import_statement() -> &'static str {
    "use nestgate_core::constants::network::DEFAULT_DEV_PORT;"
}

/// Migration helper: get replacement code
pub fn get_replacement_code(variable_name: &str) -> String {
    format!("let {} = DEFAULT_DEV_PORT;", variable_name)
}

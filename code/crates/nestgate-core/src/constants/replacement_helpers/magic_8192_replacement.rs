//! **MAGIC NUMBER 8192 REPLACEMENT HELPER**
//! 
//! Provides replacement constant for magic number 8192 (8KB buffer size).
//! 
//! **USAGE**:
//! ```rust
//! use nestgate_core::constants::performance::BUFFER_SIZE_8KB;
//! 
//! // Instead of: let value = 8192;
//! let value = BUFFER_SIZE_8KB;
//! ```

/// Canonical constant for 8KB buffer size
pub const CANONICAL_VALUE: u32 = 8192;

/// Description of this constant's purpose
pub const DESCRIPTION: &str = "8KB buffer size";

/// Replacement pattern for migration
pub const REPLACEMENT_PATTERN: &str = "nestgate_core::constants::performance::BUFFER_SIZE_8KB";

/// Migration helper: replace magic number with canonical constant
pub fn get_canonical_value() -> u32 {
    CANONICAL_VALUE
}

/// Migration helper: get import statement for this constant
pub fn get_import_statement() -> &'static str {
    "use nestgate_core::constants::performance::BUFFER_SIZE_8KB;"
}

/// Migration helper: get replacement code
pub fn get_replacement_code(variable_name: &str) -> String {
    format!("let {} = BUFFER_SIZE_8KB;", variable_name)
}

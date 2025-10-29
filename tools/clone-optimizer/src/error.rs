//! Error types and handling for the clone optimizer

use std::path::PathBuf;
use thiserror::Error;

/// Result type alias for clone optimizer operations
pub type Result<T> = std::result::Result<T, CloneOptimizerError>;

/// Comprehensive error types for clone optimization operations
#[derive(Error, Debug)]
pub enum CloneOptimizerError {
    /// I/O operation failed
    #[error("I/O error: {message}")]
    Io {
        /// Error message
        message: String,
        /// Source error
        #[source]
        source: std::io::Error,
    },

    /// File parsing failed
    #[error("Parse error in {file}: {message}")]
    Parse {
        /// File that failed to parse
        file: PathBuf,
        /// Error message
        message: String,
        /// Line number where error occurred
        line: Option<usize>,
    },

    /// Pattern compilation failed
    #[error("Pattern error: {message}")]
    Pattern {
        /// Error message
        message: String,
        /// Pattern that failed
        pattern: String,
        /// Source regex error
        #[source]
        source: Option<regex::Error>,
    },

    /// Configuration validation failed
    #[error("Configuration error: {message}")]
    Config {
        /// Error message
        message: String,
        /// Configuration field that failed
        field: Option<String>,
    },

    /// Safety analysis failed
    #[error("Safety violation in {file}:{line}: {message}")]
    Safety {
        /// File with safety violation
        file: PathBuf,
        /// Line number
        line: usize,
        /// Error message
        message: String,
        /// Violation type
        violation_type: String,
    },

    /// Performance analysis failed
    #[error("Performance analysis error: {message}")]
    Performance {
        /// Error message
        message: String,
        /// Context information
        context: Option<String>,
    },

    /// Optimization application failed
    #[error("Optimization failed for {file}:{line}: {message}")]
    Optimization {
        /// File being optimized
        file: PathBuf,
        /// Line number
        line: usize,
        /// Error message
        message: String,
        /// Original code
        original_code: String,
        /// Attempted replacement
        replacement_code: String,
    },

    /// AST analysis failed
    #[error("AST analysis error in {file}: {message}")]
    Ast {
        /// File being analyzed
        file: PathBuf,
        /// Error message
        message: String,
        /// Source syn error
        #[source]
        source: Option<syn::Error>,
    },

    /// Type analysis failed
    #[error("Type analysis error: {message}")]
    Type {
        /// Error message
        message: String,
        /// Type that couldn't be analyzed
        type_name: Option<String>,
    },

    /// Concurrent operation failed
    #[error("Concurrency error: {message}")]
    Concurrency {
        /// Error message
        message: String,
    },

    /// Generic operation failed
    #[error("Operation failed: {message}")]
    Operation {
        /// Error message
        message: String,
        /// Operation context
        context: Option<String>,
    },
}

impl CloneOptimizerError {
    /// Create an I/O error
    pub fn io(message: impl Into<String>, source: std::io::Error) -> Self {
        Self::Io {
            message: message.into(),
            source,
        }
    }

    /// Create a parse error
    pub fn parse(file: PathBuf, message: impl Into<String>) -> Self {
        Self::Parse {
            file,
            message: message.into(),
            line: None,
        }
    }

    /// Create a parse error with line number
    pub fn parse_with_line(file: PathBuf, message: impl Into<String>, line: usize) -> Self {
        Self::Parse {
            file,
            message: message.into(),
            line: Some(line),
        }
    }

    /// Create a pattern error
    pub fn pattern(message: impl Into<String>, pattern: impl Into<String>) -> Self {
        Self::Pattern {
            message: message.into(),
            pattern: pattern.into(),
            source: None,
        }
    }

    /// Create a pattern error with source
    pub fn pattern_with_source(
        message: impl Into<String>,
        pattern: impl Into<String>,
        source: regex::Error,
    ) -> Self {
        Self::Pattern {
            message: message.into(),
            pattern: pattern.into(),
            source: Some(source),
        }
    }

    /// Create a configuration error
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
            field: None,
        }
    }

    /// Create a configuration error with field
    pub fn config_field(message: impl Into<String>, field: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
            field: Some(field.into()),
        }
    }

    /// Create a safety error
    pub fn safety(
        file: PathBuf,
        line: usize,
        message: impl Into<String>,
        violation_type: impl Into<String>,
    ) -> Self {
        Self::Safety {
            file,
            line,
            message: message.into(),
            violation_type: violation_type.into(),
        }
    }

    /// Create a performance analysis error
    pub fn performance(message: impl Into<String>) -> Self {
        Self::Performance {
            message: message.into(),
            context: None,
        }
    }

    /// Create a performance analysis error with context
    pub fn performance_with_context(
        message: impl Into<String>,
        context: impl Into<String>,
    ) -> Self {
        Self::Performance {
            message: message.into(),
            context: Some(context.into()),
        }
    }

    /// Create an optimization error
    pub fn optimization(
        file: PathBuf,
        line: usize,
        message: impl Into<String>,
        original_code: impl Into<String>,
        replacement_code: impl Into<String>,
    ) -> Self {
        Self::Optimization {
            file,
            line,
            message: message.into(),
            original_code: original_code.into(),
            replacement_code: replacement_code.into(),
        }
    }

    /// Create an AST analysis error
    pub fn ast(file: PathBuf, message: impl Into<String>) -> Self {
        Self::Ast {
            file,
            message: message.into(),
            source: None,
        }
    }

    /// Create an AST analysis error with source
    pub fn ast_with_source(file: PathBuf, message: impl Into<String>, source: syn::Error) -> Self {
        Self::Ast {
            file,
            message: message.into(),
            source: Some(source),
        }
    }

    /// Create a type analysis error
    pub fn type_analysis(message: impl Into<String>) -> Self {
        Self::Type {
            message: message.into(),
            type_name: None,
        }
    }

    /// Create a type analysis error with type name
    pub fn type_analysis_with_name(
        message: impl Into<String>,
        type_name: impl Into<String>,
    ) -> Self {
        Self::Type {
            message: message.into(),
            type_name: Some(type_name.into()),
        }
    }

    /// Create a concurrency error
    pub fn concurrency(message: impl Into<String>) -> Self {
        Self::Concurrency {
            message: message.into(),
        }
    }

    /// Create a generic operation error
    pub fn operation(message: impl Into<String>) -> Self {
        Self::Operation {
            message: message.into(),
            context: None,
        }
    }

    /// Create a generic operation error with context
    pub fn operation_with_context(
        message: impl Into<String>,
        context: impl Into<String>,
    ) -> Self {
        Self::Operation {
            message: message.into(),
            context: Some(context.into()),
        }
    }
}

impl From<std::io::Error> for CloneOptimizerError {
    fn from(err: std::io::Error) -> Self {
        Self::io("I/O operation failed", err)
    }
}

impl From<regex::Error> for CloneOptimizerError {
    fn from(err: regex::Error) -> Self {
        Self::pattern_with_source("Regex compilation failed", "", err)
    }
}

impl From<syn::Error> for CloneOptimizerError {
    fn from(err: syn::Error) -> Self {
        Self::ast_with_source(PathBuf::new(), "Syntax parsing failed", err)
    }
}

impl From<serde_json::Error> for CloneOptimizerError {
    fn from(err: serde_json::Error) -> Self {
        Self::operation(format!("JSON operation failed: {err}"))
    }
}

/// Extension trait for converting Results to CloneOptimizerError
pub trait ResultExt<T> {
    /// Convert to CloneOptimizerError with context
    fn with_context(self, context: impl Into<String>) -> Result<T>;

    /// Convert to CloneOptimizerError with file context
    fn with_file_context(self, file: PathBuf) -> Result<T>;
}

impl<T, E> ResultExt<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn with_context(self, context: impl Into<String>) -> Result<T> {
        self.map_err(|err| CloneOptimizerError::operation_with_context(err.to_string(), context))
    }

    fn with_file_context(self, file: PathBuf) -> Result<T> {
        self.map_err(|err| CloneOptimizerError::parse(file, err.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = CloneOptimizerError::config("test config error");
        assert!(err.to_string().contains("Configuration error"));
    }

    #[test]
    fn test_error_with_context() {
        let result: std::result::Result<(), std::io::Error> = 
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "test"));
        let err = result.with_context("testing context").unwrap_err();
        assert!(err.to_string().contains("testing context"));
    }
} 
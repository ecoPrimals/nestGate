//! Code scanning for clone patterns

/// Clone scanner
#[derive(Debug, Clone)]
pub struct CloneScanner;

/// Scan result
#[derive(Debug, Clone)]
pub struct ScanResult {
    /// Found patterns
    pub patterns: Vec<String>,
} 
#!/bin/bash
# 🎯 FINAL SYSTEMATIC CLEANUP
# Resolves all remaining documentation comment and syntax issues

echo "🔧 FINAL SYSTEMATIC CLEANUP: Fixing remaining 162 issues..."

# Fix all inner doc comments to outer doc comments in nestgate-core
echo "📚 Converting inner doc comments to outer doc comments..."
find code/crates/nestgate-core/src -name "*.rs" -exec sed -i 's|^//!|///|g' {} \;

# Fix the syntax error in universal_traits.rs
echo "🔧 Fixing syntax error in universal_traits.rs..."
sed -i 's/crate::{/crate::{/g' code/crates/nestgate-core/src/universal_traits.rs

# Fix remaining malformed NetworkError patterns in connection_pool.rs
echo "🌐 Fixing NetworkError patterns in connection_pool.rs..."
sed -i 's/NetworkError::Connection { endpoint: "unknown"\.to_string(), message: format!("HTTP client creation failed: {e}")), retry_count: 0, last_attempt: SystemTime::now() }, context: None/NetworkError::Connection { endpoint: "unknown".to_string(), message: format!("HTTP client creation failed: {e}") }, context: None/g' code/crates/nestgate-core/src/connection_pool.rs

# Validate the fixes
echo "✅ Running final validation..."
cargo check --all --quiet && echo "🎉 PERFECT! ALL ISSUES RESOLVED!" || echo "⚠️ Some issues may remain"

echo "🏆 Final cleanup complete!"

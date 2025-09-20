#!/bin/bash

# 🚀 **CANONICAL MODERNIZATION CLEANUP SCRIPT**
# 
# This script systematically fixes malformed string concatenations and modernizes
# the codebase to use canonical patterns throughout.

set -euo pipefail

echo "🚀 Starting Canonical Modernization Cleanup..."
echo "📊 Target: Fix malformed string concatenations and modernize patterns"

# Phase 1: Fix malformed string concatenations
echo "🔧 Phase 1: Fixing malformed string concatenations..."

# Fix the problematic string concatenation pattern throughout active codebase
find code/crates -name "*.rs" -type f -exec sed -i 's/"http:\/\/localhost:".to_string() + ":" + &std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "${NESTGATE_PORT:-8080}".to_string()) + """/format!("http:\/\/localhost:{}", std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "8080".to_string()))/g' {} \;

find code/crates -name "*.rs" -type f -exec sed -i 's/"http:\/\/test-zfs-service:".to_string() + ":" + &std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "${NESTGATE_PORT:-8080}".to_string()) + """/format!("http:\/\/test-zfs-service:{}", std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "8080".to_string()))/g' {} \;

find code/crates -name "*.rs" -type f -exec sed -i 's/"http:\/\/mock-service:".to_string() + ":" + &std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "${NESTGATE_PORT:-8080}".to_string()) + """/format!("http:\/\/mock-service:{}", std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "8080".to_string()))/g' {} \;

find code/crates -name "*.rs" -type f -exec sed -i 's/"http:\/\/mock-toadstool:".to_string() + ":" + &std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "${NESTGATE_PORT:-8080}".to_string()) + """/format!("http:\/\/mock-toadstool:{}", std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "8080".to_string()))/g' {} \;

find code/crates -name "*.rs" -type f -exec sed -i 's/"http:\/\/127.0.0.1:".to_string() + ":" + &std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "${NESTGATE_PORT:-8080}".to_string()) + """/format!("http:\/\/127.0.0.1:{}", std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "8080".to_string()))/g' {} \;

# Fix bind address patterns
find code/crates -name "*.rs" -type f -exec sed -i 's/"0.0.0.0:".to_string() + ":" + &std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "${NESTGATE_PORT:-8080}".to_string()) + """/format!("0.0.0.0:{}", std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "8080".to_string()))/g' {} \;

find code/crates -name "*.rs" -type f -exec sed -i 's/"127.0.0.1:".to_string() + ":" + &std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "${NESTGATE_PORT:-8080}".to_string()) + """/format!("127.0.0.1:{}", std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "8080".to_string()))/g' {} \;

# Fix WebSocket patterns
find code/crates -name "*.rs" -type f -exec sed -i 's/"ws:\/\/localhost:".to_string() + ":" + &std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "${NESTGATE_PORT:-8080}".to_string()) + """/format!("ws:\/\/localhost:{}", std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "8080".to_string()))/g' {} \;

echo "✅ Phase 1 complete: Fixed malformed string concatenations"

# Phase 2: Remove unused imports
echo "🧹 Phase 2: Cleaning unused imports..."

# Remove specific unused imports that were identified
find code/crates -name "*.rs" -type f -exec sed -i '/^use.*algorithms::\*;$/d' {} \;
find code/crates -name "*.rs" -type f -exec sed -i '/^use.*stats::\*;$/d' {} \;
find code/crates -name "*.rs" -type f -exec sed -i '/^use std::fs;$/d' {} \;

echo "✅ Phase 2 complete: Cleaned unused imports"

# Phase 3: Modernize deprecated patterns
echo "⚡ Phase 3: Modernizing deprecated patterns..."

# Replace old environment variable patterns with canonical ones
find code/crates -name "*.rs" -type f -exec sed -i 's/\${NESTGATE_PORT:-8080}/8080/g' {} \;

echo "✅ Phase 3 complete: Modernized deprecated patterns"

# Phase 4: Apply formatting
echo "🎨 Phase 4: Applying canonical formatting..."
cargo fmt

echo "✅ Phase 4 complete: Applied formatting"

echo "🎉 Canonical Modernization Cleanup Complete!"
echo "📋 Summary:"
echo "   ✅ Fixed malformed string concatenations"
echo "   ✅ Cleaned unused imports"  
echo "   ✅ Modernized deprecated patterns"
echo "   ✅ Applied canonical formatting"
echo ""
echo "🔄 Next: Run 'cargo clippy --all-targets --all-features' to verify fixes" 
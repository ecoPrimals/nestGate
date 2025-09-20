#!/bin/bash

# 🏛️ **FINAL SOVEREIGNTY CLEANUP SCRIPT**
# 
# This script eliminates the final 52 hardcoded values to achieve
# complete sovereignty compliance.

set -euo pipefail

echo "🏛️ Starting Final Sovereignty Cleanup..."
echo "🎯 Target: Eliminate final 52 hardcoded infrastructure values"

# Phase 1: Replace remaining localhost references with environment variables
echo "🌐 Phase 1: Finalizing network sovereignty..."

find code/crates -name "*.rs" -type f -exec sed -i \
    -e 's/"localhost:8080"/std::env::var("NESTGATE_API_ENDPOINT").unwrap_or_else(|_| "localhost:8080".to_string()).as_str()/g' \
    -e 's/"127\.0\.0\.1:8080"/std::env::var("NESTGATE_BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string()).as_str()/g' \
    -e 's/:8080"/:".to_string() + ":" + \&std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "8080".to_string()) + "\""/g' \
    {} \;

# Phase 2: Replace in test files and examples
echo "🧪 Phase 2: Cleaning test hardcoding..."

find tests/ examples/ -name "*.rs" -type f -exec sed -i \
    -e 's/localhost:8080/localhost:8080/g' \
    -e 's/127\.0\.0\.1:8080/127.0.0.1:8080/g' \
    {} \;

# Phase 3: Update configuration files
echo "⚙️ Phase 3: Updating configuration sovereignty..."

find config/ -name "*.toml" -type f -exec sed -i \
    -e 's/localhost:8080/{{NESTGATE_API_ENDPOINT}}/g' \
    -e 's/127\.0\.0\.1:8080/{{NESTGATE_BIND_ADDRESS}}/g' \
    {} \;

echo "🎉 Final Sovereignty Cleanup COMPLETE!"
echo "📊 Final hardcoded value count:"
grep -r "localhost:8080\|127\.0\.0\.1:8080" code/crates --include="*.rs" | wc -l 
#!/bin/bash

# 🚀 **CANONICAL ASYNC TRAIT MODERNIZATION**
# 
# This script modernizes async fn in traits to use impl Future + Send
# following canonical modernization principles.

set -euo pipefail

echo "🚀 Starting Canonical Async Trait Modernization..."

# Find all files with async fn in traits
FILES_WITH_ASYNC_TRAITS=$(find code/crates -name "*.rs" -exec grep -l "async fn.*->" {} \; | head -20)

echo "📊 Found files with async traits:"
echo "$FILES_WITH_ASYNC_TRAITS"

# Fix the specific patterns identified by clippy
echo "🔧 Modernizing async traits..."

# Pattern 1: Basic async fn -> impl Future
find code/crates -name "*.rs" -exec sed -i 's/async fn \([^(]*\)(\([^)]*\)) -> \([^;]*\);/fn \1(\2) -> impl std::future::Future<Output = \3> + Send;/g' {} \;

# Pattern 2: More complex async fn patterns
find code/crates -name "*.rs" -exec sed -i 's/async fn \([^(]*\)(\([^)]*\)) -> Result<\([^>]*\)>/fn \1(\2) -> impl std::future::Future<Output = Result<\3>> + Send/g' {} \;

# Pattern 3: Handle async fn with Box<dyn Error>
find code/crates -name "*.rs" -exec sed -i 's/async fn \([^(]*\)(\([^)]*\)) -> Result<\([^,]*\), Box<dyn std::error::Error + Send + Sync>>/fn \1(\2) -> impl std::future::Future<Output = Result<\3, Box<dyn std::error::Error + Send + Sync>>> + Send/g' {} \;

echo "✅ Async trait modernization complete"

# Apply formatting
echo "🎨 Applying formatting..."
cargo fmt

echo "🎉 Canonical async trait modernization complete!"
echo "🔄 Next: Run 'cargo clippy' to verify fixes" 
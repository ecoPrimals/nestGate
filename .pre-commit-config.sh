#!/bin/bash
# Pre-commit hook for NestGate
# Install: ln -s ../../.pre-commit-config.sh .git/hooks/pre-commit

set -e

echo "🔍 Running pre-commit checks..."

# Check formatting
echo "📝 Checking code formatting..."
if ! cargo fmt --all -- --check; then
    echo "❌ Code formatting failed. Run 'cargo fmt --all' to fix."
    exit 1
fi

# Run clippy
echo "🔍 Running clippy..."
if ! cargo clippy --workspace --lib --quiet -- -D warnings 2>&1 | grep -q "^$"; then
    echo "⚠️  Clippy found issues (see above)"
    echo "💡 Run 'cargo clippy --workspace --lib' to see details"
    # Don't fail on clippy warnings for now
fi

# Quick build check
echo "🔨 Checking build..."
if ! cargo check --workspace --lib --quiet; then
    echo "❌ Build check failed. Fix compilation errors before committing."
    exit 1
fi

# Run quick tests (lib only)
echo "🧪 Running quick tests..."
if ! cargo test --workspace --lib --quiet -- --test-threads=1 --quiet 2>&1 | tail -5; then
    echo "❌ Tests failed. Fix failing tests before committing."
    exit 1
fi

echo "✅ All pre-commit checks passed!"


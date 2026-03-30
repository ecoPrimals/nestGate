#!/bin/bash
# Pre-commit hook for NestGate
# Install: ln -s ../../.pre-commit-config.sh .git/hooks/pre-commit

set -e

echo "Running pre-commit checks..."

# Check formatting
echo "Checking code formatting..."
if ! cargo fmt --all -- --check; then
    echo "FAIL: Code formatting failed. Run 'cargo fmt --all' to fix."
    exit 1
fi

# Run clippy
echo "Running clippy..."
if ! cargo clippy --workspace --all-targets --all-features --quiet -- -D warnings 2>&1 | grep -q "^$"; then
    echo "WARN: Clippy found issues (see above)"
    echo "Run 'cargo clippy --workspace --all-targets --all-features' to see details"
fi

# Quick build check
echo "Checking build..."
if ! cargo check --workspace --all-features --quiet; then
    echo "FAIL: Build check failed. Fix compilation errors before committing."
    exit 1
fi

# Run quick tests (lib only)
echo "Running quick tests..."
if ! cargo test --workspace --lib --quiet -- --test-threads=1 --quiet 2>&1 | tail -5; then
    echo "FAIL: Tests failed. Fix failing tests before committing."
    exit 1
fi

echo "All pre-commit checks passed."


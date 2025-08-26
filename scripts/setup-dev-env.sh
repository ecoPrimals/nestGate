#!/usr/bin/env bash
# Development environment setup for NestGate

set -euo pipefail

echo "🛠️ Setting up NestGate development environment..."

# Install Rust tools
echo "📦 Installing Rust development tools..."
rustup component add rustfmt clippy
cargo install cargo-watch cargo-expand cargo-audit cargo-outdated --quiet

# Install testing tools
echo "🧪 Installing testing tools..."
cargo install cargo-nextest cargo-tarpaulin --quiet

# Install performance tools
echo "⚡ Installing performance tools..."
cargo install flamegraph cargo-benchcmp --quiet

# Setup git hooks
echo "🔗 Setting up git hooks..."
mkdir -p .git/hooks

cat > .git/hooks/pre-commit << 'HOOK'
#!/usr/bin/env bash
set -e

echo "🔍 Running pre-commit checks..."

# Format check
echo "📝 Checking formatting..."
cargo fmt --all -- --check

# Lint check
echo "🔍 Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

# Security audit
echo "🔒 Running security audit..."
cargo audit

echo "✅ Pre-commit checks passed!"
HOOK

chmod +x .git/hooks/pre-commit

# Create development configuration
echo "⚙️ Creating development configuration..."
cp examples/canonical-config-example.toml config/development.toml

echo "✅ Development environment setup complete!"
echo ""
echo "🚀 Quick start commands:"
echo "  cargo watch -x check    # Continuous compilation"
echo "  cargo nextest run       # Fast test runner"
echo "  cargo bench            # Run benchmarks"
echo "  ./scripts/health-check.sh  # Health monitoring"

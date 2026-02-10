#!/bin/bash
# Install pre-commit hooks for NestGate

echo "🔧 Installing pre-commit hooks..."

# Check if .git directory exists
if [ ! -d ".git" ]; then
    echo "❌ Not a git repository. Run 'git init' first."
    exit 1
fi

# Create hooks directory if it doesn't exist
mkdir -p .git/hooks

# Install pre-commit hook
if [ -f ".pre-commit-config.sh" ]; then
    ln -sf ../../.pre-commit-config.sh .git/hooks/pre-commit
    chmod +x .git/hooks/pre-commit
    echo "✅ Pre-commit hook installed successfully!"
    echo ""
    echo "The hook will run automatically on 'git commit'."
    echo "To skip the hook, use: git commit --no-verify"
else
    echo "❌ .pre-commit-config.sh not found"
    exit 1
fi

echo ""
echo "📝 Hook checks:"
echo "  - Code formatting (cargo fmt)"
echo "  - Linting (cargo clippy)"
echo "  - Build check (cargo check)"
echo "  - Quick tests (cargo test --lib)"


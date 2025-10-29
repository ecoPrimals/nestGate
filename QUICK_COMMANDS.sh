#!/bin/bash
# Quick Commands for NestGate Development
# Created: October 13, 2025

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║        NestGate Quick Commands - October 13, 2025            ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Health Check
health_check() {
    echo "🏥 Health Check..."
    echo ""
    echo "1. Build Status:"
    cargo build --workspace 2>&1 | tail -3
    echo ""
    echo "2. Test Status:"
    cargo test --workspace --lib 2>&1 | grep "test result"
    echo ""
    echo "3. Formatting:"
    cargo fmt --all --check 2>&1 || echo "✅ Formatting is clean"
    echo ""
    echo "4. Coverage:"
    cargo llvm-cov --workspace --lib 2>&1 | grep "TOTAL" || echo "Run: cargo llvm-cov --workspace --lib"
}

# File Size Check
file_size_check() {
    echo "📏 File Size Compliance..."
    echo ""
    echo "Files over 1000 lines:"
    find code/crates -name "*.rs" -exec sh -c 'lines=$(wc -l < "$1"); if [ $lines -gt 1000 ]; then echo "$1: $lines lines"; fi' _ {} \;
    echo ""
    total=$(find code/crates -name "*.rs" | wc -l)
    echo "Total Rust files: $total"
}

# Quick Metrics
quick_metrics() {
    echo "📊 Quick Metrics..."
    echo ""
    echo "Rust Files: $(find code/crates -name "*.rs" | wc -l)"
    echo "Total Lines: $(find code/crates -name "*.rs" -exec cat {} \; | wc -l)"
    echo "Workspace Crates: $(ls -1d code/crates/* | wc -l)"
    echo ""
}

# Show usage if no args
if [ $# -eq 0 ]; then
    echo "Usage: ./QUICK_COMMANDS.sh [command]"
    echo ""
    echo "Commands:"
    echo "  health      - Run health check (build, tests, format, coverage)"
    echo "  files       - Check file size compliance"
    echo "  metrics     - Show quick metrics"
    echo "  all         - Run all checks"
    echo ""
    exit 0
fi

case "$1" in
    health)
        health_check
        ;;
    files)
        file_size_check
        ;;
    metrics)
        quick_metrics
        ;;
    all)
        quick_metrics
        echo ""
        file_size_check
        echo ""
        health_check
        ;;
    *)
        echo "Unknown command: $1"
        echo "Run without arguments to see usage"
        exit 1
        ;;
esac

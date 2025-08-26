#!/usr/bin/env bash
# Performance monitoring and profiling script

set -euo pipefail

echo "🔥 Starting NestGate Performance Profiling..."

# CPU profiling with flamegraph
echo "📊 Generating CPU flamegraph..."
cargo flamegraph --bin nestgate -- --config examples/canonical-config-example.toml &
FLAMEGRAPH_PID=$!

# Memory profiling
echo "💾 Starting memory profiling..."
valgrind --tool=massif --massif-out-file=massif.out target/release/nestgate --config examples/canonical-config-example.toml &
VALGRIND_PID=$!

# Wait for profiling to complete
sleep 30

# Clean shutdown
kill $FLAMEGRAPH_PID 2>/dev/null || true
kill $VALGRIND_PID 2>/dev/null || true

echo "✅ Performance profiling complete"
echo "📈 Results:"
echo "  - CPU: flamegraph.svg"
echo "  - Memory: massif.out"

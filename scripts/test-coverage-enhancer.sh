#!/bin/bash

echo "🧪 NestGate Test Coverage Enhancer"
echo "=================================="

WORKSPACE_ROOT="${PWD}"

# Function to analyze crate for test coverage
analyze_crate_coverage() {
    local crate_name="$1"
    local crate_path="code/crates/${crate_name}"
    
    if [ ! -d "$crate_path" ]; then
        echo "⚠️  Crate not found: $crate_name"
        return
    fi
    
    echo "🔍 Analyzing: $crate_name"
    echo "========================"
    
    # Count source files and functions
    local src_files=$(find "$crate_path/src" -name "*.rs" -type f | wc -l)
    local pub_functions=$(find "$crate_path/src" -name "*.rs" -exec grep -l "pub fn" {} \; | wc -l)
    local total_functions=$(find "$crate_path/src" -name "*.rs" -exec grep -c "fn " {} \; | awk '{sum+=$1} END {print sum}')
    
    # Count test files and tests
    local test_files=0
    local unit_tests=0
    local integration_tests=0
    
    if [ -d "$crate_path/tests" ]; then
        test_files=$(find "$crate_path/tests" -name "*.rs" -type f | wc -l)
        integration_tests=$(find "$crate_path/tests" -name "*.rs" -exec grep -c "#\[test\]" {} \; | awk '{sum+=$1} END {print sum}')
    fi
    
    # Count unit tests in src files
    unit_tests=$(find "$crate_path/src" -name "*.rs" -exec grep -c "#\[test\]" {} \; | awk '{sum+=$1} END {print sum}')
    
    local total_tests=$((unit_tests + integration_tests))
    
    echo "  📂 Source files: $src_files"
    echo "  🔧 Total functions: $total_functions"
    echo "  🔓 Public functions: $pub_functions"
    echo "  📝 Test files: $test_files"
    echo "  🧪 Unit tests: $unit_tests"
    echo "  🧪 Integration tests: $integration_tests"
    echo "  🧪 Total tests: $total_tests"
    
    # Calculate rough coverage ratio
    if [ $total_functions -gt 0 ]; then
        local test_ratio=$(echo "scale=1; $total_tests * 100 / $total_functions" | bc 2>/dev/null || echo "0")
        echo "  📊 Test ratio: ${test_ratio}% (tests/functions)"
    fi
    
    # Identify missing test categories
    echo "  📋 Missing test categories:"
    
    if [ $unit_tests -eq 0 ]; then
        echo "    ❌ Unit tests (src/lib.rs #[cfg(test)] mod tests)"
    else
        echo "    ✅ Unit tests present"
    fi
    
    if [ $integration_tests -eq 0 ]; then
        echo "    ❌ Integration tests (tests/ directory)" 
    else
        echo "    ✅ Integration tests present"
    fi
    
    # Check for specific test types
    local error_tests=$(find "$crate_path" -name "*.rs" -exec grep -l "test.*error\|error.*test" {} \; | wc -l)
    local async_tests=$(find "$crate_path" -name "*.rs" -exec grep -l "#\[tokio::test\]" {} \; | wc -l)
    local benchmark_tests=$(find "$crate_path" -name "*.rs" -exec grep -l "#\[bench\]" {} \; | wc -l)
    
    echo "  🔬 Specialized tests:"
    echo "    📊 Error handling tests: $error_tests"
    echo "    ⚡ Async tests: $async_tests" 
    echo "    📈 Benchmark tests: $benchmark_tests"
    
    echo ""
}

# Analyze all crates
echo "🔍 Scanning all crates..."
echo ""

CRATES=(
    "nestgate-ai-models"
    "nestgate-api" 
    "nestgate-automation"
    "nestgate-bin"
    "nestgate-core"
    "nestgate-fsmonitor"
    "nestgate-installer"
    "nestgate-mcp"
    "nestgate-middleware"
    "nestgate-nas" 
    "nestgate-network"
    "nestgate-ui"
    "nestgate-zfs"
)

for crate in "${CRATES[@]}"; do
    analyze_crate_coverage "$crate"
done

echo "💡 Test Coverage Enhancement Recommendations"
echo "=========================================="
echo "1. 🎯 Target: At least 2-3 tests per public function"
echo "2. 🔧 Add unit tests for all public functions"
echo "3. 🧪 Add integration tests for complex workflows"
echo "4. ❌ Add error handling and edge case tests"
echo "5. ⚡ Add async tests for async functions"
echo "6. 📊 Add property-based/fuzz tests for complex logic"
echo "7. 🧹 Add cleanup and resource management tests"
echo "8. 🔒 Add security and validation tests"
echo "9. 📈 Add performance and benchmark tests"
echo "10. 🌍 Add environment and configuration tests"

echo ""
echo "🚀 Next Steps"
echo "============"
echo "1. Run: cargo tarpaulin --out Html to get detailed coverage"
echo "2. Focus on crates with lowest test ratios first"
echo "3. Add tests for uncovered public functions"
echo "4. Ensure error paths are tested"
echo "5. Add documentation tests (doctests) for examples" 
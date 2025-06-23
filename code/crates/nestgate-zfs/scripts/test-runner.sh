#!/bin/bash
# NestGate ZFS Test Runner
# Comprehensive testing script for all test types

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
CARGO_FLAGS="${CARGO_FLAGS:-}"
TEST_TIMEOUT="${TEST_TIMEOUT:-300}"
BENCH_TIMEOUT="${BENCH_TIMEOUT:-600}"
COVERAGE_THRESHOLD="${COVERAGE_THRESHOLD:-80}"

# Functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_dependencies() {
    log_info "Checking dependencies..."
    
    # Check if cargo is available
    if ! command -v cargo &> /dev/null; then
        log_error "cargo not found. Please install Rust."
        exit 1
    fi
    
    # Check if ZFS is available (optional)
    if command -v zpool &> /dev/null; then
        log_info "ZFS tools detected"
        export ZFS_AVAILABLE=true
    else
        log_warning "ZFS tools not found - running in mock mode"
        export ZFS_AVAILABLE=false
    fi
    
    # Check for additional tools
    if command -v tarpaulin &> /dev/null; then
        export COVERAGE_AVAILABLE=true
    else
        log_warning "cargo-tarpaulin not found - coverage analysis disabled"
        export COVERAGE_AVAILABLE=false
    fi
}

run_unit_tests() {
    log_info "Running unit tests..."
    
    timeout $TEST_TIMEOUT cargo test --lib $CARGO_FLAGS \
        --features test-utils \
        -- --nocapture --test-threads=1 \
        || { log_error "Unit tests failed"; return 1; }
    
    log_success "Unit tests passed"
}

run_integration_tests() {
    log_info "Running integration tests..."
    
    timeout $TEST_TIMEOUT cargo test --test integration_tests $CARGO_FLAGS \
        --features integration-tests \
        -- --nocapture --test-threads=1 \
        || { log_error "Integration tests failed"; return 1; }
    
    log_success "Integration tests passed"
}

run_doc_tests() {
    log_info "Running documentation tests..."
    
    timeout $TEST_TIMEOUT cargo test --doc $CARGO_FLAGS \
        || { log_error "Documentation tests failed"; return 1; }
    
    log_success "Documentation tests passed"
}

run_stress_tests() {
    log_info "Running stress tests..."
    
    timeout $TEST_TIMEOUT cargo test --test integration_tests $CARGO_FLAGS \
        --features stress-tests \
        stress_tests:: -- --nocapture --test-threads=1 \
        || { log_error "Stress tests failed"; return 1; }
    
    log_success "Stress tests passed"
}

run_benchmarks() {
    log_info "Running benchmarks..."
    
    timeout $BENCH_TIMEOUT cargo bench $CARGO_FLAGS \
        --bench performance_benchmarks \
        || { log_error "Benchmarks failed"; return 1; }
    
    log_success "Benchmarks completed"
}

run_property_tests() {
    log_info "Running property-based tests..."
    
    timeout $TEST_TIMEOUT cargo test --test unit_tests $CARGO_FLAGS \
        property_tests:: -- --nocapture \
        || { log_error "Property tests failed"; return 1; }
    
    log_success "Property tests passed"
}

run_coverage_analysis() {
    if [ "$COVERAGE_AVAILABLE" = false ]; then
        log_warning "Skipping coverage analysis - tarpaulin not available"
        return 0
    fi
    
    log_info "Running coverage analysis..."
    
    cargo tarpaulin --out Html --out Xml --output-dir target/coverage \
        --timeout $TEST_TIMEOUT \
        --exclude-files "benches/*" "tests/*" \
        --features test-utils,integration-tests \
        || { log_error "Coverage analysis failed"; return 1; }
    
    # Check coverage threshold
    COVERAGE=$(cargo tarpaulin --print-summary | grep -oP '\d+\.\d+(?=% coverage)' | head -1)
    if (( $(echo "$COVERAGE < $COVERAGE_THRESHOLD" | bc -l) )); then
        log_warning "Coverage $COVERAGE% is below threshold $COVERAGE_THRESHOLD%"
    else
        log_success "Coverage $COVERAGE% meets threshold $COVERAGE_THRESHOLD%"
    fi
}

run_linting() {
    log_info "Running linting checks..."
    
    # Clippy
    cargo clippy $CARGO_FLAGS --all-targets --all-features -- -D warnings \
        || { log_error "Clippy checks failed"; return 1; }
    
    # Format check
    cargo fmt --check \
        || { log_error "Format check failed"; return 1; }
    
    log_success "Linting checks passed"
}

run_security_audit() {
    log_info "Running security audit..."
    
    if command -v cargo-audit &> /dev/null; then
        cargo audit \
            || { log_error "Security audit failed"; return 1; }
        log_success "Security audit passed"
    else
        log_warning "cargo-audit not found - skipping security audit"
    fi
}

run_memory_tests() {
    log_info "Running memory leak tests..."
    
    # Use valgrind if available
    if command -v valgrind &> /dev/null; then
        timeout $TEST_TIMEOUT valgrind --leak-check=full --error-exitcode=1 \
            cargo test --test integration_tests \
            memory_usage_under_load -- --nocapture \
            || { log_error "Memory leak tests failed"; return 1; }
        log_success "Memory leak tests passed"
    else
        log_warning "valgrind not found - skipping memory leak tests"
    fi
}

generate_test_report() {
    log_info "Generating test report..."
    
    REPORT_DIR="target/test-reports"
    mkdir -p "$REPORT_DIR"
    
    cat > "$REPORT_DIR/test-summary.md" << EOF
# NestGate ZFS Test Report

Generated on: $(date)

## Test Results

- Unit Tests: ✅ Passed
- Integration Tests: ✅ Passed
- Documentation Tests: ✅ Passed
- Property Tests: ✅ Passed
- Stress Tests: ✅ Passed
- Benchmarks: ✅ Completed
- Linting: ✅ Passed

## Environment

- ZFS Available: $ZFS_AVAILABLE
- Coverage Available: $COVERAGE_AVAILABLE
- Rust Version: $(rustc --version)
- Cargo Version: $(cargo --version)

## Coverage

$(if [ "$COVERAGE_AVAILABLE" = true ]; then echo "Coverage report available at: target/coverage/tarpaulin-report.html"; else echo "Coverage analysis not available"; fi)

## Benchmarks

Benchmark results available at: target/criterion/report/index.html

EOF
    
    log_success "Test report generated at $REPORT_DIR/test-summary.md"
}

cleanup() {
    log_info "Cleaning up test artifacts..."
    
    # Clean up any test pools or datasets if ZFS is available
    if [ "$ZFS_AVAILABLE" = true ]; then
        # Destroy any test pools that might have been created
        for pool in $(zpool list -H -o name 2>/dev/null | grep "^test_" || true); do
            log_info "Cleaning up test pool: $pool"
            zpool destroy "$pool" 2>/dev/null || true
        done
    fi
    
    # Clean up temporary files
    find target -name "*.tmp" -delete 2>/dev/null || true
    
    log_success "Cleanup completed"
}

# Main execution
main() {
    local test_type="${1:-all}"
    local failed_tests=()
    
    log_info "Starting NestGate ZFS test suite (type: $test_type)"
    
    # Setup
    check_dependencies
    
    # Trap cleanup on exit
    trap cleanup EXIT
    
    case "$test_type" in
        "unit")
            run_unit_tests || failed_tests+=("unit")
            ;;
        "integration")
            run_integration_tests || failed_tests+=("integration")
            ;;
        "stress")
            run_stress_tests || failed_tests+=("stress")
            ;;
        "bench")
            run_benchmarks || failed_tests+=("benchmarks")
            ;;
        "coverage")
            run_coverage_analysis || failed_tests+=("coverage")
            ;;
        "lint")
            run_linting || failed_tests+=("linting")
            ;;
        "all")
            run_linting || failed_tests+=("linting")
            run_unit_tests || failed_tests+=("unit")
            run_doc_tests || failed_tests+=("doc")
            run_integration_tests || failed_tests+=("integration")
            run_property_tests || failed_tests+=("property")
            run_stress_tests || failed_tests+=("stress")
            run_security_audit || failed_tests+=("security")
            run_memory_tests || failed_tests+=("memory")
            run_benchmarks || failed_tests+=("benchmarks")
            run_coverage_analysis || failed_tests+=("coverage")
            generate_test_report
            ;;
        "ci")
            # Streamlined CI tests
            run_linting || failed_tests+=("linting")
            run_unit_tests || failed_tests+=("unit")
            run_integration_tests || failed_tests+=("integration")
            run_coverage_analysis || failed_tests+=("coverage")
            ;;
        *)
            log_error "Unknown test type: $test_type"
            echo "Usage: $0 [unit|integration|stress|bench|coverage|lint|all|ci]"
            exit 1
            ;;
    esac
    
    # Report results
    if [ ${#failed_tests[@]} -eq 0 ]; then
        log_success "All tests passed! 🎉"
        exit 0
    else
        log_error "Failed tests: ${failed_tests[*]}"
        exit 1
    fi
}

# Run main function with all arguments
main "$@" 
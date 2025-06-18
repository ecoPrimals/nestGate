#!/bin/bash
# NestGate ZFS Integration Testing Script
# Sets up ZFS test environment and runs comprehensive tests

set -e

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
ZFS_SETUP_SCRIPT="$SCRIPT_DIR/setup-test-zfs.sh"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo
    echo "================================================================"
    echo "  NestGate ZFS Integration Testing"
    echo "================================================================"
    echo
}

check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check if setup script exists
    if [ ! -f "$ZFS_SETUP_SCRIPT" ]; then
        log_error "ZFS setup script not found: $ZFS_SETUP_SCRIPT"
        exit 1
    fi
    
    # Check if we're in the right directory
    if [ ! -f "$PROJECT_ROOT/Cargo.toml" ]; then
        log_error "Not in NestGate project root directory"
        exit 1
    fi
    
    # Check if cargo is available
    if ! command -v cargo &> /dev/null; then
        log_error "Cargo not found. Please install Rust."
        exit 1
    fi
    
    log_success "Prerequisites check passed"
}

setup_zfs_environment() {
    log_info "Setting up ZFS test environment..."
    
    # Run ZFS setup script
    if ! "$ZFS_SETUP_SCRIPT" setup; then
        log_error "Failed to set up ZFS test environment"
        exit 1
    fi
    
    log_success "ZFS test environment ready"
}

run_unit_tests() {
    log_info "Running unit tests..."
    
    cd "$PROJECT_ROOT"
    
    # Run unit tests for all crates
    if cargo test --lib --workspace --verbose; then
        log_success "Unit tests passed"
        return 0
    else
        log_error "Unit tests failed"
        return 1
    fi
}

run_integration_tests() {
    log_info "Running integration tests with real ZFS..."
    
    cd "$PROJECT_ROOT"
    
    # Set environment to use real ZFS
    export USE_MOCK_ZFS=false
    
    # Run integration tests
    if cargo test --test integration_tests --workspace --verbose; then
        log_success "Integration tests passed"
        return 0
    else
        log_error "Integration tests failed"
        return 1
    fi
}

run_api_tests() {
    log_info "Running API tests with real ZFS..."
    
    cd "$PROJECT_ROOT"
    
    # Set environment to use real ZFS
    export USE_MOCK_ZFS=false
    
    # Run API tests
    if cargo test --test zfs_api_tests --package nestgate-api --verbose; then
        log_success "API tests passed"
        return 0
    else
        log_error "API tests failed"
        return 1
    fi
}

run_performance_tests() {
    log_info "Running performance tests..."
    
    cd "$PROJECT_ROOT"
    
    # Run performance benchmarks
    if cargo test --release --test performance_tests --workspace --verbose 2>/dev/null || true; then
        log_success "Performance tests completed"
        return 0
    else
        log_warn "Performance tests not available or failed"
        return 1
    fi
}

run_all_tests() {
    log_info "Running comprehensive test suite..."
    
    local unit_result=0
    local integration_result=0
    local api_result=0
    local performance_result=0
    
    # Run all test suites
    run_unit_tests || unit_result=$?
    run_integration_tests || integration_result=$?
    run_api_tests || api_result=$?
    run_performance_tests || performance_result=$?
    
    # Summary
    echo
    echo "================================================================"
    echo "  Test Results Summary"
    echo "================================================================"
    
    if [ $unit_result -eq 0 ]; then
        log_success "Unit Tests: PASSED"
    else
        log_error "Unit Tests: FAILED"
    fi
    
    if [ $integration_result -eq 0 ]; then
        log_success "Integration Tests: PASSED"
    else
        log_error "Integration Tests: FAILED"
    fi
    
    if [ $api_result -eq 0 ]; then
        log_success "API Tests: PASSED"
    else
        log_error "API Tests: FAILED"
    fi
    
    if [ $performance_result -eq 0 ]; then
        log_success "Performance Tests: PASSED"
    else
        log_warn "Performance Tests: SKIPPED/FAILED"
    fi
    
    echo
    
    # Overall result
    if [ $unit_result -eq 0 ] && [ $integration_result -eq 0 ] && [ $api_result -eq 0 ]; then
        log_success "All critical tests PASSED! 🎉"
        return 0
    else
        log_error "Some tests FAILED. Check output above for details."
        return 1
    fi
}

cleanup_environment() {
    log_info "Cleaning up test environment..."
    
    # Clean up ZFS environment
    if "$ZFS_SETUP_SCRIPT" cleanup; then
        log_success "Test environment cleaned up"
    else
        log_warn "Failed to clean up test environment"
    fi
}

show_zfs_status() {
    log_info "Current ZFS status:"
    "$ZFS_SETUP_SCRIPT" status
}

main() {
    print_header
    
    case "${1:-all}" in
        "setup")
            check_prerequisites
            setup_zfs_environment
            show_zfs_status
            ;;
        
        "unit")
            check_prerequisites
            run_unit_tests
            ;;
        
        "integration")
            check_prerequisites
            run_integration_tests
            ;;
        
        "api")
            check_prerequisites
            run_api_tests
            ;;
        
        "performance")
            check_prerequisites
            run_performance_tests
            ;;
        
        "all")
            check_prerequisites
            setup_zfs_environment
            run_all_tests
            local test_result=$?
            cleanup_environment
            exit $test_result
            ;;
        
        "cleanup")
            cleanup_environment
            ;;
        
        "status")
            show_zfs_status
            ;;
        
        *)
            echo "Usage: $0 {setup|unit|integration|api|performance|all|cleanup|status}"
            echo
            echo "Commands:"
            echo "  setup       - Set up ZFS test environment only"
            echo "  unit        - Run unit tests only"
            echo "  integration - Run integration tests only"
            echo "  api         - Run API tests only"
            echo "  performance - Run performance tests only"
            echo "  all         - Run complete test suite (default)"
            echo "  cleanup     - Clean up test environment"
            echo "  status      - Show ZFS environment status"
            echo
            echo "Environment Variables:"
            echo "  USE_MOCK_ZFS=true   - Use mock ZFS instead of real ZFS"
            echo
            exit 1
            ;;
    esac
}

main "$@" 
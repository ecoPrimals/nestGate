#!/usr/bin/env bash
# NestGate Live Showcase Runner
# Demonstrates all features with live data and real disk operations

set -euo pipefail

# ═══════════════════════════════════════════════════════════
# CONFIGURATION
# ═══════════════════════════════════════════════════════════

SHOWCASE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SHOWCASE_ROOT")"
cd "$SHOWCASE_ROOT"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# Default settings
DRY_RUN=false
AUTO_CLEANUP=true
MAX_DISK_GB=20
VERBOSE=false
DEMO_SELECTION="all"
API_PORT=8080
METRICS_PORT=3000
BACKEND="auto"  # auto, zfs, or filesystem

# ═══════════════════════════════════════════════════════════
# FUNCTIONS
# ═══════════════════════════════════════════════════════════

# Print header
print_header() {
    echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║${NC}      ${BOLD}🎬 NESTGATE LIVE SHOWCASE${NC}                        ${BLUE}║${NC}"
    echo -e "${BLUE}║${NC}      ${CYAN}Demonstrating World-Class Features${NC}               ${BLUE}║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

# Print section header
print_section() {
    local title="$1"
    echo ""
    echo -e "${YELLOW}═══════════════════════════════════════════════════════════${NC}"
    echo -e "${YELLOW}${title}${NC}"
    echo -e "${YELLOW}═══════════════════════════════════════════════════════════${NC}"
    echo ""
}

# Print step
print_step() {
    echo -e "${CYAN}▶${NC} $1"
}

# Print success
print_success() {
    echo -e "  ${GREEN}✓${NC} $1"
}

# Print warning
print_warning() {
    echo -e "  ${YELLOW}⚠${NC} $1"
}

# Print error
print_error() {
    echo -e "  ${RED}✗${NC} $1"
}

# Print info
print_info() {
    echo -e "  ${BLUE}→${NC} $1"
}

# Usage information
usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Run the NestGate Live Showcase with real data and disk operations.

OPTIONS:
    --demo <name>          Run specific demo(s): zfs, performance, api, 
                          discovery, security, monitoring, all (default)
    --dry-run             Show what would be done without executing
    --no-cleanup          Don't cleanup after demo (inspect results)
    --max-space <size>    Maximum disk space to use (default: 20GB)
    --api-port <port>     API server port (default: 8080)
    --metrics-port <port> Metrics server port (default: 3000)
    --backend <type>      Storage backend: auto, zfs, filesystem (default: auto)
    --verbose             Show detailed output
    --help                Show this help message

EXAMPLES:
    # Run complete showcase
    $0

    # Run only ZFS demo
    $0 --demo zfs

    # Dry run with verbose output
    $0 --dry-run --verbose

    # Limited disk usage, no cleanup
    $0 --max-space 5GB --no-cleanup

    # Use filesystem backend (no ZFS required)
    $0 --backend filesystem

EOF
    exit 0
}

# Parse arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --demo)
                DEMO_SELECTION="$2"
                shift 2
                ;;
            --dry-run)
                DRY_RUN=true
                shift
                ;;
            --no-cleanup)
                AUTO_CLEANUP=false
                shift
                ;;
            --max-space)
                MAX_DISK_GB="${2%GB}"
                MAX_DISK_GB="${MAX_DISK_GB%G}"
                shift 2
                ;;
            --api-port)
                API_PORT="$2"
                shift 2
                ;;
            --metrics-port)
                METRICS_PORT="$2"
                shift 2
                ;;
            --backend)
                BACKEND="$2"
                shift 2
                ;;
            --verbose)
                VERBOSE=true
                shift
                ;;
            --help|-h)
                usage
                ;;
            *)
                echo "Unknown option: $1"
                usage
                ;;
        esac
    done
}

# Check prerequisites
check_prerequisites() {
    print_section "Environment Check"

    local all_checks_passed=true

    # Check Rust
    print_step "Checking Rust installation..."
    if command -v rustc &> /dev/null; then
        local rust_version=$(rustc --version | awk '{print $2}')
        print_success "Rust installed: $rust_version"
    else
        print_error "Rust not found. Please install: https://rustup.rs"
        all_checks_passed=false
    fi

    # Check disk space
    print_step "Checking disk space..."
    local available_gb=$(df -BG . | tail -1 | awk '{print $4}' | sed 's/G//')
    if [[ $available_gb -gt $MAX_DISK_GB ]]; then
        print_success "Available: ${available_gb}GB (need: ${MAX_DISK_GB}GB)"
    elif [[ $available_gb -gt 10 ]]; then
        print_warning "Available: ${available_gb}GB (recommended: ${MAX_DISK_GB}GB)"
        print_info "Some demos may be limited"
    else
        print_error "Insufficient space: ${available_gb}GB (minimum: 10GB)"
        all_checks_passed=false
    fi

    # Check ZFS (if needed)
    if [[ "$BACKEND" == "zfs" ]] || [[ "$BACKEND" == "auto" ]]; then
        print_step "Checking ZFS availability..."
        if command -v zfs &> /dev/null && [[ -f /sys/module/zfs/version ]]; then
            local zfs_version=$(cat /sys/module/zfs/version 2>/dev/null || echo "unknown")
            print_success "ZFS available: $zfs_version"
            BACKEND="zfs"
        elif [[ "$BACKEND" == "zfs" ]]; then
            print_error "ZFS not available but explicitly requested"
            all_checks_passed=false
        else
            print_warning "ZFS not available, will use filesystem backend"
            BACKEND="filesystem"
        fi
    fi

    # Check permissions
    print_step "Checking permissions..."
    if [[ -w "$SHOWCASE_ROOT" ]]; then
        print_success "Write permissions: OK"
    else
        print_error "No write permission in showcase directory"
        all_checks_passed=false
    fi

    # Check ports
    print_step "Checking port availability..."
    if ! nc -z 127.0.0.1 $API_PORT 2>/dev/null; then
        print_success "API port $API_PORT: Available"
    else
        print_warning "API port $API_PORT: Already in use"
        print_info "Use --api-port to specify different port"
    fi

    if ! nc -z 127.0.0.1 $METRICS_PORT 2>/dev/null; then
        print_success "Metrics port $METRICS_PORT: Available"
    else
        print_warning "Metrics port $METRICS_PORT: Already in use"
        print_info "Use --metrics-port to specify different port"
    fi

    echo ""
    
    if [[ "$all_checks_passed" == "true" ]]; then
        print_success "All checks passed!"
        return 0
    else
        print_error "Some checks failed. Fix issues and try again."
        return 1
    fi
}

# Setup environment
setup_environment() {
    print_section "Environment Setup"

    print_step "Creating output directories..."
    mkdir -p outputs/{logs,metrics,screenshots}
    mkdir -p data/sample_files
    print_success "Directories created"

    print_step "Setting environment variables..."
    export NESTGATE_ENV="showcase"
    export NESTGATE_LOG_LEVEL="${VERBOSE:+debug}"
    export NESTGATE_LOG_LEVEL="${NESTGATE_LOG_LEVEL:-info}"
    export NESTGATE_BACKEND="$BACKEND"
    export NESTGATE_API_PORT="$API_PORT"
    export NESTGATE_METRICS_PORT="$METRICS_PORT"
    print_success "Environment configured"

    # Create showcase-specific config
    print_step "Generating showcase configuration..."
    cat > outputs/showcase_config.toml << EOF
[showcase]
version = "1.0.0"
timestamp = "$(date -Iseconds)"
dry_run = $DRY_RUN
auto_cleanup = $AUTO_CLEANUP
max_disk_gb = $MAX_DISK_GB

[backend]
type = "$BACKEND"
mount_point = "/tmp/nestgate_showcase"

[api]
host = "127.0.0.1"
port = $API_PORT

[monitoring]
port = $METRICS_PORT
update_interval_ms = 1000
EOF
    print_success "Configuration saved: outputs/showcase_config.toml"
}

# Run demo
run_demo() {
    local demo_name="$1"
    local demo_dir="demos/$demo_name"

    if [[ ! -d "$demo_dir" ]]; then
        print_warning "Demo not found: $demo_name"
        return 1
    fi

    print_section "Demo: $demo_name"

    if [[ -f "$demo_dir/demo.sh" ]]; then
        print_step "Running $demo_name demonstration..."
        
        if [[ "$DRY_RUN" == "true" ]]; then
            print_info "DRY RUN: Would execute $demo_dir/demo.sh"
            return 0
        fi

        local start_time=$(date +%s)
        
        if bash "$demo_dir/demo.sh"; then
            local end_time=$(date +%s)
            local duration=$((end_time - start_time))
            print_success "Demo completed in ${duration}s"
            return 0
        else
            print_error "Demo failed"
            return 1
        fi
    else
        print_error "Demo script not found: $demo_dir/demo.sh"
        return 1
    fi
}

# Cleanup
cleanup() {
    if [[ "$AUTO_CLEANUP" != "true" ]]; then
        print_info "Skipping cleanup (--no-cleanup specified)"
        print_info "Manual cleanup: ./scripts/cleanup.sh"
        return 0
    fi

    print_section "Cleanup"

    print_step "Cleaning up resources..."
    
    if [[ "$DRY_RUN" == "true" ]]; then
        print_info "DRY RUN: Would run cleanup"
        return 0
    fi

    # Kill any running servers
    if pgrep -f "nestgate.*server" > /dev/null; then
        print_step "Stopping servers..."
        pkill -f "nestgate.*server" || true
        print_success "Servers stopped"
    fi

    # Run cleanup script if exists
    if [[ -f scripts/cleanup.sh ]]; then
        bash scripts/cleanup.sh || true
        print_success "Cleanup completed"
    else
        print_info "No cleanup script found"
    fi
}

# Generate summary report
generate_report() {
    local end_time=$(date +%s)
    local duration=$((end_time - start_time))
    
    print_section "Showcase Complete! ✨"

    echo -e "${GREEN}Summary:${NC}"
    echo -e "  Duration:        $(printf '%dm %ds' $((duration/60)) $((duration%60)))"
    echo -e "  Backend:         $BACKEND"
    echo -e "  Dry Run:         $DRY_RUN"
    echo ""
    
    echo -e "${CYAN}Results:${NC}"
    echo -e "  Logs:            $SHOWCASE_ROOT/outputs/logs/"
    echo -e "  Metrics:         $SHOWCASE_ROOT/outputs/metrics/"
    echo -e "  Config:          $SHOWCASE_ROOT/outputs/showcase_config.toml"
    echo ""

    if [[ "$BACKEND" == "zfs" ]]; then
        echo -e "${BLUE}ZFS Resources:${NC}"
        echo -e "  Pool:            nestgate_demo"
        echo -e "  Check status:    zpool status nestgate_demo"
        echo -e "  List datasets:   zfs list -r nestgate_demo"
        echo ""
    fi

    echo -e "${YELLOW}Next Steps:${NC}"
    echo -e "  View logs:       cat outputs/logs/showcase.log"
    echo -e "  See metrics:     http://localhost:$METRICS_PORT"
    echo -e "  API explorer:    http://localhost:$API_PORT/docs"
    
    if [[ "$AUTO_CLEANUP" != "true" ]]; then
        echo -e "  Cleanup:         ./scripts/cleanup.sh"
    fi
    echo ""
}

# ═══════════════════════════════════════════════════════════
# MAIN EXECUTION
# ═══════════════════════════════════════════════════════════

main() {
    local start_time=$(date +%s)

    # Parse arguments
    parse_args "$@"

    # Print header
    print_header

    # Check prerequisites
    if ! check_prerequisites; then
        exit 1
    fi

    # Setup environment
    setup_environment

    # Trap cleanup on exit
    trap cleanup EXIT

    # Determine which demos to run
    local demos_to_run=()
    case "$DEMO_SELECTION" in
        all)
            demos_to_run=(
                "01_zfs_basics"
                "02_performance"
                "03_api_showcase"
                "04_infant_discovery"
                "05_security"
                "06_monitoring"
            )
            ;;
        zfs)
            demos_to_run=("01_zfs_basics")
            ;;
        performance)
            demos_to_run=("02_performance")
            ;;
        api)
            demos_to_run=("03_api_showcase")
            ;;
        discovery)
            demos_to_run=("04_infant_discovery")
            ;;
        security)
            demos_to_run=("05_security")
            ;;
        monitoring)
            demos_to_run=("06_monitoring")
            ;;
        *)
            print_error "Unknown demo: $DEMO_SELECTION"
            usage
            ;;
    esac

    # Run selected demos
    local failed_demos=0
    for demo in "${demos_to_run[@]}"; do
        if ! run_demo "$demo"; then
            ((failed_demos++))
        fi
    done

    # Generate report
    generate_report

    # Exit with appropriate code
    if [[ $failed_demos -gt 0 ]]; then
        print_warning "$failed_demos demo(s) failed"
        exit 1
    else
        print_success "All demos completed successfully!"
        exit 0
    fi
}

# Run main if executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi


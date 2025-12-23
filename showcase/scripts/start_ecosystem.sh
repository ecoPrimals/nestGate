#!/usr/bin/env bash
# Start full ecoPrimals ecosystem for showcase testing
# Coordinates NestGate, Songbird, and ToadStool services

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m'

# Configuration
ECOPRIMAL_ROOT="/home/eastgate/Development/ecoPrimals"
NESTGATE_PORT=8080
SONGBIRD_PORT=6666
TOADSTOOL_PORT=7000

# Service PIDs
NESTGATE_PID=""
SONGBIRD_PID=""
TOADSTOOL_PID=""

# Logs
LOG_DIR="/tmp/ecoprimal_ecosystem_logs"
mkdir -p "$LOG_DIR"

# =============================================================================
# Helper Functions
# =============================================================================

log_header() {
    echo -e "\n${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║$(printf '%*s' 60 '' | tr ' ' ' ')║${NC}"
    echo -e "${BLUE}║  ${1}$(printf '%*s' $((57 - ${#1})) '' | tr ' ' ' ')║${NC}"
    echo -e "${BLUE}║$(printf '%*s' 60 '' | tr ' ' ' ')║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}\n"
}

log_section() {
    echo -e "\n${CYAN}═══════════════════════════════════════════════════════════${NC}"
    echo -e "${CYAN}${1}${NC}"
    echo -e "${CYAN}═══════════════════════════════════════════════════════════${NC}\n"
}

log_success() {
    echo -e "${GREEN}✓${NC} ${1}"
}

log_fail() {
    echo -e "${RED}✗${NC} ${1}"
}

log_info() {
    echo -e "${YELLOW}→${NC} ${1}"
}

log_service() {
    echo -e "${MAGENTA}${1}${NC} ${2}"
}

# =============================================================================
# Service Management
# =============================================================================

check_port_free() {
    local port=$1
    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1; then
        return 1
    fi
    return 0
}

wait_for_health() {
    local url=$1
    local service_name=$2
    local max_attempts=30
    local attempt=0
    
    log_info "Waiting for $service_name to be healthy..."
    
    while [ $attempt -lt $max_attempts ]; do
        if curl -sf "$url/health" > /dev/null 2>&1; then
            log_success "$service_name is healthy"
            return 0
        fi
        attempt=$((attempt + 1))
        sleep 1
    done
    
    log_fail "$service_name failed to become healthy"
    return 1
}

stop_service() {
    local pid=$1
    local name=$2
    
    if [ -n "$pid" ] && ps -p $pid > /dev/null 2>&1; then
        log_info "Stopping $name (PID: $pid)..."
        kill $pid 2>/dev/null || true
        sleep 2
        
        if ps -p $pid > /dev/null 2>&1; then
            log_info "Force killing $name..."
            kill -9 $pid 2>/dev/null || true
        fi
        
        log_success "$name stopped"
    fi
}

cleanup() {
    log_section "Cleaning Up Services"
    
    stop_service "$TOADSTOOL_PID" "ToadStool"
    stop_service "$SONGBIRD_PID" "Songbird"
    stop_service "$NESTGATE_PID" "NestGate"
    
    log_success "All services stopped"
    exit 0
}

trap cleanup EXIT INT TERM

# =============================================================================
# Service Startup
# =============================================================================

start_nestgate() {
    log_section "Starting NestGate (Storage & Data Service)"
    
    # Check if already running
    if ! check_port_free $NESTGATE_PORT; then
        log_fail "Port $NESTGATE_PORT already in use"
        log_info "Kill existing process or use different port"
        return 1
    fi
    
    # Check binary
    local binary="$ECOPRIMAL_ROOT/nestgate/target/release/nestgate"
    if [ ! -f "$binary" ]; then
        log_fail "NestGate binary not found: $binary"
        log_info "Build with: cd $ECOPRIMAL_ROOT/nestgate && cargo build --release"
        return 1
    fi
    
    log_info "Binary: $binary"
    log_info "Port: $NESTGATE_PORT"
    log_info "Logs: $LOG_DIR/nestgate.log"
    
    # Ensure config directory exists
    mkdir -p ~/.nestgate/{data,logs,config,cache,temp}
    
    # Create config if needed
    if [ ! -f ~/.nestgate/config.toml ]; then
        cat > ~/.nestgate/config.toml << 'EOF'
[service]
name = "nestgate-showcase"
bind_address = "127.0.0.1"
port = 8080

[storage]
data_dir = "/home/eastgate/.nestgate/data"
cache_dir = "/home/eastgate/.nestgate/cache"
temp_dir = "/home/eastgate/.nestgate/temp"

[logging]
level = "info"
dir = "/home/eastgate/.nestgate/logs"
format = "pretty"

[api]
enable_cors = true
enable_metrics = true
enable_health_checks = true

[discovery]
enabled = true
EOF
    fi
    
    # Start service
    cd "$ECOPRIMAL_ROOT/nestgate"
    "$binary" service start --config ~/.nestgate/config.toml > "$LOG_DIR/nestgate.log" 2>&1 &
    NESTGATE_PID=$!
    
    log_success "NestGate started (PID: $NESTGATE_PID)"
    
    # Wait for health
    if wait_for_health "http://127.0.0.1:$NESTGATE_PORT" "NestGate"; then
        log_success "NestGate is ready!"
        return 0
    else
        log_fail "NestGate failed to start"
        log_info "Check logs: tail -f $LOG_DIR/nestgate.log"
        return 1
    fi
}

start_songbird() {
    log_section "Starting Songbird (Orchestration Service)"
    
    # Check if already running
    if ! check_port_free $SONGBIRD_PORT; then
        log_fail "Port $SONGBIRD_PORT already in use"
        return 1
    fi
    
    # Check binary
    local binary="$ECOPRIMAL_ROOT/songbird/target/release/songbird-orchestrator"
    if [ ! -f "$binary" ]; then
        log_fail "Songbird binary not found: $binary"
        log_info "Build with: cd $ECOPRIMAL_ROOT/songbird && cargo build --release"
        return 1
    fi
    
    log_info "Binary: $binary"
    log_info "Port: $SONGBIRD_PORT"
    log_info "Logs: $LOG_DIR/songbird.log"
    
    # Start service
    cd "$ECOPRIMAL_ROOT/songbird"
    RUST_LOG=info "$binary" --port $SONGBIRD_PORT > "$LOG_DIR/songbird.log" 2>&1 &
    SONGBIRD_PID=$!
    
    log_success "Songbird started (PID: $SONGBIRD_PID)"
    
    # Wait for health
    if wait_for_health "http://127.0.0.1:$SONGBIRD_PORT" "Songbird"; then
        log_success "Songbird is ready!"
        return 0
    else
        log_fail "Songbird failed to start"
        log_info "Check logs: tail -f $LOG_DIR/songbird.log"
        return 1
    fi
}

start_toadstool() {
    log_section "Starting ToadStool (Universal Compute Service)"
    
    # Check if already running
    if ! check_port_free $TOADSTOOL_PORT; then
        log_fail "Port $TOADSTOOL_PORT already in use"
        return 1
    fi
    
    # Check binary - try multiple possible names
    local binary=""
    for name in toadstool-cli toadstool-server toadstool; do
        if [ -f "$ECOPRIMAL_ROOT/toadstool/target/release/$name" ]; then
            binary="$ECOPRIMAL_ROOT/toadstool/target/release/$name"
            break
        fi
    done
    
    if [ -z "$binary" ]; then
        log_fail "ToadStool binary not found"
        log_info "Build with: cd $ECOPRIMAL_ROOT/toadstool && cargo build --release"
        return 1
    fi
    
    log_info "Binary: $binary"
    log_info "Port: $TOADSTOOL_PORT"
    log_info "Logs: $LOG_DIR/toadstool.log"
    
    # Start service
    cd "$ECOPRIMAL_ROOT/toadstool"
    RUST_LOG=info "$binary" serve --port $TOADSTOOL_PORT > "$LOG_DIR/toadstool.log" 2>&1 &
    TOADSTOOL_PID=$!
    
    log_success "ToadStool started (PID: $TOADSTOOL_PID)"
    
    # Wait for health
    if wait_for_health "http://127.0.0.1:$TOADSTOOL_PORT" "ToadStool"; then
        log_success "ToadStool is ready!"
        return 0
    else
        log_fail "ToadStool failed to start"
        log_info "Check logs: tail -f $LOG_DIR/toadstool.log"
        return 1
    fi
}

# =============================================================================
# Main
# =============================================================================

main() {
    log_header "🌍 ecoPrimals Ecosystem Startup"
    
    echo "Configuration:"
    echo "  NestGate:  http://127.0.0.1:$NESTGATE_PORT"
    echo "  Songbird:  http://127.0.0.1:$SONGBIRD_PORT"
    echo "  ToadStool: http://127.0.0.1:$TOADSTOOL_PORT"
    echo "  Logs:      $LOG_DIR/"
    echo ""
    
    # Parse arguments
    local start_nestgate=true
    local start_songbird=true
    local start_toadstool=true
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            --services)
                start_nestgate=false
                start_songbird=false
                start_toadstool=false
                shift
                IFS=',' read -ra SERVICES <<< "$1"
                for service in "${SERVICES[@]}"; do
                    case $service in
                        nestgate) start_nestgate=true ;;
                        songbird) start_songbird=true ;;
                        toadstool) start_toadstool=true ;;
                        all) start_nestgate=true; start_songbird=true; start_toadstool=true ;;
                        *) log_fail "Unknown service: $service" ;;
                    esac
                done
                shift
                ;;
            --help)
                echo "Usage: $0 [OPTIONS]"
                echo ""
                echo "Options:"
                echo "  --services <list>    Comma-separated list of services to start"
                echo "                       (nestgate,songbird,toadstool,all)"
                echo "  --help              Show this help"
                echo ""
                echo "Examples:"
                echo "  $0                                  # Start all services"
                echo "  $0 --services nestgate              # Start only NestGate"
                echo "  $0 --services nestgate,songbird     # Start NestGate and Songbird"
                exit 0
                ;;
            *)
                log_fail "Unknown option: $1"
                exit 1
                ;;
        esac
    done
    
    # Start services
    local success=true
    
    if $start_nestgate; then
        start_nestgate || success=false
    fi
    
    if $start_songbird; then
        start_songbird || success=false
    fi
    
    if $start_toadstool; then
        start_toadstool || success=false
    fi
    
    # Summary
    log_header "Ecosystem Status"
    
    echo "Service Status:"
    if $start_nestgate; then
        if [ -n "$NESTGATE_PID" ] && ps -p $NESTGATE_PID > /dev/null 2>&1; then
            log_success "NestGate:  Running (PID: $NESTGATE_PID, Port: $NESTGATE_PORT)"
        else
            log_fail "NestGate:  Failed"
        fi
    fi
    
    if $start_songbird; then
        if [ -n "$SONGBIRD_PID" ] && ps -p $SONGBIRD_PID > /dev/null 2>&1; then
            log_success "Songbird:  Running (PID: $SONGBIRD_PID, Port: $SONGBIRD_PORT)"
        else
            log_fail "Songbird:  Failed"
        fi
    fi
    
    if $start_toadstool; then
        if [ -n "$TOADSTOOL_PID" ] && ps -p $TOADSTOOL_PID > /dev/null 2>&1; then
            log_success "ToadStool: Running (PID: $TOADSTOOL_PID, Port: $TOADSTOOL_PORT)"
        else
            log_fail "ToadStool: Failed"
        fi
    fi
    
    echo ""
    echo "Quick Test:"
    echo "  curl http://127.0.0.1:$NESTGATE_PORT/health"
    echo "  curl http://127.0.0.1:$SONGBIRD_PORT/health"
    echo "  curl http://127.0.0.1:$TOADSTOOL_PORT/health"
    echo ""
    echo "View Logs:"
    echo "  tail -f $LOG_DIR/nestgate.log"
    echo "  tail -f $LOG_DIR/songbird.log"
    echo "  tail -f $LOG_DIR/toadstool.log"
    echo ""
    echo "Run Tests:"
    echo "  cd showcase && ./test_all_demos.sh"
    echo ""
    echo "Stop Services:"
    echo "  Press Ctrl+C to stop all services"
    echo ""
    
    if $success; then
        log_success "Ecosystem is ready!"
        
        # Keep running until interrupted
        echo "Services running... (Press Ctrl+C to stop)"
        while true; do
            sleep 1
        done
    else
        log_fail "Some services failed to start"
        log_info "Check logs in $LOG_DIR/"
        return 1
    fi
}

# Run main function
main "$@"


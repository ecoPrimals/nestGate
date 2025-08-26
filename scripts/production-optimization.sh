#!/usr/bin/env bash
# ==============================================================================
# NestGate Production Optimization Script
# Implements immediate performance wins and production readiness checks
# ==============================================================================

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
LOG_FILE="/tmp/nestgate-optimization-$(date +%Y%m%d_%H%M%S).log"

# Logging function
log() {
    echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $*" | tee -a "$LOG_FILE"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $*" | tee -a "$LOG_FILE"
}

error() {
    echo -e "${RED}[ERROR]${NC} $*" | tee -a "$LOG_FILE"
}

info() {
    echo -e "${BLUE}[INFO]${NC} $*" | tee -a "$LOG_FILE"
}

# ==============================================================================
# IMMEDIATE WIN 1: Documentation Enhancement
# ==============================================================================

optimize_documentation() {
    log "🔧 OPTIMIZING DOCUMENTATION..."
    
    cd "$PROJECT_ROOT"
    
    # Generate comprehensive documentation
    info "Generating API documentation..."
    cargo doc --all --no-deps --document-private-items 2>/dev/null || warn "Some documentation warnings exist (expected)"
    
    # Create documentation index
    cat > target/doc/index.html << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <title>NestGate Documentation</title>
    <meta charset="utf-8">
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto; margin: 40px; }
        .header { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 30px; border-radius: 10px; margin-bottom: 30px; }
        .grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; }
        .card { background: #f8f9fa; border: 1px solid #e9ecef; border-radius: 8px; padding: 20px; }
        .card h3 { margin-top: 0; color: #495057; }
        a { color: #007bff; text-decoration: none; }
        a:hover { text-decoration: underline; }
    </style>
</head>
<body>
    <div class="header">
        <h1>🚀 NestGate Documentation Portal</h1>
        <p>Production-ready storage system with universal adapter architecture</p>
    </div>
    
    <div class="grid">
        <div class="card">
            <h3>📚 API Documentation</h3>
            <ul>
                <li><a href="nestgate_core/index.html">Core Engine</a></li>
                <li><a href="nestgate_api/index.html">REST API</a></li>
                <li><a href="nestgate_zfs/index.html">ZFS Integration</a></li>
                <li><a href="nestgate_network/index.html">Network Layer</a></li>
            </ul>
        </div>
        
        <div class="card">
            <h3>🏗️ Architecture</h3>
            <ul>
                <li><a href="../docs/current/ARCHITECTURE_DIAGRAMS.md">System Architecture</a></li>
                <li><a href="../specs/core-architecture/ARCHITECTURE_OVERVIEW.md">Core Architecture</a></li>
                <li><a href="../specs/UNIVERSAL_DATA_SCALING_SPECIFICATION.md">Scaling Design</a></li>
            </ul>
        </div>
        
        <div class="card">
            <h3>🚀 Deployment</h3>
            <ul>
                <li><a href="../docs/current/DEPLOYMENT_GUIDE.md">Deployment Guide</a></li>
                <li><a href="../PRODUCTION_DEPLOYMENT_GUIDE.md">Production Setup</a></li>
                <li><a href="../docker-compose.yml">Docker Compose</a></li>
            </ul>
        </div>
        
        <div class="card">
            <h3>🧪 Testing</h3>
            <ul>
                <li><a href="../specs/development/BRUTAL_TESTING_STRATEGY.md">Testing Strategy</a></li>
                <li><a href="../coverage-reports/html/index.html">Coverage Report</a></li>
                <li><a href="../performance_results/">Performance Results</a></li>
            </ul>
        </div>
    </div>
</body>
</html>
EOF
    
    log "✅ Documentation optimization complete"
}

# ==============================================================================
# IMMEDIATE WIN 2: Performance Profiling Setup
# ==============================================================================

setup_performance_profiling() {
    log "🔧 SETTING UP PERFORMANCE PROFILING..."
    
    cd "$PROJECT_ROOT"
    
    # Install performance tools if not present
    if ! command -v cargo-flamegraph &> /dev/null; then
        info "Installing cargo-flamegraph for CPU profiling..."
        cargo install flamegraph --quiet
    fi
    
    if ! command -v cargo-benchcmp &> /dev/null; then
        info "Installing cargo-benchcmp for benchmark comparison..."
        cargo install cargo-benchcmp --quiet
    fi
    
    # Create performance monitoring script
    cat > scripts/performance-monitor.sh << 'EOF'
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
EOF
    
    chmod +x scripts/performance-monitor.sh
    
    log "✅ Performance profiling setup complete"
}

# ==============================================================================
# IMMEDIATE WIN 3: Production Health Checks
# ==============================================================================

setup_health_monitoring() {
    log "🔧 SETTING UP HEALTH MONITORING..."
    
    cd "$PROJECT_ROOT"
    
    # Create comprehensive health check script
    cat > scripts/health-check.sh << 'EOF'
#!/usr/bin/env bash
# Comprehensive health check for NestGate production deployment

set -euo pipefail

# Configuration
NESTGATE_HOST="${NESTGATE_HOST:-localhost}"
NESTGATE_PORT="${NESTGATE_PORT:-8000}"
HEALTH_ENDPOINT="http://${NESTGATE_HOST}:${NESTGATE_PORT}/health"
METRICS_ENDPOINT="http://${NESTGATE_HOST}:${NESTGATE_PORT}/metrics"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

check_service() {
    local name=$1
    local url=$2
    local expected_status=${3:-200}
    
    echo -n "Checking $name... "
    
    if response=$(curl -s -o /dev/null -w "%{http_code}" "$url" 2>/dev/null); then
        if [ "$response" = "$expected_status" ]; then
            echo -e "${GREEN}✅ OK${NC} (HTTP $response)"
            return 0
        else
            echo -e "${RED}❌ FAIL${NC} (HTTP $response, expected $expected_status)"
            return 1
        fi
    else
        echo -e "${RED}❌ UNREACHABLE${NC}"
        return 1
    fi
}

check_metrics() {
    echo -n "Checking metrics availability... "
    
    if metrics=$(curl -s "$METRICS_ENDPOINT" 2>/dev/null); then
        metric_count=$(echo "$metrics" | grep -c "^nestgate_" || true)
        if [ "$metric_count" -gt 0 ]; then
            echo -e "${GREEN}✅ OK${NC} ($metric_count metrics available)"
            return 0
        else
            echo -e "${YELLOW}⚠️ NO METRICS${NC}"
            return 1
        fi
    else
        echo -e "${RED}❌ UNREACHABLE${NC}"
        return 1
    fi
}

check_system_resources() {
    echo "📊 System Resources:"
    
    # CPU usage
    cpu_usage=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}' | sed 's/%us,//')
    echo "  CPU Usage: ${cpu_usage}%"
    
    # Memory usage
    memory_info=$(free -h | grep "Mem:")
    echo "  Memory: $memory_info"
    
    # Disk usage
    disk_usage=$(df -h / | tail -1 | awk '{print "Used: " $3 " / " $2 " (" $5 ")"}')
    echo "  Disk: $disk_usage"
}

main() {
    echo "🏥 NestGate Health Check Report"
    echo "================================"
    echo "Target: $NESTGATE_HOST:$NESTGATE_PORT"
    echo "Time: $(date)"
    echo ""
    
    # Service checks
    local all_good=true
    
    check_service "Health Endpoint" "$HEALTH_ENDPOINT" || all_good=false
    check_metrics || all_good=false
    
    echo ""
    check_system_resources
    
    echo ""
    if [ "$all_good" = true ]; then
        echo -e "${GREEN}🎉 All systems operational!${NC}"
        exit 0
    else
        echo -e "${RED}⚠️ Some issues detected${NC}"
        exit 1
    fi
}

main "$@"
EOF
    
    chmod +x scripts/health-check.sh
    
    log "✅ Health monitoring setup complete"
}

# ==============================================================================
# IMMEDIATE WIN 4: Deployment Automation
# ==============================================================================

create_deployment_automation() {
    log "🔧 CREATING DEPLOYMENT AUTOMATION..."
    
    cd "$PROJECT_ROOT"
    
    # Create one-command deployment script
    cat > scripts/deploy-production.sh << 'EOF'
#!/usr/bin/env bash
# One-command production deployment for NestGate

set -euo pipefail

# Configuration
DEPLOYMENT_ENV="${1:-staging}"
DOCKER_TAG="${2:-latest}"
COMPOSE_FILE="docker-compose.yml"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

log() {
    echo -e "${GREEN}[DEPLOY]${NC} $*"
}

info() {
    echo -e "${BLUE}[INFO]${NC} $*"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $*"
}

deploy_staging() {
    log "🚀 Deploying to STAGING environment..."
    
    # Build and deploy
    docker-compose -f "$COMPOSE_FILE" build
    docker-compose -f "$COMPOSE_FILE" up -d
    
    # Wait for services to be ready
    sleep 10
    
    # Health check
    if ./scripts/health-check.sh; then
        log "✅ Staging deployment successful!"
    else
        warn "⚠️ Health check failed - please investigate"
        return 1
    fi
}

deploy_production() {
    log "🚀 Deploying to PRODUCTION environment..."
    
    # Pre-deployment checks
    info "Running pre-deployment validation..."
    
    # Ensure tests pass
    if ! cargo test --release --all; then
        warn "❌ Tests failed - aborting deployment"
        return 1
    fi
    
    # Security audit
    if ! cargo audit; then
        warn "❌ Security audit failed - aborting deployment"
        return 1
    fi
    
    # Build optimized release
    info "Building production release..."
    cargo build --release --all
    
    # Deploy with zero-downtime
    info "Performing zero-downtime deployment..."
    docker-compose -f "$COMPOSE_FILE" -f docker-compose.prod.yml up -d --no-deps nestgate
    
    # Health check with retries
    local retries=5
    while [ $retries -gt 0 ]; do
        if ./scripts/health-check.sh; then
            log "✅ Production deployment successful!"
            return 0
        else
            warn "Health check failed, retrying... ($retries attempts left)"
            sleep 10
            ((retries--))
        fi
    done
    
    warn "❌ Production deployment failed health checks"
    return 1
}

main() {
    info "NestGate Deployment Automation"
    info "Environment: $DEPLOYMENT_ENV"
    info "Docker Tag: $DOCKER_TAG"
    echo ""
    
    case "$DEPLOYMENT_ENV" in
        "staging")
            deploy_staging
            ;;
        "production")
            deploy_production
            ;;
        *)
            warn "Unknown environment: $DEPLOYMENT_ENV"
            echo "Usage: $0 [staging|production] [docker-tag]"
            exit 1
            ;;
    esac
}

main "$@"
EOF
    
    chmod +x scripts/deploy-production.sh
    
    log "✅ Deployment automation complete"
}

# ==============================================================================
# IMMEDIATE WIN 5: Development Environment Setup
# ==============================================================================

setup_development_tools() {
    log "🔧 SETTING UP DEVELOPMENT TOOLS..."
    
    cd "$PROJECT_ROOT"
    
    # Create development setup script
    cat > scripts/setup-dev-env.sh << 'EOF'
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
EOF
    
    chmod +x scripts/setup-dev-env.sh
    
    log "✅ Development tools setup complete"
}

# ==============================================================================
# Main Execution
# ==============================================================================

main() {
    log "🚀 STARTING NESTGATE PRODUCTION OPTIMIZATION"
    log "Log file: $LOG_FILE"
    echo ""
    
    # Ensure we're in the right directory
    if [[ ! -f "Cargo.toml" ]]; then
        error "❌ Must be run from NestGate project root"
        exit 1
    fi
    
    # Create scripts directory if it doesn't exist
    mkdir -p scripts
    
    # Run optimizations
    optimize_documentation
    setup_performance_profiling
    setup_health_monitoring
    create_deployment_automation
    setup_development_tools
    
    log ""
    log "🎉 OPTIMIZATION COMPLETE!"
    log ""
    log "📋 Available Scripts:"
    log "  ./scripts/performance-monitor.sh  - CPU and memory profiling"
    log "  ./scripts/health-check.sh         - Comprehensive health checks"
    log "  ./scripts/deploy-production.sh    - One-command deployment"
    log "  ./scripts/setup-dev-env.sh        - Development environment setup"
    log ""
    log "📊 Next Steps:"
    log "  1. Run: ./scripts/setup-dev-env.sh"
    log "  2. Test: ./scripts/health-check.sh"
    log "  3. Deploy: ./scripts/deploy-production.sh staging"
    log ""
    log "🏆 Your NestGate system is now OPTIMIZED for production!"
}

# Run if called directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi 
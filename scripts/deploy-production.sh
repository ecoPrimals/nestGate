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

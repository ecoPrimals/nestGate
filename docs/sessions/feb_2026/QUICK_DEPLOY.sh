#!/bin/bash
# Quick Deployment Script for NestGate
# Usage: ./QUICK_DEPLOY.sh [staging|production]

set -e

ENV=${1:-staging}

echo "=== NESTGATE QUICK DEPLOY ==="
echo "Environment: $ENV"
echo ""

# Verify readiness
echo "1️⃣  Verifying deployment readiness..."
./verify_deployment_readiness.sh || exit 1

echo ""
echo "2️⃣  Building release binary..."
cargo build --workspace --release

echo ""
echo "3️⃣  Preparing deployment..."
if [ "$ENV" = "production" ]; then
    echo "⚠️  PRODUCTION DEPLOYMENT"
    echo "Using: docker/docker-compose.production.yml"
    COMPOSE_FILE="docker/docker-compose.production.yml"
else
    echo "📦 STAGING DEPLOYMENT"
    COMPOSE_FILE="docker/docker-compose.yml"
fi

echo ""
echo "4️⃣  Starting services..."
if [ -f "$COMPOSE_FILE" ]; then
    docker-compose -f "$COMPOSE_FILE" up -d
    echo "✅ Services started"
else
    echo "⚠️  Docker Compose file not found: $COMPOSE_FILE"
    echo "Manual deployment required"
fi

echo ""
echo "5️⃣  Health check..."
sleep 5
if curl -s http://localhost:8080/health > /dev/null 2>&1; then
    echo "✅ Health check passed"
else
    echo "⚠️  Health check endpoint not responding"
    echo "Check logs: docker-compose logs -f"
fi

echo ""
echo "=== DEPLOYMENT COMPLETE ==="
echo ""
echo "📊 Monitor with:"
echo "  docker-compose logs -f"
echo ""
echo "🔍 Check health:"
echo "  curl http://localhost:8080/health"
echo ""
echo "📈 View metrics:"
echo "  curl http://localhost:9090/metrics"


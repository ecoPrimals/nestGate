#!/usr/bin/env bash
#
# Hardcoded Port Migration Helper
#
# This script identifies hardcoded port numbers in the codebase and generates
# a report for systematic migration to environment-driven configuration.
#
# Usage: ./scripts/migrate_hardcoded_ports.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

echo "🔍 Scanning for hardcoded ports..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Find common port numbers (excluding tests and comments)
echo "📊 Most frequently hardcoded ports:"
echo ""

rg ':\s*(\d{4,5})\b' code/crates --type rust \
    --no-heading --no-filename \
    | grep -v '//' \
    | grep -v '^\s*//' \
    | sed -E 's/.*:([0-9]{4,5}).*/\1/' \
    | sort | uniq -c | sort -rn | head -20

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📁 Files with most hardcoded ports:"
echo ""

rg ':\s*\d{4,5}' code/crates --type rust --files-with-matches \
    | xargs -I {} sh -c 'echo "$(rg -c ":\s*\d{4,5}" {} || echo 0) {}"' \
    | sort -rn | head -20

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "🎯 Port Migration Priorities:"
echo ""
echo "Port 8080  - Main API server"
echo "Port 3000  - Development server"
echo "Port 5000  - Alternative API"
echo "Port 9090  - Metrics/Monitoring"
echo "Port 6379  - Redis"
echo "Port 5432  - PostgreSQL"
echo "Port 27017 - MongoDB"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "💡 Migration Pattern:"
echo ""
echo "  BEFORE: let addr = format!(\"127.0.0.1:8080\");"
echo "  AFTER:  let config = EnvironmentConfig::from_env()?;"
echo "          let addr = config.bind_address();"
echo ""
echo "  Environment Variable: NESTGATE_PORT=8080"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

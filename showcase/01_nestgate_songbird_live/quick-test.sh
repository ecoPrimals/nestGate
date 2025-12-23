#!/bin/bash
# Quick Test - Start 2 NestGate nodes for testing
# Simplified version for immediate testing

set -e

echo "🚀 Quick Multi-Node Test Setup"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

NESTGATE_BIN="/home/eastgate/Development/ecoPrimals/nestgate/target/release/nestgate"

# Generate secure JWT secret for demo
export NESTGATE_JWT_SECRET=$(openssl rand -base64 48)
echo "Generated JWT secret for demo"
echo ""

# Clean up
pkill -f nestgate || true
sleep 2

# Start Node 1
echo "Starting Westgate (port 9005)..."
NESTGATE_API_PORT=9005 NESTGATE_JWT_SECRET="$NESTGATE_JWT_SECRET" \
  $NESTGATE_BIN service start --port 9005 > /tmp/westgate.log 2>&1 &
WEST_PID=$!
echo "  PID: $WEST_PID"
sleep 3

# Start Node 2
echo "Starting Eastgate (port 9006)..."
NESTGATE_API_PORT=9006 NESTGATE_JWT_SECRET="$NESTGATE_JWT_SECRET" \
  $NESTGATE_BIN service start --port 9006 > /tmp/eastgate.log 2>&1 &
EAST_PID=$!
echo "  PID: $EAST_PID"
sleep 3

echo ""
echo "Checking health..."
for port in 9005 9006; do
    if curl -s http://localhost:$port/health > /dev/null 2>&1; then
        echo "✅ Port $port: Running"
    else
        echo "❌ Port $port: Not responding"
        echo "   Log: /tmp/$([ $port = 9005 ] && echo westgate || echo eastgate).log"
    fi
done

echo ""
echo "PIDs saved:"
echo "  Westgate: $WEST_PID"
echo "  Eastgate: $EAST_PID"
echo ""
echo "To stop: kill $WEST_PID $EAST_PID"
echo ""
echo "To test: ./02-coordinated-storage.sh"


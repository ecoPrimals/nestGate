#!/bin/bash

# NestGate UI - Fallback Mode Test Script
# This script starts the UI with live mode but intentionally points to non-existent services 
# to demonstrate fallback to mock data behavior

echo "🟡 Starting NestGate UI in FALLBACK TEST mode"
echo "----------------------------------------"
echo "This mode simulates backend service connection failures"
echo "The UI should attempt to connect to services but fall back to mock data"
echo "----------------------------------------"

# Set environment variables for fallback testing
export REACT_APP_USE_MOCK_ALL=false
export REACT_APP_USE_MOCK_WEBSOCKET=false
export REACT_APP_USE_MOCK_BACKUP=false
export REACT_APP_USE_MOCK_TELEMETRY=false

# Point APIs to non-existent endpoints to force fallback behavior
export REACT_APP_API_BASE_URL="http://localhost:12345"
export REACT_APP_WEBSOCKET_URL="ws://localhost:12346"

# Start development server with fallback test configuration
echo "Starting development server..."
cd "$(dirname "$0")/.." || exit
DISABLE_ESLINT_PLUGIN=true npm run start:direct

# Exit gracefully when user terminates with Ctrl+C
echo "Development server has been terminated."
exit 0 
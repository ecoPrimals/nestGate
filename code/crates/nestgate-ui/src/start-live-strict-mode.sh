#!/bin/bash

# Start the NestGate UI in strict live-only mode
# This script ensures that only real data is used, with no mock data fallbacks

# Set environment variables for strict live mode
export REACT_APP_LIVE_MODE=true
export REACT_APP_USE_MOCK_DATA=false
export REACT_APP_STRICT_DATA_MODE=true

# Display the mode we're starting in
echo "Starting NestGate UI in STRICT LIVE MODE"
echo "----------------------------------------"
echo "✓ Live mode enabled"
echo "✓ Mock data disabled"
echo "✓ Strict live data mode enabled"
echo "----------------------------------------"
echo "This mode will only use real server data, with placeholder UI for unimplemented features."
echo "No mock data will be used, even as fallback."
echo

# Check if the server is running
if ! curl -s http://localhost:3000/api/status > /dev/null; then
  echo "⚠️ Server is not running or not responding"
  echo "Starting the server in live mode..."
  
  # Start the server in another terminal window if possible
  if command -v gnome-terminal &> /dev/null; then
    gnome-terminal -- bash -c "cd $(pwd) && ./start-live-server.sh; bash"
  elif command -v xterm &> /dev/null; then
    xterm -e "cd $(pwd) && ./start-live-server.sh; bash" &
  else
    echo "Please start the server manually by running './start-live-server.sh' in another terminal window"
    echo "Press Enter once the server is running..."
    read
  fi
  
  # Wait for the server to be ready
  echo "Waiting for server to start..."
  attempts=0
  max_attempts=30
  while ! curl -s http://localhost:3000/api/status > /dev/null && [ $attempts -lt $max_attempts ]; do
    echo -n "."
    sleep 1
    attempts=$((attempts+1))
  done
  
  if [ $attempts -lt $max_attempts ]; then
    echo "✓ Server is ready!"
  else
    echo "❌ Server did not start within the expected time"
    echo "Please check server logs for errors and try again"
    exit 1
  fi
fi

# Start the UI
echo "Starting UI in strict live mode..."
npm run start 
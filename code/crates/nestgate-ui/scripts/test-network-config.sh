#!/bin/bash

# Script to run tests for NetworkConfiguration component
# Usage: ./test-network-config.sh

echo "Running tests for NetworkConfiguration component..."
npm test -- --testPathPattern=NetworkConfiguration.spec --watchAll=false

# Check if tests passed
if [ $? -eq 0 ]; then
  echo -e "\n\033[0;32mSuccess! All NetworkConfiguration tests passed.\033[0m"
else
  echo -e "\n\033[0;31mFailed! Some NetworkConfiguration tests did not pass.\033[0m"
  exit 1
fi 
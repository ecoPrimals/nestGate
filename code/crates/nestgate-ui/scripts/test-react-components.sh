#!/bin/bash

echo "Running tests for React components..."

# Find all test files
TEST_FILES=$(find src -name "*.spec.tsx" -o -name "*.test.tsx" -o -name "*.spec.ts" -o -name "*.test.ts" | grep -v "__tests__" | sort)

if [ -z "$TEST_FILES" ]; then
  echo "No test files found!"
  exit 1
fi

# Run tests
npx jest $TEST_FILES

echo "All React component tests completed!"

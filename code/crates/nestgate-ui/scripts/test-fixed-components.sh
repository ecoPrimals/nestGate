#!/bin/bash

echo "Running tests for fixed components..."

# Test NasMetrics
echo -e "\n=== Testing NasMetrics ==="
npx jest --testPathPattern=src/components/dashboard/NasMetrics.spec.tsx

# Test PerformanceOptimizer
echo -e "\n=== Testing PerformanceOptimizer ==="
npx jest --testPathPattern=src/components/storage/PerformanceOptimizer.spec.tsx

echo -e "\n=== Testing NasMetrics in __tests__ directory ==="
npm test src/__tests__/components/NasMetrics.spec.tsx

echo -e "\n=== Testing Format Utilities ==="
npm test src/utils/format.test.ts

echo -e "\n=== All fixed component tests completed ===" 
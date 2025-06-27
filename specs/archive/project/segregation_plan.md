# NestGate Mock Data Containment Plan

## Overview

This document outlines the revised approach to handling mock data in the NestGate application. Rather than implementing complex middleware and mode switching, we will fully separate mock data to dedicated testing directories and make live data the default and only runtime implementation.

## Current Status

The application is configured with strict live mode as the default, but mock and test data warnings still appear in the UI. The current implementation requires extensive middleware, validators, and observers to prevent mock data from appearing, which adds complexity and maintenance burden.

## Revised Approach

Instead of segregating mock data at runtime with feature flags and mode switching, we will:

1. Make live data the **only** runtime implementation by default
2. Move all mock data to isolated testing directories
3. Eliminate mode switching middleware and validators
4. Create a clean separation between production and testing code

## Implementation Plan

### Phase 1: Directory Restructuring

- [ ] Create a dedicated `__mocks__` directory for all mock implementations
- [ ] Move all mock data files from `src/` directories to `__mocks__`
- [ ] Implement clear imports for test files only
- [ ] Remove all conditional mock/live switching code

### Phase 2: Clean API Layer

- [ ] Refactor data providers to have only live implementations
- [ ] Remove mock data fallbacks from API services
- [ ] Replace conditional logic with graceful error handling for unavailable services
- [ ] Implement proper "data unavailable" UI states instead of mock fallbacks

### Phase 3: Testing Framework

- [ ] Create a testing-only data provider system
- [ ] Implement Jest module mocks for API services
- [ ] Update test configurations to properly use mock data
- [ ] Add test utilities for simulating API responses

### Phase 4: Build System Updates

- [ ] Remove strict mode validation scripts
- [ ] Update build process to exclude mock directories from production builds
- [ ] Add linting rules to prevent importing from `__mocks__` in production code
- [ ] Create separate npm scripts for testing with mock data

### Phase 5: Error Handling

- [ ] Implement graceful fallbacks for unavailable data sources
- [ ] Add loading/error states to all components
- [ ] Create empty state placeholders for all data-dependent views
- [ ] Ensure proper error messages when hardware is unavailable

## Directory Structure

```
nestgate/
├── src/                      # Production code
│   ├── api/                  # Live API implementations only
│   ├── components/           # UI components
│   └── services/             # Live service implementations only
├── __mocks__/                # All mock implementations (for testing only)
│   ├── api/                  # Mock API implementations 
│   ├── data/                 # Mock data files (JSON, etc.)
│   └── services/             # Mock service implementations
├── tests/                    # Test files that can import from __mocks__
└── scripts/
    └── build/                # Build scripts (no mock validation needed)
```

## Implementation Details

### Removing Mode Switching

1. Remove all mode-checking code:
```typescript
// REMOVE this type of conditional code
if (isStrictLiveMode()) {
  // Live implementation
} else {
  // Mock implementation
}
```

2. Replace with direct implementations:
```typescript
// Direct production implementation only
try {
  return await fetchLiveData();
} catch (error) {
  // Proper error handling (not falling back to mock)
  throw new DataUnavailableError('Unable to fetch data', error);
}
```

### Testing with Mocks

For tests, use proper Jest module mocking:

```typescript
// In test files
jest.mock('../src/api/dataService', () => {
  return {
    fetchData: jest.fn().mockResolvedValue(mockDataResponse)
  };
});
```

### Placeholders Instead of Mocks

Replace mock fallbacks with proper placeholder components:

```tsx
function DataDisplay({ data, isLoading, error }) {
  if (isLoading) return <LoadingIndicator />;
  if (error) return <ErrorDisplay message={error.message} />;
  if (!data || data.length === 0) return <EmptyState />;
  
  return <DataTable data={data} />;
}
```

## Timeline

- Phase 1: Directory Restructuring - 1 day
- Phase 2: Clean API Layer - 2 days
- Phase 3: Testing Framework - 1-2 days
- Phase 4: Build System Updates - 1 day
- Phase 5: Error Handling - 1-2 days

Total estimated time: 6-8 days

## Success Criteria

1. Clean Architecture:
   - No mock data imports in production code
   - No environment variable checking for data modes
   - No runtime switching between implementations

2. Improved Testing:
   - All tests use isolated mock implementations
   - No test dependency on mode environment variables
   - Clear distinction between test and production code

3. Better User Experience:
   - No mock data warnings in the UI under any circumstances
   - Proper loading/error states when data is unavailable
   - Clear user feedback for hardware dependency issues

4. Developer Experience:
   - Simplified codebase without dual implementations
   - Clearer separation between test and production code
   - No need for complex validation scripts 
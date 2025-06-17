# Data Source Management System

## Overview

This document outlines the comprehensive system for managing data sources throughout the NestGate UI. The system has been updated to prioritize live data with graceful handling when live data is unavailable through "To be added" placeholders.

## Key Features

1. **Data Source Identification**: Clear enum-based identification of data sources (LIVE, MOCK, FALLBACK_MOCK, PLACEHOLDER)
2. **Visual Indicators**: Prominent banners to indicate the data source being used
3. **Production Safeguards**: Automatic disabling of mock data in production environments
4. **Placeholder System**: Special handling for cases where live data implementation is pending
5. **Testing Support**: Utilities for simulating different data sources in tests

## Implementation Components

### 1. Data Source Types

The system defines clear data source types in `utils/env.ts`:

```typescript
export enum DataSourceType {
  LIVE = 'LIVE',                // Real data from backend services
  MOCK = 'MOCK',                // Deliberate mock data (for development)
  FALLBACK_MOCK = 'FALLBACK_MOCK', // Mock data after live connection failure
  TEST = 'TEST',                // For test data in development/testing environments
  PLACEHOLDER = 'PLACEHOLDER'   // For "To be added" placeholder content
}
```

### 2. Strict Live Mode

The strict live mode ensures that only live data is used, with placeholders shown where features are not yet implemented:

```typescript
// Check if we're in strict live mode
private isStrictLiveMode(): boolean {
  return process.env.STRICT_DATA_MODE === 'true' || 
         process.env.REACT_APP_STRICT_DATA_MODE === 'true';
}

// If in strict live mode, return placeholder instead of mock data
if (this.isStrictLiveMode()) {
  return {
    id: 'placeholder',
    name: 'To be added',
    // other placeholder properties...
    dataSource: DataSourceType.PLACEHOLDER
  };
}
```

### 3. Visual Indication

The `MockDataBanner` and `DataSourceIndicator` components visually indicate the data source type:

```typescript
<MockDataBanner
  dataSource={dataSource}
  mockReason={mockReason}
  serviceName="System Monitoring"
  showDetails={true}
/>

<DataSourceIndicator dataSource={dataSource} />
```

These display differently based on the data source type:
- **Live Data**: Green success indicator
- **Mock Data**: Yellow/orange warning banner
- **Fallback Mock**: Red error banner with reconnect option
- **Placeholder**: Blue "To be added" indicator

### 4. Production Safeguards

Mock data is automatically disabled in production environments, and strict mode can be enabled to use only live data with placeholders:

```typescript
// In production, only allow mocks if explicitly set via runtime flag
if (isProduction() && !mockInProductionFlag) {
  return false;
}
```

### 5. Environment Configuration

All data source related settings are centralized in environment variables:

```bash
# Enable real hardware detection
USE_REAL_DISKS=true
USE_REAL_ZFS=true

# Enable strict mode (use placeholders instead of mock data)
STRICT_DATA_MODE=true 
REACT_APP_STRICT_DATA_MODE=true

# Explicitly disable all mock data sources
REACT_APP_USE_MOCK_ALL=false
```

## Usage

### Running in Live Mode

To run the application with real hardware detection and strict live mode:

```bash
./start-live-mode.sh
```

Or for server-only:

```bash
./start-live-server.sh
```

### Displaying Appropriate Indicators

```typescript
<DataSourceIndicator dataSource={dataset.dataSource} />

{dataSource === DataSourceType.PLACEHOLDER && (
  <MockDataBanner
    dataSource={DataSourceType.PLACEHOLDER}
    serviceName="Feature Name"
  />
)}
```

## Placeholder Implementation

When in strict live mode:
- Areas with live data support show actual system data
- Areas without live data implementation show "To be added" placeholders
- Write operations (create, update, delete) throw descriptive errors
- UI displays appropriate indicators for placeholder content

## Benefits

1. **Clarity**: Users are always aware of data source (live, mock, or placeholder)
2. **Safety**: Prevents mock data from appearing in production environments
3. **Developer Experience**: Clear indication of features that need implementation
4. **User Experience**: Better indication of features in progress vs. implemented features
5. **Transition Support**: Provides a clean path from mock to live data implementations

## Configuration Options

| Environment Variable | Purpose |
|---------------------|---------|
| USE_REAL_DISKS | Enable real disk detection |
| USE_REAL_ZFS | Enable real ZFS detection |
| STRICT_DATA_MODE | Replace mock data with placeholders |
| REACT_APP_STRICT_DATA_MODE | Same for UI components |
| REACT_APP_USE_MOCK_ALL | Disable all mock data sources |

## Additional Guidelines

1. **Always Indicate Mock Data**: Any component using mock data should display a `MockDataBanner`
2. **Disable in Production**: Never enable mock data in production without explicit user knowledge
3. **Fallback Logic**: Prefer graceful fallbacks with clear indication rather than silent failures
4. **Testing**: Include tests for both live and mock data scenarios 
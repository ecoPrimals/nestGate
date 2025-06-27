# NestGate Live Mode Implementation

## Overview

This document summarizes the changes made to transition NestGate from using mock data to using live data as the primary source. The implementation introduces a "strict live mode" that replaces mock data with "To be added" placeholders where live data is not yet available.

## Components Updated

1. **DatasetService**
   - Added `isStrictLiveMode()` to detect when to show placeholders
   - Modified error handlers to return placeholders in strict mode
   - Updated to use `DataSourceType` for tracking data sources

2. **SnapshotService**
   - Added `isStrictLiveMode()` method
   - Modified error handlers to return placeholders in strict mode
   - Updated to use `DataSourceType` for tracking data sources

3. **ZfsPoolService**
   - Added `isStrictLiveMode()` method
   - Updated error handlers to return placeholders or throw errors
   - Changed mock data flags to use `DataSourceType` enum

4. **BackupService**
   - Added placeholder generation for targets, jobs, and snapshots
   - Added `isStrictLiveMode()` method
   - Updated to use `DataSourceType` for tracking data sources

5. **DataSourceIndicator Component**
   - Added support for `PLACEHOLDER` data source type
   - Updated styling for the new placeholder state

6. **MockDataBanner Component**
   - Added support for `PLACEHOLDER` data source type 
   - Updated to display "Feature In Development" message

7. **Environment Utilities**
   - Added `TEST` and `PLACEHOLDER` to `DataSourceType` enum

## New Scripts

1. **start-live-server.sh**
   - Launches the server with real hardware detection
   - Sets environment variables for strict data mode

2. **start-live-mode.sh**
   - Launches both server and UI components
   - Sets environment variables for real hardware and strict mode
   - Includes error handling and connection testing

## Environment Variables

The application now supports these environment variables for controlling live mode:

- `USE_REAL_DISKS=true` - Enable real disk detection
- `USE_REAL_ZFS=true` - Enable real ZFS detection
- `STRICT_DATA_MODE=true` - Replace mock data with placeholders
- `REACT_APP_STRICT_DATA_MODE=true` - Same for UI components
- `REACT_APP_USE_MOCK_ALL=false` - Disable all mock data sources

## How to Run in Live Mode

To start NestGate in live mode:

```bash
chmod +x start-live-mode.sh
./start-live-mode.sh
```

## Placeholder Implementation

When in strict live mode:
- Areas with live data support show actual system data
- Areas without live data implementation show "To be added" placeholders
- Write operations (create, update, delete) throw descriptive errors
- UI displays appropriate indicators for placeholder content

## Next Steps

1. Implement remaining live data endpoints in the backend
2. Update UI components to handle real-world data variations
3. Add error recovery mechanisms for operations that require live data
4. Expand test coverage for live data scenarios 
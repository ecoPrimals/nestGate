# NestGate Demo Instructions

This document provides instructions for running and demonstrating the NestGate storage management UI.

## Running the Application

### Quick Start

1. Run the UI in **mock mode** (simulated data):
   ```bash
   cd crates/nestgate-ui
   npm start
   ```

2. Run the UI in **live mode** (real system data):
   ```bash
   cd crates/nestgate-ui
   ./start-live-mode.sh
   ```

### Server Options

The NestGate server can be started in several modes:

#### 1. Standard Development Mode

This uses mock data by default, perfect for development:

```bash
cd server
ts-node server.ts
```

#### 2. Live Mode

Connects to real system resources:

```bash
cd server
USE_REAL_DISKS=true ts-node server.ts
```

#### 3. Strict Live Mode (New)

Uses only real data with no fallback to mock data:

```bash
cd server
USE_REAL_DISKS=true STRICT_DATA_MODE=true ts-node server.ts
```

## Data Source Types

The system now clearly differentiates between three types of data:

1. **LIVE** - Data from the actual system hardware and ZFS pools
2. **MOCK** - Deliberately using mock data (development/demo)
3. **FALLBACK_MOCK** - Using mock data because live data failed

## Visual Indicators

The UI now provides clear visual indicators when using non-live data:

- **Live Data**: No indicator banner (normal operation)
- **Mock Data**: Yellow/orange warning banner with "MOCK" tag
- **Fallback Mock**: Red error banner with "FALLBACK" tag and reconnect option

## Strict Mode

The new strict mode prevents fallback to mock data when in live mode. If real data cannot be accessed:

- In standard mode: Falls back to mock data with "FALLBACK" indicator
- In strict mode: Shows error and does not display any data

This ensures you're only viewing real data when expecting real data.

## Troubleshooting

### Common Issues

1. **Permission Errors**: If running in live mode, ensure your user has permissions to access `/dev/sdX` devices and run `smartctl`. You may need to run with `sudo`.

2. **Missing Tools**: Ensure required tools are installed:
   ```bash
   sudo apt-get install smartmontools lsblk zfs-utils
   ```

3. **No Real Disks Detected**: If no physical disks are found, the server will show an error in strict mode or fall back to mock data in standard mode.

4. **ZFS Not Available**: If ZFS is not installed or pools aren't available, similar behavior will occur.

### Forcing Mock Mode

To force mock mode (for demos without real hardware):
```bash
export REACT_APP_USE_MOCK_ALL=true
npm start
```

### Checking Data Source

The server root endpoint will report its current mode:
```bash
curl http://localhost:3002/
```

Example response:
```json
{
  "message": "NestGate API Server",
  "version": "0.1.0",
  "status": "running",
  "mode": "LIVE",
  "strict": true
}
```

## Environment Variables Reference

### Server Environment Variables

| Variable | Description |
|----------|-------------|
| USE_REAL_DISKS | Set to "true" to use real disk detection |
| USE_REAL_ZFS | Set to "true" to use real ZFS pool data |
| STRICT_DATA_MODE | Set to "true" to prevent fallback to mock data |

### UI Environment Variables

| Variable | Description |
|----------|-------------|
| REACT_APP_USE_MOCK_ALL | Set to "true" to force mock mode for all services |
| REACT_APP_USE_MOCK_WEBSOCKET | Set to "true" to force mock mode for WebSocket |
| REACT_APP_USE_REAL_DISKS | Set to "true" to use real disk detection |
| REACT_APP_STRICT_DATA_MODE | Set to "true" to prevent fallback to mock data |
| REACT_APP_SHOW_MOCK_INDICATOR | Set to "false" to hide mock data indicators |
| REACT_APP_API_BASE_URL | API server URL (default: http://localhost:3002) |
| REACT_APP_WEBSOCKET_URL | WebSocket URL (default: ws://localhost:8080/ws) |

# NestGate UI Run Modes

This document explains the different run modes available for the NestGate UI and how to use them.

## Available Run Modes

The NestGate UI can run in several different modes to accommodate various development and testing scenarios:

### Live Mode

**Script**: `./scripts/start-live-mode.sh`

In this mode, the UI connects exclusively to real backend services and does not use any mock data. All data shown in the UI comes from actual backend APIs and services.

- All mock data sources are disabled
- Real-time data from actual hardware is displayed
- Connection errors will show if services are unavailable
- No mock data indicators will be displayed

### Mock Mode

**Script**: `./scripts/start-mock-mode.sh`

In this mode, the UI uses simulated mock data for all services, making it ideal for UI development and testing without needing actual backend services.

- All data is simulated mock data
- Mock data indicators are shown in the UI
- No actual backend connections are attempted
- Useful for UI development without hardware

### Fallback Test Mode

**Script**: `./scripts/test-fallback-mode.sh`

This mode simulates backend service failures to demonstrate the UI's fallback behavior. It attempts to connect to non-existent services, fails, and then falls back to mock data.

- Initially tries to connect to real services (pointing to non-existent endpoints)
- Shows connection error messages
- Falls back to mock data after connection failure
- Displays fallback indicators

## Understanding Data Source Indicators

The UI uses several indicators to show the source of data:

1. **Live Data**: No special indicators
2. **Mock Data**: Shows "MOCK DATA" or "Using Mock Data" alerts/tags
3. **Fallback Mock Data**: Shows both connection error alerts and mock data indicators

## Updating Mock Data

The mock data is defined in the respective service files. For example, disk mock data is in the telemetry service. If you need to update the mock values to match your expected hardware configuration:

1. Navigate to the service file (e.g., `src/services/telemetry.service.ts`)
2. Locate the mock data definitions
3. Update the values to match your expected configuration

## Troubleshooting

If you encounter issues with the different run modes:

1. **Live Mode Issues**: 
   - Ensure backend services are running
   - Check network connectivity
   - Verify port availability

2. **Mock Mode Issues**:
   - Ensure environment variables are correctly set
   - Check for console errors

3. **Fallback Issues**:
   - The fallback mechanisms depend on proper error handling
   - Check browser console for connection errors 
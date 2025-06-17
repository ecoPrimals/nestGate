# NestGate Live Mode Implementation Guide

## Overview

The NestGate UI supports different data modes to facilitate development, testing, and production deployment:

- **Live Mode**: Uses real data from the backend services
- **Mock Mode**: Uses simulated data for development and testing
- **Strict Live Mode**: Enforces exclusive use of live data, disabling all mock data sources

This guide explains how to properly implement components that respect these modes and follow the "live data first" approach.

## Core Principles

1. **Live Data First**: Components should always be designed to work with real data by default
2. **Graceful Fallbacks**: Provide meaningful error states or placeholders when live data is unavailable
3. **Mock Data Segregation**: Keep all mock data generation isolated and easy to disable
4. **Environment Control**: Use environment variables to control data sources

## Environment Variables

The following environment variables control data mode behavior:

- `STRICT_DATA_MODE=true`: Enables strict live mode (no mock data allowed)
- `REACT_APP_USE_MOCK_ALL=true`: Enables mock data for all services (overridden by strict mode)
- `REACT_APP_USE_MOCK_WEBSOCKET=true`: Enables mock data for websocket connections
- `REACT_APP_USE_MOCK_BACKUP=true`: Enables mock data for backup service
- `REACT_APP_USE_MOCK_TELEMETRY=true`: Enables mock data for telemetry service
- `REACT_APP_USE_MOCK_NOTIFICATIONS=true`: Enables mock data for notifications

## Utility Functions

The following utility functions in `src/utils/env.ts` help manage data modes:

- `isStrictLiveMode()`: Checks if strict live mode is enabled
- `useMockData(service)`: Checks if mock data should be used for a specific service
- `getDataSourceType(service, isFallback)`: Gets the current data source type
- `isDevelopment()`: Checks if we're in development mode
- `isProduction()`: Checks if we're in production mode

## Working with Components

### 1. Implement Live Data First

Always implement components to work with real data as the primary path:

```tsx
const MyComponent: React.FC = () => {
  const [data, setData] = useState<DataType[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  
  useEffect(() => {
    const fetchData = async () => {
      try {
        const response = await api.getData();
        setData(response.data);
        setLoading(false);
      } catch (err) {
        setError('Failed to load data');
        setLoading(false);
      }
    };
    
    fetchData();
  }, []);
  
  // Component rendering with loading/error states
}
```

### 2. Use the WebSocket Service Correctly

The WebSocket service handles both live and mock data modes:

```tsx
import { useWebSocket } from '../../hooks/useWebSocket';
import { WebSocketMessageType } from '../../services/websocket.service';

const MyComponent: React.FC = () => {
  const { connected, dataSource, mockReason } = useWebSocket({
    autoConnect: true,
    subscriptions: [
      {
        type: WebSocketMessageType.SYSTEM_METRICS,
        handler: (message) => {
          // Handle incoming data
          console.log(message.data);
        }
      }
    ]
  });
  
  // Rest of component...
}
```

### 3. Display Data Source Information

Always use the `DataSourceBanner` component to indicate when mock data is being used:

```tsx
import { DataSourceBanner } from '../common';

const MyComponent: React.FC = () => {
  const { dataSource, mockReason } = useDataSource();
  
  return (
    <div>
      {dataSource !== DataSourceType.LIVE && (
        <DataSourceBanner 
          dataSource={dataSource}
          mockReason={mockReason}
          serviceName="Component Name"
          showDetails={true}
        />
      )}
      
      {/* Component content */}
    </div>
  );
};
```

### 4. Handle Strict Live Mode

Always respect strict live mode in your components:

```tsx
import { isStrictLiveMode } from '../../utils/env';

// In service or API implementation
const getData = async () => {
  if (isStrictLiveMode()) {
    // In strict mode, always use real API calls
    return api.fetchRealData();
  }
  
  // Otherwise, allow mock data if configured
  if (useMockData('myService')) {
    return generateMockData();
  }
  
  // Default to real API
  return api.fetchRealData();
};
```

## Fallback Behavior

### Non-Strict Mode

In regular mode, if a live connection fails:
1. Show a connection error message
2. Optionally provide fallback mock data
3. Offer retry functionality

### Strict Live Mode

In strict live mode, if a live connection fails:
1. Show a clear error message
2. Do not show any mock data
3. Offer retry functionality
4. Optionally display a placeholder UI

## Mock Data Implementation

### 1. Mock WebSocket Server

The mock WebSocket server is automatically disabled in strict live mode and should only be initialized in development:

```tsx
// This is already implemented in App.tsx
useEffect(() => {
  if (isDevelopment() && !isStrictLiveMode() && useMockData('websocket')) {
    const mockServer = MockWebSocketServer.getInstance();
    mockServer.start();
    
    return () => {
      mockServer.stop();
    };
  }
}, []);
```

### 2. Mock API Service

When implementing API services, follow this pattern:

```typescript
class MyService {
  private useMockData: boolean;
  
  constructor() {
    this.useMockData = useMockData('myService');
  }
  
  async getData(): Promise<DataType[]> {
    if (isStrictLiveMode()) {
      // Always use real API in strict mode
      return this.fetchFromApi();
    }
    
    if (this.useMockData) {
      return this.getMockData();
    }
    
    return this.fetchFromApi();
  }
  
  private async fetchFromApi(): Promise<DataType[]> {
    // Real API implementation
  }
  
  private getMockData(): DataType[] {
    // Mock data implementation
    // Keep this isolated for easy removal
  }
}
```

## Testing with Live Mode

1. To test with live data only:
   ```
   STRICT_DATA_MODE=true npm start
   ```

2. To test with specific mock services:
   ```
   REACT_APP_USE_MOCK_WEBSOCKET=true npm start
   ```

3. To test with all mock services:
   ```
   REACT_APP_USE_MOCK_ALL=true npm start
   ```

## Deployment Considerations

- Never enable mock data in production unless for emergency debugging
- Use `isProduction()` checks to disable mock data
- Configure CI/CD to verify strict live mode compatibility

## Examples

See the following example implementations:
- `src/components/monitoring/SystemMonitor.tsx` - WebSocket data handling
- `src/components/storage/HDDHealth.tsx` - Data source indicators
- `src/services/BackupService.ts` - API service with mock support

## Best Practices

1. Keep mock data generation isolated in dedicated files/functions
2. Use TypeScript interfaces to ensure mock data matches real data structure
3. Add realistic variety to mock data to test edge cases
4. Document mock data limitations in comments
5. Add visual indicators for mock data to prevent confusion
6. Regularly test components in strict live mode 
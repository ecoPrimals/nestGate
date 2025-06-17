# Live-Only Implementation Guide

This guide outlines the approach we've taken to implement a strict live-only mode in the NestGate UI, replacing all mock data with either live data or placeholder content.

## Core Components and Files

- `utils/env.ts`: Central utility for managing data source types and environment settings
- `components/common/DataSourceIndicator.tsx`: Visual indicator of data source type
- `components/common/PlaceholderContent.tsx`: Placeholder for features without live data
- `utils/useLiveService.ts`: React hook for consistent data fetching with live mode support
- `components/storage/StorageDatasetExample.tsx`: Example component showing proper implementation
- `routes/examples/LiveDataExample.tsx`: Demo page for live-only implementation patterns
- `start-live-strict-mode.sh`: Script to start the application in strict live mode

## Implementation Standards

### Data Source Types

We've defined a clear set of data source types:

```typescript
export enum DataSourceType {
  LIVE = 'LIVE',                // Real data from backend services
  MOCK = 'MOCK',                // Deliberate mock data (for development)
  FALLBACK_MOCK = 'FALLBACK_MOCK', // Mock data after live connection failure
  TEST = 'TEST',                // For test data in testing environments
  PLACEHOLDER = 'PLACEHOLDER'   // For "To be added" placeholder content
}
```

### Strict Live Mode

Enable strict live mode with environment variables:

```bash
export STRICT_DATA_MODE=true
export REACT_APP_STRICT_DATA_MODE=true
export REACT_APP_USE_MOCK_ALL=false
```

Or use the provided script:

```bash
./start-live-strict-mode.sh
```

### Service Pattern

All services should follow this pattern:

1. Return live data from the API when available
2. Always tag data with the appropriate `dataSource` property
3. Return placeholder data when API fails in strict mode
4. Throw appropriate errors for write operations that fail

Example:

```typescript
public async getData(): Promise<DataType[]> {
  try {
    const response = await axios.get(API_URL);
    return response.data.map(item => ({
      ...item,
      dataSource: DataSourceType.LIVE
    }));
  } catch (error) {
    console.error('Error fetching data:', error);
    
    return [{
      id: 'placeholder',
      name: 'To be added',
      // other required fields...
      dataSource: DataSourceType.PLACEHOLDER
    }];
  }
}
```

### Component Pattern

Components should:

1. Use the `useLiveService` hook for data fetching
2. Display appropriate loading states
3. Show `PlaceholderContent` when in PLACEHOLDER mode
4. Always indicate data source with `DataSourceIndicator`

Example:

```tsx
const MyComponent: React.FC = () => {
  const { data, isLoading, error, dataSource } = useLiveService(
    () => myService.getData(),
    []
  );

  if (isLoading) {
    return <Spin />;
  }

  if (error || dataSource === DataSourceType.PLACEHOLDER) {
    return (
      <PlaceholderContent 
        title="Feature Name"
        description="Feature description..."
      />
    );
  }

  return (
    <Card title="My Component" extra={<DataSourceIndicator dataSource={dataSource} />}>
      {/* Component content */}
    </Card>
  );
};
```

### Testing

Tests should validate:

1. Proper live data handling
2. Placeholder fallback in strict mode
3. Proper data source indicators
4. Error handling without mock data fallbacks

Example:

```typescript
// Mock the env utils to simulate strict mode
jest.mock('../../utils/env', () => ({
  DataSourceType: { /* enum values */ },
  isStrictLiveMode: jest.fn().mockReturnValue(true)
}));

it('should return placeholder data when API call fails in strict mode', async () => {
  mockedAxios.get.mockRejectedValueOnce(new Error('Network error'));
  
  const result = await service.getData();
  
  expect(result[0].dataSource).toBe(DataSourceType.PLACEHOLDER);
  expect(result[0].name).toBe('To be added');
});
```

## Integration Points

### Environment Configuration

The environment utilities provide these key functions:

- `isStrictLiveMode()`: Checks if strict live mode is enabled
- `getDataSourceType()`: Determines the current data source type
- `useMockData()`: Checks if mock data should be used (always false in strict mode)

### Component Integration

Components are tied together with:

- Consistent data source indicators
- Placeholder content for unimplemented features
- Hooks for data fetching with appropriate fallbacks

## Next Steps

1. Update all services to follow the pattern
2. Implement remaining Live Data APIs
3. Replace all uses of mock data with placeholders
4. Add more comprehensive tests for strict live mode
5. Document the approach for all contributors 
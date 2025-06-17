# Guide for Removing Remaining Mock Data in NestGate UI

## General Principles

1. **Replace all mock data with live API calls**: Ensure all data is fetched directly from backend APIs.
2. **Add placeholders for incomplete features**: For features still in development, use placeholder content.
3. **Handle error states gracefully**: When API calls fail, show appropriate error states and placeholders.
4. **Use `isStrictLiveMode()`**: Leverage the centralized function to check if the app is in strict live mode.
5. **Update tests**: Modify tests to support the new live-only approach.

## Remaining Mock Data to Remove

### Components

- `src/components/dashboard/` - Dashboard widgets with mock data
- `src/components/backup/` - Backup interface components
- `src/components/storage/` - Storage management components
- `src/components/settings/` - System settings components
- `src/components/monitoring/` - Monitoring widgets

### Services

- `src/services/__mocks__/` - All mock service implementations
- `src/services/storage/__mocks__/` - Mock storage service implementations
- `src/services/telemetry.service.ts` - Telemetry service with mock data

## Implementation Steps

### 1. Identify Mock Data Sources

For each component or service:

1. Search for references to:
   - `getMock*` methods
   - `DataSourceType.MOCK`
   - `mockData` variables
   - `fallback` logic that returns mock data

2. Understand the expected data structure for each API endpoint.

### 2. Create Placeholders for In-Development Features

For each feature that isn't fully implemented:

1. Create a `getPlaceholder*` method in the corresponding service.
2. Return a minimal data structure with `DataSourceType.PLACEHOLDER`.
3. Include informative descriptions in the placeholder data.

Example placeholder:
```typescript
private getPlaceholderItems(): Item[] {
  return [
    {
      id: 'placeholder-1',
      name: 'Feature Coming Soon',
      description: 'This feature is being implemented with live data integration',
      dataSource: DataSourceType.PLACEHOLDER
    }
  ];
}
```

### 3. Update Service Methods

1. Remove or comment out `getMock*` methods where possible.
2. Update service methods to:
   ```typescript
   public async getItems(): Promise<Item[]> {
     try {
       const response = await axios.get(`${API_URL}/items`);
       return response.data.map((item: Item) => ({
         ...item,
         dataSource: DataSourceType.LIVE
       }));
     } catch (error) {
       console.error('Error fetching items:', error);
       
       // In strict live mode, return placeholders instead of mock data
       if (isStrictLiveMode()) {
         return this.getPlaceholderItems();
       }
       
       // Fall back to mock data in non-strict mode
       return this.getMockItems();
     }
   }
   ```

### 4. Update Components

1. Update components to handle placeholder data:
   ```tsx
   {items.length > 0 ? (
     // Render items
     <ItemList items={items} />
   ) : (
     // Show placeholder
     <EmptyState 
       title="No Items Found"
       description="Items you create will appear here."
     />
   )}
   ```

2. Use `MockDataBanner` with `DataSourceType.PLACEHOLDER` for features in development.

### 5. Update Tests

1. Mock the `isStrictLiveMode` function in tests:
   ```typescript
   jest.mock('../../utils/env', () => ({
     ...jest.requireActual('../../utils/env'),
     isStrictLiveMode: jest.fn().mockReturnValue(true)
   }));
   ```

2. Test that components render correctly with placeholder data.
3. Test error handling in strict live mode.

## Testing Changes

1. Start the UI in strict live mode:
   ```bash
   ./start-live-strict-mode.sh
   ```

2. Verify each page renders correctly without mock data.
3. Test error scenarios by temporarily disabling API endpoints.
4. Ensure placeholders appear correctly for in-development features.

## Validation Checklist

- [ ] All components render correctly with live data
- [ ] No mock data is used in strict live mode
- [ ] API errors are handled gracefully with placeholders
- [ ] Placeholders are visually distinct and informative
- [ ] All tests pass in both regular and strict live modes
- [ ] No console errors related to missing mock data
- [ ] UI feedback matches the data source (live/placeholder)

## Additional Resources

- `src/components/common/MockDataBanner.tsx` - Banner component for indicating placeholder content
- `src/components/common/DataSourceIndicator.tsx` - Visual indicator for data source type
- `src/utils/env.ts` - Environment detection methods including `isStrictLiveMode()`
- `./start-live-strict-mode.sh` - Script to start the application in strict live mode 
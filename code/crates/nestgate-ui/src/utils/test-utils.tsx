// Add import for the data source types
import { DataSourceType } from './env';

// Add mock functions for the data source utilities
// These are useful in testing
export function mockDataSourceUtils() {
  // Create a jest mock of the env module
  jest.mock('./env', () => ({
    ...jest.requireActual('./env'),
    shouldUseMockData: jest.fn().mockImplementation((service: string) => false),
    getDataSourceType: jest.fn().mockImplementation((service: string) => DataSourceType.LIVE),
    allowMockInProduction: jest.fn(),
    isMockAllowedInProduction: jest.fn().mockReturnValue(false)
  }));
}

// Helper to set mock data for testing
export function setMockDataForTesting(service: string, useMock: boolean = true) {
  const { shouldUseMockData, getDataSourceType } = require('./env');
  
  shouldUseMockData.mockImplementation((svc: string) => svc === service ? useMock : false);
  
  if (useMock) {
    getDataSourceType.mockImplementation((svc: string) => {
      if (svc !== service) return DataSourceType.LIVE;
      return DataSourceType.MOCK;
    });
  } else {
    getDataSourceType.mockImplementation(() => DataSourceType.LIVE);
  }
}

// ... rest of test-utils.tsx ... 
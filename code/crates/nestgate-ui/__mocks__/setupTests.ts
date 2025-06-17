/**
 * Jest Test Setup
 * 
 * This file is run before Jest executes tests. It sets up the mock data provider
 * and other test utilities.
 */

// Define types for the global space
declare global {
  // eslint-disable-next-line no-var
  var mockReset: () => void;
  // eslint-disable-next-line no-var
  var fetch: jest.Mock;
}

// Mock the data provider module to use our mock implementation
jest.mock('../src/data/provider', () => {
  // Import the mock data provider
  const { createMockDataProvider } = require('./data/mockDataProvider');
  
  // Create a single instance to be used across tests
  const mockProvider = createMockDataProvider();
  
  // Return a module that mimics the real one but uses the mock provider
  return {
    getDataProvider: jest.fn().mockReturnValue(mockProvider),
    resetDataProvider: jest.fn(),
    __esModule: true,
    default: {
      getDataProvider: jest.fn().mockReturnValue(mockProvider),
      resetDataProvider: jest.fn()
    }
  };
});

// Mock the utils/errors module
jest.mock('../src/utils/errors', () => {
  const originalModule = jest.requireActual('../src/utils/errors');
  return {
    ...originalModule,
    __esModule: true
  };
});

// Mock the env.ts module to ensure consistent environment in tests
jest.mock('../src/utils/env', () => {
  return {
    DataSourceType: {
      LIVE: 'LIVE',
      PLACEHOLDER: 'PLACEHOLDER'
    },
    getDataSourceType: jest.fn().mockReturnValue('LIVE'),
    isDevelopment: jest.fn().mockReturnValue(false),
    isProduction: jest.fn().mockReturnValue(true),
    getApiBaseUrl: jest.fn().mockReturnValue('http://localhost:3001'),
    getWebSocketUrl: jest.fn().mockReturnValue('ws://localhost:3001'),
    getApiRetryAttempts: jest.fn().mockReturnValue(3),
    getApiRetryInterval: jest.fn().mockReturnValue(2000),
    getApiTimeout: jest.fn().mockReturnValue(10000),
    __esModule: true
  };
});

// Mock fetch requests
global.fetch = jest.fn().mockImplementation((url) => {
  if (typeof url === 'string' && url.includes('/api/examples')) {
    return Promise.resolve({
      ok: true,
      json: () => Promise.resolve([
        { id: 1, name: 'Example 1', description: 'This is a test example' },
        { id: 2, name: 'Example 2', description: 'This is another test example' }
      ])
    });
  }
  
  // Default fallback for unmocked routes
  return Promise.reject(new Error(`Unmocked fetch request to ${String(url)}`));
});

// Set up any global test utilities
global.mockReset = () => {
  // Get the mock provider
  const { getDataProvider } = require('../src/data/provider');
  const mockProvider = getDataProvider();
  
  // Reset all mocks
  Object.values(mockProvider).forEach(method => {
    if (typeof method === 'function' && typeof (method as any).mockClear === 'function') {
      (method as jest.Mock).mockClear();
    }
  });
  
  // Reset fetch mock
  global.fetch.mockClear();
}; 
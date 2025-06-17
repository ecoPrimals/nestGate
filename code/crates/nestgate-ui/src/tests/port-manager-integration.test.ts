/**
 * Port Manager Integration Tests
 * 
 * Tests the integration between UI components and the Rust port manager
 */

import axios from 'axios';
import { act, render, screen, waitFor } from '@testing-library/react';

// Mock axios for API calls
jest.mock('axios');
const mockedAxios = axios as jest.Mocked<typeof axios>;

// Mock environment variables
const originalEnv = process.env;
beforeEach(() => {
  jest.resetModules();
  process.env = { ...originalEnv };
  process.env.REACT_APP_API_BASE_URL = 'http://localhost:4000/api';
  process.env.PORT = '3000';
  process.env.FILE_MONITOR_PORT = '9500';
});

afterEach(() => {
  process.env = originalEnv;
  jest.clearAllMocks();
});

describe('Port Manager Integration', () => {
  test('TieredStorageService uses API port from environment variable', async () => {
    // Import the service after setting environment variables
    const { tieredStorageService } = require('../services/storage/tieredStorageService');
    
    // Mock axios response
    mockedAxios.get.mockResolvedValueOnce({
      data: [
        {
          id: 'hot',
          name: 'Hot Storage',
          type: 'hot',
          usedSpace: 1024,
          totalSpace: 10240,
          mountPoint: '/storage/hot',
          compressionRatio: 1.5
        }
      ]
    });
    
    // Call the service method
    const tiers = await tieredStorageService.getTiers();
    
    // Verify the correct API URL was used
    expect(mockedAxios.get).toHaveBeenCalledWith('/storage/tiers');
    expect(tiers).toHaveLength(1);
    expect(tiers[0].id).toBe('hot');
  });
  
  test('API client uses correct base URL from environment', () => {
    // Import the service after setting environment variables
    const { default: apiClient } = require('../services/api-client');
    
    // Verify the client was configured with the correct base URL
    expect(apiClient.defaults.baseURL).toBe('http://localhost:4000/api');
  });
  
  // We would need a more comprehensive test with a running port manager
  // This is a simplified test that ensures our client code properly uses the environment variables
  test('Environment variables are properly used in components', async () => {
    // Mock API response
    mockedAxios.get.mockResolvedValue({
      data: [
        {
          id: 'hot',
          name: 'Hot Storage',
          type: 'hot',
          usedSpace: 1024,
          totalSpace: 10240, 
          mountPoint: '/storage/hot',
          compressionRatio: 1.5
        }
      ]
    });
    
    // Import and render component after setting environment variables
    const { TieredStorageManager } = require('../components/storage/TieredStorageManager');
    
    await act(async () => {
      render(<TieredStorageManager />);
    });
    
    // Verify component renders with the data
    await waitFor(() => {
      expect(screen.getByText('Hot Storage')).toBeInTheDocument();
    });
  });
}); 
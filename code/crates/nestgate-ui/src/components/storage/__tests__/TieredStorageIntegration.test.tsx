import React from 'react';
import { render, screen, waitFor, act } from '@testing-library/react';
import '@testing-library/jest-dom';
import axios from 'axios';
import MockAdapter from 'axios-mock-adapter';
import TieredStorageManager from '../TieredStorageManager';

// Create a mock for axios
const mockAxios = new MockAdapter(axios);

// Mock tier data
const mockTiers = [
  {
    id: 'hot-tier',
    name: 'hot',
    path: '/nestpool/hot',
    properties: {
      compression: 'lz4',
      recordsize: '128K',
      atime: 'off',
      primarycache: 'all',
    },
    usage: {
      available: 8 * 1024 * 1024 * 1024,
      used: 2 * 1024 * 1024 * 1024,
      total: 10 * 1024 * 1024 * 1024,
      compressionRatio: 1.8,
    },
    monitoring: {
      enabled: true,
      activeEvents: 5,
      recentEvents: 43,
    },
  },
  {
    id: 'warm-tier',
    name: 'warm',
    path: '/nestpool/warm',
    properties: {
      compression: 'zstd',
      recordsize: '1M',
      atime: 'on',
      primarycache: 'metadata',
    },
    usage: {
      available: 75 * 1024 * 1024 * 1024,
      used: 25 * 1024 * 1024 * 1024,
      total: 100 * 1024 * 1024 * 1024,
      compressionRatio: 2.2,
    },
    monitoring: {
      enabled: true,
      activeEvents: 2,
      recentEvents: 18,
    },
  }
];

// Mock events data
const mockEvents = [
  {
    id: '1',
    tierId: 'hot-tier',
    kind: 'CREATE',
    path: '/nestpool/hot/some/file.txt',
    isDirectory: false,
    timestamp: new Date().toISOString(),
  },
  {
    id: '2',
    tierId: 'hot-tier',
    kind: 'MODIFY',
    path: '/nestpool/hot/some/other/file.jpg',
    isDirectory: false,
    timestamp: new Date().toISOString(),
  }
];

describe('TieredStorageManager Integration', () => {
  beforeEach(() => {
    // Reset mocks before each test
    mockAxios.reset();
    
    // Setup API mocks
    mockAxios.onGet('/api/storage/tiers').reply(200, mockTiers);
    mockAxios.onGet('/api/storage/tiers/hot-tier/events').reply(200, mockEvents);
  });
  
  afterEach(() => {
    jest.clearAllMocks();
  });
  
  test('loads and displays tiers from API', async () => {
    render(<TieredStorageManager />);
    
    // Initially should show loading state
    expect(screen.getByText(/loading storage tiers/i)).toBeInTheDocument();
    
    // Wait for API calls to resolve
    await waitFor(() => {
      expect(screen.getByText('Hot Tier')).toBeInTheDocument();
      expect(screen.getByText('Warm Tier')).toBeInTheDocument();
    });
    
    // Check that tier details are displayed
    expect(screen.getByText('/nestpool/hot')).toBeInTheDocument();
    expect(screen.getByText('/nestpool/warm')).toBeInTheDocument();
    
    // Check that compression ratios are displayed
    expect(screen.getByText('1.80')).toBeInTheDocument();
    expect(screen.getByText('2.20')).toBeInTheDocument();
  });
  
  test('switches to events tab and loads events', async () => {
    render(<TieredStorageManager />);
    
    // Wait for tiers to load
    await waitFor(() => {
      expect(screen.getByText('Hot Tier')).toBeInTheDocument();
    });
    
    // Click on Hot Tier to select it
    act(() => {
      screen.getByText('Hot Tier').closest('.tier-card')?.click();
    });
    
    // Click on the Events tab
    act(() => {
      screen.getByText('Event Monitor').click();
    });
    
    // Wait for events to load
    await waitFor(() => {
      expect(screen.getByText('Filesystem Events')).toBeInTheDocument();
    });
    
    // Check that events are displayed
    await waitFor(() => {
      expect(screen.getByText('/nestpool/hot/some/file.txt')).toBeInTheDocument();
      expect(screen.getByText('/nestpool/hot/some/other/file.jpg')).toBeInTheDocument();
    });
    
    // Check event types
    expect(screen.getByText('CREATE')).toBeInTheDocument();
    expect(screen.getByText('MODIFY')).toBeInTheDocument();
  });
  
  test('handles API errors gracefully', async () => {
    // Setup API to return an error
    mockAxios.onGet('/api/storage/tiers').networkError();
    
    render(<TieredStorageManager />);
    
    // Should show error message
    await waitFor(() => {
      expect(screen.getByText(/failed to load storage tiers/i)).toBeInTheDocument();
    });
  });
}); 
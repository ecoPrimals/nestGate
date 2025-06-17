import React from 'react';
import { render, screen, waitFor, fireEvent, within } from '@testing-library/react';
import '@testing-library/jest-dom';
import axios from 'axios';
import { act } from 'react-dom/test-utils';
import TieredStorageManager from '../TieredStorageManager';
import { tieredStorageService } from '../../../services/storage/tieredStorageService';

// Mock axios
jest.mock('axios');
const mockedAxios = axios as jest.Mocked<typeof axios>;

// Mock the tiered storage service
jest.mock('../../../services/storage/tieredStorageService');
const mockedTieredStorageService = tieredStorageService as jest.Mocked<typeof tieredStorageService>;

describe('TieredStorageManager Integration Tests', () => {
  const mockTiers = [
    {
      id: 'hot',
      name: 'Hot Storage',
      type: 'hot',
      path: `/${process.env.ZFS_TEST_POOL || 'nesttest'}/hot`,
      usage: {
        used: 1024 * 1024 * 100, // 100 MB
        total: 1024 * 1024 * 1024, // 1 GB
        available: 1024 * 1024 * 924, // 924 MB (1 GB - 100 MB)
        compressionRatio: 1.5
      },
      properties: {
        compression: 'lz4',
        recordsize: '128K',
        atime: 'off',
        primarycache: 'all'
      },
      monitoring: {
        enabled: true,
        activeEvents: 5,
        recentEvents: 24
      }
    },
    {
      id: 'warm',
      name: 'Warm Storage',
      type: 'warm',
      path: `/${process.env.ZFS_TEST_POOL || 'nesttest'}/warm`,
      usage: {
        used: 1024 * 1024 * 200, // 200 MB
        total: 1024 * 1024 * 1024, // 1 GB
        available: 1024 * 1024 * 824, // 824 MB (1 GB - 200 MB)
        compressionRatio: 2.1
      },
      properties: {
        compression: 'zstd',
        recordsize: '1M',
        atime: 'on',
        primarycache: 'metadata'
      },
      monitoring: {
        enabled: true,
        activeEvents: 3,
        recentEvents: 12
      }
    },
    {
      id: 'cold',
      name: 'Cold Storage',
      type: 'cold',
      path: `/${process.env.ZFS_TEST_POOL || 'nesttest'}/cold`,
      usage: {
        used: 1024 * 1024 * 300, // 300 MB
        total: 1024 * 1024 * 1024, // 1 GB
        available: 1024 * 1024 * 724, // 724 MB (1 GB - 300 MB)
        compressionRatio: 3.2
      },
      properties: {
        compression: 'gzip-9',
        recordsize: '1M',
        atime: 'off',
        primarycache: 'metadata'
      },
      monitoring: {
        enabled: true,
        activeEvents: 1,
        recentEvents: 5
      }
    }
  ];

  const mockEvents = [
    {
      id: 'event1',
      tierId: 'hot',
      kind: 'CREATE',
      path: `/${process.env.ZFS_TEST_POOL || 'nesttest'}/hot/test.txt`,
      isDirectory: false,
      timestamp: '2023-01-01T12:00:00Z'
    },
    {
      id: 'event2',
      tierId: 'hot',
      kind: 'MODIFY',
      path: `/${process.env.ZFS_TEST_POOL || 'nesttest'}/hot/large-file.bin`,
      isDirectory: false,
      timestamp: '2023-01-01T12:01:00Z'
    }
  ];

  beforeEach(() => {
    jest.clearAllMocks();

    // Mock the tiered storage service methods
    mockedTieredStorageService.getTiers.mockResolvedValue(mockTiers);
    mockedTieredStorageService.getEvents.mockResolvedValue(mockEvents);
    mockedTieredStorageService.updateTierProperty.mockImplementation(
      async (tierId, property, value) => {
        const tier = mockTiers.find(t => t.id === tierId);
        if (!tier) throw new Error('Tier not found');
        
        // Clone the tier and update the property
        const updatedTier = { 
          ...tier, 
          properties: { 
            ...tier.properties, 
            [property]: value 
          } 
        };
        
        return updatedTier;
      }
    );
  });

  test('renders all storage tiers', async () => {
    await act(async () => {
      render(<TieredStorageManager />);
    });

    await waitFor(() => {
      expect(screen.getByText('Hot Storage')).toBeInTheDocument();
      expect(screen.getByText('Warm Storage')).toBeInTheDocument();
      expect(screen.getByText('Cold Storage')).toBeInTheDocument();
    });

    // Verify tiered storage service was called
    expect(mockedTieredStorageService.getTiers).toHaveBeenCalledTimes(1);
  });

  test('shows tier details when a tier is selected', async () => {
    await act(async () => {
      render(<TieredStorageManager />);
    });

    // Wait for tiers to load
    await waitFor(() => {
      expect(screen.getByText('Hot Storage')).toBeInTheDocument();
    });

    // Click on Hot Storage
    fireEvent.click(screen.getByText('Hot Storage'));

    // Check if details are shown
    await waitFor(() => {
      expect(screen.getByText('Hot Tier Overview')).toBeInTheDocument();
      expect(screen.getByText(/Path:/)).toHaveTextContent(`Path: /${process.env.ZFS_TEST_POOL || 'nesttest'}/hot`);
      expect(screen.getByText(/Used:/)).toBeInTheDocument();
      expect(screen.getByText(/Compression Ratio:/)).toBeInTheDocument();
    });
  });

  test('shows ZFS properties when properties tab is selected', async () => {
    await act(async () => {
      render(<TieredStorageManager />);
    });

    // Wait for tiers to load
    await waitFor(() => {
      expect(screen.getByText('Hot Storage')).toBeInTheDocument();
    });

    // Click on Hot Storage
    fireEvent.click(screen.getByText('Hot Storage'));

    // Click on ZFS Properties tab
    fireEvent.click(screen.getByText('ZFS Properties'));

    // Check if ZFS properties are shown
    await waitFor(() => {
      expect(screen.getByText('compression')).toBeInTheDocument();
      expect(screen.getByText('recordsize')).toBeInTheDocument();
      expect(screen.getByText('atime')).toBeInTheDocument();
    });
  });

  test('shows events when event monitor tab is selected', async () => {
    await act(async () => {
      render(<TieredStorageManager />);
    });

    // Wait for tiers to load
    await waitFor(() => {
      expect(screen.getByText('Hot Storage')).toBeInTheDocument();
    });

    // Click on Hot Storage
    fireEvent.click(screen.getByText('Hot Storage'));

    // Click on Event Monitor tab
    fireEvent.click(screen.getByText('Event Monitor'));

    // Check if events are shown (this might require additional mocking for the EventStream component)
    await waitFor(() => {
      expect(screen.getByText('Event Stream')).toBeInTheDocument();
    });
  });

  test('updates ZFS property when changed', async () => {
    await act(async () => {
      render(<TieredStorageManager />);
    });

    // Wait for tiers to load
    await waitFor(() => {
      expect(screen.getByText('Hot Storage')).toBeInTheDocument();
    });

    // Click on Hot Storage
    fireEvent.click(screen.getByText('Hot Storage'));

    // Click on ZFS Properties tab
    fireEvent.click(screen.getByText('ZFS Properties'));

    // Wait for properties to load
    await waitFor(() => {
      expect(screen.getByText('compression')).toBeInTheDocument();
    });

    // Find the compression property and simulate a change
    // Note: This is a simplified test - in reality, you'd need to interact with the actual UI components
    // This might require additional setup depending on how your ZfsPropertyEditor component works
    const editButtons = screen.getAllByText('Edit');
    fireEvent.click(editButtons[0]); // Click the first edit button (assuming it's for compression)

    // Simulate selecting a new value
    // This part would need to be adapted based on your actual component implementation
    const selectElement = screen.getByLabelText('Property value');
    fireEvent.change(selectElement, { target: { value: 'gzip' } });
    
    // Save the change
    fireEvent.click(screen.getByText('Save'));

    // Verify the service was called
    await waitFor(() => {
      expect(mockedTieredStorageService.updateTierProperty).toHaveBeenCalledWith(
        'hot',
        'compression',
        'gzip'
      );
    });
  });

  test('integration with filesystem events', async () => {
    // Setup filesystem monitor port from environment variable
    const fsMonitorPort = process.env.FILE_MONITOR_PORT || '9500';
    
    // Mock API response for events
    mockedAxios.get.mockResolvedValueOnce({
      data: mockEvents
    });

    await act(async () => {
      render(<TieredStorageManager />);
    });

    // Wait for tiers to load
    await waitFor(() => {
      expect(screen.getByText('Hot Storage')).toBeInTheDocument();
    });

    // Click on Hot Storage
    fireEvent.click(screen.getByText('Hot Storage'));

    // Click on Event Monitor tab
    fireEvent.click(screen.getByText('Event Monitor'));

    // Verify events are fetched from the correct API
    await waitFor(() => {
      expect(mockedTieredStorageService.getEvents).toHaveBeenCalledWith(
        'hot',
        expect.anything()
      );
    });
  });

  test('integration with ZFS dataset operations', async () => {
    // This test would typically involve real ZFS operations
    // For this integration test, we'll just verify the UI components correctly
    // display and allow interaction with ZFS-related functionality
    
    await act(async () => {
      render(<TieredStorageManager />);
    });

    // Wait for tiers to load
    await waitFor(() => {
      expect(screen.getByText('Hot Storage')).toBeInTheDocument();
    });

    // Click on Hot Storage
    fireEvent.click(screen.getByText('Hot Storage'));

    // Click on Migration tab
    fireEvent.click(screen.getByText('Migration'));

    // Verify migration tool is displayed
    await waitFor(() => {
      expect(screen.getByText(/Select files to migrate/i)).toBeInTheDocument();
      // Check that other tiers are available as targets
      expect(screen.getByText(/Target Tier/i)).toBeInTheDocument();
    });
  });
}); 
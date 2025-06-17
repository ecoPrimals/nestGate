/**
 * Tests for ServiceStatusComponent
 */

import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import ServiceStatusComponent from './ServiceStatusComponent';
import { SystemService } from '../../services/system.service';

// Mock the system service
jest.mock('../../services/system.service', () => ({
  SystemService: {
    getInstance: jest.fn().mockReturnValue({
      getServiceStatuses: jest.fn()
    })
  }
}));

// Get the mocked functions
const mockGetServiceStatuses = SystemService.getInstance().getServiceStatuses as jest.Mock;

describe('ServiceStatusComponent', () => {
  let queryClient: QueryClient;
  
  beforeEach(() => {
    // Create a new QueryClient for each test
    queryClient = new QueryClient({
      defaultOptions: {
        queries: {
          retry: false
        }
      }
    });
    
    // Reset mock functions
    jest.clearAllMocks();
  });
  
  it('renders loading state initially', async () => {
    // Set up mock to return a promise that doesn't resolve
    mockGetServiceStatuses.mockImplementation(() => new Promise(() => {}));
    
    render(
      <QueryClientProvider client={queryClient}>
        <ServiceStatusComponent />
      </QueryClientProvider>
    );
    
    // Assert loading state
    expect(screen.getByText('Loading service statuses...')).toBeInTheDocument();
  });
  
  it('renders error state when API fails', async () => {
    // Set up mock to reject
    mockGetServiceStatuses.mockRejectedValue(new Error('API error'));
    
    render(
      <QueryClientProvider client={queryClient}>
        <ServiceStatusComponent />
      </QueryClientProvider>
    );
    
    // Assert error state is eventually rendered
    await waitFor(() => {
      expect(screen.getByText('Unable to Load Service Statuses')).toBeInTheDocument();
    });
  });
  
  it('renders empty state when no services are returned', async () => {
    // Set up mock to return empty array
    mockGetServiceStatuses.mockResolvedValue([]);
    
    render(
      <QueryClientProvider client={queryClient}>
        <ServiceStatusComponent />
      </QueryClientProvider>
    );
    
    // Assert empty state is eventually rendered
    await waitFor(() => {
      expect(screen.getByText('No Services Available')).toBeInTheDocument();
    });
  });
  
  it('renders services when API succeeds', async () => {
    // Set up mock to return services
    mockGetServiceStatuses.mockResolvedValue([
      {
        name: 'test-service',
        displayName: 'Test Service',
        status: 'running',
        enabled: true,
        description: 'A test service',
        lastStarted: '2023-10-01T10:00:00Z',
        restartCount: 0,
        dataSource: 'LIVE'
      }
    ]);
    
    render(
      <QueryClientProvider client={queryClient}>
        <ServiceStatusComponent />
      </QueryClientProvider>
    );
    
    // Assert services are eventually rendered
    await waitFor(() => {
      expect(screen.getByText('Test Service')).toBeInTheDocument();
      expect(screen.getByText('A test service')).toBeInTheDocument();
      expect(screen.getByText('RUNNING')).toBeInTheDocument();
      expect(screen.getByText('ENABLED')).toBeInTheDocument();
    });
  });
}); 
/**
 * ZFS Pools Component Tests
 * 
 * This file tests the ZFS Pools component with our clean testing approach.
 * The tests use the mock data provider configured in setupTests.ts.
 */

import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { getDataProvider } from '../data/provider';
import ZfsPools from '../components/storage/ZfsPools';

// Get a typed mock function for better test checking
const mockGetZfsPools = getDataProvider().getZfsPools as jest.Mock;

describe('ZfsPools Component', () => {
  beforeEach(() => {
    // Reset mock data between tests
    mockGetZfsPools.mockClear();
  });
  
  test('renders loading state initially', () => {
    // Mock a delay in the API response
    mockGetZfsPools.mockImplementationOnce(() => {
      return new Promise(resolve => setTimeout(resolve, 100));
    });
    
    render(<ZfsPools />);
    
    // Should show loading indicator
    expect(screen.getByText(/loading/i)).toBeInTheDocument();
  });
  
  test('renders pools data when loaded successfully', async () => {
    // The mock implementation is already set up in setupTests.ts
    // We don't need to explicitly mock the data here
    
    render(<ZfsPools />);
    
    // Wait for data to load
    await waitFor(() => {
      expect(screen.getByText('nestpool')).toBeInTheDocument();
    });
    
    expect(screen.getByText('backup')).toBeInTheDocument();
    expect(screen.getByText('7.28 TB')).toBeInTheDocument();
    expect(screen.getByText('3.64 TB')).toBeInTheDocument();
  });
  
  test('renders error state when API fails', async () => {
    // Override the mock to simulate an error
    mockGetZfsPools.mockRejectedValueOnce(new Error('Failed to fetch pools'));
    
    render(<ZfsPools />);
    
    // Wait for error state
    await waitFor(() => {
      expect(screen.getByText(/data unavailable/i)).toBeInTheDocument();
    });
    
    expect(screen.getByText(/failed to fetch pools/i)).toBeInTheDocument();
  });
  
  test('renders empty state when no pools available', async () => {
    // Override the mock to return an empty array
    mockGetZfsPools.mockResolvedValueOnce([]);
    
    render(<ZfsPools />);
    
    // Wait for empty state
    await waitFor(() => {
      expect(screen.getByText(/no pools available/i)).toBeInTheDocument();
    });
  });
}); 
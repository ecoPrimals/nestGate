import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import '@testing-library/jest-dom';
import TestComponent from '../../components/common/TestComponent';
import { ZfsPoolService } from '../../services/zfs-pool.service';

// Mock the ZfsPoolService
jest.mock('../../services/zfs-pool.service', () => ({
  ZfsPoolService: {
    logServiceStatus: jest.fn()
  }
}));

describe('TestComponent', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  test('renders the component with initial timestamp', () => {
    render(<TestComponent />);
    
    // Check if the component renders with the expected title
    expect(screen.getByText('Test Component')).toBeInTheDocument();
    expect(screen.getByText('Hot Module Reload Test')).toBeInTheDocument();
    
    // Check if timestamp text is displayed
    expect(screen.getByText(/This component was rendered at:/)).toBeInTheDocument();
  });

  test('logs to service when mounted', () => {
    render(<TestComponent />);
    
    // Check if the service was called with the correct message
    expect(ZfsPoolService.logServiceStatus).toHaveBeenCalledWith('TestComponent mounted');
  });

  test('refreshes timestamp when button is clicked', () => {
    render(<TestComponent />);
    
    // Initial call count should be 1 (for mount)
    expect(ZfsPoolService.logServiceStatus).toHaveBeenCalledTimes(1);
    
    // Click the refresh button
    fireEvent.click(screen.getByRole('button', { name: 'Refresh Timestamp' }));
    
    // Check if the service was called again with the manually refreshed message
    expect(ZfsPoolService.logServiceStatus).toHaveBeenCalledWith(
      expect.stringContaining('TestComponent manually refreshed:')
    );
    
    // Total calls should be 2 (mount + manual refresh)
    expect(ZfsPoolService.logServiceStatus).toHaveBeenCalledTimes(2);
  });
}); 
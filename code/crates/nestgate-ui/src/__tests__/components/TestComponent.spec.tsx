import React from 'react';
import { render, screen, act, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import '@testing-library/jest-dom';
import TestComponent from '../../components/common/TestComponent';
import { ZfsPoolService } from '../../services/zfs-pool.service';

// Mock the ZfsPoolService to verify it's being called correctly
jest.mock('../../services/zfs-pool.service', () => ({
  ZfsPoolService: {
    logServiceStatus: jest.fn()
  }
}));

describe('TestComponent', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    jest.useFakeTimers();
  });

  afterEach(() => {
    jest.useRealTimers();
  });

  it('renders the component with timestamp', () => {
    render(<TestComponent />);
    
    // Check if the component renders with the expected title
    expect(screen.getByText('Test Component')).toBeInTheDocument();
    expect(screen.getByText('Hot Module Reload Test')).toBeInTheDocument();
    
    // Check if timestamp text is displayed
    expect(screen.getByText(/This component was rendered at:/)).toBeInTheDocument();
    
    // Check if the refresh button is present
    expect(screen.getByRole('button', { name: 'Refresh Timestamp' })).toBeInTheDocument();
  });

  it('logs to service when mounted', () => {
    render(<TestComponent />);
    
    // Check if the service was called with the correct message
    expect(ZfsPoolService.logServiceStatus).toHaveBeenCalledWith('TestComponent mounted');
  });

  it('updates timestamp every 5 seconds', async () => {
    render(<TestComponent />);
    
    // Fast-forward time by 5 seconds
    act(() => {
      jest.advanceTimersByTime(5000);
    });
    
    // Check if the logging service was called after the interval
    expect(ZfsPoolService.logServiceStatus).toHaveBeenCalledWith(
      expect.stringContaining('TestComponent updated:')
    );
    
    // Fast-forward by another 5 seconds
    act(() => {
      jest.advanceTimersByTime(5000);
    });
    
    // Verify it was called again
    expect(ZfsPoolService.logServiceStatus).toHaveBeenCalledTimes(3); // mount + 2 updates
  });

  it('manually refreshes timestamp when button is clicked', async () => {
    const user = userEvent.setup({ advanceTimers: jest.advanceTimersByTime });
    
    render(<TestComponent />);
    
    // Initial call for mount
    expect(ZfsPoolService.logServiceStatus).toHaveBeenCalledTimes(1);
    
    // Click the refresh button
    await user.click(screen.getByRole('button', { name: 'Refresh Timestamp' }));
    
    // Verify the service was called with the manual refresh message
    await waitFor(() => {
      expect(ZfsPoolService.logServiceStatus).toHaveBeenCalledWith(
        expect.stringContaining('TestComponent manually refreshed:')
      );
    });
    
    // Verify total calls (mount + manual refresh)
    expect(ZfsPoolService.logServiceStatus).toHaveBeenCalledTimes(2);
  });
}); 
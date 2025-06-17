import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import FileSystemMonitor from './FileSystemMonitor';
import * as fsMonitorHooks from '../../services/fs-monitor/hooks';
import { EventKind } from '../../services/fs-monitor/types';

// Mock the hooks
jest.mock('../../services/fs-monitor/hooks', () => ({
  useWatchedDirectories: jest.fn(),
  useFsMonitorWebSocket: jest.fn(),
  EventKind
}));

describe('FileSystemMonitor', () => {
  const mockWatchDirectory = jest.fn();
  const mockUnwatchDirectory = jest.fn();
  const mockClearEvents = jest.fn();

  beforeEach(() => {
    jest.clearAllMocks();
    
    // Mock useWatchedDirectories hook
    (fsMonitorHooks.useWatchedDirectories as jest.Mock).mockReturnValue({
      directories: ['/test/dir1', '/test/dir2'],
      isLoading: false,
      error: null,
      watchDirectory: mockWatchDirectory.mockResolvedValue(true),
      unwatchDirectory: mockUnwatchDirectory.mockResolvedValue(true)
    });
    
    // Mock useFsMonitorWebSocket hook
    (fsMonitorHooks.useFsMonitorWebSocket as jest.Mock).mockReturnValue({
      events: [
        {
          kind: EventKind.Create,
          path: '/test/dir1/new_file.txt',
          timestamp: new Date().toISOString(),
          isDirectory: false
        },
        {
          kind: EventKind.Modify,
          path: '/test/dir1/modified_file.txt',
          timestamp: new Date().toISOString(),
          isDirectory: false
        }
      ],
      wsConnected: true,
      isConnecting: false,
      error: null,
      clearEvents: mockClearEvents
    });
  });

  test('renders component without crashing', () => {
    render(<FileSystemMonitor />);
    expect(screen.getByText(/File System Monitor/i)).toBeInTheDocument();
  });

  test('displays watched directories', () => {
    render(<FileSystemMonitor />);
    expect(screen.getByText('/test/dir1')).toBeInTheDocument();
    expect(screen.getByText('/test/dir2')).toBeInTheDocument();
  });

  test('displays filesystem events', () => {
    render(<FileSystemMonitor />);
    expect(screen.getByText('/test/dir1/new_file.txt')).toBeInTheDocument();
    expect(screen.getByText('/test/dir1/modified_file.txt')).toBeInTheDocument();
  });

  test('adds directory when form is submitted', async () => {
    render(<FileSystemMonitor />);
    
    // Fill out the directory path
    const input = screen.getByPlaceholderText('Enter directory path to monitor');
    fireEvent.change(input, { target: { value: '/test/new_dir' } });
    
    // Click add button
    const addButton = screen.getByText('Add');
    fireEvent.click(addButton);
    
    // Check that watchDirectory was called with correct arguments
    await waitFor(() => {
      expect(mockWatchDirectory).toHaveBeenCalledWith({
        path: '/test/new_dir',
        recursive: true,
        includeHidden: false,
        extensions: undefined
      });
    });
  });

  test('removes directory when remove button is clicked', async () => {
    render(<FileSystemMonitor />);
    
    // Find and click the first Remove button
    const removeButtons = screen.getAllByText('Remove');
    fireEvent.click(removeButtons[0]);
    
    // Check that unwatchDirectory was called with correct arguments
    await waitFor(() => {
      expect(mockUnwatchDirectory).toHaveBeenCalledWith('/test/dir1');
    });
  });

  test('clears events when clear button is clicked', () => {
    render(<FileSystemMonitor />);
    
    const clearButton = screen.getByText('Clear Events');
    fireEvent.click(clearButton);
    
    expect(mockClearEvents).toHaveBeenCalled();
  });

  test('displays detailed information when showDetailed is true', () => {
    render(<FileSystemMonitor showDetailed={true} />);
    
    // The Size column should be present
    expect(screen.getByText('Size')).toBeInTheDocument();
    expect(screen.getByText('Directory')).toBeInTheDocument();
  });

  test('does not display detailed information when showDetailed is false', () => {
    render(<FileSystemMonitor />);
    
    // The Size column should not be present
    expect(screen.queryByText('Size')).not.toBeInTheDocument();
    expect(screen.queryByText('Directory')).not.toBeInTheDocument();
  });

  test('shows loading indicator when loading', () => {
    (fsMonitorHooks.useWatchedDirectories as jest.Mock).mockReturnValue({
      directories: [],
      isLoading: true,
      error: null,
      watchDirectory: mockWatchDirectory,
      unwatchDirectory: mockUnwatchDirectory
    });
    
    render(<FileSystemMonitor />);
    
    expect(screen.getByText('Watched Directories')).toBeInTheDocument();
    expect(screen.getByRole('status')).toBeInTheDocument(); // Spin component
  });

  test('shows error message when there is an error', () => {
    const errorMessage = 'Failed to connect to server';
    (fsMonitorHooks.useWatchedDirectories as jest.Mock).mockReturnValue({
      directories: [],
      isLoading: false,
      error: new Error(errorMessage),
      watchDirectory: mockWatchDirectory,
      unwatchDirectory: mockUnwatchDirectory
    });
    
    render(<FileSystemMonitor />);
    
    expect(screen.getByText(errorMessage)).toBeInTheDocument();
  });
}); 
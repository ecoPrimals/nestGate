import React from 'react';
import { render, screen, waitFor, act } from '@testing-library/react';
import '@testing-library/jest-dom';
import DiskHealthMonitor from '../../components/monitoring/DiskHealthMonitor';
import { WebSocketService, WebSocketMessageType, DiskMetrics } from '../../services/websocket.service';

// Mock the WebSocketService
jest.mock('../../services/websocket.service', () => {
  // Store the callbacks for testing
  const handlers: Map<string, Set<Function>> = new Map();
  
  return {
    WebSocketService: {
      getInstance: jest.fn().mockReturnValue({
        connect: jest.fn().mockResolvedValue(undefined),
        isConnected: jest.fn().mockReturnValue(true),
        subscribe: jest.fn().mockImplementation((type, callback) => {
          if (!handlers.has(type)) {
            handlers.set(type, new Set());
          }
          handlers.get(type)!.add(callback);
          
          // Return unsubscribe function
          return () => {
            const typeHandlers = handlers.get(type);
            if (typeHandlers) {
              typeHandlers.delete(callback);
            }
          };
        }),
        // Function to simulate WebSocket messages for testing
        __simulateMessage: (type: string, data: any) => {
          const message = {
            type,
            timestamp: Date.now(),
            data
          };
          
          const typeHandlers = handlers.get(type);
          if (typeHandlers) {
            typeHandlers.forEach(handler => handler(message));
          }
        }
      })
    },
    WebSocketMessageType: {
      SYSTEM_METRICS: 'system_metrics',
      ZFS_METRICS: 'zfs_metrics',
      DISK_METRICS: 'disk_metrics',
      NOTIFICATION: 'notification',
      ERROR: 'error'
    }
  };
});

describe('DiskHealthMonitor Component', () => {
  let wsService: any;
  
  // Sample disk metrics for testing
  const mockDisks: DiskMetrics[] = [
    {
      diskId: 'disk0',
      temperature: 35.2,
      health: 'good',
      status: 'online',
      readErrors: 0,
      writeErrors: 0,
      iops: {
        read: 120,
        write: 80
      },
      throughput: {
        read: 1024 * 1024 * 2, // 2 MB/s
        write: 1024 * 1024 * 1  // 1 MB/s
      }
    },
    {
      diskId: 'disk1',
      temperature: 42.7,
      health: 'warning',
      status: 'online',
      readErrors: 2,
      writeErrors: 1,
      iops: {
        read: 100,
        write: 60
      },
      throughput: {
        read: 1024 * 1024 * 1.5, // 1.5 MB/s
        write: 1024 * 1024 * 0.8  // 0.8 MB/s
      }
    },
    {
      diskId: 'disk2',
      temperature: 55.3,
      health: 'critical',
      status: 'degraded',
      readErrors: 10,
      writeErrors: 5,
      iops: {
        read: 80,
        write: 40
      },
      throughput: {
        read: 1024 * 1024 * 1, // 1 MB/s
        write: 1024 * 1024 * 0.5  // 0.5 MB/s
      }
    }
  ];
  
  beforeEach(() => {
    jest.clearAllMocks();
    wsService = WebSocketService.getInstance();
  });
  
  it('renders loading state initially', () => {
    render(<DiskHealthMonitor />);
    
    expect(screen.getByText('Loading disk metrics...')).toBeInTheDocument();
  });
  
  it('connects to WebSocketService on mount', () => {
    render(<DiskHealthMonitor />);
    
    expect(wsService.connect).toHaveBeenCalled();
  });
  
  it('subscribes to disk metrics', () => {
    render(<DiskHealthMonitor />);
    
    expect(wsService.subscribe).toHaveBeenCalledWith(
      WebSocketMessageType.DISK_METRICS,
      expect.any(Function)
    );
  });
  
  it('displays disk metrics when data is received', async () => {
    render(<DiskHealthMonitor />);
    
    // Simulate receiving disk metrics one by one
    act(() => {
      mockDisks.forEach(disk => {
        wsService.__simulateMessage(WebSocketMessageType.DISK_METRICS, disk);
      });
    });
    
    // Wait for component to update
    await waitFor(() => {
      expect(screen.queryByText('Loading disk metrics...')).not.toBeInTheDocument();
    });
    
    // Check that disk info is displayed
    expect(screen.getByText('disk0')).toBeInTheDocument();
    expect(screen.getByText('disk1')).toBeInTheDocument();
    expect(screen.getByText('disk2')).toBeInTheDocument();
    
    // Check status labels
    expect(screen.getAllByText('ONLINE').length).toBe(2);
    expect(screen.getByText('DEGRADED')).toBeInTheDocument();
    
    // Check health labels
    expect(screen.getByText('GOOD')).toBeInTheDocument();
    expect(screen.getByText('WARNING')).toBeInTheDocument();
    expect(screen.getByText('CRITICAL')).toBeInTheDocument();
    
    // Check temperatures
    expect(screen.getByText('35.2°C')).toBeInTheDocument();
    expect(screen.getByText('42.7°C')).toBeInTheDocument();
    expect(screen.getByText('55.3°C')).toBeInTheDocument();
    
    // Check health summary is displayed
    expect(screen.getByText('Total Disks')).toBeInTheDocument();
    expect(screen.getByText('3')).toBeInTheDocument(); // Total disks
    expect(screen.getByText('1')).toBeInTheDocument(); // Healthy disks (there's only one 'good' disk)
  });
  
  it('shows warning when disks are in warning state', async () => {
    render(<DiskHealthMonitor />);
    
    // Simulate receiving disk metrics with warnings but no critical issues
    act(() => {
      wsService.__simulateMessage(WebSocketMessageType.DISK_METRICS, {
        ...mockDisks[0],
        health: 'good'
      });
      wsService.__simulateMessage(WebSocketMessageType.DISK_METRICS, {
        ...mockDisks[1],
        health: 'warning'
      });
    });
    
    // Wait for component to update
    await waitFor(() => {
      expect(screen.queryByText('Loading disk metrics...')).not.toBeInTheDocument();
    });
    
    // Check that warning is displayed
    expect(screen.getByText('Disk Warnings Detected')).toBeInTheDocument();
    expect(screen.getByText('One or more disks have warnings. Please investigate at your convenience.')).toBeInTheDocument();
  });
  
  it('shows critical alert when disks are in critical state', async () => {
    render(<DiskHealthMonitor />);
    
    // Simulate receiving disk metrics with a critical disk
    act(() => {
      wsService.__simulateMessage(WebSocketMessageType.DISK_METRICS, {
        ...mockDisks[0],
        health: 'good'
      });
      wsService.__simulateMessage(WebSocketMessageType.DISK_METRICS, {
        ...mockDisks[2],
        health: 'critical'
      });
    });
    
    // Wait for component to update
    await waitFor(() => {
      expect(screen.queryByText('Loading disk metrics...')).not.toBeInTheDocument();
    });
    
    // Check that critical alert is displayed
    expect(screen.getByText('Critical Disk Issues Detected')).toBeInTheDocument();
    expect(screen.getByText('One or more disks are in critical state. Immediate action recommended.')).toBeInTheDocument();
  });
  
  it('shows a connection error when WebSocket connection fails', async () => {
    // Make connect method reject
    wsService.connect.mockRejectedValueOnce(new Error('Connection failed'));
    
    render(<DiskHealthMonitor />);
    
    // Wait for error to be displayed
    await waitFor(() => {
      expect(screen.getByText('Connection Error')).toBeInTheDocument();
      expect(screen.getByText('Failed to connect to the disk monitoring service')).toBeInTheDocument();
    });
  });
  
  it('unsubscribes from WebSocket on unmount', async () => {
    const { unmount } = render(<DiskHealthMonitor />);
    
    // Wait for component to initialize
    await waitFor(() => {
      expect(wsService.subscribe).toHaveBeenCalled();
    });
    
    // Get the unsubscribe function that was returned by the mock
    const unsubscribeFn = wsService.subscribe.mock.results[0].value;
    
    // Unmount the component
    unmount();
    
    // Verify the unsubscribe function was called
    expect(unsubscribeFn).toHaveBeenCalled();
  });
  
  it('displays disk throughput when showDetailed is true', async () => {
    render(<DiskHealthMonitor showDetailed={true} />);
    
    // Simulate receiving disk metrics
    act(() => {
      mockDisks.forEach(disk => {
        wsService.__simulateMessage(WebSocketMessageType.DISK_METRICS, disk);
      });
    });
    
    // Wait for component to update
    await waitFor(() => {
      expect(screen.queryByText('Loading disk metrics...')).not.toBeInTheDocument();
    });
    
    // Check that throughput section is displayed
    expect(screen.getByText('Disk Throughput')).toBeInTheDocument();
    
    // Check that read/write throughput is displayed for each disk
    expect(screen.getByText('Read Throughput')).toBeInTheDocument();
    expect(screen.getByText('Write Throughput')).toBeInTheDocument();
  });
  
  it('does not display disk throughput when showDetailed is false', async () => {
    render(<DiskHealthMonitor showDetailed={false} />);
    
    // Simulate receiving disk metrics
    act(() => {
      mockDisks.forEach(disk => {
        wsService.__simulateMessage(WebSocketMessageType.DISK_METRICS, disk);
      });
    });
    
    // Wait for component to update
    await waitFor(() => {
      expect(screen.queryByText('Loading disk metrics...')).not.toBeInTheDocument();
    });
    
    // Check that throughput section is not displayed
    expect(screen.queryByText('Disk Throughput')).not.toBeInTheDocument();
  });
}); 
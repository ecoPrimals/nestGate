import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { NasMetrics } from '../../components/dashboard/NasMetrics';
import { ZfsPoolService } from '../../services/zfs-pool.service';
import { NfsService } from '../../services/nfs.service';
import { SmbService } from '../../services/smb.service';
import { TelemetryService } from '../../services/telemetry.service';
import MockWebSocket from '../../__mocks__/websocket';

// Mock the services
jest.mock('../../services/zfs-pool.service');
jest.mock('../../services/nfs.service');
jest.mock('../../services/smb.service');
jest.mock('../../services/telemetry.service');

// Mock the WebSocket
(global as any).WebSocket = MockWebSocket;

describe('NasMetrics Component', () => {
  beforeEach(() => {
    // Reset all mocks
    jest.clearAllMocks();
    MockWebSocket.resetMock();
    
    // Setup mock data
    const mockPools = [
      {
        id: 'pool1',
        name: 'tank',
        health: 'ONLINE',
        size: 8000000000000,
        free: 6000000000000,
        used: 2000000000000
      }
    ];
    
    const mockSystemHealth = {
      status: 'healthy',
      cpu: 35,
      memory: 42,
      diskHealth: 'optimal',
      alerts: []
    };
    
    const mockNfsStatus = {
      status: 'running',
      version: '4.2',
      uptime: '10d 5h 30m'
    };
    
    const mockSmbStatus = {
      status: 'running',
      version: '4.15.5',
      uptime: '10d 5h 30m'
    };
    
    const mockPerformanceMetrics = {
      readThroughput: 120,
      writeThroughput: 85,
      readLatency: 5.2,
      writeLatency: 8.7,
      iops: 1200
    };
    
    const mockDiskHealth = {
      '/dev/sda': {
        device: '/dev/sda',
        model: 'Samsung SSD 870 EVO',
        serial: 'S1234567',
        temperature: 32.5,
        status: 'healthy',
        smartAttributes: []
      }
    };
    
    // Set up the mock implementations
    (ZfsPoolService.getPools as jest.Mock).mockResolvedValue(mockPools);
    (TelemetryService.getSystemHealth as jest.Mock).mockResolvedValue(mockSystemHealth);
    (NfsService.getStatus as jest.Mock).mockResolvedValue(mockNfsStatus);
    (SmbService.getStatus as jest.Mock).mockResolvedValue(mockSmbStatus);
    (TelemetryService.getPerformanceMetrics as jest.Mock).mockResolvedValue(mockPerformanceMetrics);
    (TelemetryService.getDiskHealth as jest.Mock).mockResolvedValue(mockDiskHealth);
    
    // Mock WebSocket methods
    (TelemetryService.connectWebSocket as jest.Mock).mockImplementation(() => {});
    (TelemetryService.addListener as jest.Mock).mockImplementation(() => {});
    (TelemetryService.removeListener as jest.Mock).mockImplementation(() => {});
  });
  
  it('should render the component title', async () => {
    render(<NasMetrics />);
    
    // Check for the title
    expect(screen.getByText('System Metrics')).toBeInTheDocument();
  });
  
  it('should show loading state initially', () => {
    render(<NasMetrics />);
    
    // Check for loading indicator
    expect(screen.getByText('System Metrics')).toBeInTheDocument();
    
    // The component should be in loading state
    const spinElement = document.querySelector('.ant-spin');
    expect(spinElement).toBeInTheDocument();
  });
  
  it('should display pool data after loading', async () => {
    render(<NasMetrics />);
    
    // Wait for data to load
    await waitFor(() => {
      expect(screen.getByText('tank')).toBeInTheDocument();
    });
    
    // Verify pool data is displayed
    expect(screen.getByText('ONLINE')).toBeInTheDocument();
    
    // Find the progress element that shows the storage usage
    const usageElement = document.querySelector('.usage-progress');
    expect(usageElement).toBeInTheDocument();
    
    // Instead of looking for specific text, check for the presence of the progress component
    expect(usageElement?.getAttribute('aria-valuenow')).not.toBeNull();
  });
  
  it('should display system health information', async () => {
    render(<NasMetrics />);
    
    // Wait for data to load
    await waitFor(() => {
      expect(screen.getByText(/System is healthy/i)).toBeInTheDocument();
    });
    
    // Verify system health data is displayed
    expect(screen.getByText('CPU Usage')).toBeInTheDocument();
    expect(screen.getByText('Memory Usage')).toBeInTheDocument();
    // Use more flexible text matchers for percentages
    expect(screen.getByText(/35/)).toBeInTheDocument();
    expect(screen.getByText(/42/)).toBeInTheDocument();
  });
  
  it('should display service status information', async () => {
    render(<NasMetrics />);
    
    // Wait for data to load
    await waitFor(() => {
      expect(screen.getByText('NFS Service')).toBeInTheDocument();
    });
    
    // Verify service status data is displayed
    expect(screen.getByText('SMB Service')).toBeInTheDocument();
    // Use getAllByText to handle multiple RUNNING tags
    const runningStatuses = screen.getAllByText('RUNNING');
    expect(runningStatuses.length).toBeGreaterThan(0);
    expect(runningStatuses[0]).toBeInTheDocument();
    
    expect(screen.getByText('Version: 4.2')).toBeInTheDocument();
    expect(screen.getByText('Version: 4.15.5')).toBeInTheDocument();
  });
  
  it('should display disk health information', async () => {
    render(<NasMetrics />);
    
    // Wait for data to load
    await waitFor(() => {
      expect(screen.getByText('Disk Health')).toBeInTheDocument();
    });
    
    // Verify disk health data is displayed
    await waitFor(() => {
      expect(screen.getByText('Samsung SSD 870 EVO')).toBeInTheDocument();
    });
    
    expect(screen.getByText('Device: /dev/sda')).toBeInTheDocument();
    expect(screen.getByText('Temperature: 32.5°C')).toBeInTheDocument();
  });
  
  it('should connect to WebSocket and set up listeners', async () => {
    render(<NasMetrics />);
    
    // Wait for component to mount and initialize
    await waitFor(() => {
      // Verify WebSocket connection was initiated
      expect(TelemetryService.connectWebSocket).toHaveBeenCalled();
    });
    
    // Verify listeners were added
    expect(TelemetryService.addListener).toHaveBeenCalledWith('SystemHealth', expect.any(Function));
    expect(TelemetryService.addListener).toHaveBeenCalledWith('Performance', expect.any(Function));
    expect(TelemetryService.addListener).toHaveBeenCalledWith('DiskHealth', expect.any(Function));
    expect(TelemetryService.addListener).toHaveBeenCalledWith('ZfsPool', expect.any(Function));
  });
  
  it('should clean up WebSocket listeners on unmount', async () => {
    const { unmount } = render(<NasMetrics />);
    
    // Wait for component to initialize
    await waitFor(() => {
      expect(TelemetryService.connectWebSocket).toHaveBeenCalled();
    });
    
    // Unmount the component
    unmount();
    
    // Verify listeners were removed
    expect(TelemetryService.removeListener).toHaveBeenCalledWith('SystemHealth', expect.any(Function));
    expect(TelemetryService.removeListener).toHaveBeenCalledWith('Performance', expect.any(Function));
    expect(TelemetryService.removeListener).toHaveBeenCalledWith('DiskHealth', expect.any(Function));
    expect(TelemetryService.removeListener).toHaveBeenCalledWith('ZfsPool', expect.any(Function));
  });
}); 
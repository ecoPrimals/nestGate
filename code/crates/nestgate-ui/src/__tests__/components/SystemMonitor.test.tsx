// Mock services and modules before importing the component
const mockConnect = jest.fn().mockResolvedValue(undefined);
const mockIsConnected = jest.fn().mockReturnValue(true);
const mockSubscribe = jest.fn().mockImplementation((type, callback) => {
  // Store the callback for later use in tests
  return jest.fn(); // Return unsubscribe function
});

// Define WebSocket message types
const SYSTEM_METRICS = 'system_metrics';

// Mock WebSocketService module
jest.mock('../../services/websocket.service', () => ({
  WebSocketService: {
    getInstance: () => ({
      connect: mockConnect,
      isConnected: mockIsConnected,
      subscribe: mockSubscribe
    })
  },
  WebSocketMessageType: {
    SYSTEM_METRICS: 'system_metrics',
    ZFS_METRICS: 'zfs_metrics',
    DISK_METRICS: 'disk_metrics',
    CPU_METRICS: 'cpu_metrics',
    MEMORY_METRICS: 'memory_metrics',
    NETWORK_METRICS: 'network_metrics',
    NOTIFICATION: 'notification',
    ERROR: 'error',
    POOL_STATUS: 'pool_status',
    TASK_STATUS: 'task_status'
  }
}));

// Mock format utils
jest.mock('../../utils/format.utils', () => ({
  formatBytes: (bytes: number) => `${bytes} bytes`,
  formatDuration: (seconds: number) => `${seconds} seconds`,
  formatPercent: (value: number) => `${value}%`,
}));

// Mock antd components
jest.mock('antd', () => {
  const originalModule = jest.requireActual('antd');
  
  // Mock Typography components
  const Typography = {
    Text: ({ children, ...props }: any) => (
      <div data-testid="typography-text" {...props}>{children}</div>
    ),
    Title: ({ children, level, ...props }: any) => (
      <div data-testid="typography-title" data-level={level} {...props}>{children}</div>
    ),
  };
  
  return {
    Card: ({ children, title, ...props }: any) => (
      <div data-testid="card" {...props}>
        {title && <div data-testid="card-title">{title}</div>}
        {children}
      </div>
    ),
    Row: ({ children, ...props }: any) => <div data-testid="row" {...props}>{children}</div>,
    Col: ({ children, ...props }: any) => <div data-testid="col" {...props}>{children}</div>,
    Statistic: ({ value, suffix, ...props }: any) => (
      <div data-testid="statistic" {...props}>
        {value}{suffix}
      </div>
    ),
    Progress: ({ percent, format, width, ...props }: any) => {
      const formattedValue = format ? format(percent) : `${percent}%`;
      return (
        <div data-testid="progress" data-percent={percent} {...props}>
          {formattedValue}
        </div>
      );
    },
    Space: ({ children, ...props }: any) => <div data-testid="space" {...props}>{children}</div>,
    Divider: (props: any) => <div data-testid="divider" {...props} />,
    Alert: ({ message, description, ...props }: any) => (
      <div data-testid="alert" {...props}>
        <div data-testid="alert-message">{message}</div>
        <div data-testid="alert-description">{description}</div>
      </div>
    ),
    Typography,
  };
});

// Mock icons
jest.mock('@ant-design/icons', () => ({
  LoadingOutlined: () => <span data-testid="icon-loading">Loading Icon</span>,
  SyncOutlined: (props: any) => <span data-testid="sync-icon" {...props}>Sync</span>,
  DesktopOutlined: () => <span data-testid="desktop-icon">Desktop</span>,
  SwapOutlined: () => <span data-testid="swap-icon">Swap</span>,
  ClockCircleOutlined: () => <span data-testid="clock-icon">Clock</span>,
  ThunderboltOutlined: () => <span data-testid="thunderbolt-icon">Thunderbolt</span>,
  DashboardOutlined: () => <span data-testid="dashboard-icon">Dashboard</span>,
  DatabaseOutlined: () => <span data-testid="database-icon">Database</span>,
  ArrowUpOutlined: () => <span data-testid="arrow-up-icon">ArrowUp</span>,
  ArrowDownOutlined: () => <span data-testid="arrow-down-icon">ArrowDown</span>,
  WarningOutlined: () => <span data-testid="warning-icon">Warning</span>,
  CheckCircleOutlined: () => <span data-testid="check-icon">Check</span>,
}));

// Now import React and test utilities
import React from 'react';
import { render, screen, waitFor, act } from '@testing-library/react';
import '@testing-library/jest-dom';

// Finally, import the component under test
import SystemMonitor from '../../components/monitoring/SystemMonitor';

describe('SystemMonitor Component', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    mockConnect.mockResolvedValue(undefined);
    mockIsConnected.mockReturnValue(true);
  });

  it('renders loading state initially', async () => {
    await act(async () => {
      render(<SystemMonitor />);
    });
    
    expect(screen.getByTestId('icon-loading')).toBeInTheDocument();
    expect(screen.getByTestId('typography-text')).toHaveTextContent('Loading system metrics...');
  });
  
  it('connects to WebSocketService on mount', async () => {
    await act(async () => {
      render(<SystemMonitor />);
    });
    
    expect(mockConnect).toHaveBeenCalled();
  });
  
  it('subscribes to system metrics', async () => {
    await act(async () => {
      render(<SystemMonitor />);
    });
    
    expect(mockSubscribe).toHaveBeenCalledWith(
      SYSTEM_METRICS,
      expect.any(Function)
    );
  });
  
  it('displays system metrics when data is received', async () => {
    // Create sample metrics data
    const sampleMetrics = {
      cpuUsage: 45.5,
      memoryUsage: 60.2,
      diskUsage: 55.0,
      uptime: 86400, // 1 day in seconds
      temperature: 42.5,
      loadAverage: [1.2, 1.5, 1.8],
      // Add missing properties needed by component
      diskIO: {
        read: 1024 * 1024, // 1 MB/s
        write: 512 * 1024  // 512 KB/s
      },
      networkIO: {
        received: 1024000,
        sent: 512000
      },
      memoryTotal: 16384,
      memoryFree: 6530
    };
    
    let subscribeCb: Function;
    mockSubscribe.mockImplementation((type, callback) => {
      subscribeCb = callback;
      return jest.fn();
    });
    
    await act(async () => {
      render(<SystemMonitor />);
    });
    
    // Call the subscription callback with sample data
    await act(async () => {
      subscribeCb({ type: SYSTEM_METRICS, data: sampleMetrics });
    });
    
    // Wait for metrics to be displayed
    await waitFor(() => {
      expect(screen.queryByTestId('icon-loading')).not.toBeInTheDocument();
    });
  });
  
  it('shows a connection error when WebSocket connection fails', async () => {
    mockConnect.mockRejectedValueOnce(new Error('Connection failed'));
    
    await act(async () => {
      render(<SystemMonitor />);
    });
    
    await waitFor(() => {
      expect(screen.getByTestId('alert')).toBeInTheDocument();
      expect(screen.getByTestId('alert-message')).toHaveTextContent('Connection Error');
    });
  });
  
  it('unsubscribes from WebSocket on unmount', async () => {
    const unsubscribeSpy = jest.fn();
    mockSubscribe.mockReturnValueOnce(unsubscribeSpy);
    
    let unmountFn: () => void;
    await act(async () => {
      const { unmount } = render(<SystemMonitor />);
      unmountFn = unmount;
    });
    
    await act(async () => {
      unmountFn();
    });
    
    expect(unsubscribeSpy).toHaveBeenCalled();
  });
});
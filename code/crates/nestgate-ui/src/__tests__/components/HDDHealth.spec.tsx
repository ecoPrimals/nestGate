import React from 'react';
import { render, screen, waitFor, act } from '@testing-library/react';
import '@testing-library/jest-dom';
import HDDHealth from '../../components/storage/HDDHealth';
import { WebSocketService } from '../../services/websocket.service';
import { TelemetryService } from '../../services/telemetry.service';

// Create a mock callback variable we can access in tests
let mockWebSocketCallback: ((message: any) => void) | null = null;

// Mock both services
jest.mock('../../services/websocket.service', () => ({
  WebSocketService: {
    getInstance: jest.fn().mockReturnValue({
      addListener: jest.fn().mockReturnValue(() => {}),
      connect: jest.fn(),
      subscribe: jest.fn().mockImplementation((type, callback) => {
        // Store the callback for later use in tests
        mockWebSocketCallback = callback;
        return () => {};
      })
    }),
    subscribe: jest.fn().mockReturnValue(() => {})
  }
}));

jest.mock('../../services/telemetry.service', () => ({
  TelemetryService: {
    getSystemHealth: jest.fn().mockResolvedValue({
      diskHealth: [
        {
          device: '/dev/sda',
          status: 'healthy',
          temperature: 32.4,
          model: 'WD Red 4TB',
          size: '4.0 TB',
          type: 'HDD',
          smart: {
            reallocatedSectors: 0,
            pendingSectors: 0,
            uncorrectableSectors: 0,
            powerOnHours: 8760,
            lastTestStatus: 'passed'
          },
          smartAttributes: [
            { id: 1, name: 'Raw Read Error Rate', value: 100, threshold: 50, status: 'ok' },
            { id: 5, name: 'Reallocated Sectors Count', value: 100, threshold: 50, status: 'ok' },
            { id: 194, name: 'Temperature', value: 32, threshold: 65, status: 'ok' }
          ]
        },
        {
          device: '/dev/sdb',
          status: 'warning',
          temperature: 41.7,
          model: 'Seagate IronWolf 6TB',
          size: '6.0 TB',
          type: 'HDD',
          smart: {
            reallocatedSectors: 2,
            pendingSectors: 1,
            uncorrectableSectors: 0,
            powerOnHours: 17520,
            lastTestStatus: 'warning'
          },
          smartAttributes: [
            { id: 1, name: 'Raw Read Error Rate', value: 100, threshold: 50, status: 'ok' },
            { id: 5, name: 'Reallocated Sectors Count', value: 98, threshold: 50, status: 'warning' },
            { id: 194, name: 'Temperature', value: 42, threshold: 65, status: 'ok' }
          ]
        }
      ]
    })
  }
}));

describe('HDDHealth Component', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    // Reset the callback before each test
    mockWebSocketCallback = null;
  });

  it('renders the loading state initially', () => {
    render(<HDDHealth />);
    expect(screen.getByText('Loading disk health information...')).toBeInTheDocument();
  });

  it('fetches and displays disk health information', async () => {
    render(<HDDHealth />);
    
    // Wait for the data to load
    await waitFor(() => {
      expect(screen.queryByText('Loading disk health information...')).not.toBeInTheDocument();
    });
    
    // Check if the disk information is displayed
    expect(screen.getByText('HDD Health Status')).toBeInTheDocument();
    expect(screen.getByText('/dev/sda')).toBeInTheDocument();
    expect(screen.getByText('/dev/sdb')).toBeInTheDocument();
    
    // Check if status badges are displayed
    expect(screen.getByText('Healthy')).toBeInTheDocument();
    expect(screen.getByText('Warning')).toBeInTheDocument();
    
    // Check for temperature information
    expect(screen.getByText('32.4°C')).toBeInTheDocument();
    expect(screen.getByText('41.7°C')).toBeInTheDocument();
    
    // Check for disk models
    expect(screen.getByText('WD Red 4TB')).toBeInTheDocument();
    expect(screen.getByText('Seagate IronWolf 6TB')).toBeInTheDocument();
  });

  it('subscribes to WebSocket updates', async () => {
    render(<HDDHealth />);
    const webSocketInstance = WebSocketService.getInstance();
    
    await waitFor(() => {
      expect(webSocketInstance.subscribe).toHaveBeenCalledWith(
        'diskHealth',
        expect.any(Function)
      );
    });
  });
  
  it('updates disk health data on WebSocket message', async () => {
    render(<HDDHealth />);
    
    // Wait for initial render
    await waitFor(() => {
      expect(screen.queryByText('Loading disk health information...')).not.toBeInTheDocument();
    });
    
    // Ensure the callback was set during component initialization
    expect(mockWebSocketCallback).not.toBeNull();
    
    // Call the update callback with new data
    act(() => {
      if (mockWebSocketCallback) {
        mockWebSocketCallback({
          diskHealth: [
            {
              device: '/dev/sda',
              status: 'critical',
              temperature: 55.4,
              model: 'WD Red 4TB',
              size: '4.0 TB',
              type: 'HDD',
              smart: {
                reallocatedSectors: 10,
                pendingSectors: 5,
                uncorrectableSectors: 2,
                powerOnHours: 8760,
                lastTestStatus: 'failed'
              },
              smartAttributes: [
                { id: 1, name: 'Raw Read Error Rate', value: 70, threshold: 50, status: 'ok' },
                { id: 5, name: 'Reallocated Sectors Count', value: 90, threshold: 50, status: 'critical' },
                { id: 194, name: 'Temperature', value: 55, threshold: 65, status: 'warning' }
              ]
            }
          ]
        });
      }
    });
    
    // Check if the updated data is displayed
    await waitFor(() => {
      expect(screen.getByText('Critical')).toBeInTheDocument();
      expect(screen.getByText('55.4°C')).toBeInTheDocument();
    });
  });
}); 
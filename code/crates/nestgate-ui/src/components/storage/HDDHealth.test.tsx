import React from 'react';
import { render, screen, waitFor, fireEvent } from '@testing-library/react';
import '@testing-library/jest-dom';
import HDDHealth from './HDDHealth';
import { WebSocketService } from '../../services/websocket.service';

// Mock the WebSocketService
jest.mock('../../services/websocket.service', () => ({
  WebSocketService: {
    getInstance: jest.fn().mockReturnValue({
      connect: jest.fn(),
      send: jest.fn(),
      subscribe: jest.fn().mockImplementation((topic, callback) => {
        // Store callbacks for triggering later
        if (topic === 'connection') {
          mockWebSocketHandlers.connection = callback;
        } else if (topic === 'disks') {
          mockWebSocketHandlers.disks = callback;
        }
        // Return unsubscribe function
        return jest.fn();
      })
    })
  }
}));

// Mock handler storage
const mockWebSocketHandlers: any = {};

describe('HDDHealth Component', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  test('renders loading state initially', () => {
    render(<HDDHealth />);
    
    expect(screen.getByText('Disk Health Monitoring')).toBeInTheDocument();
    expect(screen.getByText('Refresh Data')).toBeInTheDocument();
    
    // Should show test data notification
    expect(screen.getByText(/MOCK DATA: Disk health information is simulated/i)).toBeInTheDocument();
  });

  test('shows connection status correctly', async () => {
    render(<HDDHealth />);
    
    // Initially shows test data status
    expect(screen.getByText('Test Data')).toBeInTheDocument();
    
    // Simulate WebSocket connection
    mockWebSocketHandlers.connection({ data: { status: 'connected' } });
    
    await waitFor(() => {
      expect(screen.getByText('Live Data')).toBeInTheDocument();
    });
  });

  test('fetches and displays disk data', async () => {
    render(<HDDHealth />);
    
    // Simulate connection and disk data response
    mockWebSocketHandlers.connection({ data: { status: 'connected' } });
    
    // Mock disk data
    const mockDisks = [
      {
        id: 'sda',
        status: 'PASSED',
        model: 'Samsung SSD 860 EVO',
        serial: 'S12345ABC',
        size: '1TB',
        temperature: 35
      },
      {
        id: 'sdb',
        status: 'WARNING',
        model: 'WD Blue 2TB',
        serial: 'WD123456',
        size: '2TB',
        temperature: 47
      }
    ];
    
    mockWebSocketHandlers.disks({ data: mockDisks });
    
    await waitFor(() => {
      expect(screen.getByText('sda')).toBeInTheDocument();
      expect(screen.getByText('sdb')).toBeInTheDocument();
      expect(screen.getByText('Samsung SSD 860 EVO')).toBeInTheDocument();
      expect(screen.getByText('WD Blue 2TB')).toBeInTheDocument();
      expect(screen.getByText('35°C')).toBeInTheDocument();
      expect(screen.getByText('47°C')).toBeInTheDocument();
    });
  });

  test('displays disk details when View Details is clicked', async () => {
    render(<HDDHealth />);
    
    // Simulate connection and disk data response
    mockWebSocketHandlers.connection({ data: { status: 'connected' } });
    
    // Mock disk data
    const mockDisks = [
      {
        id: 'sda',
        status: 'PASSED',
        model: 'Samsung SSD 860 EVO',
        serial: 'S12345ABC',
        size: '1TB',
        temperature: 35
      }
    ];
    
    mockWebSocketHandlers.disks({ data: mockDisks });
    
    await waitFor(() => {
      expect(screen.getByText('sda')).toBeInTheDocument();
    });
    
    // Click View Details button
    fireEvent.click(screen.getByText('View Details'));
    
    await waitFor(() => {
      expect(screen.getByText('Disk Details: sda')).toBeInTheDocument();
      expect(screen.getByText('Overview')).toBeInTheDocument();
      expect(screen.getByText('SMART Attributes')).toBeInTheDocument();
    });
  });

  test('refreshes data when refresh button is clicked', async () => {
    render(<HDDHealth />);
    
    // Click refresh button
    fireEvent.click(screen.getByText('Refresh Data'));
    
    // Verify WebSocketService.send was called
    expect(WebSocketService.getInstance().send).toHaveBeenCalledWith('get_disks');
  });

  test('switches between overview and SMART attributes tabs', async () => {
    render(<HDDHealth />);
    
    // Simulate connection and disk data response
    mockWebSocketHandlers.connection({ data: { status: 'connected' } });
    
    // Mock disk data
    const mockDisks = [
      {
        id: 'sda',
        status: 'PASSED',
        model: 'Samsung SSD 860 EVO',
        serial: 'S12345ABC',
        size: '1TB',
        temperature: 35
      }
    ];
    
    mockWebSocketHandlers.disks({ data: mockDisks });
    
    await waitFor(() => {
      expect(screen.getByText('View Details')).toBeInTheDocument();
    });
    
    // Click View Details button
    fireEvent.click(screen.getByText('View Details'));
    
    // Check Overview tab is visible by default
    await waitFor(() => {
      expect(screen.getByText('Temperature')).toBeInTheDocument();
      expect(screen.getByText('Power On Time')).toBeInTheDocument();
    });
    
    // Click SMART Attributes tab
    fireEvent.click(screen.getByText('SMART Attributes'));
    
    await waitFor(() => {
      expect(screen.getByText('Understanding SMART Attributes')).toBeInTheDocument();
      expect(screen.getByText('Value:')).toBeInTheDocument();
      expect(screen.getByText('Threshold:')).toBeInTheDocument();
    });
  });
}); 
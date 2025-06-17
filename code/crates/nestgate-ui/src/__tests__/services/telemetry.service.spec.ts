import { TelemetryService } from '../../services/telemetry.service';
import MockWebSocket from '../../__mocks__/websocket';

// Mock fetch API
global.fetch = jest.fn();
const mockFetch = global.fetch as jest.Mock;

// Override WebSocket with our mock implementation
(global as any).WebSocket = MockWebSocket;

describe('TelemetryService', () => {
  beforeEach(() => {
    // Reset all mocks before each test
    jest.clearAllMocks();
    mockFetch.mockClear();
    MockWebSocket.resetMock();
  });

  afterEach(() => {
    // Clean up any WebSocket connections
    if (MockWebSocket.instances.length > 0) {
      MockWebSocket.instances.forEach(ws => ws.close());
    }
  });

  describe('WebSocket connections', () => {
    it('should connect to WebSocket endpoint', () => {
      // Call the connect method
      TelemetryService.connectWebSocket();

      // Verify WebSocket was created with correct URL
      expect(MockWebSocket.instances.length).toBe(1);
      expect(MockWebSocket.instances[0].url).toBe('ws://localhost:3000/api/metrics/ws');
    });

    it('should send subscription requests on connection', async () => {
      // Connect to WebSocket
      TelemetryService.connectWebSocket();
      
      // Wait for connection to establish
      await new Promise(resolve => setTimeout(resolve, 20));
      
      // Check the sent messages
      const instance = MockWebSocket.instances[0];
      expect(instance.sentMessages.length).toBeGreaterThan(0);
      
      // Verify subscription to system health
      const systemHealthSub = instance.sentMessages.find(msg => 
        msg.includes('SystemHealth')
      );
      expect(systemHealthSub).toBeTruthy();
      
      // Verify subscription to performance metrics
      const performanceSub = instance.sentMessages.find(msg => 
        msg.includes('Performance')
      );
      expect(performanceSub).toBeTruthy();
    });

    it('should call listeners when receiving data', async () => {
      // Create mock listeners
      const mockSystemHealthListener = jest.fn();
      
      // Add listener
      TelemetryService.addListener('SystemHealth', mockSystemHealthListener);
      
      // Connect to WebSocket
      TelemetryService.connectWebSocket();
      
      // Wait for connection to establish
      await new Promise(resolve => setTimeout(resolve, 20));
      
      // Simulate receiving a message
      const mockData = {
        metric_type: 'SystemHealth',
        data: {
          status: 'healthy',
          cpu_usage: 45.2,
          memory_usage: 62.8,
          alerts: []
        }
      };
      
      // Send the message
      MockWebSocket.instances[0].receiveMessage(JSON.stringify(mockData));
      
      // Verify the listener was called with the data
      expect(mockSystemHealthListener).toHaveBeenCalledWith(mockData.data);
    });

    it('should remove listeners correctly', async () => {
      // Create mock listeners
      const mockListener = jest.fn();
      
      // Add listener
      TelemetryService.addListener('Performance', mockListener);
      
      // Connect to WebSocket
      TelemetryService.connectWebSocket();
      
      // Wait for connection to establish
      await new Promise(resolve => setTimeout(resolve, 20));
      
      // Remove the listener
      TelemetryService.removeListener('Performance', mockListener);
      
      // Simulate receiving a message
      const mockData = {
        metric_type: 'Performance',
        data: {
          read_throughput_mbps: 120,
          write_throughput_mbps: 85
        }
      };
      
      // Send the message
      MockWebSocket.instances[0].receiveMessage(JSON.stringify(mockData));
      
      // Verify the listener was not called
      expect(mockListener).not.toHaveBeenCalled();
    });
  });

  describe('API calls', () => {
    it('should fetch system health data', async () => {
      // Mock successful response
      const mockResponse = {
        data: {
          status: 'healthy',
          cpu_usage: 35.2,
          memory_usage: 42.5,
          alerts: []
        },
        error: null
      };
      
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => mockResponse
      });
      
      // Call the method
      const result = await TelemetryService.getSystemHealth();
      
      // Verify fetch was called with correct URL
      expect(mockFetch).toHaveBeenCalledWith('http://localhost:3000/api/metrics/system');
      
      // Verify returned data structure
      expect(result).toEqual({
        status: 'healthy',
        cpu: 35.2,
        memory: 42.5,
        diskHealth: 'optimal',
        alerts: []
      });
    });

    it('should handle system health API errors gracefully', async () => {
      // Mock error response
      mockFetch.mockRejectedValueOnce(new Error('Network error'));
      
      // Call the method
      const result = await TelemetryService.getSystemHealth();
      
      // Verify default data is returned
      expect(result).toEqual({
        status: 'healthy',
        cpu: 35,
        memory: 42,
        diskHealth: 'optimal',
        alerts: []
      });
    });

    it('should fetch disk health data', async () => {
      // Mock successful response
      const mockDiskData = {
        '/dev/sda': {
          device: '/dev/sda',
          model: 'Samsung SSD 870 EVO',
          serial: 'S1234567',
          temperature: 32.5,
          status: 'Healthy',
          smart_attributes: [
            {
              id: 5,
              name: 'Reallocated_Sector_Ct',
              value: 0,
              threshold: 10,
              status: 'Healthy'
            }
          ]
        }
      };
      
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({ data: mockDiskData, error: null })
      });
      
      // Call the method
      const result = await TelemetryService.getDiskHealth();
      
      // Verify fetch was called with correct URL
      expect(mockFetch).toHaveBeenCalledWith('http://localhost:3000/api/metrics/disk');
      
      // Verify returned data structure
      expect(result).toHaveProperty('/dev/sda');
      expect(result['/dev/sda'].model).toBe('Samsung SSD 870 EVO');
      expect(result['/dev/sda'].status).toBe('healthy');
    });

    it('should fetch performance metrics', async () => {
      // Mock successful response
      const mockPerfData = {
        read_throughput_mbps: 120.5,
        write_throughput_mbps: 85.3,
        read_latency_ms: 5.2,
        write_latency_ms: 8.7,
        iops: 1200
      };
      
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({ data: mockPerfData, error: null })
      });
      
      // Call the method without dataset IDs
      const result = await TelemetryService.getPerformanceMetrics();
      
      // Verify fetch was called with correct URL
      expect(mockFetch).toHaveBeenCalledWith('http://localhost:3000/api/metrics/performance');
      
      // Verify returned data structure
      expect(result).toHaveProperty('readThroughput', 120.5);
      expect(result).toHaveProperty('writeThroughput', 85.3);
      expect(result).toHaveProperty('iops', 1200);
    });

    it('should fetch optimization metrics when dataset IDs are provided', async () => {
      // Mock successful response
      const mockOptData = {
        current_metrics: {
          read_throughput_mbps: 120.5,
          write_throughput_mbps: 85.3,
          read_latency_ms: 5.2,
          write_latency_ms: 8.7
        },
        estimated_optimized_metrics: {
          read_throughput_mbps: 150.2,
          write_throughput_mbps: 95.8,
          read_latency_ms: 3.1,
          write_latency_ms: 6.2
        }
      };
      
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({ data: mockOptData, error: null })
      });
      
      // Call the method with dataset IDs
      const result = await TelemetryService.getPerformanceMetrics(['dataset-1']);
      
      // Verify fetch was called with correct URL
      expect(mockFetch).toHaveBeenCalledWith('http://localhost:3000/api/datasets/dataset-1/optimization');
      
      // Type assertion to treat the result as OptimizationMetrics
      const optimizationResult = result as {
        beforeOptimization: { readThroughput: number, writeThroughput: number };
        afterOptimization: { readThroughput: number, writeThroughput: number };
      };
      
      // Verify returned data structure
      expect(optimizationResult).toHaveProperty('beforeOptimization');
      expect(optimizationResult).toHaveProperty('afterOptimization');
      expect(optimizationResult.beforeOptimization.readThroughput).toBe(120.5);
      expect(optimizationResult.afterOptimization.writeThroughput).toBe(95.8);
    });
  });
}); 
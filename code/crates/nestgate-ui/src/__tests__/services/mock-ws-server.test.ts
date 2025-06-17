import MockWebSocketServer from '../../services/mock-ws-server';
import { WebSocketMessageType } from '../../services/websocket.service';

// Mock document event dispatching and listening
const originalAddEventListener = document.addEventListener;
const originalDispatchEvent = document.dispatchEvent;
const mockAddEventListener = jest.fn();
const mockDispatchEvent = jest.fn();

describe('MockWebSocketServer', () => {
  let mockServer: MockWebSocketServer;
  let mockMessageListener: jest.Mock;
  
  beforeEach(() => {
    // Override document methods
    document.addEventListener = mockAddEventListener;
    document.dispatchEvent = mockDispatchEvent;
    
    // Clear mocks
    mockAddEventListener.mockClear();
    mockDispatchEvent.mockClear();
    
    // Create a test message listener
    mockMessageListener = jest.fn();
    
    // Get server instance
    mockServer = MockWebSocketServer.getInstance();
  });
  
  afterEach(() => {
    // Stop the server
    mockServer.stop();
    
    // Restore original document methods
    document.addEventListener = originalAddEventListener;
    document.dispatchEvent = originalDispatchEvent;
  });
  
  describe('Singleton pattern', () => {
    it('should return the same instance on multiple calls to getInstance', () => {
      const instance1 = MockWebSocketServer.getInstance();
      const instance2 = MockWebSocketServer.getInstance();
      
      expect(instance1).toBe(instance2);
    });
  });
  
  describe('Server lifecycle', () => {
    it('should set up an interval when started', () => {
      // Spy on setInterval
      const setIntervalSpy = jest.spyOn(window, 'setInterval');
      
      // Start the server
      mockServer.start();
      
      // Verify setInterval was called
      expect(setIntervalSpy).toHaveBeenCalled();
    });
    
    it('should not start multiple intervals if already running', () => {
      // Spy on setInterval
      const setIntervalSpy = jest.spyOn(window, 'setInterval');
      
      // Start the server
      mockServer.start();
      const callCount = setIntervalSpy.mock.calls.length;
      
      // Try to start it again
      mockServer.start();
      
      // Verify setInterval wasn't called again
      expect(setIntervalSpy.mock.calls.length).toBe(callCount);
    });
    
    it('should clear the interval when stopped', () => {
      // Spy on clearInterval
      const clearIntervalSpy = jest.spyOn(window, 'clearInterval');
      
      // Start then stop the server
      mockServer.start();
      mockServer.stop();
      
      // Verify clearInterval was called
      expect(clearIntervalSpy).toHaveBeenCalled();
    });
  });
  
  describe('Data generation and broadcasting', () => {
    it('should dispatch system metrics events', () => {
      // Manually trigger the private method
      // @ts-ignore - Access private method for testing
      mockServer.updateMockData();
      // @ts-ignore
      mockServer.broadcastMockData();
      
      // Check dispatched events
      const systemMetricsEvent = mockDispatchEvent.mock.calls.find(call => {
        const event = call[0];
        return event.detail && event.detail.type === WebSocketMessageType.SYSTEM_METRICS;
      });
      
      // Verify system metrics were dispatched
      expect(systemMetricsEvent).toBeTruthy();
      
      // Verify the data structure
      const eventData = systemMetricsEvent[0].detail.data;
      expect(eventData).toHaveProperty('cpuUsage');
      expect(eventData).toHaveProperty('memoryUsage');
      expect(eventData).toHaveProperty('uptime');
      expect(eventData).toHaveProperty('loadAverage');
      expect(eventData).toHaveProperty('diskIO');
      expect(eventData).toHaveProperty('networkIO');
      expect(eventData).toHaveProperty('temperature');
    });
    
    it('should dispatch disk metrics events', () => {
      // Manually trigger the private method
      // @ts-ignore - Access private method for testing
      mockServer.updateMockData();
      // @ts-ignore
      mockServer.broadcastMockData();
      
      // Check dispatched events
      const diskMetricsEvents = mockDispatchEvent.mock.calls.filter(call => {
        const event = call[0];
        return event.detail && event.detail.type === WebSocketMessageType.DISK_METRICS;
      });
      
      // Verify disk metrics were dispatched
      expect(diskMetricsEvents.length).toBeGreaterThan(0);
      
      // Verify one example of the data structure
      const eventData = diskMetricsEvents[0][0].detail.data;
      expect(eventData).toHaveProperty('diskId');
      expect(eventData).toHaveProperty('temperature');
      expect(eventData).toHaveProperty('health');
      expect(eventData).toHaveProperty('status');
      expect(eventData).toHaveProperty('readErrors');
      expect(eventData).toHaveProperty('writeErrors');
      expect(eventData).toHaveProperty('iops');
      expect(eventData).toHaveProperty('throughput');
    });
    
    it('should dispatch ZFS metrics events', () => {
      // Manually trigger the private method
      // @ts-ignore - Access private method for testing
      mockServer.updateMockData();
      // @ts-ignore
      mockServer.broadcastMockData();
      
      // Check dispatched events
      const zfsMetricsEvents = mockDispatchEvent.mock.calls.filter(call => {
        const event = call[0];
        return event.detail && event.detail.type === WebSocketMessageType.ZFS_METRICS;
      });
      
      // Verify ZFS metrics were dispatched
      expect(zfsMetricsEvents.length).toBeGreaterThan(0);
      
      // Verify one example of the data structure
      const eventData = zfsMetricsEvents[0][0].detail.data;
      expect(eventData).toHaveProperty('poolName');
      expect(eventData).toHaveProperty('status');
      expect(eventData).toHaveProperty('health');
      expect(eventData).toHaveProperty('capacityUsed');
      expect(eventData).toHaveProperty('capacityTotal');
      expect(eventData).toHaveProperty('diskErrors');
      expect(eventData).toHaveProperty('children');
    });
    
    it('should occasionally send notifications', () => {
      // Mock Math.random to always return a value that will trigger notifications
      const originalRandom = Math.random;
      Math.random = jest.fn().mockReturnValue(0.9);
      
      // @ts-ignore - Access private method for testing
      mockServer.broadcastMockData();
      
      // Check dispatched events
      const notificationEvents = mockDispatchEvent.mock.calls.filter(call => {
        const event = call[0];
        return event.detail && event.detail.type === WebSocketMessageType.NOTIFICATION;
      });
      
      // Verify notifications were dispatched
      expect(notificationEvents.length).toBeGreaterThan(0);
      
      // Verify the data structure
      const eventData = notificationEvents[0][0].detail.data;
      expect(eventData).toHaveProperty('id');
      expect(eventData).toHaveProperty('type');
      expect(eventData).toHaveProperty('title');
      expect(eventData).toHaveProperty('message');
      expect(eventData).toHaveProperty('read');
      expect(eventData).toHaveProperty('timestamp');
      
      // Restore original Math.random
      Math.random = originalRandom;
    });
  });
  
  describe('Data updates', () => {
    it('should update mock data values within reasonable ranges', () => {
      // Get initial system metrics
      // @ts-ignore - Access private property for testing
      const initialCpuUsage = mockServer.mockSystemMetrics.cpuUsage;
      
      // Update mock data
      // @ts-ignore - Access private method for testing
      mockServer.updateMockData();
      
      // Get updated system metrics
      // @ts-ignore - Access private property for testing
      const updatedCpuUsage = mockServer.mockSystemMetrics.cpuUsage;
      
      // Verify changes are within expected range (±5%)
      expect(Math.abs(updatedCpuUsage - initialCpuUsage)).toBeLessThanOrEqual(5);
    });
  });
}); 
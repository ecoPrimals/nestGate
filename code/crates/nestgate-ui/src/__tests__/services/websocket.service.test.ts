import { WebSocketService, WebSocketMessageType } from '../../services/websocket.service';
import { AuthService } from '../../services/auth.service';

// Mock the AuthService
jest.mock('../../services/auth.service', () => ({
  AuthService: {
    getToken: jest.fn().mockReturnValue('mock-token')
  }
}));

// Create a mock WebSocket class
class MockWebSocket {
  url: string;
  onopen: any;
  onclose: any;
  onmessage: any;
  onerror: any;
  readyState: number;
  OPEN: number = 1;
  CLOSED: number = 3;
  CONNECTING: number = 0;
  
  static instances: MockWebSocket[] = [];
  sentMessages: any[] = [];

  constructor(url: string) {
    this.url = url;
    this.readyState = this.CONNECTING;
    MockWebSocket.instances.push(this);
    
    // Simulate connection after a short delay
    setTimeout(() => {
      this.readyState = this.OPEN;
      if (this.onopen) this.onopen({ target: this });
    }, 10);
  }

  send(data: string) {
    this.sentMessages.push(data);
  }

  close() {
    this.readyState = this.CLOSED;
    if (this.onclose) this.onclose({ code: 1000, reason: 'Normal closure', wasClean: true });
  }

  // Utility method to simulate receiving a message
  simulateMessage(data: any) {
    if (this.onmessage) {
      this.onmessage({ data: JSON.stringify(data) });
    }
  }

  // Utility method to simulate an error
  simulateError(error: any) {
    if (this.onerror) {
      this.onerror(error);
    }
  }

  // Utility method to simulate a connection close
  simulateClose(code: number = 1000, reason: string = 'Normal closure') {
    this.readyState = this.CLOSED;
    if (this.onclose) {
      this.onclose({ code, reason, wasClean: true });
    }
  }
}

// Replace the global WebSocket with our mock
global.WebSocket = MockWebSocket as any;

describe('WebSocketService', () => {
  let service: WebSocketService;
  
  beforeEach(() => {
    // Clear all mock WebSocket instances
    MockWebSocket.instances = [];
    // Reset mocks
    jest.clearAllMocks();
    // Get a fresh instance
    service = WebSocketService.getInstance();
    
    // Reset the service (not ideal but necessary for testing singleton)
    // @ts-ignore - Accessing private property for testing
    service.socket = null;
    // @ts-ignore
    service.reconnectTimer = null;
    // @ts-ignore
    service.reconnectAttempts = 0;
    // @ts-ignore
    service.pingInterval = null;
    // @ts-ignore
    service.isConnecting = false;
    // @ts-ignore
    service.messageHandlers = new Map();
  });

  afterEach(() => {
    // Cleanup
    // @ts-ignore
    if (service.reconnectTimer) {
      // @ts-ignore
      clearTimeout(service.reconnectTimer);
    }
    // @ts-ignore
    if (service.pingInterval) {
      // @ts-ignore
      clearInterval(service.pingInterval);
    }
  });

  describe('connect', () => {
    it('should establish a WebSocket connection', async () => {
      await service.connect();
      
      expect(MockWebSocket.instances.length).toBe(1);
      expect(MockWebSocket.instances[0].url).toContain('token=mock-token');
    });

    it('should not connect twice if already connecting', async () => {
      // Set isConnecting flag
      // @ts-ignore - Accessing private property for testing
      service.isConnecting = true;
      
      await service.connect();
      
      expect(MockWebSocket.instances.length).toBe(0);
    });

    it('should not connect twice if already connected', async () => {
      // Connect once
      await service.connect();
      
      // Connect again
      await service.connect();
      
      // Should still have only one instance
      expect(MockWebSocket.instances.length).toBe(1);
    });

    it('should handle connection errors', async () => {
      // Make AuthService.getToken return null
      (AuthService.getToken as jest.Mock).mockReturnValueOnce(null);
      
      // @ts-ignore - Mock environment for testing
      service.useMockData = false;
      
      // This should now reject
      await expect(service.connect()).rejects.toThrow();
    });
  });

  describe('subscribe and message handling', () => {
    it('should subscribe to specific message types', async () => {
      // Connect
      await service.connect();
      
      // Create mock handler
      const mockHandler = jest.fn();
      
      // Subscribe to a specific message type
      service.subscribe(WebSocketMessageType.SYSTEM_METRICS, mockHandler);
      
      // Verify handler was added
      // @ts-ignore - Accessing private property for testing
      expect(service.messageHandlers.get(WebSocketMessageType.SYSTEM_METRICS).size).toBe(1);
    });

    it('should call handlers when messages are received', async () => {
      // Connect
      await service.connect();
      
      // Create mock handlers
      const specificHandler = jest.fn();
      const allHandler = jest.fn();
      
      // Subscribe to handlers
      service.subscribe(WebSocketMessageType.SYSTEM_METRICS, specificHandler);
      service.subscribe('all', allHandler);
      
      // Simulate receiving a message
      const message = {
        type: WebSocketMessageType.SYSTEM_METRICS,
        timestamp: Date.now(),
        data: { cpuUsage: 50 }
      };
      
      // Send the message
      MockWebSocket.instances[0].simulateMessage(message);
      
      // Both handlers should be called
      expect(specificHandler).toHaveBeenCalledWith(message);
      expect(allHandler).toHaveBeenCalledWith(message);
    });

    it('should allow unsubscribing from messages', async () => {
      // Connect
      await service.connect();
      
      // Create mock handler
      const mockHandler = jest.fn();
      
      // Subscribe and get unsubscribe function
      const unsubscribe = service.subscribe(WebSocketMessageType.SYSTEM_METRICS, mockHandler);
      
      // Unsubscribe
      unsubscribe();
      
      // Verify handler was removed
      // @ts-ignore - Accessing private property for testing
      expect(service.messageHandlers.has(WebSocketMessageType.SYSTEM_METRICS)).toBe(false);
    });
  });

  describe('reconnection', () => {
    it('should attempt to reconnect when connection closes', async () => {
      // Connect
      await service.connect();
      
      // Spy on connect method for reconnection
      const connectSpy = jest.spyOn(service, 'connect');
      
      // Simulate connection close
      MockWebSocket.instances[0].simulateClose();
      
      // Wait for reconnect timer
      jest.advanceTimersByTime(2000);
      
      // Should attempt to reconnect
      expect(connectSpy).toHaveBeenCalled();
    });

    it('should not exceed max reconnection attempts', async () => {
      // Connect
      await service.connect();
      
      // Set reconnect attempts to max
      // @ts-ignore - Accessing private property for testing
      service.reconnectAttempts = service.maxReconnectAttempts;
      
      // Spy on connect method
      const connectSpy = jest.spyOn(service, 'connect');
      
      // Simulate connection close
      MockWebSocket.instances[0].simulateClose();
      
      // Wait for potential reconnect
      jest.advanceTimersByTime(10000);
      
      // Should not attempt to reconnect
      expect(connectSpy).not.toHaveBeenCalled();
    });
  });

  describe('mock data mode', () => {
    it('should use mock data in development mode', async () => {
      // Force mock data mode
      // @ts-ignore - Accessing private property for testing
      service.useMockData = true;
      
      // Spy on mock data listener
      // @ts-ignore - Accessing private method for testing
      const listenerSpy = jest.spyOn(service, 'startMockDataListener');
      
      // Connect
      await service.connect();
      
      // Should start mock data listener
      expect(listenerSpy).toHaveBeenCalled();
      
      // Should not create a WebSocket
      expect(MockWebSocket.instances.length).toBe(0);
    });
    
    it('should clean up mock data listener on disconnect', async () => {
      // Force mock data mode
      // @ts-ignore - Accessing private property for testing
      service.useMockData = true;
      
      // Connect
      await service.connect();
      
      // Spy on stop mock data listener
      // @ts-ignore - Accessing private method for testing
      const stopListenerSpy = jest.spyOn(service, 'stopMockDataListener');
      
      // Disconnect
      service.disconnect();
      
      // Should stop mock data listener
      expect(stopListenerSpy).toHaveBeenCalled();
    });
  });

  describe('utility methods', () => {
    it('should report connection status correctly', async () => {
      // Initially should not be connected
      expect(service.isConnected()).toBe(false);
      
      // Connect
      await service.connect();
      
      // Should now be connected
      expect(service.isConnected()).toBe(true);
      
      // Disconnect
      service.disconnect();
      
      // Should now be disconnected
      expect(service.isConnected()).toBe(false);
    });
  });
}); 
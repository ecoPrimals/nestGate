import { WebSocketService, WebSocketMessageType } from '../../services/websocket.service';

// Mock WebSocket implementation
class MockWebSocket {
  onopen: (() => void) | null = null;
  onmessage: ((event: { data: string }) => void) | null = null;
  onclose: (() => void) | null = null;
  onerror: ((error: any) => void) | null = null;
  readyState = WebSocket.OPEN;
  close = jest.fn();
  send = jest.fn();

  constructor(url: string) {}

  // Simulate receiving a message
  simulateMessage(data: any) {
    if (this.onmessage) {
      this.onmessage({ data: JSON.stringify(data) });
    }
  }

  // Simulate connection open
  simulateOpen() {
    if (this.onopen) {
      this.onopen();
    }
  }

  // Simulate connection close
  simulateClose() {
    if (this.onclose) {
      this.onclose();
    }
  }

  // Simulate error
  simulateError(error: any) {
    if (this.onerror) {
      this.onerror(error);
    }
  }
}

// Mock environment utilities
jest.mock('../../utils/env', () => ({
  useMockData: jest.fn().mockReturnValue(false),
  getWebSocketUrl: jest.fn().mockReturnValue('ws://localhost:8080/ws')
}));

// Replace global WebSocket with our mock
global.WebSocket = MockWebSocket as any;

describe('WebSocketService', () => {
  let wsService: WebSocketService;
  let latestMockSocket: MockWebSocket;

  beforeEach(() => {
    // Clear all mocks
    jest.clearAllMocks();
    
    // Create a new instance (singleton pattern makes this tricky)
    // @ts-ignore - accessing private property for testing
    WebSocketService['instance'] = null;
    wsService = WebSocketService.getInstance();
    
    // Get the latest mock socket instance
    // @ts-ignore - accessing private property for testing
    latestMockSocket = wsService['socket'] as MockWebSocket;
  });

  test('should initialize with default configuration', () => {
    expect(wsService).toBeDefined();
    // @ts-ignore - accessing private property for testing
    expect(wsService['url']).toBe('ws://localhost:8080/ws');
  });
  
  test('should connect successfully to WebSocket server', () => {
    latestMockSocket.simulateOpen();
    expect(wsService.isConnected()).toBe(true);
  });
  
  test('should handle connection close', () => {
    latestMockSocket.simulateOpen();
    expect(wsService.isConnected()).toBe(true);
    
    latestMockSocket.simulateClose();
    expect(wsService.isConnected()).toBe(false);
  });
  
  test('should handle string command in send method', () => {
    latestMockSocket.simulateOpen();
    
    wsService.send('get_disks');
    
    expect(latestMockSocket.send).toHaveBeenCalled();
    const sentData = JSON.parse(latestMockSocket.send.mock.calls[0][0]);
    expect(sentData.type).toBe(WebSocketMessageType.GET_DISKS);
  });
  
  test('should handle object in send method', () => {
    latestMockSocket.simulateOpen();
    
    const message = {
      type: WebSocketMessageType.SYSTEM_HEALTH,
      data: { cpu: 50, memory: 60 }
    };
    
    wsService.send(message);
    
    expect(latestMockSocket.send).toHaveBeenCalled();
    const sentData = JSON.parse(latestMockSocket.send.mock.calls[0][0]);
    expect(sentData.type).toBe(WebSocketMessageType.SYSTEM_HEALTH);
    expect(sentData.data).toEqual({ cpu: 50, memory: 60 });
  });
  
  test('should handle string command with data in send method', () => {
    latestMockSocket.simulateOpen();
    
    const data = {
      pool_name: 'test_pool',
      time_range: '7d'
    };
    
    wsService.send('get_pool_performance', data);
    
    expect(latestMockSocket.send).toHaveBeenCalled();
    const sentData = JSON.parse(latestMockSocket.send.mock.calls[0][0]);
    expect(sentData.type).toBe(WebSocketMessageType.GET_POOL_PERFORMANCE);
    expect(sentData.data).toEqual(data);
  });
  
  test('should handle subscription to message types', (done) => {
    latestMockSocket.simulateOpen();
    
    // Subscribe to system health messages
    wsService.subscribe(WebSocketMessageType.SYSTEM_HEALTH, (message) => {
      expect(message.type).toBe(WebSocketMessageType.SYSTEM_HEALTH);
      expect(message.data.cpu).toBe(75);
      done();
    });
    
    // Simulate receiving a message
    latestMockSocket.simulateMessage({
      type: WebSocketMessageType.SYSTEM_HEALTH,
      data: { cpu: 75, memory: 80 },
      timestamp: Date.now()
    });
  });
  
  test('should handle subscription using string message type', (done) => {
    latestMockSocket.simulateOpen();
    
    // Subscribe using string type
    wsService.subscribe('notification', (message) => {
      expect(message.type).toBe(WebSocketMessageType.NOTIFICATION);
      expect(message.data.message).toBe('Test notification');
      done();
    });
    
    // Simulate receiving a message
    latestMockSocket.simulateMessage({
      type: WebSocketMessageType.NOTIFICATION,
      data: { message: 'Test notification', type: 'info' },
      timestamp: Date.now()
    });
  });
  
  test('should handle unsubscribe correctly', () => {
    latestMockSocket.simulateOpen();
    
    const handler = jest.fn();
    
    // Subscribe and get unsubscribe function
    const unsubscribe = wsService.subscribe(WebSocketMessageType.SYSTEM_HEALTH, handler);
    
    // Simulate message - handler should be called
    latestMockSocket.simulateMessage({
      type: WebSocketMessageType.SYSTEM_HEALTH,
      data: { cpu: 50 },
      timestamp: Date.now()
    });
    
    expect(handler).toHaveBeenCalledTimes(1);
    
    // Unsubscribe
    unsubscribe();
    
    // Simulate another message - handler should not be called
    latestMockSocket.simulateMessage({
      type: WebSocketMessageType.SYSTEM_HEALTH,
      data: { cpu: 60 },
      timestamp: Date.now()
    });
    
    expect(handler).toHaveBeenCalledTimes(1); // Still just once
  });
}); 
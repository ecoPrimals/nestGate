// Mock implementation of WebSocket
class MockWebSocket {
  static instances: MockWebSocket[] = [];
  private listeners: Record<string, Set<Function>> = {
    open: new Set(),
    message: new Set(),
    error: new Set(),
    close: new Set(),
  };
  public readyState: number = 0; // 0: CONNECTING, 1: OPEN, 2: CLOSING, 3: CLOSED
  public url: string;
  public sentMessages: string[] = [];

  static get CONNECTING() { return 0; }
  static get OPEN() { return 1; }
  static get CLOSING() { return 2; }
  static get CLOSED() { return 3; }

  constructor(url: string) {
    this.url = url;
    MockWebSocket.instances.push(this);

    // Simulate connection after a short delay
    setTimeout(() => {
      this.readyState = MockWebSocket.OPEN;
      this.listeners['open'].forEach(listener => listener({}));
    }, 10);
  }

  // Register event listeners
  addEventListener(event: string, callback: Function) {
    if (this.listeners[event]) {
      this.listeners[event].add(callback);
    }
  }

  // Remove event listeners
  removeEventListener(event: string, callback: Function) {
    if (this.listeners[event]) {
      this.listeners[event].delete(callback);
    }
  }

  // Set event handlers
  set onopen(callback: (event: any) => void) {
    this.listeners['open'].clear();
    this.listeners['open'].add(callback);
  }

  set onmessage(callback: (event: { data: string }) => void) {
    this.listeners['message'].clear();
    this.listeners['message'].add(callback);
  }

  set onerror(callback: (event: any) => void) {
    this.listeners['error'].clear();
    this.listeners['error'].add(callback);
  }

  set onclose(callback: (event: any) => void) {
    this.listeners['close'].clear();
    this.listeners['close'].add(callback);
  }

  // Send method
  send(data: string) {
    this.sentMessages.push(data);
  }

  // Close the connection
  close() {
    this.readyState = MockWebSocket.CLOSED;
    this.listeners['close'].forEach(listener => listener({ code: 1000 }));
  }

  // Test helper to simulate receiving a message
  receiveMessage(data: string) {
    this.listeners['message'].forEach(listener => listener({ data }));
  }

  // Test helper to simulate an error
  simulateError(errorData: any) {
    this.listeners['error'].forEach(listener => listener(errorData));
  }

  // Reset all mock instances
  static resetMock() {
    MockWebSocket.instances = [];
  }
}

export default MockWebSocket; 
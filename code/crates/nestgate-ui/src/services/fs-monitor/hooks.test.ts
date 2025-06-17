import { renderHook, act, waitFor } from '@testing-library/react';
import { useFsMonitor, useWatchedDirectories, useFsMonitorWebSocket } from './hooks';
import { FsMonitorClient } from './client';
import { EventKind } from './types';

// Mock the client class
jest.mock('./client', () => {
  return {
    FsMonitorClient: jest.fn().mockImplementation(() => ({
      connect: jest.fn().mockResolvedValue(true),
      getWatchedDirectories: jest.fn().mockResolvedValue(['/test/dir1', '/test/dir2']),
      watchDirectory: jest.fn().mockResolvedValue(true),
      unwatchDirectory: jest.fn().mockResolvedValue(true),
      getWebSocketUrl: jest.fn().mockReturnValue('ws://localhost:9500/ws')
    }))
  };
});

// Mock WebSocket
class MockWebSocket {
  static CONNECTING = 0;
  static OPEN = 1;
  static CLOSING = 2;
  static CLOSED = 3;

  readyState = MockWebSocket.CLOSED;
  onopen: any = null;
  onmessage: any = null;
  onerror: any = null;
  onclose: any = null;

  constructor(public url: string) {
    setTimeout(() => {
      this.readyState = MockWebSocket.OPEN;
      if (this.onopen) this.onopen();
    }, 10);
  }

  close() {
    this.readyState = MockWebSocket.CLOSED;
    if (this.onclose) this.onclose();
  }
}

// @ts-ignore
global.WebSocket = MockWebSocket;

describe('Filesystem Monitor Hooks', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    jest.useFakeTimers();
  });

  afterEach(() => {
    jest.useRealTimers();
  });

  describe('useFsMonitor', () => {
    test('should initialize and connect to the client', async () => {
      const { result } = renderHook(() => useFsMonitor());
      
      // Initially loading
      expect(result.current.isLoading).toBe(true);
      expect(result.current.isConnected).toBe(false);
      
      // Wait for connect to complete
      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });
      
      expect(result.current.isConnected).toBe(true);
      expect(result.current.error).toBe(null);
      expect(result.current.client).toBeInstanceOf(FsMonitorClient);
      expect(result.current.client.connect).toHaveBeenCalled();
    });

    test('should auto-reconnect on interval if not connected', async () => {
      const mockConnect = jest.fn()
        .mockResolvedValueOnce(false)
        .mockResolvedValueOnce(true);
      
      (FsMonitorClient as jest.Mock).mockImplementationOnce(() => ({
        connect: mockConnect,
        getWatchedDirectories: jest.fn(),
        watchDirectory: jest.fn(),
        unwatchDirectory: jest.fn(),
        getWebSocketUrl: jest.fn()
      }));
      
      const { result } = renderHook(() => useFsMonitor());
      
      // Initially loading
      expect(result.current.isLoading).toBe(true);
      
      // Wait for first connect attempt to complete
      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });
      
      expect(result.current.isConnected).toBe(false);
      expect(mockConnect).toHaveBeenCalledTimes(1);
      
      // Advance timer to trigger reconnect
      act(() => {
        jest.advanceTimersByTime(10000);
      });
      
      // Wait for second connect attempt to complete
      await waitFor(() => {
        expect(result.current.isConnected).toBe(true);
      });
      
      expect(mockConnect).toHaveBeenCalledTimes(2);
    });
  });

  describe('useWatchedDirectories', () => {
    test('should fetch watched directories when connected', async () => {
      const { result } = renderHook(() => useWatchedDirectories());
      
      // Initially loading
      expect(result.current.isLoading).toBe(true);
      
      // Wait for connect and fetch to complete
      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });
      
      expect(result.current.directories).toEqual(['/test/dir1', '/test/dir2']);
      expect(result.current.error).toBe(null);
    });

    test('should watch a directory successfully', async () => {
      const { result } = renderHook(() => useWatchedDirectories());
      
      // Wait for initial loading
      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });
      
      let success = false;
      
      await act(async () => {
        success = await result.current.watchDirectory({
          path: '/test/new_dir',
          recursive: true
        });
      });
      
      expect(success).toBe(true);
      expect(result.current.isLoading).toBe(false);
    });

    test('should unwatch a directory successfully', async () => {
      const { result } = renderHook(() => useWatchedDirectories());
      
      // Wait for initial loading
      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
      });
      
      let success = false;
      
      await act(async () => {
        success = await result.current.unwatchDirectory('/test/dir1');
      });
      
      expect(success).toBe(true);
      expect(result.current.isLoading).toBe(false);
    });
  });

  describe('useFsMonitorWebSocket', () => {
    test('should connect to WebSocket when client is connected', async () => {
      const { result } = renderHook(() => useFsMonitorWebSocket());
      
      // Wait for the hook to finish initializing
      await waitFor(() => {
        expect(result.current.isConnecting).toBe(true);
      });
      
      // Fast-forward timer to let the mock WebSocket connect
      act(() => {
        jest.advanceTimersByTime(50);
      });
      
      // Wait for WebSocket connection
      await waitFor(() => {
        expect(result.current.wsConnected).toBe(true);
      });
      
      expect(result.current.isConnecting).toBe(false);
      expect(result.current.events).toEqual([]);
    });

    test('should receive WebSocket events', async () => {
      const { result } = renderHook(() => useFsMonitorWebSocket());
      
      // Wait for the hook to finish initializing and WebSocket to connect
      await waitFor(() => {
        expect(result.current.isConnecting).toBe(true);
      });
      
      // Fast-forward timer to let the mock WebSocket connect
      act(() => {
        jest.advanceTimersByTime(50);
      });
      
      await waitFor(() => {
        expect(result.current.wsConnected).toBe(true);
      });
      
      // Send a mock event through the WebSocket
      act(() => {
        const wsInstance = result.current.connect as unknown as MockWebSocket;
        if (wsInstance && wsInstance.onmessage) {
          wsInstance.onmessage({
            data: JSON.stringify({
              kind: EventKind.Create,
              path: '/test/new_file.txt',
              timestamp: new Date().toISOString(),
              isDirectory: false
            })
          });
        }
      });
      
      // Check that the event was received
      expect(result.current.events.length).toBe(1);
      expect(result.current.events[0].path).toBe('/test/new_file.txt');
    });

    test('should clear events', async () => {
      const { result } = renderHook(() => useFsMonitorWebSocket());
      
      // Wait for the hook to finish initializing and WebSocket to connect
      await waitFor(() => {
        expect(result.current.isConnecting).toBe(true);
      });
      
      // Fast-forward timer to let the mock WebSocket connect
      act(() => {
        jest.advanceTimersByTime(50);
      });
      
      await waitFor(() => {
        expect(result.current.wsConnected).toBe(true);
      });
      
      // Add a mock event
      act(() => {
        const wsInstance = result.current.connect as unknown as MockWebSocket;
        if (wsInstance && wsInstance.onmessage) {
          wsInstance.onmessage({
            data: JSON.stringify({
              kind: EventKind.Create,
              path: '/test/new_file.txt',
              timestamp: new Date().toISOString(),
              isDirectory: false
            })
          });
        }
      });
      
      // Verify the event was added
      expect(result.current.events.length).toBe(1);
      
      // Clear events
      act(() => {
        result.current.clearEvents();
      });
      
      // Verify events were cleared
      expect(result.current.events.length).toBe(0);
    });
  });
}); 
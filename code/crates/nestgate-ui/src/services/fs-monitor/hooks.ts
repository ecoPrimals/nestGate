/**
 * React hooks for using the Filesystem Monitor client
 */

import { useState, useEffect, useCallback, useRef } from 'react';
import { FsMonitorClient, FsMonitorClientConfig } from './client';
import { 
  FsEvent,
  EventKind,
  WatchRequest
} from './types';

// Re-export types for convenience
export { EventKind };

// Default config for Filesystem Monitor client
const defaultConfig: FsMonitorClientConfig = {
  baseUrl: process.env.REACT_APP_FS_MONITOR_URL || '', // Must come from Port Manager
  timeout: 5000,
  autoReconnect: true,
  maxReconnectAttempts: 5,
};

/**
 * Hook for using the Filesystem Monitor client
 */
export function useFsMonitor(config: Partial<FsMonitorClientConfig> = {}) {
  const [client] = useState<FsMonitorClient>(() => 
    new FsMonitorClient({
      ...defaultConfig,
      ...config,
    })
  );
  const [isConnected, setIsConnected] = useState<boolean>(false);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<Error | null>(null);
  
  // Connect to Filesystem Monitor
  const connect = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    
    try {
      const connected = await client.connect();
      setIsConnected(connected);
      return connected;
    } catch (err) {
      setError(err instanceof Error ? err : new Error(String(err)));
      setIsConnected(false);
      return false;
    } finally {
      setIsLoading(false);
    }
  }, [client]);
  
  // Connect on mount
  useEffect(() => {
    connect();
    
    // Reconnect on interval
    const intervalId = setInterval(() => {
      if (!isConnected) {
        connect();
      }
    }, 10000);
    
    return () => clearInterval(intervalId);
  }, [connect, isConnected]);
  
  return {
    client,
    isConnected,
    isLoading,
    error,
    connect,
  };
}

/**
 * Hook for working with watched directories
 */
export function useWatchedDirectories() {
  const { client, isConnected, isLoading: isClientLoading, error: clientError } = useFsMonitor();
  const [directories, setDirectories] = useState<string[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<Error | null>(null);
  
  // Fetch watched directories
  const fetchDirectories = useCallback(async () => {
    if (!isConnected) return;
    
    setIsLoading(true);
    setError(null);
    
    try {
      const dirs = await client.getWatchedDirectories();
      setDirectories(dirs);
    } catch (err) {
      setError(err instanceof Error ? err : new Error(String(err)));
    } finally {
      setIsLoading(false);
    }
  }, [client, isConnected]);
  
  // Watch a directory
  const watchDirectory = useCallback(async (request: WatchRequest) => {
    if (!isConnected) return false;
    
    try {
      const success = await client.watchDirectory(request);
      if (success) {
        await fetchDirectories(); // Refresh directories
      }
      return success;
    } catch (err) {
      setError(err instanceof Error ? err : new Error(String(err)));
      return false;
    }
  }, [client, fetchDirectories, isConnected]);
  
  // Unwatch a directory
  const unwatchDirectory = useCallback(async (path: string) => {
    if (!isConnected) return false;
    
    try {
      const success = await client.unwatchDirectory(path);
      if (success) {
        await fetchDirectories(); // Refresh directories
      }
      return success;
    } catch (err) {
      setError(err instanceof Error ? err : new Error(String(err)));
      return false;
    }
  }, [client, fetchDirectories, isConnected]);
  
  // Fetch directories on mount and when connection changes
  useEffect(() => {
    if (isConnected) {
      fetchDirectories();
      
      // Refresh directories on interval
      const intervalId = setInterval(() => {
        fetchDirectories();
      }, 10000);
      
      return () => clearInterval(intervalId);
    }
    
    // Return empty function for cleanup when not connected
    return () => {};
  }, [isConnected, fetchDirectories]);
  
  return {
    directories,
    isLoading: isLoading || isClientLoading,
    error: error || clientError,
    fetchDirectories,
    watchDirectory,
    unwatchDirectory,
  };
}

/**
 * Hook for handling filesystem events via WebSocket
 */
export function useFsMonitorWebSocket() {
  const { client, isConnected } = useFsMonitor();
  const [events, setEvents] = useState<FsEvent[]>([]);
  const [isConnecting, setIsConnecting] = useState<boolean>(false);
  const [wsConnected, setWsConnected] = useState<boolean>(false);
  const [error, setError] = useState<Error | null>(null);
  const wsRef = useRef<WebSocket | null>(null);
  const maxEvents = 100; // Maximum number of events to keep
  
  // Connect to WebSocket
  const connect = useCallback(() => {
    if (!isConnected || wsRef.current?.readyState === WebSocket.OPEN) return;
    
    setIsConnecting(true);
    setError(null);
    
    try {
      const wsUrl = client.getWebSocketUrl();
      const ws = new WebSocket(wsUrl);
      
      ws.onopen = () => {
        setWsConnected(true);
        setIsConnecting(false);
        console.log('Filesystem monitor WebSocket connected');
      };
      
      ws.onmessage = (event) => {
        try {
          const fsEvent = JSON.parse(event.data) as FsEvent;
          setEvents(prev => {
            const newEvents = [fsEvent, ...prev];
            // Keep only the most recent events
            if (newEvents.length > maxEvents) {
              return newEvents.slice(0, maxEvents);
            }
            return newEvents;
          });
        } catch (err) {
          console.error('Error parsing filesystem event:', err);
        }
      };
      
      ws.onerror = (event) => {
        console.error('Filesystem monitor WebSocket error:', event);
        setError(new Error('WebSocket connection error'));
        setWsConnected(false);
        setIsConnecting(false);
      };
      
      ws.onclose = () => {
        console.log('Filesystem monitor WebSocket closed');
        setWsConnected(false);
        setIsConnecting(false);
      };
      
      wsRef.current = ws;
    } catch (err) {
      setError(err instanceof Error ? err : new Error(String(err)));
      setWsConnected(false);
      setIsConnecting(false);
    }
  }, [client, isConnected]);
  
  // Disconnect WebSocket
  const disconnect = useCallback(() => {
    if (wsRef.current) {
      wsRef.current.close();
      wsRef.current = null;
      setWsConnected(false);
    }
  }, []);
  
  // Clear events
  const clearEvents = useCallback(() => {
    setEvents([]);
  }, []);
  
  // Connect when client is connected
  useEffect(() => {
    if (isConnected && !wsConnected && !isConnecting) {
      connect();
    }
    
    return () => {
      disconnect();
    };
  }, [isConnected, wsConnected, isConnecting, connect, disconnect]);
  
  return {
    events,
    wsConnected,
    isConnecting,
    error,
    connect,
    disconnect,
    clearEvents,
  };
} 
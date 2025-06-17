import { useEffect, useState, useCallback } from 'react';
import { 
  WebSocketService, 
  WebSocketMessage, 
  WebSocketMessageType,
  ConnectionStatus,
  MockModeReason
} from '../services/websocket.service';
import { DataSourceType } from '../utils/env';

// Define interface for subscription options with better generic support
export interface MessageSubscription<T = unknown> {
  type: WebSocketMessageType | string;
  handler: (message: WebSocketMessage<T>) => void;
}

// Define options for the hook
export interface WebSocketOptions {
  autoConnect?: boolean;
  subscriptions?: MessageSubscription<any>[];
  onError?: (error: Error) => void;
  onMockModeChange?: (isMock: boolean, reason: MockModeReason) => void;
}

/**
 * React hook for WebSocket integration
 */
function useWebSocket(options: WebSocketOptions = {}) {
  const [connected, setConnected] = useState(false);
  const [dataSource, setDataSource] = useState<DataSourceType>(DataSourceType.LIVE);
  const [mockReason, setMockReason] = useState<MockModeReason>(MockModeReason.NONE);
  
  const { 
    autoConnect = true, 
    subscriptions = [], 
    onError, 
    onMockModeChange 
  } = options;

  // Initialize WebSocket service
  const wsService = WebSocketService.getInstance();

  // Update data source information
  const updateDataSourceInfo = useCallback(() => {
    const currentDataSource = wsService.getDataSourceType();
    const currentMockReason = wsService.getMockModeReason();
    
    setDataSource(currentDataSource);
    setMockReason(currentMockReason);
    
    if (onMockModeChange && currentDataSource === DataSourceType.MOCK) {
      onMockModeChange(true, currentMockReason);
    }
  }, [wsService, onMockModeChange]);

  // Connect to the WebSocket server
  const connect = useCallback(async () => {
    try {
      await wsService.connect();
      updateDataSourceInfo();
    } catch (error) {
      if (onError && error instanceof Error) {
        onError(error);
      } else if (onError) {
        onError(new Error('Failed to connect to WebSocket server'));
      }
    }
  }, [wsService, onError, updateDataSourceInfo]);

  // Disconnect from the WebSocket server
  const disconnect = useCallback(() => {
    wsService.disconnect();
  }, [wsService]);

  // Send a message to the WebSocket server
  const send = useCallback((command: string | Omit<WebSocketMessage<unknown>, 'timestamp'>, data?: unknown) => {
    wsService.send(command, data);
  }, [wsService]);

  // Handle connection status updates using the ConnectionStatus interface
  const handleConnectionStatus = useCallback((message: WebSocketMessage<ConnectionStatus>) => {
    setConnected(message.data.connected);
    updateDataSourceInfo();
  }, [updateDataSourceInfo]);

  // Handle error updates
  const handleErrorMessage = useCallback((message: WebSocketMessage<any>) => {
    if (message.data && message.data.code === 'CONNECTION_FAILED') {
      updateDataSourceInfo();
    }
  }, [updateDataSourceInfo]);

  // Subscribe to messages
  useEffect(() => {
    // Initialize data source info
    updateDataSourceInfo();
    
    // Subscribe to connection status with proper type
    const unsubscribeConnection = wsService.subscribe<ConnectionStatus>(
      'connection', 
      handleConnectionStatus
    );
    
    // Subscribe to error messages to detect fallback mode
    const unsubscribeError = wsService.subscribe(
      WebSocketMessageType.ERROR,
      handleErrorMessage
    );

    // Subscribe to all requested message types with proper generics
    const unsubscribeFunctions = subscriptions.map(subscription => {
      // Using type assertion here to preserve the generic type information
      return wsService.subscribe(
        subscription.type, 
        subscription.handler
      );
    });

    // Connect if autoConnect is true
    if (autoConnect) {
      connect();
    }

    // Cleanup subscriptions on unmount
    return () => {
      unsubscribeConnection();
      unsubscribeError();
      unsubscribeFunctions.forEach(unsubscribe => unsubscribe());
    };
  }, [
    wsService,
    autoConnect,
    subscriptions,
    connect,
    handleConnectionStatus,
    handleErrorMessage,
    updateDataSourceInfo
  ]);

  return {
    connected,
    dataSource,
    mockReason,
    isLive: dataSource === DataSourceType.LIVE,
    isMock: dataSource === DataSourceType.MOCK,
    connect,
    disconnect,
    send
  };
}

export default useWebSocket; 
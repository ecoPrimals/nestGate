/**
 * WebSocket Service for handling real-time data connections
 */

import { WS_BASE_URL, getWebSocketUrl } from '../config';
import { shouldUseMockData, DataSourceType, getDataSourceType } from '../utils/env';

export enum WebSocketMessageType {
  SYSTEM_METRICS = 'system_metrics',
  ZFS_METRICS = 'zfs_metrics',
  DISK_METRICS = 'disk_metrics',
  CPU_METRICS = 'cpu_metrics',
  MEMORY_METRICS = 'memory_metrics',
  NETWORK_METRICS = 'network_metrics',
  NOTIFICATION = 'notification',
  ERROR = 'error',
  POOL_STATUS = 'pool_status',
  TASK_STATUS = 'task_status',
  CONNECTION = 'connection',
  ALERT = 'alert',
  DISKS = 'disks',
  POOLS = 'pools',
  POOL_PERFORMANCE = 'pool_performance',
  POOL_PERFORMANCE_HISTORY = 'pool_performance_history',
  PERFORMANCE_METRICS = 'performance_metrics',
  SYSTEM_STATUS = 'system_status',
  SERVICES = 'services',
  SYSTEM_HEALTH = 'SystemHealth',
  PERFORMANCE = 'Performance',
  DISK_HEALTH = 'DiskHealth',
  ZFS_POOL = 'ZfsPool',
  TELEMETRY = 'TELEMETRY',
  BACKUP_STATUS = 'BACKUP_STATUS',
  ZFS_STATUS = 'ZFS_STATUS',
  GET_DISKS = 'get_disks',
  GET_POOLS = 'get_pools',
  GET_POOL_PERFORMANCE = 'get_pool_performance',
  GET_POOL_PERFORMANCE_HISTORY = 'get_pool_performance_history',
  GET_SYSTEM_STATUS = 'get_system_status',
  GET_PERFORMANCE_METRICS = 'get_performance_metrics',
  GET_SERVICES = 'get_services',
  REFRESH_SERVICES = 'refresh_services'
}

export interface WebSocketMessage<T = unknown> {
  type: WebSocketMessageType;
  timestamp: number;
  data: T;
}

export interface SystemMetrics {
  cpuUsage: number;
  memoryUsage: number;
  uptime: number;
  loadAverage: number[];
  diskIO: {
    read: number;
    write: number;
  };
  networkIO: {
    received: number;
    sent: number;
  };
  temperature: number;
}

export interface DiskMetrics {
  diskId: string;
  temperature: number;
  health: 'good' | 'warning' | 'critical';
  status: 'online' | 'offline' | 'degraded';
  readErrors: number;
  writeErrors: number;
  iops: {
    read: number;
    write: number;
  };
  throughput: {
    read: number;
    write: number;
  };
}

export interface ZFSMetrics {
  poolName: string;
  status: 'online' | 'degraded' | 'faulted' | 'offline' | 'unavailable' | 'removed';
  health: 'good' | 'warning' | 'critical';
  capacityUsed: number;
  capacityTotal: number;
  diskErrors: {
    read: number;
    write: number;
    checksum: number;
  };
  children: Array<{
    name: string;
    status: 'online' | 'degraded' | 'faulted' | 'offline' | 'unavailable' | 'removed';
    errors: {
      read: number;
      write: number;
      checksum: number;
    };
  }>;
  properties?: {
    dedupratio?: string;
    compression?: string;
    readonly?: string;
    atime?: string;
    recordsize?: string;
  };
  performance?: {
    iops?: {
      read: number;
      write: number;
    };
    throughput?: {
      read: number;
      write: number;
    };
    latency?: {
      read: number;
      write: number;
      sync: number;
    };
    cache?: {
      arcHits: number;
      arcMisses: number;
      l2arcHits: number;
      l2arcMisses: number;
      size: number;
    };
  };
}

// Define common interfaces for message data types
export interface ConnectionStatus {
  status: 'connected' | 'disconnected';
  connected: boolean;
}

export interface Disk {
  id: string;
  name: string;
  model: string;
  serial: string;
  size: number;
  health: string;
  temperature: number;
  status: string;
  smartAttributes?: SmartAttribute[];
  lastScan?: string;
  powerOnHours?: number;
  interface?: string;
}

export interface SmartAttribute {
  id: number;
  name: string;
  value: number;
  worst: number;
  threshold: number;
  status: 'GOOD' | 'WARNING' | 'FAILED';
  raw: string;
}

export interface Pool {
  name: string;
  status: string;
  health: string;
  capacity: {
    used: number;
    available: number;
    total: number;
  };
}

export interface PoolPerformance {
  pool_name: string;
  timestamp: number;
  read_iops: number;
  write_iops: number;
  read_throughput: number;
  write_throughput: number;
  latency: {
    read: number;
    write: number;
    sync: number;
  };
  cpu_usage: number;
  memory_usage: number;
  metrics?: PerformanceMetric[];
}

export interface PerformanceMetric {
  timestamp: number;
  read_iops: number;
  write_iops: number;
  read_throughput: number;
  write_throughput: number;
  latency: {
    read: number;
    write: number;
    sync: number;
  };
  cpu_usage: number;
  memory_usage: number;
}

export interface Service {
  name: string;
  status: 'running' | 'stopped' | 'error';
  uptime: number;
  memory_usage: number;
  cpu_usage: number;
}

export interface SystemStatus {
  status: string;
  uptime: number;
  cpu_usage: number;
  memory_usage: number;
  storage_usage: number;
  temperature: number;
  lastUpdate?: string;
}

export interface PerformanceMetrics {
  cpu: number;
  memory: number;
  disk_read: number;
  disk_write: number;
  network_in: number;
  network_out: number;
  lastUpdate?: string;
  cpu_usage?: number;
  memory_usage?: number;
  disk_usage?: number;
  network_activity?: number;
  read_latency?: number;
  write_latency?: number;
}

// Type guard utilities to check message data types
export function isConnectionStatus(data: unknown): data is ConnectionStatus {
  return (
    typeof data === 'object' && 
    data !== null && 
    'status' in data && 
    'connected' in data &&
    (typeof (data as ConnectionStatus).connected === 'boolean')
  );
}

export function isDisks(data: unknown): data is Disk[] {
  return (
    Array.isArray(data) && 
    (data.length === 0 || 
      ('id' in data[0] && 
       'status' in data[0] && 
       'health' in data[0]))
  );
}

export function isPools(data: unknown): data is Pool[] {
  return (
    Array.isArray(data) && 
    (data.length === 0 || 
      ('name' in data[0] && 
       'status' in data[0] && 
       'health' in data[0]))
  );
}

export function isPoolPerformance(data: unknown): data is PoolPerformance {
  return (
    typeof data === 'object' && 
    data !== null && 
    'pool_name' in data && 
    'read_iops' in data && 
    'write_iops' in data
  );
}

export function isSystemStatus(data: unknown): data is SystemStatus {
  return (
    typeof data === 'object' && 
    data !== null && 
    'status' in data && 
    'cpu_usage' in data && 
    'memory_usage' in data
  );
}

export function isPerformanceMetrics(data: unknown): data is PerformanceMetrics {
  return (
    typeof data === 'object' && 
    data !== null && 
    'cpu' in data && 
    'memory' in data && 
    'disk_read' in data && 
    'disk_write' in data
  );
}

export function isServices(data: unknown): data is Service[] {
  return (
    Array.isArray(data) && 
    (data.length === 0 || 
      ('name' in data[0] && 
       'status' in data[0] && 
       'uptime' in data[0]))
  );
}

// Update MessageHandler to be generic
type MessageHandler<T = unknown> = (message: WebSocketMessage<T>) => void;

export enum MockModeReason {
  DELIBERATE = 'DELIBERATE',  // Deliberately using mock mode (from environment)
  NONE = 'NONE'               // Not using mock mode
}

const isMockMode = (): boolean => {
  return shouldUseMockData('websocket');
};

export class WebSocketService {
  private static instance: WebSocketService | null = null;
  private socket: WebSocket | null = null;
  private connected = false;
  private mockMode = false;
  private mockModeReason: MockModeReason = MockModeReason.NONE;
  // Update handlers Map to support generic type information
  private handlers: Map<WebSocketMessageType | string | 'all', Set<MessageHandler<any>>> = new Map();
  private mockIntervalId: number | null = null;
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private reconnectDelay = 2000;
  private url: string = '';
  private isInitialized = false;

  // Private constructor to enforce singleton
  private constructor() {
    // Always use live mode
    this.mockMode = false;
    this.mockModeReason = MockModeReason.NONE;
    // Default URL that will be updated during connection
    this.url = WS_BASE_URL;
    
    console.log('WebSocketService initialized with real WebSocket');
    this.initConnection();
  }

  // Get singleton instance
  public static getInstance(): WebSocketService {
    if (!WebSocketService.instance) {
      WebSocketService.instance = new WebSocketService();
    }
    return WebSocketService.instance;
  }

  // Initialize connection with dynamic URL from port manager if available
  private async initConnection(): Promise<void> {
    try {
      // Try to get WebSocket URL from Port Manager if enabled
      this.url = await getWebSocketUrl();
      console.log(`Using WebSocket URL: ${this.url}`);
      this.connect();
      this.isInitialized = true;
    } catch (error) {
      console.error('Error initializing WebSocket connection:', error);
      // Fallback to default URL
      this.url = WS_BASE_URL;
      this.connect();
      this.isInitialized = true;
    }
  }

  // Connect to WebSocket server (changed from private to public to fix TS errors)
  public connect(): void {
    if (this.socket) {
      this.socket.close();
    }

    try {
      this.socket = new WebSocket(this.url);

      this.socket.onopen = () => {
        console.log('WebSocket connected to', this.url);
        this.connected = true;
        this.reconnectAttempts = 0;
        this.broadcastConnectionStatus(true);
      };

      this.socket.onmessage = (event) => {
        try {
          const message: WebSocketMessage = JSON.parse(event.data);
          this.handleMessage(message);
        } catch (error) {
          console.error('Error parsing WebSocket message:', error);
        }
      };

      this.socket.onclose = () => {
        console.log('WebSocket disconnected');
        this.connected = false;
        this.broadcastConnectionStatus(false);
        this.reconnect();
      };

      this.socket.onerror = (error) => {
        console.error('WebSocket error:', error);
        this.connected = false;
        this.broadcastConnectionStatus(false);
      };
    } catch (error) {
      console.error('Error creating WebSocket:', error);
      this.reconnect();
    }
  }

  // Wait for initialization to complete
  private async ensureInitialized(): Promise<void> {
    if (!this.isInitialized) {
      await new Promise<void>((resolve) => {
        const checkInterval = setInterval(() => {
          if (this.isInitialized) {
            clearInterval(checkInterval);
            resolve();
          }
        }, 100);
      });
    }
  }

  // Handle reconnection
  private reconnect(): void {
    if (this.reconnectAttempts < this.maxReconnectAttempts) {
      this.reconnectAttempts++;
      console.log(`Attempting to reconnect (${this.reconnectAttempts}/${this.maxReconnectAttempts})...`);
      setTimeout(() => {
        this.connect();
      }, this.reconnectDelay);
    } else {
      console.error('Max reconnect attempts reached, connection failed');
      // Do not switch to mock mode, just remain disconnected
      this.connected = false;
      
      // Broadcast connection failure message
      const errorMessage: WebSocketMessage = {
        type: WebSocketMessageType.ERROR,
        data: {
          message: 'Connection failed after max reconnect attempts',
          code: 'CONNECTION_FAILED',
          dataSource: DataSourceType.LIVE
        },
        timestamp: Date.now()
      };
      
      setTimeout(() => {
        this.handleMessage(errorMessage);
      }, 100);
    }
  }

  // Update the subscribe method to properly handle generic types
  public subscribe<T = unknown>(
    type: WebSocketMessageType | string | 'all', 
    handler: MessageHandler<T>
  ): () => void {
    // Convert string types to enum values
    let messageType: WebSocketMessageType | string | 'all';
    
    if (type === 'all') {
      messageType = 'all';
    } else if (typeof type === 'string' && type !== 'all') {
      // Convert string to enum if needed
      switch(type.toLowerCase()) {
        case 'connection':
          messageType = WebSocketMessageType.CONNECTION;
          break;
        case 'notification':
          messageType = WebSocketMessageType.NOTIFICATION;
          break;
        case 'telemetry':
          messageType = WebSocketMessageType.TELEMETRY;
          break;
        case 'system_health':
        case 'systemhealth':
          messageType = WebSocketMessageType.SYSTEM_HEALTH;
          break;
        case 'backup_status':
        case 'backupstatus':
          messageType = WebSocketMessageType.BACKUP_STATUS;
          break;
        case 'zfs_status':
        case 'zfsstatus':
          messageType = WebSocketMessageType.ZFS_STATUS;
          break;
        case 'disk_health':
        case 'diskhealth':
          messageType = WebSocketMessageType.DISK_HEALTH;
          break;
        case 'performance':
          messageType = WebSocketMessageType.PERFORMANCE;
          break;
        case 'error':
          messageType = WebSocketMessageType.ERROR;
          break;
        default:
          // If it's already a valid enum value in uppercase, use it
          messageType = type.toUpperCase() as WebSocketMessageType;
      }
    } else {
      // It's already an enum value
      messageType = type as WebSocketMessageType;
    }

    if (!this.handlers.has(messageType)) {
      this.handlers.set(messageType, new Set());
    }

    const handlers = this.handlers.get(messageType)!;
    handlers.add(handler);

    // Return function to unsubscribe
    return () => {
      if (this.handlers.has(messageType)) {
        const handlersSet = this.handlers.get(messageType)!;
        handlersSet.delete(handler);
      }
    };
  }

  // Convert string command to WebSocketMessage
  private createMessageFromCommand(command: string, data: unknown = {}): Omit<WebSocketMessage, 'timestamp'> {
    let messageType: WebSocketMessageType;
    
    // Map command strings to enum values
    switch (command) {
      case 'get_disks':
        messageType = WebSocketMessageType.GET_DISKS;
        break;
      case 'get_pools':
        messageType = WebSocketMessageType.GET_POOLS;
        break;
      case 'get_pool_performance':
        messageType = WebSocketMessageType.GET_POOL_PERFORMANCE;
        break;
      case 'get_pool_performance_history':
        messageType = WebSocketMessageType.GET_POOL_PERFORMANCE_HISTORY;
        break;
      case 'get_system_status':
        messageType = WebSocketMessageType.GET_SYSTEM_STATUS;
        break;
      case 'get_performance_metrics':
        messageType = WebSocketMessageType.GET_PERFORMANCE_METRICS;
        break;
      case 'get_services':
        messageType = WebSocketMessageType.GET_SERVICES;
        break;
      case 'refresh_services':
        messageType = WebSocketMessageType.REFRESH_SERVICES;
        break;
      default:
        // Try to map directly to enum if it exists
        messageType = command.toUpperCase() as WebSocketMessageType;
    }
    
    return {
      type: messageType,
      data: data
    };
  }

  // Send a message - enhanced to handle string commands
  public async send(messageOrCommand: Omit<WebSocketMessage, 'timestamp'> | string, data?: unknown): Promise<void> {
    // Wait for initialization to complete
    await this.ensureInitialized();
    
    let message: WebSocketMessage;
    
    if (typeof messageOrCommand === 'string') {
      const commandData = this.createMessageFromCommand(messageOrCommand, data);
      message = {
        ...commandData,
        timestamp: Date.now()
      };
    } else {
      message = {
        ...messageOrCommand,
        timestamp: Date.now()
      };
    }
    
    if (this.mockMode) {
      console.log('Mock mode active, not sending WebSocket message', message);
      return;
    }
    
    if (!this.socket || this.socket.readyState !== WebSocket.OPEN) {
      console.error('WebSocket not connected');
      return;
    }
    
    try {
      this.socket.send(JSON.stringify(message));
    } catch (error) {
      console.error('Error sending WebSocket message:', error);
    }
  }

  // Update handleMessage to properly handle typed data
  private handleMessage(message: WebSocketMessage): void {
    // Convert string timestamp to number if needed
    if (typeof message.timestamp === 'string') {
      message.timestamp = new Date(message.timestamp).getTime();
    }
    
    // Handle 'all' subscriptions
    const allHandlers = this.handlers.get('all');
    if (allHandlers) {
      allHandlers.forEach(handler => {
        try {
          handler(message);
        } catch (error) {
          console.error('Error in WebSocket message handler:', error);
        }
      });
    }

    // Handle specific message type subscriptions
    const typeHandlers = this.handlers.get(message.type);
    if (typeHandlers) {
      typeHandlers.forEach(handler => {
        try {
          handler(message);
        } catch (error) {
          console.error(`Error in ${message.type} message handler:`, error);
        }
      });
    }
  }

  // Update broadcastConnectionStatus to use ConnectionStatus interface
  private broadcastConnectionStatus(isConnected: boolean): void {
    const message: WebSocketMessage<ConnectionStatus> = {
      type: WebSocketMessageType.CONNECTION,
      data: { 
        connected: isConnected,
        status: isConnected ? 'connected' : 'disconnected'
      },
      timestamp: Date.now()
    };
    this.handleMessage(message);
  }

  // Check if connected
  public isConnected(): boolean {
    return this.connected;
  }

  // Force reconnection
  public reconnectWebSocket(): void {
    if (this.mockMode) {
      console.log('Already in mock mode, no reconnection needed');
      return;
    }
    
    this.reconnectAttempts = 0;
    this.connect();
  }

  // Force disconnect (for cleanup)
  public disconnect(): void {
    if (this.socket) {
      this.socket.close();
      this.socket = null;
    }

    if (this.mockIntervalId !== null) {
      window.clearInterval(this.mockIntervalId);
      this.mockIntervalId = null;
    }

    this.connected = false;
    this.broadcastConnectionStatus(false);
  }

  // Get the current data source type
  public getDataSourceType(): DataSourceType {
    // Always return LIVE
    return DataSourceType.LIVE;
  }
  
  // Get the mock mode reason
  public getMockModeReason(): MockModeReason {
    // Always return NONE since we're not using mock mode
    return MockModeReason.NONE;
  }

  // Replace toggleMockMode method to always use real connection
  public toggleMockMode(useMock: boolean): void {
    // In strict live mode, log a warning and ignore the request to use mock mode
    if (useMock) {
      console.warn('Mock mode is disabled in strict live mode');
    }
    
    // Ensure we're using a real connection
    if (this.mockIntervalId !== null) {
      window.clearInterval(this.mockIntervalId);
      this.mockIntervalId = null;
    }
    
    // Always ensure we're connected to real WebSocket
    if (!this.connected) {
      this.connect();
    }
  }
}

// Export singleton instance
export const webSocketService = WebSocketService.getInstance(); 
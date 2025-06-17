import axios from 'axios';
import { API_BASE_URL } from '../constants';

/**
 * System status interface
 */
export interface SystemStatus {
  hostname: string;
  uptime: number;
  load: number[];
  cpuUsage: number;
  memoryUsage: number;
  totalMemory: number;
  usedMemory: number;
  freeMemory: number;
  osName: string;
  osVersion: string;
  kernelVersion: string;
  cpuModel: string;
  cpuCores: number;
  systemTime: string;
  status: 'healthy' | 'warning' | 'critical';
}

/**
 * Disk health interface
 */
export interface DiskHealth {
  id: string;
  device: string;
  model: string;
  serial: string;
  size: number;
  temperature: number;
  status: 'healthy' | 'warning' | 'critical';
  powerOnHours: number;
  warningMessages?: string[];
  smartAttributes: Array<{
    id: number;
    name: string;
    value: number;
    threshold: number;
    status: 'healthy' | 'warning' | 'critical';
  }>;
}

/**
 * Performance metrics interface
 */
export interface PerformanceMetrics {
  timestamp: number;
  iops: {
    read: number;
    write: number;
    total: number;
    readHistory: Array<{ time: number; value: number }>;
    writeHistory: Array<{ time: number; value: number }>;
  };
  throughput: {
    read: number;
    write: number;
    total: number;
    readHistory: Array<{ time: number; value: number }>;
    writeHistory: Array<{ time: number; value: number }>;
  };
  latency: {
    read: number;
    write: number;
    average: number;
    readHistory: Array<{ time: number; value: number }>;
    writeHistory: Array<{ time: number; value: number }>;
  };
}

/**
 * Types for telemetry data
 */
export interface SystemHealth {
  status: 'healthy' | 'warning' | 'critical';
  cpu: number;
  memory: number;
  temperature: number;
  diskHealth: DiskHealthStatus[];
  cpuUsage?: number;
  memoryUsage?: number;
  hostname?: string;
  uptime?: number;
}

export interface DiskHealthStatus {
  device: string;
  status: 'healthy' | 'warning' | 'critical';
  temperature: number;
  smart: {
    reallocatedSectors: number;
    pendingSectors: number;
    uncorrectableSectors: number;
    powerOnHours: number;
    lastTestStatus: string;
  };
}

export interface NetworkStatus {
  interfaces: NetworkInterface[];
  throughput: {
    rx: number;
    tx: number;
  };
}

export interface NetworkInterface {
  name: string;
  status: 'up' | 'down';
  ipv4: string;
  ipv6: string;
  mac: string;
  speed: number | string;
  type: string;
  rx_bytes?: number;
  tx_bytes?: number;
  rx_packets?: number;
  tx_packets?: number;
}

export interface Alert {
  id: string;
  severity: 'info' | 'warning' | 'critical';
  message: string;
  timestamp: string;
  acknowledged: boolean;
  source: string;
}

type EventType = 'health' | 'network' | 'alerts' | string;
type ListenerCallback = (data: any) => void;
interface ListenersMap {
  [key: string]: ListenerCallback[];
}

/**
 * Service for telemetry and system status
 */
export class TelemetryService {
  private static API_URL = `${API_BASE_URL}/api/telemetry`;
  private static listeners: ListenersMap = {};
  private static socket: WebSocket | null = null;
  private static connected = false;
  private static reconnectTimer: ReturnType<typeof setTimeout> | null = null;
  private static reconnectAttempts = 0;
  private static maxReconnectAttempts = 10;
  
  // Mock data for development and testing
  private static mockSystemHealth = {
    hostname: 'nestgate-server',
    uptime: 1209600, // 14 days in seconds
    load: [0.75, 0.65, 0.55],
    cpuUsage: 35,
    memoryUsage: 45,
    totalMemory: 34359738368, // 32GB
    usedMemory: 15461882265, // ~14.4GB
    freeMemory: 18897856102, // ~17.6GB
    osName: 'NestGate OS',
    osVersion: '1.0.0',
    kernelVersion: '5.15.0-1023-custom',
    cpuModel: 'AMD Ryzen 9 5950X',
    cpuCores: 16,
    systemTime: new Date().toISOString(),
    status: 'healthy' as const
  };
  
  private static mockDiskHealth: Record<string, DiskHealth> = {
    disk1: {
      id: 'disk1',
      device: '/dev/sda',
      model: 'WDC WD101EFAX-68',
      serial: 'WD-M1H9876A',
      size: 10995116277760, // 10TB
      temperature: 38,
      status: 'healthy',
      powerOnHours: 8760, // 1 year
      smartAttributes: [
        {
          id: 1,
          name: 'Raw Read Error Rate',
          value: 100,
          threshold: 50,
          status: 'healthy'
        },
        {
          id: 5,
          name: 'Reallocated Sectors Count',
          value: 100,
          threshold: 50,
          status: 'healthy'
        },
        {
          id: 194,
          name: 'Temperature',
          value: 38,
          threshold: 50,
          status: 'healthy'
        }
      ]
    },
    disk2: {
      id: 'disk2',
      device: '/dev/sdb',
      model: 'WDC WD101EFAX-68',
      serial: 'WD-M1H9877B',
      size: 10995116277760, // 10TB
      temperature: 40,
      status: 'healthy',
      powerOnHours: 8760, // 1 year
      smartAttributes: [
        {
          id: 1,
          name: 'Raw Read Error Rate',
          value: 100,
          threshold: 50,
          status: 'healthy'
        },
        {
          id: 5,
          name: 'Reallocated Sectors Count',
          value: 100,
          threshold: 50,
          status: 'healthy'
        },
        {
          id: 194,
          name: 'Temperature',
          value: 40,
          threshold: 50,
          status: 'healthy'
        }
      ]
    },
    disk3: {
      id: 'disk3',
      device: '/dev/sdc',
      model: 'Seagate ST12000NM0009',
      serial: 'ZPS2MN5V',
      size: 12000138625024, // 12TB
      temperature: 42,
      status: 'healthy',
      powerOnHours: 4380, // 6 months
      smartAttributes: [
        {
          id: 1,
          name: 'Raw Read Error Rate',
          value: 100,
          threshold: 50,
          status: 'healthy'
        },
        {
          id: 5,
          name: 'Reallocated Sectors Count',
          value: 100,
          threshold: 50,
          status: 'healthy'
        },
        {
          id: 194,
          name: 'Temperature',
          value: 42,
          threshold: 50,
          status: 'healthy'
        }
      ]
    },
    disk4: {
      id: 'disk4',
      device: '/dev/sdd',
      model: 'Seagate ST12000NM0009',
      serial: 'ZPS2MN6V',
      size: 12000138625024, // 12TB
      temperature: 55,
      status: 'warning',
      powerOnHours: 4380, // 6 months
      warningMessages: ['Temperature above normal'],
      smartAttributes: [
        {
          id: 1,
          name: 'Raw Read Error Rate',
          value: 100,
          threshold: 50,
          status: 'healthy'
        },
        {
          id: 5,
          name: 'Reallocated Sectors Count',
          value: 100,
          threshold: 50,
          status: 'healthy'
        },
        {
          id: 194,
          name: 'Temperature',
          value: 55,
          threshold: 50,
          status: 'warning'
        }
      ]
    }
  };
  
  private static generateHistoryData(count: number, min: number, max: number): Array<{ time: number; value: number }> {
    const now = Date.now();
    const result = [];
    for (let i = 0; i < count; i++) {
      result.push({
        time: now - (count - i) * 60000, // Every minute
        value: Math.floor(Math.random() * (max - min + 1)) + min
      });
    }
    return result;
  }
  
  private static mockPerformanceMetrics: PerformanceMetrics = {
    timestamp: Date.now(),
    iops: {
      read: 1450,
      write: 850,
      total: 2300,
      readHistory: [],
      writeHistory: []
    },
    throughput: {
      read: 125 * 1024 * 1024, // 125 MB/s
      write: 85 * 1024 * 1024, // 85 MB/s
      total: 210 * 1024 * 1024, // 210 MB/s
      readHistory: [],
      writeHistory: []
    },
    latency: {
      read: 2.5, // ms
      write: 4.2, // ms
      average: 3.35, // ms
      readHistory: [],
      writeHistory: []
    }
  };
  
  // Initialize the history data without using a static block
  private static initMockHistoryData(): void {
    TelemetryService.mockPerformanceMetrics.iops.readHistory = TelemetryService.generateHistoryData(60, 1000, 2000);
    TelemetryService.mockPerformanceMetrics.iops.writeHistory = TelemetryService.generateHistoryData(60, 500, 1000);
    TelemetryService.mockPerformanceMetrics.throughput.readHistory = TelemetryService.generateHistoryData(60, 100 * 1024 * 1024, 150 * 1024 * 1024);
    TelemetryService.mockPerformanceMetrics.throughput.writeHistory = TelemetryService.generateHistoryData(60, 75 * 1024 * 1024, 100 * 1024 * 1024);
    TelemetryService.mockPerformanceMetrics.latency.readHistory = TelemetryService.generateHistoryData(60, 1, 5);
    TelemetryService.mockPerformanceMetrics.latency.writeHistory = TelemetryService.generateHistoryData(60, 2, 7);
  }
  
  // Call the initialization function immediately
  private static _initMockDataCalled = TelemetryService.initMockHistoryData();
  
  /**
   * Initialize the telemetry service
   */
  public static initialize(): void {
    this.setupWebSocket();
  }

  /**
   * Clean up resources
   */
  public static cleanup(): void {
    if (this.socket) {
      this.socket.close();
      this.socket = null;
    }

    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }

    this.connected = false;
    this.listeners = {};
  }

  /**
   * Get system health data
   */
  public static async getSystemHealth(): Promise<SystemHealth> {
    try {
      const response = await axios.get<SystemHealth>(`${this.API_URL}/health`);
      return response.data;
    } catch (error) {
      console.error('Error fetching system health:', error);
      return this.getMockSystemHealth();
    }
  }

  /**
   * Get network status data
   */
  public static async getNetworkStatus(): Promise<NetworkStatus> {
    try {
      const response = await axios.get<NetworkStatus>(`${this.API_URL}/network`);
      return response.data;
    } catch (error) {
      console.error('Error fetching network status:', error);
      return this.getMockNetworkStatus();
    }
  }

  /**
   * Get active system alerts
   */
  public static async getAlerts(): Promise<Alert[]> {
    try {
      const response = await axios.get<Alert[]>(`${this.API_URL}/alerts`);
      return response.data;
    } catch (error) {
      console.error('Error fetching alerts:', error);
      return this.getMockAlerts();
    }
  }

  /**
   * Acknowledge a system alert
   */
  public static async acknowledgeAlert(alertId: string): Promise<boolean> {
    try {
      await axios.post(`${this.API_URL}/alerts/${alertId}/acknowledge`);
      return true;
    } catch (error) {
      console.error('Error acknowledging alert:', error);
      return false;
    }
  }

  /**
   * Subscribe to real-time telemetry updates
   */
  public static subscribe(
    eventType: 'health' | 'network' | 'alerts',
    callback: (data: any) => void
  ): () => void {
    if (!this.listeners[eventType]) {
      this.listeners[eventType] = [];
    }

    this.listeners[eventType].push(callback);

    // Return unsubscribe function
    return () => {
      this.listeners[eventType] = this.listeners[eventType].filter(
        (cb) => cb !== callback
      );
    };
  }

  /**
   * Set up WebSocket connection
   */
  private static setupWebSocket(): void {
    if (this.socket) {
      return;
    }

    try {
      const wsUrl = `${API_BASE_URL.replace(/^http/, 'ws')}/api/telemetry/ws`;
      this.socket = new WebSocket(wsUrl);

      this.socket.onopen = () => {
        console.log('Telemetry WebSocket connected');
        this.connected = true;
        this.reconnectAttempts = 0;
      };

      this.socket.onmessage = (event: MessageEvent) => {
        try {
          const data = JSON.parse(event.data);
          if (data.type && this.listeners[data.type]) {
            this.listeners[data.type].forEach((callback: ListenerCallback) => callback(data.payload));
          }
        } catch (error) {
          console.error('Error parsing WebSocket message:', error);
        }
      };

      this.socket.onclose = () => {
        this.connected = false;
        this.handleReconnect();
      };

      this.socket.onerror = (error: Event) => {
        console.error('WebSocket error:', error);
        this.socket?.close();
      };
    } catch (error) {
      console.error('Error setting up WebSocket:', error);
      this.handleReconnect();
    }
  }

  /**
   * Handle WebSocket reconnection
   */
  private static handleReconnect(): void {
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
    }

    if (this.reconnectAttempts < this.maxReconnectAttempts) {
      this.reconnectAttempts++;
      const delay = Math.min(1000 * Math.pow(2, this.reconnectAttempts), 30000);
      
      this.reconnectTimer = setTimeout(() => {
        console.log(`Attempting to reconnect (${this.reconnectAttempts}/${this.maxReconnectAttempts})...`);
        this.setupWebSocket();
      }, delay);
    } else {
      console.error('Max reconnect attempts reached. Giving up.');
    }
  }

  /**
   * Get mock system health data for development/testing
   */
  private static getMockSystemHealth(): SystemHealth {
    return {
      status: 'healthy',
      cpu: 23.5,
      memory: 42.8,
      temperature: 38.2,
      diskHealth: [
        {
          device: '/dev/sda',
          status: 'healthy',
          temperature: 32.4,
          smart: {
            reallocatedSectors: 0,
            pendingSectors: 0,
            uncorrectableSectors: 0,
            powerOnHours: 8760,
            lastTestStatus: 'passed'
          }
        },
        {
          device: '/dev/sdb',
          status: 'warning',
          temperature: 41.7,
          smart: {
            reallocatedSectors: 2,
            pendingSectors: 1,
            uncorrectableSectors: 0,
            powerOnHours: 17520,
            lastTestStatus: 'warning'
          }
        }
      ]
    };
  }

  /**
   * Get mock network status data for development/testing
   */
  private static getMockNetworkStatus(): NetworkStatus {
    return {
      interfaces: [
        {
          name: 'eth0',
          status: 'up',
          ipv4: '192.168.1.10',
          ipv6: 'fe80::1234:5678:9abc:def0',
          mac: '00:11:22:33:44:55',
          speed: 1000,
          type: 'ethernet'
        },
        {
          name: 'eth1',
          status: 'down',
          ipv4: '',
          ipv6: '',
          mac: '00:11:22:33:44:56',
          speed: 0,
          type: 'ethernet'
        }
      ],
      throughput: {
        rx: 1.2,
        tx: 0.5
      }
    };
  }

  /**
   * Get mock alerts for development/testing
   */
  private static getMockAlerts(): Alert[] {
    return [
      {
        id: '1',
        severity: 'warning',
        message: 'Disk sdb has 2 reallocated sectors',
        timestamp: new Date().toISOString(),
        acknowledged: false,
        source: 'storage'
      },
      {
        id: '2',
        severity: 'info',
        message: 'System update available',
        timestamp: new Date(Date.now() - 86400000).toISOString(),
        acknowledged: true,
        source: 'system'
      }
    ];
  }

  /**
   * Get performance metrics data
   */
  public static async getPerformanceMetrics(): Promise<PerformanceMetrics> {
    try {
      const response = await axios.get<PerformanceMetrics>(`${this.API_URL}/performance`);
      return response.data;
    } catch (error) {
      console.error('Error fetching performance metrics:', error);
      return this.mockPerformanceMetrics;
    }
  }

  /**
   * Get disk health data
   */
  public static async getDiskHealth(): Promise<Record<string, DiskHealth>> {
    try {
      const response = await axios.get<Record<string, DiskHealth>>(`${this.API_URL}/disks`);
      return response.data;
    } catch (error) {
      console.error('Error fetching disk health:', error);
      return this.mockDiskHealth;
    }
  }

  /**
   * Connect to WebSocket service
   */
  public static connectWebSocket(): void {
    this.setupWebSocket();
  }

  /**
   * Add event listener
   */
  public static addListener(eventType: string, callback: (data: any) => void): void {
    if (!this.listeners[eventType]) {
      this.listeners[eventType] = [];
    }
    this.listeners[eventType].push(callback);
  }

  /**
   * Remove event listener
   */
  public static removeListener(eventType: string, callback: (data: any) => void): void {
    if (!this.listeners[eventType]) {
      return;
    }
    this.listeners[eventType] = this.listeners[eventType].filter(cb => cb !== callback);
  }
} 
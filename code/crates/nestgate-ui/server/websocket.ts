/**
 * WebSocket Manager for NestGate
 * 
 * Handles real-time updates and client connections
 */

import WebSocket from 'ws';
import { exec } from 'child_process';
import * as os from 'os';
import * as fs from 'fs/promises';
import * as path from 'path';
import ZfsMonitor from './monitor';
import { Server } from 'http';
import {
  ZfsPool,
  ZfsDataset,
  SystemMetrics,
  DiskHealth,
  ServiceStatus,
  Performance
} from './types/index';
import util from 'util';

// Interface for update intervals
interface UpdateIntervals {
  [key: string]: NodeJS.Timeout;
}

// Interface for client-specific update intervals using string keys
interface ClientUpdateIntervals {
  [key: string]: UpdateIntervals;
}

interface WebSocketMessage {
  type?: string;
  metric_type?: string;
  interval_ms?: number;
}

interface ResponseMessage {
  type: string;
  message?: string;
  data?: any;
  timestamp: number;
}

export class WebSocketManager {
  private wss: WebSocket.Server;
  private clients: Set<WebSocket>;
  private updateIntervals: ClientUpdateIntervals;
  private zfsMonitor: ZfsMonitor;
  
  constructor(server?: Server) {
    this.wss = server ? new WebSocket.Server({ server }) : new WebSocket.Server({ port: 8080 });
    this.clients = new Set();
    this.updateIntervals = {};
    this.zfsMonitor = new ZfsMonitor(this);
    
    // Initialize WebSocket server
    this.init();
  }
  
  /**
   * Initialize WebSocket server
   */
  private init(): void {
    console.log('Initializing WebSocket server');
    
    this.wss.on('connection', (ws, req) => {
      this.handleConnection(ws, req);
    });
    
    // Start the ZFS monitor with 3 second updates
    this.zfsMonitor.start(3000);
  }
  
  /**
   * Handle a new WebSocket connection
   */
  private handleConnection(ws: WebSocket, req: any): void {
    const ip = req.socket.remoteAddress;
    console.log(`New WebSocket connection from ${ip}`);
    
    // Add the client to our set
    this.clients.add(ws);
    
    // Send welcome message
    this.sendToClient(ws, {
      type: 'welcome',
      message: 'Connected to NestGate WebSocket server',
      timestamp: Date.now()
    });
    
    // Send initial data
    this.sendInitialData(ws);
    
    // Set up event handlers
    ws.on('message', (message) => {
      this.handleMessage(ws, message.toString());
    });
    
    ws.on('close', () => {
      console.log(`WebSocket connection closed from ${ip}`);
      this.clients.delete(ws);
      
      // Clean up any intervals for this client
      this.cleanupClientIntervals(ws);
    });
    
    ws.on('error', (error) => {
      console.error(`WebSocket error from ${ip}:`, error);
      this.clients.delete(ws);
      
      // Clean up any intervals for this client
      this.cleanupClientIntervals(ws);
    });
  }
  
  /**
   * Handle a WebSocket message
   */
  private handleMessage(ws: WebSocket, message: string): void {
    try {
      const parsedMessage: WebSocketMessage = JSON.parse(message);
      
      console.log('Received message:', parsedMessage);
      
      // Handle different message types
      if (parsedMessage.type) {
        // Handle standard type-based messages
        switch (parsedMessage.type) {
          case 'ping':
            this.sendToClient(ws, {
              type: 'pong',
              timestamp: Date.now()
            });
            break;
            
          case 'getSystemStatus':
            this.sendSystemStatus(ws);
            break;
            
          case 'getZfsPools':
            this.sendZfsPools(ws);
            break;
            
          case 'getPerformanceMetrics':
            this.sendPerformanceMetrics(ws);
            break;
            
          default:
            console.log(`Unknown message type: ${parsedMessage.type}`);
        }
      } 
      // Handle legacy or dashboard-specific messages with metric_type
      else if (parsedMessage.metric_type) {
        switch (parsedMessage.metric_type) {
          case 'SystemHealth':
            // Set up periodic system status updates
            this.setupMetricInterval(ws, 'SystemHealth', parsedMessage.interval_ms || 5000, () => {
              this.sendSystemStatus(ws);
            });
            break;
            
          case 'Performance':
            // Set up periodic performance metrics updates
            this.setupMetricInterval(ws, 'Performance', parsedMessage.interval_ms || 5000, () => {
              this.sendPerformanceMetrics(ws);
            });
            break;
            
          case 'DiskHealth':
            // Set up periodic disk health updates
            this.setupMetricInterval(ws, 'DiskHealth', parsedMessage.interval_ms || 5000, () => {
              this.sendDiskHealth(ws);
            });
            break;
            
          case 'ZfsPools':
            // Set up periodic ZFS pools updates
            this.setupMetricInterval(ws, 'ZfsPools', parsedMessage.interval_ms || 5000, () => {
              this.sendZfsPools(ws);
            });
            break;
            
          case 'Services':
            // Set up periodic services status updates
            this.setupMetricInterval(ws, 'Services', parsedMessage.interval_ms || 5000, () => {
              this.sendServicesStatus(ws);
            });
            break;
            
          default:
            console.log(`Unknown metric_type: ${parsedMessage.metric_type}`);
        }
      } else {
        console.log('Message without type or metric_type:', parsedMessage);
      }
    } catch (error) {
      console.error('Error handling WebSocket message:', error);
    }
  }
  
  /**
   * Set up a periodic interval for sending metric updates
   */
  private setupMetricInterval(ws: WebSocket, metricType: string, intervalMs: number, callback: () => void): void {
    // Clear any existing interval for this metric type
    this.clearMetricInterval(ws, metricType);
    
    // Get unique key for this websocket
    const wsKey = this.getWebSocketKey(ws);
    
    // Store client intervals by WebSocket key and metric type
    if (!this.updateIntervals[wsKey]) {
      this.updateIntervals[wsKey] = {};
    }
    
    // Create a new interval
    this.updateIntervals[wsKey][metricType] = setInterval(() => {
      if (ws.readyState === WebSocket.OPEN) {
        callback();
      } else {
        // Clean up if the connection is closed
        this.clearMetricInterval(ws, metricType);
      }
    }, intervalMs);
    
    // Send initial data immediately
    callback();
    
    console.log(`Set up ${metricType} updates every ${intervalMs}ms`);
  }
  
  /**
   * Clear a metric interval for a client
   */
  private clearMetricInterval(ws: WebSocket, metricType: string): void {
    const wsKey = this.getWebSocketKey(ws);
    if (this.updateIntervals[wsKey] && this.updateIntervals[wsKey][metricType]) {
      clearInterval(this.updateIntervals[wsKey][metricType]);
      delete this.updateIntervals[wsKey][metricType];
    }
  }
  
  /**
   * Clean up all intervals for a client
   */
  private cleanupClientIntervals(ws: WebSocket): void {
    const wsKey = this.getWebSocketKey(ws);
    if (this.updateIntervals[wsKey]) {
      for (const metricType in this.updateIntervals[wsKey]) {
        clearInterval(this.updateIntervals[wsKey][metricType]);
      }
      
      delete this.updateIntervals[wsKey];
    }
  }
  
  /**
   * Get a unique key for a WebSocket instance
   */
  private getWebSocketKey(ws: WebSocket): string {
    return (ws as any)._socket?.remoteAddress + ':' + (ws as any)._socket?.remotePort || 
      Date.now().toString() + Math.random().toString();
  }
  
  /**
   * Send data to a specific client
   */
  private sendToClient(ws: WebSocket, data: ResponseMessage): void {
    if (ws.readyState === WebSocket.OPEN) {
      ws.send(JSON.stringify(data));
    }
  }
  
  /**
   * Broadcast data to all connected clients
   */
  public broadcast(data: ResponseMessage): void {
    this.clients.forEach((client) => {
      if (client.readyState === WebSocket.OPEN) {
        client.send(JSON.stringify(data));
      }
    });
  }
  
  /**
   * Send initial data to a client when they connect
   */
  private sendInitialData(ws: WebSocket): void {
    // Send system info
    this.sendSystemStatus(ws);
    
    // Send ZFS pools
    this.sendZfsPools(ws);
    
    // Send performance metrics
    this.sendPerformanceMetrics(ws);
    
    // Send disk health
    this.sendDiskHealth(ws);
    
    // Send services status
    this.sendServicesStatus(ws);
  }
  
  /**
   * Send system status to a client
   */
  private async sendSystemStatus(ws: WebSocket): Promise<void> {
    try {
      const status = await this.getSystemStatus();
      
      this.sendToClient(ws, {
        type: 'systemStatus',
        data: status,
        timestamp: Date.now()
      });
    } catch (error) {
      console.error('Error sending system status:', error);
    }
  }
  
  /**
   * Send ZFS pools to a client
   */
  private async sendZfsPools(ws: WebSocket): Promise<void> {
    try {
      const pools = await this.getZfsPools();
      
      this.sendToClient(ws, {
        type: 'zfsPools',
        data: pools,
        timestamp: Date.now()
      });
    } catch (error) {
      console.error('Error sending ZFS pools:', error);
    }
  }
  
  /**
   * Send performance metrics to a client
   */
  private async sendPerformanceMetrics(ws: WebSocket): Promise<void> {
    try {
      const metrics = await this.getPerformanceMetrics();
      
      this.sendToClient(ws, {
        type: 'performanceMetrics',
        data: metrics,
        timestamp: Date.now()
      });
    } catch (error) {
      console.error('Error sending performance metrics:', error);
    }
  }
  
  /**
   * Send disk health to a client
   */
  private async sendDiskHealth(ws: WebSocket): Promise<void> {
    try {
      const diskHealth = await this.getDiskHealth();
      
      this.sendToClient(ws, {
        type: 'diskHealth',
        data: diskHealth,
        timestamp: Date.now()
      });
    } catch (error) {
      console.error('Error sending disk health:', error);
    }
  }
  
  /**
   * Send services status to a client
   */
  private async sendServicesStatus(ws: WebSocket): Promise<void> {
    try {
      const services = await this.getServicesStatus();
      
      this.sendToClient(ws, {
        type: 'servicesStatus',
        data: services,
        timestamp: Date.now()
      });
    } catch (error) {
      console.error('Error sending services status:', error);
    }
  }
  
  /**
   * Format uptime in seconds to a human-readable string
   */
  private formatUptime(uptime: number): string {
    const days = Math.floor(uptime / (24 * 60 * 60));
    const hours = Math.floor((uptime % (24 * 60 * 60)) / (60 * 60));
    const minutes = Math.floor((uptime % (60 * 60)) / 60);
    
    return `${days}d ${hours}h ${minutes}m`;
  }
  
  /**
   * Get system status
   */
  public async getSystemStatus(): Promise<SystemMetrics> {
    return {
      cpu: {
        usage: Math.round(Math.random() * 100),
        temperature: Math.round(30 + Math.random() * 20),
        cores: os.cpus().length
      },
      memory: {
        total: os.totalmem(),
        used: os.totalmem() - os.freemem(),
        available: os.freemem()
      },
      storage: {
        total: 1000000000000,
        used: 400000000000,
        available: 600000000000
      },
      network: {
        interfaces: [
          {
            name: 'eth0',
            rx_bytes: Math.round(Math.random() * 1000000),
            tx_bytes: Math.round(Math.random() * 1000000),
            rx_packets: Math.round(Math.random() * 1000),
            tx_packets: Math.round(Math.random() * 1000),
            status: 'up'
          }
        ]
      }
    };
  }
  
  /**
   * Get ZFS pools
   */
  public async getZfsPools(): Promise<ZfsPool[]> {
    return this.zfsMonitor.getPools();
  }
  
  /**
   * Get performance metrics
   */
  public async getPerformanceMetrics(): Promise<Performance> {
    return this.zfsMonitor.getPerformanceMetrics();
  }
  
  /**
   * Get disk health
   */
  public async getDiskHealth(): Promise<DiskHealth[]> {
    // In the future, replace with real disk health monitoring
    const mockDisks: DiskHealth[] = [
      {
        device: '/dev/sda',
        model: 'Samsung SSD 850 EVO 1TB',
        serial: 'S3Z1NB0K700001A',
        size: 1000000000000,
        temperature: 28,
        health: 98,
        errors: 0,
        status: 'ONLINE'
      },
      {
        device: '/dev/sdb',
        model: 'WDC WD20EFRX-68E',
        serial: 'WD-ABC123456789',
        size: 2000000000000,
        temperature: 32,
        health: 97,
        errors: 0,
        status: 'ONLINE'
      },
      {
        device: '/dev/sdc',
        model: 'WDC WD20EFRX-68E',
        serial: 'WD-DEF987654321',
        size: 2000000000000,
        temperature: 33,
        health: 95,
        errors: 2,
        status: 'DEGRADED'
      },
      {
        device: '/dev/sdd',
        model: 'Seagate ST8000NM0055',
        serial: 'ZA12345678',
        size: 8000000000000,
        temperature: 36,
        health: 100,
        errors: 0,
        status: 'ONLINE'
      }
    ];
    
    return mockDisks;
  }
  
  /**
   * Get services status
   */
  public async getServicesStatus(): Promise<ServiceStatus[]> {
    // In the future, replace with real service status monitoring
    const mockServices: ServiceStatus[] = [
      {
        id: 'sshd',
        name: 'SSH Server',
        status: 'running',
        uptime: 3600 * 24 * 3, // 3 days
        pid: 1234,
        memory_usage: 4567890,
        cpu_usage: 0.1
      },
      {
        id: 'samba',
        name: 'Samba Share',
        status: 'running',
        uptime: 3600 * 24 * 2, // 2 days
        pid: 2345,
        memory_usage: 23456789,
        cpu_usage: 0.5
      },
      {
        id: 'nginx',
        name: 'Nginx Web Server',
        status: 'running',
        uptime: 3600 * 24 * 5, // 5 days
        pid: 3456,
        memory_usage: 34567890,
        cpu_usage: 0.3
      }
    ];
    
    return mockServices;
  }
  
  /**
   * Close the WebSocket server
   */
  public close(): void {
    // Stop the ZFS monitor
    this.zfsMonitor.stop();
    
    // Close all client connections
    this.clients.forEach((client) => {
      client.close();
    });
    
    // Clean up all intervals
    Object.keys(this.updateIntervals).forEach((wsKey) => {
      const intervals = this.updateIntervals[wsKey];
      Object.keys(intervals).forEach((metricType) => {
        clearInterval(intervals[metricType]);
      });
    });
    
    // Clear intervals object
    this.updateIntervals = {};
    
    // Close the WebSocket server
    this.wss.close();
  }
}

// Export as both default and named
export default WebSocketManager; 
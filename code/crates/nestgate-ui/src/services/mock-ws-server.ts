/**
 * Mock WebSocket Server for local development
 * 
 * This is a simple mock implementation that simulates a WebSocket server
 * for local development purposes. It generates and sends random data
 * at regular intervals to simulate real-time updates.
 */

import { WebSocketMessageType } from './websocket.service';
import { isStrictLiveMode } from '../utils/env';

// Mock WebSocket data interfaces
interface SystemMetrics {
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

interface DiskMetrics {
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

interface ZFSMetrics {
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
    dedupratio: string;
    compression: string;
  };
  performance?: {
    iops: {
      read: number;
      write: number;
    };
    throughput: {
      read: number;
      write: number;
    };
    latency: {
      read: number;
      write: number;
      sync: number;
    };
    cache: {
      arcHits: number;
      arcMisses: number;
      l2arcHits: number;
      l2arcMisses: number;
    };
  };
}

interface Notification {
  id: string;
  type: 'info' | 'warning' | 'error' | 'success';
  title: string;
  message: string;
  read: boolean;
  timestamp: number;
  category?: string;
  source?: string;
}

class MockWebSocketServer {
  private static instance: MockWebSocketServer;
  private intervalId: number | null = null;
  private updateFrequency = 2000; // Update every 2 seconds
  private mockSystemMetrics: SystemMetrics;
  private mockDiskMetrics: DiskMetrics[];
  private mockZFSMetrics: ZFSMetrics[];
  private notificationCounter = 0;
  
  private constructor() {
    // Initialize with default values
    this.mockSystemMetrics = {
      cpuUsage: 25 + Math.random() * 15,
      memoryUsage: 40 + Math.random() * 20,
      uptime: 3600 * 24 * (1 + Math.random() * 7), // 1-8 days in seconds
      loadAverage: [
        0.5 + Math.random() * 1.5,
        0.6 + Math.random() * 1.8,
        0.7 + Math.random() * 2.0
      ],
      diskIO: { 
        read: 1024 * 1024 * (1 + Math.random() * 5), // 1-6 MB/s
        write: 1024 * 1024 * (0.5 + Math.random() * 3) // 0.5-3.5 MB/s
      },
      networkIO: { 
        received: 1024 * 256 * (1 + Math.random() * 4), // 256-1280 KB/s
        sent: 1024 * 128 * (1 + Math.random() * 3) // 128-512 KB/s
      },
      temperature: 35 + Math.random() * 15
    };

    // Initialize mock disks
    const diskCount = 4 + Math.floor(Math.random() * 4); // 4-8 disks
    this.mockDiskMetrics = [];
    
    for (let i = 0; i < diskCount; i++) {
      this.mockDiskMetrics.push({
        diskId: `disk${i}`,
        temperature: 30 + Math.random() * 25,
        health: Math.random() > 0.9 ? 'warning' : (Math.random() > 0.98 ? 'critical' : 'good'),
        status: Math.random() > 0.95 ? 'degraded' : 'online',
        readErrors: Math.floor(Math.random() * 5),
        writeErrors: Math.floor(Math.random() * 3),
        iops: {
          read: 100 + Math.floor(Math.random() * 400),
          write: 50 + Math.floor(Math.random() * 200)
        },
        throughput: {
          read: 1024 * 1024 * (0.5 + Math.random() * 2),
          write: 1024 * 1024 * (0.2 + Math.random() * 1.5)
        }
      });
    }

    // Initialize mock ZFS pools
    const poolCount = 1 + Math.floor(Math.random() * 2); // 1-3 pools
    this.mockZFSMetrics = [];
    
    for (let i = 0; i < poolCount; i++) {
      const childCount = 2 + Math.floor(Math.random() * 4); // 2-6 vdevs
      const children = [];
      
      for (let j = 0; j < childCount; j++) {
        children.push({
          name: `vdev${j}`,
          status: Math.random() > 0.95 ? 'degraded' as const : 'online' as const,
          errors: {
            read: Math.floor(Math.random() * 3),
            write: Math.floor(Math.random() * 2),
            checksum: Math.floor(Math.random() * 4)
          }
        });
      }
      
      const totalCapacity = 1024 * 1024 * 1024 * 1024; // 1TB
      const usedCapacity = totalCapacity * (0.3 + Math.random() * 0.5); // 30-80% of 1TB
      
      this.mockZFSMetrics.push({
        poolName: `pool${i}`,
        status: Math.random() > 0.95 ? 'degraded' : 'online',
        health: Math.random() > 0.9 ? 'warning' : (Math.random() > 0.98 ? 'critical' : 'good'),
        capacityUsed: usedCapacity,
        capacityTotal: totalCapacity,
        diskErrors: {
          read: Math.floor(Math.random() * 5),
          write: Math.floor(Math.random() * 3),
          checksum: Math.floor(Math.random() * 10)
        },
        children,
        properties: {
          dedupratio: `${(1.0 + Math.random() * 0.5).toFixed(2)}x`,
          compression: Math.random() > 0.5 ? 'lz4' : 'zstd'
        },
        performance: {
          iops: {
            read: 1000 + Math.floor(Math.random() * 2000),
            write: 500 + Math.floor(Math.random() * 1000)
          },
          throughput: {
            read: 50 * 1024 * 1024 + Math.random() * 100 * 1024 * 1024,
            write: 30 * 1024 * 1024 + Math.random() * 70 * 1024 * 1024
          },
          latency: {
            read: 0.5 + Math.random() * 2,
            write: 1 + Math.random() * 3,
            sync: 2 + Math.random() * 5
          },
          cache: {
            arcHits: 1000 + Math.floor(Math.random() * 5000),
            arcMisses: 100 + Math.floor(Math.random() * 1000),
            l2arcHits: 500 + Math.floor(Math.random() * 2000),
            l2arcMisses: 50 + Math.floor(Math.random() * 500)
          }
        }
      });
    }
  }

  public static getInstance(): MockWebSocketServer {
    if (!MockWebSocketServer.instance) {
      MockWebSocketServer.instance = new MockWebSocketServer();
    }
    return MockWebSocketServer.instance;
  }

  public start(): void {
    // Check if strict live mode is enabled
    if (isStrictLiveMode()) {
      console.warn('❌ [MockWebSocketServer] Cannot start mock server in strict live mode');
      console.warn('Mock server disabled due to STRICT_DATA_MODE=true');
      return;
    }
    
    if (this.intervalId !== null) {
      console.warn('⚠️ [MockWebSocketServer] Mock server is already running');
      return;
    }

    this.intervalId = window.setInterval(() => {
      this.updateMockData();
      this.broadcastMockData();
    }, this.updateFrequency);

    console.log('🔄 [MockWebSocketServer] Mock server started');
    console.log(`Generating mock data every ${this.updateFrequency}ms`);
    console.log('⚠️ This data is simulated and does not reflect a real system');
  }

  public stop(): void {
    if (this.intervalId !== null) {
      window.clearInterval(this.intervalId);
      this.intervalId = null;
      console.log('⏹️ [MockWebSocketServer] Mock server stopped');
    } else {
      console.log('ℹ️ [MockWebSocketServer] Mock server was not running');
    }
  }

  public isRunning(): boolean {
    return this.intervalId !== null;
  }

  private updateMockData(): void {
    // Update system metrics with small variations
    this.mockSystemMetrics.cpuUsage = Math.min(100, Math.max(5, this.mockSystemMetrics.cpuUsage + (Math.random() * 10 - 5)));
    this.mockSystemMetrics.memoryUsage = Math.min(100, Math.max(10, this.mockSystemMetrics.memoryUsage + (Math.random() * 8 - 4)));
    this.mockSystemMetrics.uptime += this.updateFrequency / 1000;
    this.mockSystemMetrics.loadAverage = this.mockSystemMetrics.loadAverage.map(load => 
      Math.max(0.1, load + (Math.random() * 0.4 - 0.2)));
    this.mockSystemMetrics.diskIO.read = Math.max(1024 * 512, this.mockSystemMetrics.diskIO.read + (Math.random() * 1024 * 512 - 1024 * 256));
    this.mockSystemMetrics.diskIO.write = Math.max(1024 * 256, this.mockSystemMetrics.diskIO.write + (Math.random() * 1024 * 256 - 1024 * 128));
    this.mockSystemMetrics.networkIO.received = Math.max(1024 * 64, this.mockSystemMetrics.networkIO.received + (Math.random() * 1024 * 128 - 1024 * 64));
    this.mockSystemMetrics.networkIO.sent = Math.max(1024 * 32, this.mockSystemMetrics.networkIO.sent + (Math.random() * 1024 * 64 - 1024 * 32));
    this.mockSystemMetrics.temperature = Math.min(70, Math.max(30, this.mockSystemMetrics.temperature + (Math.random() * 2 - 1)));

    // Update disk metrics
    this.mockDiskMetrics.forEach(disk => {
      disk.temperature = Math.min(65, Math.max(25, disk.temperature + (Math.random() * 3 - 1.5)));
      disk.health = Math.random() > 0.995 ? 'critical' : (Math.random() > 0.97 ? 'warning' : disk.health);
      disk.status = Math.random() > 0.998 ? 'degraded' : disk.status;
      disk.readErrors += Math.random() > 0.98 ? 1 : 0;
      disk.writeErrors += Math.random() > 0.99 ? 1 : 0;
      disk.iops.read = Math.max(50, disk.iops.read + (Math.random() * 100 - 50));
      disk.iops.write = Math.max(20, disk.iops.write + (Math.random() * 60 - 30));
      disk.throughput.read = Math.max(1024 * 512, disk.throughput.read + (Math.random() * 1024 * 256 - 1024 * 128));
      disk.throughput.write = Math.max(1024 * 256, disk.throughput.write + (Math.random() * 1024 * 128 - 1024 * 64));
    });

    // Update ZFS metrics
    this.mockZFSMetrics.forEach(pool => {
      pool.capacityUsed = Math.min(pool.capacityTotal * 0.95, Math.max(pool.capacityTotal * 0.2, pool.capacityUsed + (Math.random() * pool.capacityTotal * 0.05 - pool.capacityTotal * 0.025)));
      pool.health = Math.random() > 0.995 ? 'critical' : (Math.random() > 0.97 ? 'warning' : pool.health);
      pool.status = Math.random() > 0.998 ? 'degraded' : pool.status;
      pool.diskErrors.read += Math.random() > 0.98 ? 1 : 0;
      pool.diskErrors.write += Math.random() > 0.99 ? 1 : 0;
      pool.diskErrors.checksum += Math.random() > 0.97 ? 1 : 0;
      
      if (pool.performance) {
        pool.performance.iops.read = Math.max(500, pool.performance.iops.read + (Math.random() * 400 - 200));
        pool.performance.iops.write = Math.max(200, pool.performance.iops.write + (Math.random() * 200 - 100));
        pool.performance.throughput.read = Math.max(10 * 1024 * 1024, pool.performance.throughput.read + (Math.random() * 10 * 1024 * 1024 - 5 * 1024 * 1024));
        pool.performance.throughput.write = Math.max(5 * 1024 * 1024, pool.performance.throughput.write + (Math.random() * 5 * 1024 * 1024 - 2.5 * 1024 * 1024));
        pool.performance.latency.read = Math.max(0.1, pool.performance.latency.read + (Math.random() * 0.5 - 0.25));
        pool.performance.latency.write = Math.max(0.2, pool.performance.latency.write + (Math.random() * 0.6 - 0.3));
        pool.performance.latency.sync = Math.max(0.5, pool.performance.latency.sync + (Math.random() * 1 - 0.5));
        
        // Cache hits and misses
        pool.performance.cache.arcHits += Math.floor(Math.random() * 100);
        pool.performance.cache.arcMisses += Math.floor(Math.random() * 20);
        pool.performance.cache.l2arcHits += Math.floor(Math.random() * 50);
        pool.performance.cache.l2arcMisses += Math.floor(Math.random() * 10);
      }
      
      pool.children.forEach(child => {
        child.status = Math.random() > 0.998 ? 'degraded' : child.status;
        child.errors.read += Math.random() > 0.99 ? 1 : 0;
        child.errors.write += Math.random() > 0.995 ? 1 : 0;
        child.errors.checksum += Math.random() > 0.98 ? 1 : 0;
      });
    });
  }

  private broadcastMockData(): void {
    // Broadcast system metrics
    this.sendWebSocketMessage({
      type: WebSocketMessageType.SYSTEM_METRICS,
      timestamp: Date.now(),
      data: this.mockSystemMetrics
    });

    // Broadcast disk metrics
    this.mockDiskMetrics.forEach(disk => {
      this.sendWebSocketMessage({
        type: WebSocketMessageType.DISK_METRICS,
        timestamp: Date.now(),
        data: disk
      });
    });

    // Broadcast ZFS metrics
    this.mockZFSMetrics.forEach(pool => {
      this.sendWebSocketMessage({
        type: WebSocketMessageType.ZFS_METRICS,
        timestamp: Date.now(),
        data: pool
      });
    });

    // Occasionally send a notification
    if (Math.random() > 0.85) {
      this.sendRandomNotification();
    }
  }

  private sendRandomNotification(): void {
    const notificationTypes: ('info' | 'warning' | 'error' | 'success')[] = ['info', 'warning', 'error', 'success'];
    const notificationType = notificationTypes[Math.floor(Math.random() * notificationTypes.length)];
    
    const messages = [
      'Scrub completed on pool0',
      'Disk temperature warning on disk2',
      'Snapshot cleanup complete',
      'Pool capacity approaching 80%',
      'System update available',
      'Backup task completed',
      'Network interface eth0 down',
      'ZFS error detected'
    ];
    const message = messages[Math.floor(Math.random() * messages.length)];
    
    const categories = ['System', 'Storage', 'Network', 'Backup', 'Security', 'Updates'];
    const category = categories[Math.floor(Math.random() * categories.length)];
    
    const sources = ['system', 'zfs', 'smartd', 'backup', 'network', 'update'];
    const source = sources[Math.floor(Math.random() * sources.length)];

    const notification: Notification = {
      id: `notify-${this.notificationCounter++}-${Date.now()}`,
      type: notificationType,
      title: `${notificationType.charAt(0).toUpperCase()}${notificationType.slice(1)} Notification`,
      message,
      read: false,
      timestamp: Date.now(),
      category,
      source
    };

    this.sendWebSocketMessage({
      type: WebSocketMessageType.NOTIFICATION,
      timestamp: Date.now(),
      data: notification
    });
  }

  private sendWebSocketMessage(message: any): void {
    // Create a custom event to simulate WebSocket message
    const event = new CustomEvent('mockWebSocketMessage', {
      detail: message
    });
    
    // Dispatch event to document for WebSocketService to listen to
    document.dispatchEvent(event);
  }
}

export default MockWebSocketServer; 
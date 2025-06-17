/**
 * Type definitions for NestGate server
 */

// User type definitions
export interface User {
  id: string;
  username: string;
  role: 'admin' | 'readonly';
}

// API response format
export interface ApiResponse<T = any> {
  status: 'success' | 'error';
  data: T | null;
  message: string;
  timestamp: number;
}

// ZFS Pool definitions
export interface ZfsPool {
  name: string;
  health: string;
  size: number;
  free: number;
  used: number;
  isEncrypted?: boolean;
  datasets?: ZfsDataset[];
  snapshots?: ZfsSnapshot[];
}

// ZFS Dataset definitions
export interface ZfsDataset {
  name: string;
  used: number;
  available: number;
  mountpoint: string;
  compression?: string;
  recordsize?: number;
  type?: string;
}

// ZFS Snapshot definitions
export interface ZfsSnapshot {
  name: string;
  dataset: string;
  used: number;
  creation: number;
}

// Disk health definitions
export interface DiskHealth {
  device: string;
  model: string;
  serial: string;
  size: number;
  temperature: number;
  health: number;
  errors: number;
  status: 'ONLINE' | 'DEGRADED' | 'FAILING';
}

// System metrics
export interface SystemMetrics {
  cpu: {
    usage: number;
    temperature: number;
    cores: number;
  };
  memory: {
    total: number;
    used: number;
    available: number;
  };
  storage: {
    total: number;
    used: number;
    available: number;
  };
  network: {
    interfaces: NetworkInterface[];
  };
}

export interface NetworkInterface {
  name: string;
  rx_bytes: number;
  tx_bytes: number;
  rx_packets: number;
  tx_packets: number;
  status: 'up' | 'down';
}

// Service status
export interface ServiceStatus {
  id: string;
  name: string;
  status: 'running' | 'stopped' | 'failed';
  uptime?: number;
  pid?: number;
  memory_usage?: number;
  cpu_usage?: number;
}

// Request with user
export interface AuthenticatedRequest extends Express.Request {
  user?: User;
}

// Performance metrics
export interface Performance {
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
  };
} 
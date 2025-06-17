/**
 * Data interfaces for NestGate
 * 
 * These interfaces define the contracts between the application and data sources,
 * both real and mock.
 */

/**
 * System status information
 */
export interface SystemStatus {
  status: string;
  cpu_usage: number | string;
  memory_usage: number | string;
  uptime: string;
}

/**
 * ZFS pool information
 */
export interface ZfsPool {
  name: string;
  health: string;
  size: number;
  used: number;
  available: number;
  capacity: number;
  state: string;
}

/**
 * Dataset information
 */
export interface Dataset {
  name: string;
  used: number;
  available: number;
  mountpoint: string;
}

/**
 * Snapshot information
 */
export interface Snapshot {
  name: string;
  dataset: string;
  used: number;
  creation: number;
}

/**
 * Performance metrics
 */
export interface PerformanceMetrics {
  timestamp: number;
  throughput?: {
    read: number | string;
    write: number | string;
  };
  iops?: {
    read: number;
    write: number;
    total?: number;
  };
  latency?: {
    read: number | string;
    write: number | string;
  };
}

/**
 * Disk health information
 */
export interface DiskHealth {
  device: string;
  model: string;
  serial: string;
  size: number;
  status: string;
  temperature: number;
  smart_status?: {
    passed: boolean;
    attributes?: any[];
  };
}

/**
 * Service status
 */
export interface ServiceStatus {
  nfs: string;
  smb: string;
  [key: string]: string;
}

/**
 * Base data provider interface - all data providers must implement this
 */
export interface DataProvider {
  getSystemStatus(): Promise<SystemStatus>;
  getZfsPools(): Promise<ZfsPool[]>;
  getDatasets(poolName: string): Promise<Dataset[]>;
  getSnapshots(poolName: string): Promise<Snapshot[]>;
  getPerformanceMetrics(): Promise<PerformanceMetrics>;
  getDiskHealth(): Promise<DiskHealth[]>;
  getServicesStatus(): Promise<ServiceStatus>;
}

/**
 * Factory function type to create data providers
 */
export type DataProviderFactory = () => DataProvider; 
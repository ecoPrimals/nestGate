/**
 * Mock Data Provider
 * 
 * This file provides a mock implementation of the data provider interface.
 * It should ONLY be used in testing environments, never in production code.
 */

import { DataProvider, ZfsPool, Dataset, Snapshot, SystemStatus, PerformanceMetrics, DiskHealth, ServiceStatus } from '../../src/data/interfaces';

/**
 * Creates a mock data provider for testing
 */
export function createMockDataProvider(): DataProvider {
  return {
    // ZFS Pool related methods
    getZfsPools: jest.fn().mockResolvedValue([
      {
        name: 'nestpool',
        health: 'ONLINE',
        size: 7810371559424, // 7.28 TB
        used: 1952592889856, // 1.82 TB
        available: 5857778669568, // 5.46 TB
        capacity: 25,
        state: 'ONLINE'
      },
      {
        name: 'backup',
        health: 'ONLINE',
        size: 3905185779712, // 3.64 TB
        used: 506474647348, // 471.7 GB
        available: 3398711132364, // 3.17 TB
        capacity: 13,
        state: 'ONLINE'
      }
    ] as ZfsPool[]),
    
    getDatasets: jest.fn().mockImplementation((poolName: string) => {
      if (poolName === 'nestpool') {
        return Promise.resolve([
          {
            name: 'nestpool/hot',
            used: 1366814968899, // ~1.3TB
            available: 2928889334784, // ~2.7TB
            mountpoint: '/nestpool/hot'
          },
          {
            name: 'nestpool/warm',
            used: 390518577971, // ~360GB
            available: 1171555733913, // ~1.1TB
            mountpoint: '/nestpool/warm'
          },
          {
            name: 'nestpool/cold',
            used: 195259288985, // ~180GB
            available: 1171555733913, // ~1.1TB
            mountpoint: '/nestpool/cold'
          }
        ] as Dataset[]);
      } else if (poolName === 'backup') {
        return Promise.resolve([
          {
            name: 'backup/data',
            used: 506474647348, // 471.7 GB
            available: 3398711132364, // 3.17 TB
            mountpoint: '/backup/data'
          }
        ] as Dataset[]);
      }
      return Promise.resolve([]);
    }),
    
    getSnapshots: jest.fn().mockImplementation((datasetName: string) => {
      if (datasetName.includes('nestpool')) {
        return Promise.resolve([
          {
            name: `${datasetName}@auto-1week-ago`,
            dataset: datasetName,
            used: 1073741824, // 1GB
            creation: Date.now() - 7 * 24 * 60 * 60 * 1000 // 1 week ago
          },
          {
            name: `${datasetName}@auto-1day-ago`,
            dataset: datasetName,
            used: 536870912, // 512MB
            creation: Date.now() - 24 * 60 * 60 * 1000 // 1 day ago
          }
        ] as Snapshot[]);
      }
      return Promise.resolve([]);
    }),
    
    // System status related methods
    getSystemStatus: jest.fn().mockResolvedValue({
      status: 'online',
      cpu_usage: 25,
      memory_usage: 50,
      uptime: '10 days, 5 hours, 30 minutes'
    } as SystemStatus),
    
    getDiskHealth: jest.fn().mockResolvedValue([
      {
        device: '/dev/sda',
        model: 'Samsung SSD 860 EVO 1TB',
        serial: 'S3Z1NB0K123456',
        size: 1000204886016, // 1TB
        status: 'GOOD',
        temperature: 35,
        smart_status: {
          passed: true
        }
      },
      {
        device: '/dev/sdb',
        model: 'WDC WD40EFRX-68N32N0',
        serial: 'WD-WCC7K1234567',
        size: 4000787030016, // 4TB
        status: 'GOOD',
        temperature: 38,
        smart_status: {
          passed: true
        }
      }
    ] as DiskHealth[]),
    
    getServicesStatus: jest.fn().mockResolvedValue({
      nfs: 'running',
      smb: 'running',
      ssh: 'running',
      iscsi: 'stopped'
    } as ServiceStatus),
    
    // Performance metrics related methods
    getPerformanceMetrics: jest.fn().mockResolvedValue({
      timestamp: Date.now(),
      throughput: {
        read: '120 MB/s',
        write: '85 MB/s'
      },
      iops: {
        read: 1200,
        write: 800,
        total: 2000
      },
      latency: {
        read: '5.2 ms',
        write: '8.7 ms'
      }
    } as PerformanceMetrics)
  };
}

export default createMockDataProvider; 
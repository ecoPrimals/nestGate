/**
 * Mock TelemetryService for testing
 */
export class TelemetryService {
  static getSystemStatus = jest.fn().mockResolvedValue({
    cpuUsage: 25,
    memoryUsage: 50,
    uptime: 3600,
    load: [0.5, 0.7, 0.6],
    diskHealth: [
      { device: '/dev/sda', status: 'healthy', temperature: 35 },
      { device: '/dev/sdb', status: 'healthy', temperature: 38 }
    ],
    services: [
      { name: 'smb', status: 'running' },
      { name: 'nfs', status: 'running' },
      { name: 'ssh', status: 'running' }
    ]
  });
  
  static getStorageStatus = jest.fn().mockResolvedValue({
    total: 1000000000,
    used: 250000000,
    free: 750000000,
    usedPercentage: 25,
  });
  
  static getPerformanceMetrics = jest.fn().mockResolvedValue({
    read: 100,
    write: 50,
    iops: 150,
    throughput: 250,
    latency: 5,
    history: [
      { timestamp: Date.now() - 600000, read: 90, write: 45 },
      { timestamp: Date.now() - 300000, read: 95, write: 48 },
      { timestamp: Date.now(), read: 100, write: 50 }
    ]
  });

  static getNetworkStatus = jest.fn().mockResolvedValue({
    interfaces: [
      { name: 'eth0', status: 'up', speed: '1 Gbps', ip: '192.168.1.100' },
      { name: 'eth1', status: 'down', speed: '1 Gbps', ip: '' }
    ],
    throughput: {
      in: 25,
      out: 10
    }
  });

  static getAlerts = jest.fn().mockResolvedValue([
    { id: 1, severity: 'info', message: 'System updated', timestamp: Date.now() - 86400000 },
    { id: 2, severity: 'warning', message: 'High disk usage', timestamp: Date.now() - 3600000 }
  ]);
} 
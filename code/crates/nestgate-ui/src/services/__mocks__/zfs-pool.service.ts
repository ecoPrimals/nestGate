/**
 * Mock ZfsPoolService for testing
 */
export class ZfsPoolService {
  static instance: ZfsPoolService;

  static getInstance = jest.fn().mockImplementation(() => {
    if (!ZfsPoolService.instance) {
      ZfsPoolService.instance = new ZfsPoolService();
    }
    return ZfsPoolService.instance;
  });

  getPools = jest.fn().mockResolvedValue([
    { 
      name: 'tank', 
      size: 1000000000, 
      free: 700000000,
      allocated: 300000000,
      health: 'ONLINE',
      status: 'healthy',
      devices: [
        { name: 'sda', status: 'ONLINE', size: 500000000 },
        { name: 'sdb', status: 'ONLINE', size: 500000000 }
      ]
    },
    { 
      name: 'backup', 
      size: 2000000000, 
      free: 1800000000,
      allocated: 200000000,
      health: 'ONLINE',
      status: 'healthy',
      devices: [
        { name: 'sdc', status: 'ONLINE', size: 1000000000 },
        { name: 'sdd', status: 'ONLINE', size: 1000000000 }
      ]
    }
  ]);

  getDatasets = jest.fn().mockResolvedValue([
    { 
      name: 'tank/data', 
      available: 500000000, 
      used: 200000000,
      referenced: 150000000,
      compression: 'lz4',
      recordsize: '128K',
      readonly: 'off'
    },
    { 
      name: 'tank/home', 
      available: 200000000, 
      used: 100000000,
      referenced: 80000000,
      compression: 'lz4',
      recordsize: '128K',
      readonly: 'off'
    }
  ]);

  getSnapshots = jest.fn().mockResolvedValue([
    {
      name: 'tank/data@snap1',
      dataset: 'tank/data',
      creation: new Date(Date.now() - 86400000).toISOString(),
      used: 50000000,
      referenced: 150000000
    },
    {
      name: 'tank/data@snap2',
      dataset: 'tank/data',
      creation: new Date(Date.now() - 3600000).toISOString(),
      used: 10000000,
      referenced: 150000000
    }
  ]);

  createSnapshot = jest.fn().mockResolvedValue({ 
    success: true, 
    message: 'Snapshot created successfully' 
  });

  deleteSnapshot = jest.fn().mockResolvedValue({ 
    success: true, 
    message: 'Snapshot deleted successfully' 
  });

  rollbackSnapshot = jest.fn().mockResolvedValue({ 
    success: true, 
    message: 'Snapshot rollback completed successfully' 
  });

  updateDatasetProperty = jest.fn().mockResolvedValue({
    success: true,
    message: 'Dataset property updated successfully'
  });
} 
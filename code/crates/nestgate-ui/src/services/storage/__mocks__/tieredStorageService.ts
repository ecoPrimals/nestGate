/**
 * Mock Tiered Storage Service
 * 
 * Used in tests to provide mock data for tiered storage
 */
import { StorageTier } from '../../../components/storage/TierCard';
import { EventFilter, FsEvent, MigrationJob } from '../tieredStorageService';

// Mock tiers data
const mockTiers: StorageTier[] = [
  {
    id: 'hot',
    name: 'hot',
    path: '/nesttest/hot',
    usage: {
      used: 1024 * 1024 * 100, // 100 MB
      total: 1024 * 1024 * 1024, // 1 GB
      available: 1024 * 1024 * 924, // 924 MB
      compressionRatio: 1.5
    },
    properties: {
      compression: 'lz4',
      recordsize: '128K',
      atime: 'off',
      primarycache: 'all'
    },
    monitoring: {
      enabled: true,
      activeEvents: 5,
      recentEvents: 24
    }
  },
  {
    id: 'warm',
    name: 'warm',
    path: '/nesttest/warm',
    usage: {
      used: 1024 * 1024 * 200, // 200 MB
      total: 1024 * 1024 * 1024, // 1 GB
      available: 1024 * 1024 * 824, // 824 MB
      compressionRatio: 2.0
    },
    properties: {
      compression: 'zstd',
      recordsize: '1M',
      atime: 'off',
      primarycache: 'metadata'
    },
    monitoring: {
      enabled: true,
      activeEvents: 3,
      recentEvents: 12
    }
  },
  {
    id: 'cold',
    name: 'cold',
    path: '/nesttest/cold',
    usage: {
      used: 1024 * 1024 * 300, // 300 MB
      total: 1024 * 1024 * 1024, // 1 GB
      available: 1024 * 1024 * 724, // 724 MB
      compressionRatio: 3.0
    },
    properties: {
      compression: 'gzip-9',
      recordsize: '1M',
      atime: 'off',
      primarycache: 'metadata'
    },
    monitoring: {
      enabled: true,
      activeEvents: 1,
      recentEvents: 5
    }
  }
];

// Mock events data
const mockEvents: FsEvent[] = [
  {
    id: 'event1',
    tierId: 'hot',
    kind: 'CREATE',
    path: '/nesttest/hot/test.txt',
    isDirectory: false,
    timestamp: new Date().toISOString()
  },
  {
    id: 'event2',
    tierId: 'hot',
    kind: 'MODIFY',
    path: '/nesttest/hot/large-file.bin',
    isDirectory: false,
    timestamp: new Date().toISOString()
  }
];

// Mock migration jobs
const mockMigrations: MigrationJob[] = [];

// Mock implementation
const tieredStorageService = {
  getTiers: jest.fn().mockResolvedValue(mockTiers),
  
  getTier: jest.fn().mockImplementation((tierId: string) => {
    const tier = mockTiers.find(t => t.id === tierId);
    if (!tier) {
      return Promise.reject(new Error(`Tier not found: ${tierId}`));
    }
    return Promise.resolve(tier);
  }),
  
  getEvents: jest.fn().mockResolvedValue(mockEvents),
  
  updateTierProperty: jest.fn().mockImplementation((tierId: string, property: string, value: any) => {
    const tier = mockTiers.find(t => t.id === tierId);
    if (!tier) {
      return Promise.reject(new Error(`Tier not found: ${tierId}`));
    }
    
    // Create a new tier with updated property
    const updatedTier = {
      ...tier,
      properties: {
        ...tier.properties,
        [property]: value
      }
    };
    
    return Promise.resolve(updatedTier);
  }),
  
  startMigration: jest.fn().mockImplementation((migrationJob: Partial<MigrationJob>) => {
    const newJob: MigrationJob = {
      id: `job-${Date.now()}`,
      sourceTierId: migrationJob.sourceTierId || '',
      targetTierId: migrationJob.targetTierId || '',
      paths: migrationJob.paths || [],
      status: 'pending',
      progress: 0,
      startTime: new Date().toISOString()
    };
    
    mockMigrations.push(newJob);
    return Promise.resolve(newJob);
  }),
  
  getActiveMigrations: jest.fn().mockResolvedValue([]),
  
  getMigrationHistory: jest.fn().mockResolvedValue([]),
  
  updateEventFilter: jest.fn().mockImplementation((tierId: string, filter: Partial<EventFilter>) => {
    return Promise.resolve({
      id: 'filter-1',
      name: 'Default Filter',
      includeDirectories: filter.includeDirectories || false,
      includeHidden: filter.includeHidden || false,
      extensions: filter.extensions || [],
      patterns: filter.patterns || [],
      eventTypes: filter.eventTypes || ['CREATE', 'MODIFY', 'DELETE']
    });
  })
};

export { tieredStorageService }; 
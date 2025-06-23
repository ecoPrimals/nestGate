// Storage tier enumeration for tiered storage management
export enum StorageTier {
  Hot = 'Hot',
  Warm = 'Warm', 
  Cold = 'Cold'
}

// Storage tier interface (re-exported from TierCard for compatibility)
export interface StorageTierInterface {
  id: string;
  name: string;
  path: string;
  properties: {
    compression: string;
    recordsize: string;
    atime: string;
    primarycache: string;
    [key: string]: string;
  };
  usage: {
    available: number;
    used: number;
    total: number;
    compressionRatio: number;
  };
  monitoring: {
    enabled: boolean;
    activeEvents: number;
    recentEvents: number;
  };
}

// ZFS Dataset types
export interface ZfsDatasetProperties {
  compression: string;
  recordsize: string;
  quota: number;
  reservation: number;
  readonly: boolean;
  atime: boolean;
  encryption: boolean;
  deduplication: boolean;
  tier: StorageTier;
  copies: number;
  primarycache: string;
  secondarycache: string;
  logbias: string;
  sync: string;
}

export interface ZfsDataset {
  id: string;
  name: string;
  fullName: string;
  pool: string;
  parent?: string;
  children: ZfsDataset[];
  type: 'filesystem' | 'volume';
  mountpoint: string;
  mounted: boolean;
  properties: ZfsDatasetProperties;
  health: 'healthy' | 'warning' | 'error';
  lastModified: string;
}

// Storage management types
export interface StoragePool {
  id: string;
  name: string;
  health: string;
  status: string;
  size: number;
  allocated: number;
  used: number;
  free: number;
  fragmentation?: number;
  readonly?: boolean;
  dedupratio?: number;
}

// Export commonly used types
export type DatasetType = 'filesystem' | 'volume';
export type PoolStatus = 'ONLINE' | 'DEGRADED' | 'FAULTED' | 'OFFLINE' | 'UNAVAIL';
export type DatasetHealth = 'healthy' | 'warning' | 'error'; 
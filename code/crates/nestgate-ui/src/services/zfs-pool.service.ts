import axios from 'axios';
import { API_BASE_URL } from '../config';
import { DataSourceType, isStrictLiveMode } from '../utils/env';
import { StorageTier } from '../types/storage';

/**
 * ZFS Pool interface
 */
export interface ZfsPool {
  id: string;
  name: string;
  health: string;
  status: string; // ONLINE, DEGRADED, FAULTED, etc.
  size: number;
  allocated: number; // alias for used
  used: number;
  free: number;
  fragmentation?: number;
  readonly?: boolean;
  dedupratio?: number;
  dataSource?: DataSourceType;
}

/**
 * Represents a ZFS dataset with its properties
 */
export interface ZfsDataset {
  id: string;
  name: string;
  pool: string;
  type: string;
  used: string;
  available: string;
  referenced: string;
  recordsize: string;
  compression: string;
  compressratio: string;
  mounted: boolean;
  mountpoint: string;
  quota: string;
  reservation: string;
  readonly: boolean;
  dataSource?: DataSourceType;
}

/**
 * Dataset creation configuration
 */
export interface DatasetCreateConfig {
  name: string;
  type: 'filesystem' | 'volume';
  properties?: {
    compression?: string;
    recordsize?: string;
    quota?: number;
    reservation?: number;
    readonly?: boolean;
    atime?: boolean;
    encryption?: boolean;
    encryptionKey?: string;
    deduplication?: boolean;
    mountpoint?: string;
    tier?: StorageTier;
    copies?: number;
    primarycache?: string;
    secondarycache?: string;
    logbias?: string;
    sync?: string;
  };
}

/**
 * Dataset property update configuration
 */
export interface DatasetPropertyUpdate {
  property: string;
  value: string | number | boolean;
}

/**
 * ZFS Pool service for managing ZFS pools
 */
export class ZfsPoolService {
  private static readonly API_URL = `${API_BASE_URL}/api/zfs/pools`;
  private static readonly DATASETS_API_URL = `${API_BASE_URL}/api/zfs/datasets`;
  private static instance: ZfsPoolService;
  
  /**
   * Get singleton instance
   */
  public static getInstance(): ZfsPoolService {
    if (!ZfsPoolService.instance) {
      ZfsPoolService.instance = new ZfsPoolService();
    }
    return ZfsPoolService.instance;
  }
  
  /**
   * Log service status for debugging
   */
  public static logServiceStatus(message: string): void {
    console.log(`ZfsPoolService: ${message}`);
  }
  
  /**
   * Get all ZFS pools - Alias for getZfsPools() to maintain compatibility
   */
  public static async getPools(): Promise<ZfsPool[]> {
    return this.getZfsPools();
  }
  
  /**
   * Get all ZFS pools
   */
  public static async getZfsPools(): Promise<ZfsPool[]> {
    this.logServiceStatus('Fetching all ZFS pools');
    try {
      const response = await axios.get<ZfsPool[]>(this.API_URL);
      return response.data.map(pool => ({
        ...pool,
        dataSource: DataSourceType.LIVE
      }));
    } catch (error) {
      console.error('Error fetching ZFS pools:', error);
      
      // Return placeholder in strict live mode
      return [{
        id: 'placeholder',
        name: 'To be added',
        health: 'UNAVAIL',
        status: 'UNAVAIL',
        size: 0,
        allocated: 0,
        used: 0,
        free: 0,
        dataSource: DataSourceType.PLACEHOLDER
      }];
    }
  }
  
  /**
   * Get a specific ZFS pool by name
   */
  public static async getZfsPool(name: string): Promise<ZfsPool | null> {
    this.logServiceStatus(`Fetching ZFS pool: ${name}`);
    try {
      const response = await axios.get<ZfsPool>(`${this.API_URL}/${name}`);
      return {
        ...response.data,
        dataSource: DataSourceType.LIVE
      };
    } catch (error) {
      console.error(`Error fetching ZFS pool '${name}':`, error);
      
      // Return placeholder in strict live mode
      return {
        id: name || 'placeholder',
        name: name || 'To be added',
        health: 'UNAVAIL',
        status: 'UNAVAIL',
        size: 0,
        allocated: 0,
        used: 0,
        free: 0,
        dataSource: DataSourceType.PLACEHOLDER
      };
    }
  }
  
  /**
   * Create a new ZFS pool
   */
  public static async createZfsPool(name: string, devices: string[]): Promise<ZfsPool> {
    this.logServiceStatus(`Creating ZFS pool: ${name}`);
    try {
      const response = await axios.post<ZfsPool>(this.API_URL, { name, devices });
      return {
        ...response.data,
        dataSource: DataSourceType.LIVE
      };
    } catch (error) {
      console.error(`Error creating ZFS pool '${name}':`, error);
      throw new Error('Failed to create pool. Live data required in strict mode.');
    }
  }
  
  /**
   * Get all ZFS datasets in a pool
   */
  public static async getDatasets(poolName: string): Promise<any[]> {
    this.logServiceStatus(`Fetching datasets for pool: ${poolName}`);
    try {
      const response = await axios.get<any[]>(`${this.API_URL}/${poolName}/datasets`);
      return response.data.map(dataset => ({
        ...dataset,
        dataSource: DataSourceType.LIVE
      }));
    } catch (error) {
      console.error(`Error fetching datasets for pool '${poolName}':`, error);
      
      // Return placeholder dataset in strict live mode
      return [{
        id: 'placeholder',
        name: 'To be added',
        pool: poolName,
        type: 'filesystem',
        used: '0',
        available: '0',
        referenced: '0',
        recordsize: '0',
        compression: 'none',
        compressratio: '1.0',
        mounted: false,
        mountpoint: '/',
        quota: '0',
        reservation: '0',
        readonly: false,
        dataSource: DataSourceType.PLACEHOLDER
      }];
    }
  }

  /**
   * Create a new ZFS dataset
   */
  public static async createDataset(config: DatasetCreateConfig): Promise<any> {
    this.logServiceStatus(`Creating dataset: ${config.name}`);
    try {
      const response = await axios.post(`${this.DATASETS_API_URL}`, config);
      return {
        ...response.data,
        dataSource: DataSourceType.LIVE
      };
    } catch (error) {
      console.error(`Error creating dataset '${config.name}':`, error);
      throw new Error(`Failed to create dataset: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }

  /**
   * Destroy a ZFS dataset
   */
  public static async destroyDataset(name: string, recursive: boolean = false): Promise<void> {
    this.logServiceStatus(`Destroying dataset: ${name} (recursive: ${recursive})`);
    try {
      await axios.delete(`${this.DATASETS_API_URL}/${encodeURIComponent(name)}`, {
        params: { recursive }
      });
    } catch (error) {
      console.error(`Error destroying dataset '${name}':`, error);
      throw new Error(`Failed to destroy dataset: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }

  /**
   * Clone a ZFS dataset
   */
  public static async cloneDataset(sourceName: string, targetName: string): Promise<any> {
    this.logServiceStatus(`Cloning dataset from ${sourceName} to ${targetName}`);
    try {
      const response = await axios.post(`${this.DATASETS_API_URL}/${encodeURIComponent(sourceName)}/clone`, {
        targetName
      });
      return {
        ...response.data,
        dataSource: DataSourceType.LIVE
      };
    } catch (error) {
      console.error(`Error cloning dataset '${sourceName}' to '${targetName}':`, error);
      throw new Error(`Failed to clone dataset: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }

  /**
   * Rename a ZFS dataset
   */
  public static async renameDataset(oldName: string, newName: string): Promise<void> {
    this.logServiceStatus(`Renaming dataset from ${oldName} to ${newName}`);
    try {
      await axios.put(`${this.DATASETS_API_URL}/${encodeURIComponent(oldName)}/rename`, {
        newName
      });
    } catch (error) {
      console.error(`Error renaming dataset '${oldName}' to '${newName}':`, error);
      throw new Error(`Failed to rename dataset: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }

  /**
   * Update dataset properties
   */
  public static async updateDatasetProperties(name: string, properties: DatasetPropertyUpdate[]): Promise<void> {
    this.logServiceStatus(`Updating properties for dataset: ${name}`);
    try {
      await axios.put(`${this.DATASETS_API_URL}/${encodeURIComponent(name)}/properties`, {
        properties
      });
    } catch (error) {
      console.error(`Error updating properties for dataset '${name}':`, error);
      throw new Error(`Failed to update dataset properties: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }

  /**
   * Get dataset properties
   */
  public static async getDatasetProperties(name: string): Promise<Record<string, any>> {
    this.logServiceStatus(`Fetching properties for dataset: ${name}`);
    try {
      const response = await axios.get(`${this.DATASETS_API_URL}/${encodeURIComponent(name)}/properties`);
      return response.data;
    } catch (error) {
      console.error(`Error fetching properties for dataset '${name}':`, error);
      return {};
    }
  }

  /**
   * Mount a ZFS dataset
   */
  public static async mountDataset(name: string): Promise<void> {
    this.logServiceStatus(`Mounting dataset: ${name}`);
    try {
      await axios.post(`${this.DATASETS_API_URL}/${encodeURIComponent(name)}/mount`);
    } catch (error) {
      console.error(`Error mounting dataset '${name}':`, error);
      throw new Error(`Failed to mount dataset: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }

  /**
   * Unmount a ZFS dataset
   */
  public static async unmountDataset(name: string, force: boolean = false): Promise<void> {
    this.logServiceStatus(`Unmounting dataset: ${name} (force: ${force})`);
    try {
      await axios.post(`${this.DATASETS_API_URL}/${encodeURIComponent(name)}/unmount`, {
        force
      });
    } catch (error) {
      console.error(`Error unmounting dataset '${name}':`, error);
      throw new Error(`Failed to unmount dataset: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }

  /**
   * Create a snapshot of a dataset
   */
  public static async createSnapshot(datasetName: string, snapshotName: string): Promise<any> {
    this.logServiceStatus(`Creating snapshot ${snapshotName} for dataset: ${datasetName}`);
    try {
      const response = await axios.post(`${this.DATASETS_API_URL}/${encodeURIComponent(datasetName)}/snapshots`, {
        snapshotName
      });
      return {
        ...response.data,
        dataSource: DataSourceType.LIVE
      };
    } catch (error) {
      console.error(`Error creating snapshot '${snapshotName}' for dataset '${datasetName}':`, error);
      throw new Error(`Failed to create snapshot: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }

  /**
   * Get snapshots for a dataset
   */
  public static async getDatasetSnapshots(datasetName: string): Promise<any[]> {
    this.logServiceStatus(`Fetching snapshots for dataset: ${datasetName}`);
    try {
      const response = await axios.get(`${this.DATASETS_API_URL}/${encodeURIComponent(datasetName)}/snapshots`);
      return response.data.map((snapshot: any) => ({
        ...snapshot,
        dataSource: DataSourceType.LIVE
      }));
    } catch (error) {
      console.error(`Error fetching snapshots for dataset '${datasetName}':`, error);
      return [];
    }
  }

  /**
   * Get dataset usage statistics
   */
  public static async getDatasetUsage(name: string): Promise<{
    used: number;
    available: number;
    referenced: number;
    compressratio: number;
    usedbychildren: number;
    usedbydataset: number;
    usedbyrefreservation: number;
    usedbysnapshots: number;
  }> {
    this.logServiceStatus(`Fetching usage statistics for dataset: ${name}`);
    try {
      const response = await axios.get(`${this.DATASETS_API_URL}/${encodeURIComponent(name)}/usage`);
      return response.data;
    } catch (error) {
      console.error(`Error fetching usage for dataset '${name}':`, error);
      return {
        used: 0,
        available: 0,
        referenced: 0,
        compressratio: 1.0,
        usedbychildren: 0,
        usedbydataset: 0,
        usedbyrefreservation: 0,
        usedbysnapshots: 0
      };
    }
  }
  
  /**
   * Import a ZFS pool
   */
  public static async importPool(name: string): Promise<void> {
    this.logServiceStatus(`Importing ZFS pool: ${name}`);
    try {
      await axios.post(`${this.API_URL}/${name}/import`);
    } catch (error) {
      console.error(`Error importing ZFS pool '${name}':`, error);
      throw new Error('Failed to import pool. Live data required in strict mode.');
    }
  }
  
  /**
   * Export a ZFS pool
   */
  public static async exportPool(name: string): Promise<void> {
    this.logServiceStatus(`Exporting ZFS pool: ${name}`);
    try {
      await axios.post(`${this.API_URL}/${name}/export`);
    } catch (error) {
      console.error(`Error exporting ZFS pool '${name}':`, error);
      throw new Error('Failed to export pool. Live data required in strict mode.');
    }
  }
  
  /**
   * Initiate a scrub operation on a ZFS pool
   */
  public static async scrubPool(name: string): Promise<void> {
    this.logServiceStatus(`Starting scrub for ZFS pool: ${name}`);
    try {
      await axios.post(`${this.API_URL}/${name}/scrub`);
    } catch (error) {
      console.error(`Error starting scrub for ZFS pool '${name}':`, error);
      throw new Error('Failed to start scrub. Live data required in strict mode.');
    }
  }
  
  /**
   * Update dataset record size
   */
  public static async updateDatasetRecordSize(
    datasetName: string, 
    newRecordSize: string
  ): Promise<void> {
    this.logServiceStatus(`Updating record size for dataset: ${datasetName} to ${newRecordSize}`);
    try {
      await axios.put(`${this.API_URL}/datasets/${datasetName}/recordsize`, {
        recordsize: newRecordSize
      });
    } catch (error) {
      console.error(`Error updating record size for dataset '${datasetName}':`, error);
      throw new Error('Failed to update dataset record size. Live data required in strict mode.');
    }
  }
  
  /**
   * Create a ZFS pool with advanced configuration
   */
  public static async createPool(config: {
    name: string;
    devices: string[];
    vdevType?: string;
    properties?: Record<string, any>;
  }): Promise<ZfsPool> {
    this.logServiceStatus(`Creating ZFS pool with advanced config: ${config.name}`);
    try {
      const response = await axios.post<ZfsPool>(`${this.API_URL}/create`, config);
      return {
        ...response.data,
        dataSource: DataSourceType.LIVE
      };
    } catch (error) {
      console.error(`Error creating ZFS pool '${config.name}':`, error);
      throw new Error(`Failed to create pool: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }
  
  /**
   * Destroy a ZFS pool
   */
  public static async destroyPool(name: string): Promise<void> {
    this.logServiceStatus(`Destroying ZFS pool: ${name}`);
    try {
      await axios.delete(`${this.API_URL}/${name}`);
    } catch (error) {
      console.error(`Error destroying ZFS pool '${name}':`, error);
      throw new Error('Failed to destroy pool. Live data required in strict mode.');
    }
  }
  
  /**
   * Get pool status
   */
  public static async getPoolStatus(name: string): Promise<string> {
    this.logServiceStatus(`Getting status for ZFS pool: ${name}`);
    try {
      const response = await axios.get<{status: string}>(`${this.API_URL}/${name}/status`);
      return response.data.status;
    } catch (error) {
      console.error(`Error getting status for ZFS pool '${name}':`, error);
      return 'UNAVAIL';
    }
  }
} 
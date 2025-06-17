import axios from 'axios';
import { API_BASE_URL } from '../config';
import { DataSourceType, isStrictLiveMode } from '../utils/env';

/**
 * ZFS Pool interface
 */
export interface ZfsPool {
  id: string;
  name: string;
  health: string;
  size: number;
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
 * ZFS Pool service for managing ZFS pools
 */
export class ZfsPoolService {
  private static readonly API_URL = `${API_BASE_URL}/api/zfs/pools`;
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
        size: 0,
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
        size: 0,
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
   * Start a scrub operation on a ZFS pool
   */
  public static async scrubPool(name: string): Promise<void> {
    this.logServiceStatus(`Starting scrub on ZFS pool: ${name}`);
    try {
      await axios.post(`${this.API_URL}/${name}/scrub`);
    } catch (error) {
      console.error(`Error starting scrub on ZFS pool '${name}':`, error);
      throw new Error('Failed to start scrub. Live data required in strict mode.');
    }
  }
  
  /**
   * Update a dataset's record size
   */
  public static async updateDatasetRecordSize(
    datasetName: string, 
    newRecordSize: string
  ): Promise<void> {
    this.logServiceStatus(`Updating record size for dataset: ${datasetName}`);
    try {
      await axios.put(`${API_BASE_URL}/api/datasets/${datasetName}`, {
        recordsize: newRecordSize
      });
    } catch (error) {
      console.error(`Error updating record size for dataset '${datasetName}':`, error);
      throw new Error('Failed to update dataset record size. Live data required in strict mode.');
    }
  }
} 
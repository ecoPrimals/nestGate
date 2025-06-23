import { invoke } from '@tauri-apps/api/tauri';

// Define ZFS types
export interface ZfsPool {
  id: string;
  name: string;
  health: string;
  status: string;
  size: number;
  allocated: number;
  free: number;
  used: number;
  isMock?: boolean;
}

export interface ZfsDataset {
  id: string;
  name: string;
  pool: string;
  type: string;
  used: number;
  available: number;
  referenced: number;
  recordsize: string;
  compression: string;
  compressratio: number;
  mounted: boolean;
  mountpoint: string;
  quota: number;
  reservation: number;
  readonly: boolean;
  isMock?: boolean;
}

/**
 * ZFS Plugin for Tauri integration
 * Provides ZFS pool and dataset management through Tauri backend
 */
export class ZfsPlugin {

  /**
   * Check if ZFS is available on the system
   */
  static async isAvailable(): Promise<boolean> {
    try {
      return await invoke('plugin:zfs|is_available');
    } catch (error) {
      console.error('Error checking ZFS availability via Tauri:', error);
      return false;
    }
  }

  /**
   * List all ZFS pools
   */
  static async listPools(): Promise<ZfsPool[]> {
    try {
      const pools = await invoke('plugin:zfs|list_pools') as any[];
      console.log('Raw pool data:', pools);
      
      // Transform the data to match our interface
      return pools.map((pool: any) => ({
        id: pool.id || pool.name || '', // Use id or name as ID
        name: pool.name || '',
        health: pool.health || 'UNKNOWN',
        status: pool.status || 'ONLINE', // Add missing status
        size: pool.size || pool.capacity || 0,
        allocated: pool.used || 0, // Add missing allocated (use used as fallback)
        free: pool.free || pool.available || 0,
        used: pool.used || 0,
        isMock: true // Add mock flag for development testing
      }));
    } catch (error) {
      console.error('Error listing ZFS pools via Tauri:', error);
      throw error;
    }
  }

  /**
   * List all datasets for a pool
   */
  static async listDatasets(poolName: string): Promise<ZfsDataset[]> {
    try {
      const datasets = await invoke('plugin:zfs|list_datasets', { poolName }) as any[];
      console.log('Raw dataset data:', datasets);
      
      // Transform the data to match our interface
      return datasets.map((dataset: any) => ({
        id: dataset.id || dataset.name || '', // Use id or name as ID
        name: dataset.name || '',
        pool: poolName,
        type: dataset.type || 'filesystem',
        used: dataset.used || 0,
        available: dataset.available || 0,
        referenced: dataset.referenced || dataset.used || 0,
        recordsize: dataset.recordsize || '128K',
        compression: dataset.compression || 'lz4',
        compressratio: dataset.compressratio || 1.0,
        mounted: dataset.mounted !== undefined ? dataset.mounted : true,
        mountpoint: dataset.mountpoint || `/mnt/${dataset.name}`,
        quota: dataset.quota || 0,
        reservation: dataset.reservation || 0,
        readonly: dataset.readonly !== undefined ? dataset.readonly : false,
        isMock: true // Add mock flag for development testing
      }));
    } catch (error) {
      console.error(`Error listing datasets for pool ${poolName} via Tauri:`, error);
      throw error;
    }
  }

  /**
   * Get a specific ZFS pool
   */
  static async getPool(poolName: string): Promise<ZfsPool | null> {
    try {
      const pools = await this.listPools();
      const pool = pools.find(p => p.name === poolName || p.id === poolName);
      console.log(`Pool ${poolName} found:`, pool);
      return pool || null;
    } catch (error) {
      console.error(`Error getting ZFS pool ${poolName} via Tauri:`, error);
      throw error;
    }
  }

  /**
   * Create a new ZFS dataset
   */
  static async createDataset(name: string, properties?: Record<string, string>): Promise<void> {
    try {
      await invoke('plugin:zfs|create_dataset', { 
        datasetName: name, 
        properties: properties || {} 
      });
    } catch (error) {
      console.error(`Error creating dataset ${name} via Tauri:`, error);
      throw error;
    }
  }

  /**
   * Get metrics for a pool
   */
  static async getPoolMetrics(poolName: string): Promise<any> {
    try {
      const metrics = await invoke('plugin:zfs|get_pool_metrics', { poolName });
      console.log(`Pool ${poolName} metrics:`, metrics);
      return metrics;
    } catch (error) {
      console.error(`Error getting metrics for pool ${poolName} via Tauri:`, error);
      throw error;
    }
  }
} 
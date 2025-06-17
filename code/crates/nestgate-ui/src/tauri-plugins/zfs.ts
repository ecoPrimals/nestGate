// Replace the import from @tauri-apps/api/tauri with either a custom implementation or use a mock
// import { invoke } from '@tauri-apps/api/tauri';
import { ZfsPool, ZfsDataset } from '../services/zfs-pool.service';

// Create a mock invoke function for development
const invoke = async (command: string, args?: Record<string, any>): Promise<any> => {
  // Check if we're in a Tauri environment with the real invoke function
  if (window.__TAURI__?.invoke) {
    return window.__TAURI__.invoke(command, args);
  }
  
  // If not, throw an error to indicate the need to use the shim
  throw new Error('Tauri invoke not available, make sure the tauri-plugin-zfs.js shim is loaded');
};

// Add window.__TAURI__ type definition
declare global {
  interface Window {
    __TAURI__?: {
      invoke: (command: string, args?: Record<string, any>) => Promise<any>;
      process?: {
        exit: (code?: number) => void;
      };
    };
  }
}

/**
 * Tauri plugin for ZFS integration
 * 
 * This plugin provides a bridge between the UI and the ZFS functionality
 * in the Rust backend. It uses Tauri's invoke mechanism to call Rust functions.
 */
export class ZfsPlugin {
  /**
   * Check if the ZFS plugin is available
   */
  static async isAvailable(): Promise<boolean> {
    try {
      const available = await invoke('plugin:zfs|is_available');
      console.log('ZFS plugin available:', available);
      return !!available;
    } catch (error) {
      console.log('ZFS plugin not available:', error);
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
        size: pool.size || pool.capacity || 0,
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

  /**
   * Terminate the Tauri process gracefully
   */
  static terminate(exitCode: number = 0): void {
    console.log(`Terminating Tauri process with exit code ${exitCode}`);
    if (window.__TAURI__?.process?.exit) {
      window.__TAURI__.process.exit(exitCode);
    }
  }
}

// Set up an event listener for the beforeunload event to clean up resources
window.addEventListener('beforeunload', () => {
  console.log('Window unloading, terminating process');
  try {
    // Attempt to gracefully shut down
    ZfsPlugin.terminate();
  } catch (error) {
    console.error('Error during termination:', error);
  }
});

// Export the plugin as default
export default ZfsPlugin; 
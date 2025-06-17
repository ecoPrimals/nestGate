import axios from 'axios';
import { DataSourceType } from '../utils/env';

export interface Snapshot {
  id: string;
  name: string;
  dataset: string;
  created: string;
  size: string;
  isReplicated: boolean;
  dataSource?: DataSourceType;
}

export class SnapshotService {
  private static instance: SnapshotService;
  private baseUrl: string;

  private constructor() {
    // Get API port from environment or use default
    const apiPort = process.env.REACT_APP_API_PORT || '3051';
    this.baseUrl = `http://localhost:${apiPort}/api/v1/snapshots`;
  }

  public static getInstance(): SnapshotService {
    if (!SnapshotService.instance) {
      SnapshotService.instance = new SnapshotService();
    }
    return SnapshotService.instance;
  }

  /**
   * Check if we're in strict live mode
   */
  private isStrictLiveMode(): boolean {
    return process.env.REACT_APP_STRICT_DATA_MODE === 'true';
  }

  /**
   * Get all snapshots from the API
   */
  public async getSnapshots(dataset?: string): Promise<Snapshot[]> {
    try {
      const url = dataset ? `${this.baseUrl}?dataset=${encodeURIComponent(dataset)}` : this.baseUrl;
      const response = await axios.get(url);
      
      return response.data.map((snapshot: any) => ({
        ...snapshot,
        dataSource: DataSourceType.LIVE
      }));
    } catch (error) {
      console.error('Error fetching snapshots:', error);
      
      if (this.isStrictLiveMode()) {
        // In strict mode, return placeholder data with the dataset if provided
        return [{
          id: dataset ? `${dataset}@placeholder` : 'placeholder@placeholder',
          name: 'To be added',
          dataset: dataset || 'To be added',
          created: new Date().toISOString(),
          size: '0 B',
          isReplicated: false,
          dataSource: DataSourceType.PLACEHOLDER
        }];
      }
      
      throw error;
    }
  }

  /**
   * Get a snapshot by ID
   */
  public async getSnapshotById(id: string): Promise<Snapshot | null> {
    try {
      const response = await axios.get(`${this.baseUrl}/${encodeURIComponent(id)}`);
      
      return {
        ...response.data,
        dataSource: DataSourceType.LIVE
      };
    } catch (error) {
      console.error(`Error fetching snapshot ${id}:`, error);
      
      if (this.isStrictLiveMode()) {
        // Return placeholder data with the original ID structure
        const [dataset] = id.split('@');
        return {
          id,
          name: 'To be added',
          dataset: dataset || 'To be added',
          created: new Date().toISOString(),
          size: '0 B',
          isReplicated: false,
          dataSource: DataSourceType.PLACEHOLDER
        };
      }
      
      return null;
    }
  }

  /**
   * Create a new snapshot
   */
  public async createSnapshot(datasetName: string, snapshotName: string, recursive: boolean = false): Promise<Snapshot> {
    try {
      const response = await axios.post(this.baseUrl, {
        dataset: datasetName,
        name: snapshotName,
        recursive
      });
      
      return {
        ...response.data,
        dataSource: DataSourceType.LIVE
      };
    } catch (error) {
      console.error('Error creating snapshot:', error);
      
      if (this.isStrictLiveMode()) {
        throw new Error('Failed to create snapshot: ' + (error as Error).message);
      }
      
      throw error;
    }
  }

  /**
   * Delete a snapshot
   */
  public async deleteSnapshot(id: string, recursive: boolean = false): Promise<void> {
    try {
      await axios.delete(`${this.baseUrl}/${encodeURIComponent(id)}`, {
        params: { recursive }
      });
    } catch (error) {
      console.error(`Error deleting snapshot ${id}:`, error);
      
      if (this.isStrictLiveMode()) {
        throw new Error('Failed to delete snapshot: ' + (error as Error).message);
      }
      
      throw error;
    }
  }

  /**
   * Rollback to a snapshot
   */
  public async rollbackSnapshot(id: string, force: boolean = false): Promise<void> {
    try {
      await axios.post(`${this.baseUrl}/${encodeURIComponent(id)}/rollback`, {
        force
      });
    } catch (error) {
      console.error(`Error rolling back to snapshot ${id}:`, error);
      
      if (this.isStrictLiveMode()) {
        throw new Error('Failed to rollback snapshot: ' + (error as Error).message);
      }
      
      throw error;
    }
  }

  /**
   * Get snapshots for a specific dataset
   */
  public async getDatasetSnapshots(datasetName: string): Promise<Snapshot[]> {
    return this.getSnapshots(datasetName);
  }
} 
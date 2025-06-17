import axios from 'axios';
import { API_BASE_URL } from '../../config';
import { DataSourceType } from '../../utils/env';

/**
 * ZFS Snapshot interface
 */
export interface ZfsSnapshot {
  id: string;
  name: string;
  dataset: string;
  created: string;
  size: string;
  isReplicated?: boolean;
  dataSource?: DataSourceType;
}

export class SnapshotService {
  private static instance: SnapshotService;
  private static readonly API_URL = `${API_BASE_URL}/api/snapshots`;

  /**
   * Get singleton instance
   */
  public static getInstance(): SnapshotService {
    if (!SnapshotService.instance) {
      SnapshotService.instance = new SnapshotService();
    }
    return SnapshotService.instance;
  }

  /**
   * Get all snapshots or snapshots for a specific dataset
   * @param dataset Optional dataset to filter snapshots
   */
  public async getSnapshots(dataset?: string): Promise<ZfsSnapshot[]> {
    try {
      const url = dataset ? `${SnapshotService.API_URL}?dataset=${dataset}` : SnapshotService.API_URL;
      const response = await axios.get(url);
      
      return response.data.map((snapshot: ZfsSnapshot) => ({
        ...snapshot,
        dataSource: DataSourceType.LIVE
      }));
    } catch (error) {
      console.error('Error fetching snapshots:', error);
      
      // Return placeholder snapshot data
      return [{
        id: 'placeholder',
        name: 'To be added',
        dataset: dataset || 'unknown',
        created: new Date().toISOString(),
        size: '0',
        dataSource: DataSourceType.PLACEHOLDER
      }];
    }
  }

  /**
   * Get a snapshot by its ID
   */
  public async getSnapshotById(id: string): Promise<ZfsSnapshot | null> {
    try {
      const response = await axios.get(`${SnapshotService.API_URL}/${id}`);
      return {
        ...response.data,
        dataSource: DataSourceType.LIVE
      };
    } catch (error) {
      console.error(`Error fetching snapshot with ID '${id}':`, error);
      
      // Return placeholder snapshot
      return {
        id: id || 'placeholder',
        name: 'To be added',
        dataset: 'unknown',
        created: new Date().toISOString(),
        size: '0',
        dataSource: DataSourceType.PLACEHOLDER
      };
    }
  }

  /**
   * Create a new snapshot
   */
  public async createSnapshot(dataset: string, name: string): Promise<ZfsSnapshot> {
    try {
      const response = await axios.post(SnapshotService.API_URL, { dataset, name });
      return {
        ...response.data,
        dataSource: DataSourceType.LIVE
      };
    } catch (error) {
      console.error('Error creating snapshot:', error);
      throw new Error('Failed to create snapshot. Live data required in strict mode.');
    }
  }

  /**
   * Delete a snapshot
   */
  public async deleteSnapshot(id: string): Promise<void> {
    try {
      await axios.delete(`${SnapshotService.API_URL}/${id}`);
    } catch (error) {
      console.error(`Error deleting snapshot with ID '${id}':`, error);
      throw new Error('Failed to delete snapshot. Live data required in strict mode.');
    }
  }

  /**
   * Rollback to a snapshot
   */
  public async rollbackSnapshot(id: string): Promise<void> {
    try {
      await axios.post(`${SnapshotService.API_URL}/${id}/rollback`);
    } catch (error) {
      console.error(`Error rolling back to snapshot with ID '${id}':`, error);
      throw new Error('Failed to rollback snapshot. Live data required in strict mode.');
    }
  }
} 
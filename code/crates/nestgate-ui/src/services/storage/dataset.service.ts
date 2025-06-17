import axios from 'axios';
import { API_BASE_URL } from '../../config';
import { DataSourceType, isStrictLiveMode } from '../../utils/env';

/**
 * ZFS Dataset interface
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
 * Service responsible for dataset management operations.
 */
export class DatasetService {
  private static instance: DatasetService;
  private static readonly API_URL = `${API_BASE_URL}/api/datasets`;
  
  /**
   * Gets the singleton instance of DatasetService
   */
  public static getInstance(): DatasetService {
    if (!DatasetService.instance) {
      DatasetService.instance = new DatasetService();
    }
    return DatasetService.instance;
  }

  /**
   * Get all datasets
   */
  public async getDatasets(): Promise<ZfsDataset[]> {
    try {
      const response = await axios.get(DatasetService.API_URL);
      return response.data.map((dataset: ZfsDataset) => ({
        ...dataset,
        dataSource: DataSourceType.LIVE
      }));
    } catch (error) {
      console.error('Error fetching datasets:', error);
      
      // Return placeholder dataset in strict live mode
      return [{
        id: 'placeholder',
        name: 'To be added',
        pool: 'live',
        type: 'filesystem',
        used: '0',
        available: '0',
        referenced: '0',
        recordsize: '0',
        compression: 'none',
        compressratio: '0',
        mounted: false,
        mountpoint: 'N/A',
        quota: 'none',
        reservation: 'none',
        readonly: false,
        dataSource: DataSourceType.PLACEHOLDER
      }];
    }
  }

  /**
   * Get a dataset by its ID
   */
  public async getDatasetById(id: string): Promise<ZfsDataset | null> {
    try {
      const response = await axios.get(`${DatasetService.API_URL}/${id}`);
      return {
        ...response.data,
        dataSource: DataSourceType.LIVE
      };
    } catch (error) {
      console.error(`Error fetching dataset with ID '${id}':`, error);
      
      // Return placeholder dataset in strict live mode
      return {
        id: id || 'placeholder',
        name: 'To be added',
        pool: 'live',
        type: 'filesystem',
        used: '0',
        available: '0',
        referenced: '0',
        recordsize: '0',
        compression: 'none',
        compressratio: '0',
        mounted: false,
        mountpoint: 'N/A',
        quota: 'none',
        reservation: 'none',
        readonly: false,
        dataSource: DataSourceType.PLACEHOLDER
      };
    }
  }

  /**
   * Create a new dataset
   */
  public async createDataset(dataset: Partial<ZfsDataset>): Promise<ZfsDataset> {
    try {
      const response = await axios.post(DatasetService.API_URL, dataset);
      return {
        ...response.data,
        dataSource: DataSourceType.LIVE
      };
    } catch (error) {
      console.error('Error creating dataset:', error);
      throw new Error('Failed to create dataset. Live data required in strict mode.');
    }
  }

  /**
   * Update a dataset
   */
  public async updateDataset(id: string, dataset: Partial<ZfsDataset>): Promise<ZfsDataset> {
    try {
      const response = await axios.put(`${DatasetService.API_URL}/${id}`, dataset);
      return {
        ...response.data,
        dataSource: DataSourceType.LIVE
      };
    } catch (error) {
      console.error(`Error updating dataset with ID '${id}':`, error);
      throw new Error('Failed to update dataset. Live data required in strict mode.');
    }
  }

  /**
   * Delete a dataset
   */
  public async deleteDataset(id: string): Promise<void> {
    try {
      await axios.delete(`${DatasetService.API_URL}/${id}`);
    } catch (error) {
      console.error(`Error deleting dataset with ID '${id}':`, error);
      throw new Error('Failed to delete dataset. Live data required in strict mode.');
    }
  }
} 
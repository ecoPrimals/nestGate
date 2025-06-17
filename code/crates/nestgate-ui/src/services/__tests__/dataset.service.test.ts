import axios from 'axios';
import { DatasetService } from '../storage/dataset.service';
import { ZfsDataset } from '../zfs-pool.service';

jest.mock('axios');
const mockedAxios = axios as jest.Mocked<typeof axios>;

describe('DatasetService', () => {
  let datasetService: DatasetService;

  // Sample mock dataset
  const mockDatasets: ZfsDataset[] = [
    {
      id: 'tank/data',
      name: 'data',
      pool: 'tank',
      type: 'filesystem',
      used: '56.7 GB',
      available: '843.3 GB',
      referenced: '56.7 GB',
      recordsize: '128K',
      compression: 'lz4',
      compressratio: '1.8x',
      mounted: true,
      mountpoint: '/mnt/tank/data',
      quota: 'none',
      reservation: 'none',
      readonly: false
    },
    {
      id: 'tank/backups',
      name: 'backups',
      pool: 'tank',
      type: 'filesystem',
      used: '120.5 GB',
      available: '779.5 GB',
      referenced: '120.5 GB',
      recordsize: '1M',
      compression: 'gzip-9',
      compressratio: '3.2x',
      mounted: true,
      mountpoint: '/mnt/tank/backups',
      quota: 'none',
      reservation: '100G',
      readonly: true
    }
  ];

  beforeEach(() => {
    jest.clearAllMocks();
    datasetService = DatasetService.getInstance();
  });

  describe('getInstance', () => {
    it('should return the same instance on multiple calls', () => {
      const instance1 = DatasetService.getInstance();
      const instance2 = DatasetService.getInstance();
      expect(instance1).toBe(instance2);
    });
  });

  describe('getDatasets', () => {
    it('should fetch datasets from API', async () => {
      mockedAxios.get.mockResolvedValueOnce({ data: mockDatasets });
      
      const result = await datasetService.getDatasets();
      
      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
      expect(result).toEqual(mockDatasets);
    });

    it('should return mock datasets when API fails', async () => {
      mockedAxios.get.mockRejectedValueOnce(new Error('Network error'));
      
      const result = await datasetService.getDatasets();
      
      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
      expect(result.length).toBeGreaterThan(0);
      expect(result[0].isMock).toBe(true);
    });
  });

  describe('getDatasetById', () => {
    it('should fetch a dataset by ID from API', async () => {
      const mockDataset = mockDatasets[0];
      mockedAxios.get.mockResolvedValueOnce({ data: mockDataset });
      
      const result = await datasetService.getDatasetById('tank/data');
      
      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
      expect(mockedAxios.get).toHaveBeenCalledWith(expect.stringContaining('/tank/data'));
      expect(result).toEqual(mockDataset);
    });

    it('should return mock dataset when API fails', async () => {
      mockedAxios.get.mockRejectedValueOnce(new Error('Network error'));
      
      const result = await datasetService.getDatasetById('tank/data');
      
      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
      expect(result?.isMock).toBe(true);
      expect(result?.id).toBe('tank/data');
    });

    it('should return null when dataset not found', async () => {
      mockedAxios.get.mockRejectedValueOnce(new Error('Dataset not found'));
      
      const result = await datasetService.getDatasetById('nonexistent/dataset');
      
      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
      expect(result).toBeNull();
    });
  });

  describe('createDataset', () => {
    const newDatasetOptions = {
      recordsize: '128K',
      compression: 'lz4',
      quota: '100G',
      readonly: false
    };

    it('should create a dataset through API', async () => {
      const createdDataset = {
        id: 'tank/newdata',
        name: 'newdata',
        pool: 'tank',
        type: 'filesystem',
        used: '0',
        available: '900 GB',
        referenced: '0',
        recordsize: '128K',
        compression: 'lz4',
        compressratio: '1.00x',
        mounted: true,
        mountpoint: '/mnt/tank/newdata',
        quota: '100G',
        reservation: 'none',
        readonly: false
      };
      
      mockedAxios.post.mockResolvedValueOnce({ data: createdDataset });
      
      const result = await datasetService.createDataset('tank', 'newdata', newDatasetOptions);
      
      expect(mockedAxios.post).toHaveBeenCalledTimes(1);
      expect(mockedAxios.post).toHaveBeenCalledWith(
        expect.any(String),
        expect.objectContaining({
          poolName: 'tank',
          datasetName: 'newdata',
          ...newDatasetOptions
        })
      );
      expect(result).toEqual(createdDataset);
    });

    it('should create a mock dataset when API fails', async () => {
      mockedAxios.post.mockRejectedValueOnce(new Error('Network error'));
      
      const result = await datasetService.createDataset('tank', 'newdata', newDatasetOptions);
      
      expect(mockedAxios.post).toHaveBeenCalledTimes(1);
      expect(result.isMock).toBe(true);
      expect(result.id).toBe('tank/newdata');
      expect(result.name).toBe('newdata');
      expect(result.pool).toBe('tank');
      expect(result.compression).toBe('lz4');
      expect(result.quota).toBe('100G');
    });
  });

  describe('updateDataset', () => {
    const updateOptions = {
      recordsize: '1M',
      compression: 'zstd',
      quota: '200G'
    };

    it('should update a dataset through API', async () => {
      const updatedDataset = {
        ...mockDatasets[0],
        recordsize: '1M',
        compression: 'zstd',
        quota: '200G'
      };
      
      mockedAxios.put.mockResolvedValueOnce({ data: updatedDataset });
      
      const result = await datasetService.updateDataset('tank/data', updateOptions);
      
      expect(mockedAxios.put).toHaveBeenCalledTimes(1);
      expect(mockedAxios.put).toHaveBeenCalledWith(
        expect.stringContaining('/tank/data'),
        updateOptions
      );
      expect(result).toEqual(updatedDataset);
    });

    it('should update a mock dataset when API fails', async () => {
      mockedAxios.put.mockRejectedValueOnce(new Error('Network error'));
      
      // Call getDatasets first to populate mock data internally
      mockedAxios.get.mockRejectedValueOnce(new Error('Network error'));
      await datasetService.getDatasets();
      
      const result = await datasetService.updateDataset('tank/data', updateOptions);
      
      expect(mockedAxios.put).toHaveBeenCalledTimes(1);
      expect(result.isMock).toBe(true);
      expect(result.recordsize).toBe('1M');
      expect(result.compression).toBe('zstd');
      expect(result.quota).toBe('200G');
    });

    it('should throw error when dataset not found', async () => {
      mockedAxios.put.mockRejectedValueOnce(new Error('Dataset not found'));
      
      await expect(
        datasetService.updateDataset('nonexistent/dataset', updateOptions)
      ).rejects.toThrow('Dataset with ID \'nonexistent/dataset\' not found');
    });
  });

  describe('deleteDataset', () => {
    it('should delete a dataset through API', async () => {
      mockedAxios.delete.mockResolvedValueOnce({});
      
      const result = await datasetService.deleteDataset('tank/data');
      
      expect(mockedAxios.delete).toHaveBeenCalledTimes(1);
      expect(mockedAxios.delete).toHaveBeenCalledWith(expect.stringContaining('/tank/data'));
      expect(result).toBe(true);
    });

    it('should return false when deletion fails', async () => {
      mockedAxios.delete.mockRejectedValueOnce(new Error('Network error'));
      
      const result = await datasetService.deleteDataset('tank/data');
      
      expect(mockedAxios.delete).toHaveBeenCalledTimes(1);
      expect(result).toBe(false);
    });
  });
}); 
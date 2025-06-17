import axios from 'axios';
import { DatasetService } from '../storage/dataset.service';
import { DataSourceType } from '../../utils/env';

// Mock the axios module
jest.mock('axios');
const mockedAxios = axios as jest.Mocked<typeof axios>;

// Mock the env utils
jest.mock('../../utils/env', () => ({
  DataSourceType: {
    LIVE: 'LIVE',
    MOCK: 'MOCK',
    TEST: 'TEST',
    PLACEHOLDER: 'PLACEHOLDER'
  },
  isStrictLiveMode: jest.fn().mockReturnValue(true)
}));

describe('DatasetService in strict live mode', () => {
  let datasetService: DatasetService;

  beforeEach(() => {
    jest.clearAllMocks();
    datasetService = DatasetService.getInstance();
  });

  describe('getDatasets', () => {
    it('should return live data when API call succeeds', async () => {
      const mockApiResponse = [
        {
          id: 'tank/data',
          name: 'data',
          pool: 'tank',
          type: 'filesystem',
          used: '50GB',
          available: '950GB',
          referenced: '50GB',
          recordsize: '128K',
          compression: 'lz4',
          compressratio: '1.5',
          mounted: true,
          mountpoint: '/mnt/tank/data',
          quota: 'none',
          reservation: 'none',
          readonly: false
        }
      ];

      mockedAxios.get.mockResolvedValueOnce({ data: mockApiResponse });

      const result = await datasetService.getDatasets();

      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
      expect(result).toHaveLength(1);
      expect(result[0].dataSource).toBe(DataSourceType.LIVE);
      expect(result[0].name).toBe('data');
    });

    it('should return placeholder data when API call fails', async () => {
      mockedAxios.get.mockRejectedValueOnce(new Error('Network error'));

      const result = await datasetService.getDatasets();

      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
      expect(result).toHaveLength(1);
      expect(result[0].dataSource).toBe(DataSourceType.PLACEHOLDER);
      expect(result[0].name).toBe('To be added');
    });
  });

  describe('getDatasetById', () => {
    it('should return live data when API call succeeds', async () => {
      const mockApiResponse = {
        id: 'tank/data',
        name: 'data',
        pool: 'tank',
        type: 'filesystem',
        used: '50GB',
        available: '950GB',
        referenced: '50GB',
        recordsize: '128K',
        compression: 'lz4',
        compressratio: '1.5',
        mounted: true,
        mountpoint: '/mnt/tank/data',
        quota: 'none',
        reservation: 'none',
        readonly: false
      };

      mockedAxios.get.mockResolvedValueOnce({ data: mockApiResponse });

      const result = await datasetService.getDatasetById('tank/data');

      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
      expect(result).not.toBeNull();
      expect(result?.dataSource).toBe(DataSourceType.LIVE);
      expect(result?.name).toBe('data');
    });

    it('should return placeholder data when API call fails', async () => {
      mockedAxios.get.mockRejectedValueOnce(new Error('Network error'));

      const result = await datasetService.getDatasetById('tank/data');

      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
      expect(result).not.toBeNull();
      expect(result?.dataSource).toBe(DataSourceType.PLACEHOLDER);
      expect(result?.name).toBe('To be added');
      expect(result?.id).toBe('tank/data'); // Should keep original ID
    });
  });

  describe('createDataset', () => {
    it('should return live data when API call succeeds', async () => {
      const newDataset = {
        pool: 'tank',
        name: 'newdata',
        type: 'filesystem',
        recordsize: '128K',
        compression: 'lz4'
      };

      const mockApiResponse = {
        id: 'tank/newdata',
        name: 'newdata',
        pool: 'tank',
        type: 'filesystem',
        used: '0',
        available: '1000GB',
        referenced: '0',
        recordsize: '128K',
        compression: 'lz4',
        compressratio: '1.0',
        mounted: true,
        mountpoint: '/mnt/tank/newdata',
        quota: 'none',
        reservation: 'none',
        readonly: false
      };

      mockedAxios.post.mockResolvedValueOnce({ data: mockApiResponse });

      const result = await datasetService.createDataset(newDataset);

      expect(mockedAxios.post).toHaveBeenCalledTimes(1);
      expect(result.dataSource).toBe(DataSourceType.LIVE);
      expect(result.name).toBe('newdata');
    });

    it('should throw error when API call fails in strict live mode', async () => {
      const newDataset = {
        pool: 'tank',
        name: 'newdata',
        type: 'filesystem',
        recordsize: '128K',
        compression: 'lz4'
      };

      mockedAxios.post.mockRejectedValueOnce(new Error('Network error'));

      await expect(datasetService.createDataset(newDataset)).rejects.toThrow(
        'Failed to create dataset'
      );
      expect(mockedAxios.post).toHaveBeenCalledTimes(1);
    });
  });

  describe('updateDataset', () => {
    it('should return live data when API call succeeds', async () => {
      const updates = {
        recordsize: '256K',
        compression: 'gzip'
      };

      const mockApiResponse = {
        id: 'tank/data',
        name: 'data',
        pool: 'tank',
        type: 'filesystem',
        used: '50GB',
        available: '950GB',
        referenced: '50GB',
        recordsize: '256K',
        compression: 'gzip',
        compressratio: '1.5',
        mounted: true,
        mountpoint: '/mnt/tank/data',
        quota: 'none',
        reservation: 'none',
        readonly: false
      };

      mockedAxios.put.mockResolvedValueOnce({ data: mockApiResponse });

      const result = await datasetService.updateDataset('tank/data', updates);

      expect(mockedAxios.put).toHaveBeenCalledTimes(1);
      expect(result.dataSource).toBe(DataSourceType.LIVE);
      expect(result.recordsize).toBe('256K');
      expect(result.compression).toBe('gzip');
    });

    it('should throw error when API call fails in strict live mode', async () => {
      const updates = {
        recordsize: '256K'
      };

      mockedAxios.put.mockRejectedValueOnce(new Error('Network error'));

      await expect(datasetService.updateDataset('tank/data', updates)).rejects.toThrow(
        'Failed to update dataset'
      );
      expect(mockedAxios.put).toHaveBeenCalledTimes(1);
    });
  });

  describe('deleteDataset', () => {
    it('should succeed when API call succeeds', async () => {
      mockedAxios.delete.mockResolvedValueOnce({});

      await expect(datasetService.deleteDataset('tank/data')).resolves.not.toThrow();
      expect(mockedAxios.delete).toHaveBeenCalledTimes(1);
    });

    it('should throw error when API call fails in strict live mode', async () => {
      mockedAxios.delete.mockRejectedValueOnce(new Error('Network error'));

      await expect(datasetService.deleteDataset('tank/data')).rejects.toThrow(
        'Failed to delete dataset'
      );
      expect(mockedAxios.delete).toHaveBeenCalledTimes(1);
    });
  });
}); 
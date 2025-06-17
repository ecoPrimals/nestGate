import { DatasetService } from '../../services/storage/dataset.service';
import * as env from '../../utils/env';

// Mock env utilities
jest.mock('../../utils/env', () => ({
  isLiveMode: jest.fn().mockReturnValue(true),
  useMockData: jest.fn().mockReturnValue(false),
  isStrictLiveMode: jest.fn().mockReturnValue(true)
}));

// Mock axios instead of fetch
jest.mock('axios', () => ({
  get: jest.fn(),
  post: jest.fn(),
  put: jest.fn(),
  delete: jest.fn()
}));

import axios from 'axios';
const mockAxios = axios as jest.Mocked<typeof axios>;

describe('DatasetService in strict live mode', () => {
  let service: DatasetService;

  beforeEach(() => {
    jest.clearAllMocks();
    service = DatasetService.getInstance();
  });

  describe('getDatasets', () => {
    it('should return live data when API call succeeds', async () => {
      // Mock successful API response
      const mockApiResponse = [
        {
          id: 'tank/data',
          name: 'data',
          pool: 'tank',
          type: 'filesystem',
          mountpoint: '/mnt/tank/data',
          available: '1024000',
          used: '512000',
          referenced: '256000',
          compressratio: '1.8',
          mounted: true,
          recordsize: '128K',
          compression: 'lz4',
          quota: 'none',
          reservation: 'none',
          readonly: false
        }
      ];

      mockAxios.get.mockResolvedValueOnce({
        data: mockApiResponse
      });

      const datasets = await service.getDatasets();

      expect(mockAxios.get).toHaveBeenCalledWith('http://localhost:3000/api/datasets');
      expect(datasets).toEqual(expect.arrayContaining([
        expect.objectContaining({
          id: 'tank/data',
          dataSource: 'LIVE'
        })
      ]));
      expect(env.isStrictLiveMode).not.toHaveBeenCalled(); // This is checked inside the service
    });

    it('should provide placeholder data when API call fails', async () => {
      // Mock failed API response
      mockAxios.get.mockRejectedValueOnce(new Error('Network error'));

      const datasets = await service.getDatasets();

      expect(mockAxios.get).toHaveBeenCalledWith('http://localhost:3000/api/datasets');
      expect(datasets).toEqual(expect.arrayContaining([
        expect.objectContaining({
          id: expect.any(String),
          name: 'To be added',
          dataSource: 'PLACEHOLDER'
        })
      ]));
    });
  });

  describe('getDatasetById', () => {
    it('should return live data when API call succeeds', async () => {
      // Mock successful API response
      const mockApiResponse = {
        id: 'tank/data',
        name: 'data',
        pool: 'tank',
        type: 'filesystem',
        mountpoint: '/mnt/tank/data',
        available: '1024000',
        used: '512000',
        referenced: '256000',
        compressratio: '1.8',
        mounted: true,
        recordsize: '128K',
        compression: 'lz4',
        quota: 'none',
        reservation: 'none',
        readonly: false
      };

      mockAxios.get.mockResolvedValueOnce({
        data: mockApiResponse
      });

      const dataset = await service.getDatasetById('tank/data');

      expect(mockAxios.get).toHaveBeenCalledWith('http://localhost:3000/api/datasets/tank/data');
      expect(dataset).toEqual(expect.objectContaining({
        id: 'tank/data',
        dataSource: 'LIVE'
      }));
    });

    it('should provide placeholder data when API call fails', async () => {
      // Mock failed API response
      mockAxios.get.mockRejectedValueOnce(new Error('Network error'));

      const dataset = await service.getDatasetById('tank/data');

      expect(mockAxios.get).toHaveBeenCalledWith('http://localhost:3000/api/datasets/tank/data');
      expect(dataset).toEqual(expect.objectContaining({
        id: 'tank/data',
        name: 'To be added',
        dataSource: 'PLACEHOLDER'
      }));
    });
  });

  describe('createDataset', () => {
    it('should create dataset when API call succeeds', async () => {
      // Mock successful API response
      const mockApiResponse = {
        id: 'tank/newdata',
        name: 'newdata',
        pool: 'tank',
        type: 'filesystem',
        mountpoint: '/mnt/tank/newdata',
        available: '1024000',
        used: '0',
        referenced: '0',
        compressratio: '1.0',
        mounted: true,
        recordsize: '128K',
        compression: 'lz4',
        quota: 'none',
        reservation: 'none',
        readonly: false
      };

      mockAxios.post.mockResolvedValueOnce({
        data: mockApiResponse
      });

      const newDataset = await service.createDataset({
        name: 'newdata',
        pool: 'tank',
        mountpoint: '/mnt/tank/newdata',
        compression: 'lz4',
        recordsize: '128K'
      });

      expect(mockAxios.post).toHaveBeenCalledWith(
        'http://localhost:3000/api/datasets',
        expect.objectContaining({
          name: 'newdata',
          pool: 'tank'
        })
      );
      expect(newDataset).toEqual(expect.objectContaining({
        id: 'tank/newdata',
        dataSource: 'LIVE'
      }));
    });

    it('should throw error when API call fails', async () => {
      // Mock failed API response
      mockAxios.post.mockRejectedValueOnce(new Error('Invalid dataset name'));

      try {
        await service.createDataset({
          name: 'invalid/name',
          pool: 'tank',
          mountpoint: '/mnt/tank/invalid'
        });
        fail('Should have thrown an error');
      } catch (error) {
        expect(error).toEqual(expect.objectContaining({
          message: expect.stringContaining('Failed to create dataset')
        }));
      }
    });
  });

  describe('updateDataset', () => {
    it('should update dataset when API call succeeds', async () => {
      // Mock successful API response
      const mockApiResponse = {
        id: 'tank/data',
        name: 'data',
        pool: 'tank',
        type: 'filesystem',
        mountpoint: '/mnt/tank/data',
        available: '1024000',
        used: '512000',
        referenced: '256000',
        compressratio: '1.8',
        mounted: true,
        recordsize: '128K',
        compression: 'zstd',
        quota: 'none',
        reservation: 'none',
        readonly: false
      };

      mockAxios.put.mockResolvedValueOnce({
        data: mockApiResponse
      });

      const updatedDataset = await service.updateDataset('tank/data', {
        compression: 'zstd'
      });

      expect(mockAxios.put).toHaveBeenCalledWith(
        'http://localhost:3000/api/datasets/tank/data',
        expect.objectContaining({
          compression: 'zstd'
        })
      );
      expect(updatedDataset).toEqual(expect.objectContaining({
        id: 'tank/data',
        compression: 'zstd',
        dataSource: 'LIVE'
      }));
    });

    it('should throw error when API call fails', async () => {
      // Mock failed API response
      mockAxios.put.mockRejectedValueOnce(new Error('Server error'));

      try {
        await service.updateDataset('tank/data', {
          compression: 'zstd'
        });
        fail('Should have thrown an error');
      } catch (error) {
        expect(error).toEqual(expect.objectContaining({
          message: expect.stringContaining('Failed to update dataset')
        }));
      }
    });
  });

  describe('deleteDataset', () => {
    it('should delete dataset when API call succeeds', async () => {
      // Mock successful API response
      mockAxios.delete.mockResolvedValueOnce({});

      await service.deleteDataset('tank/data');

      expect(mockAxios.delete).toHaveBeenCalledWith(
        'http://localhost:3000/api/datasets/tank/data'
      );
    });

    it('should throw error when API call fails', async () => {
      // Mock failed API response
      mockAxios.delete.mockRejectedValueOnce(
        new Error('Dataset has child datasets that must be deleted first')
      );

      try {
        await service.deleteDataset('tank/data');
        fail('Should have thrown an error');
      } catch (error) {
        expect(error).toEqual(expect.objectContaining({
          message: expect.stringContaining('Failed to delete dataset')
        }));
      }
    });
  });
}); 
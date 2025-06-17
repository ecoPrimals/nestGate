import axios from 'axios';
import { SnapshotService } from '../storage/snapshot.service';
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

describe('SnapshotService in strict live mode', () => {
  let snapshotService: SnapshotService;

  beforeEach(() => {
    jest.clearAllMocks();
    snapshotService = SnapshotService.getInstance();
  });

  describe('getSnapshots', () => {
    it('should return live data when API call succeeds', async () => {
      const mockApiResponse = [
        {
          id: 'tank/data@snap1',
          name: 'snap1',
          dataset: 'tank/data',
          created: '2023-01-01T00:00:00Z',
          size: '1.2GB',
          isReplicated: false
        },
        {
          id: 'tank/data@snap2',
          name: 'snap2',
          dataset: 'tank/data',
          created: '2023-01-02T00:00:00Z',
          size: '1.5GB',
          isReplicated: true
        }
      ];

      mockedAxios.get.mockResolvedValueOnce({ data: mockApiResponse });

      const result = await snapshotService.getSnapshots();

      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
      expect(result).toHaveLength(2);
      expect(result[0].dataSource).toBe(DataSourceType.LIVE);
      expect(result[0].name).toBe('snap1');
      expect(result[1].name).toBe('snap2');
    });

    it('should return placeholder data when API call fails', async () => {
      mockedAxios.get.mockRejectedValueOnce(new Error('Network error'));

      const result = await snapshotService.getSnapshots();

      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
      expect(result).toHaveLength(1);
      expect(result[0].dataSource).toBe(DataSourceType.PLACEHOLDER);
      expect(result[0].name).toBe('To be added');
    });

    it('should filter snapshots by dataset when parameter is provided', async () => {
      const mockApiResponse = [
        {
          id: 'tank/data@snap1',
          name: 'snap1',
          dataset: 'tank/data',
          created: '2023-01-01T00:00:00Z',
          size: '1.2GB',
          isReplicated: false
        }
      ];

      mockedAxios.get.mockResolvedValueOnce({ data: mockApiResponse });

      const result = await snapshotService.getSnapshots('tank/data');

      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
      expect(mockedAxios.get).toHaveBeenCalledWith(expect.stringContaining('?dataset=tank/data'));
      expect(result).toHaveLength(1);
      expect(result[0].dataset).toBe('tank/data');
    });

    it('should include dataset name in placeholder when API call fails with dataset parameter', async () => {
      mockedAxios.get.mockRejectedValueOnce(new Error('Network error'));

      const result = await snapshotService.getSnapshots('tank/data');

      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
      expect(result).toHaveLength(1);
      expect(result[0].dataSource).toBe(DataSourceType.PLACEHOLDER);
      expect(result[0].dataset).toBe('tank/data');
    });
  });

  describe('getSnapshotById', () => {
    it('should return live data when API call succeeds', async () => {
      const mockApiResponse = {
        id: 'tank/data@snap1',
        name: 'snap1',
        dataset: 'tank/data',
        created: '2023-01-01T00:00:00Z',
        size: '1.2GB',
        isReplicated: false
      };

      mockedAxios.get.mockResolvedValueOnce({ data: mockApiResponse });

      const result = await snapshotService.getSnapshotById('tank/data@snap1');

      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
      expect(result).not.toBeNull();
      expect(result?.dataSource).toBe(DataSourceType.LIVE);
      expect(result?.name).toBe('snap1');
    });

    it('should return placeholder data when API call fails', async () => {
      mockedAxios.get.mockRejectedValueOnce(new Error('Network error'));

      const result = await snapshotService.getSnapshotById('tank/data@snap1');

      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
      expect(result).not.toBeNull();
      expect(result?.dataSource).toBe(DataSourceType.PLACEHOLDER);
      expect(result?.name).toBe('To be added');
      expect(result?.id).toBe('tank/data@snap1'); // Should keep original ID
    });
  });

  describe('createSnapshot', () => {
    it('should return live data when API call succeeds', async () => {
      const mockApiResponse = {
        id: 'tank/data@snap-new',
        name: 'snap-new',
        dataset: 'tank/data',
        created: '2023-01-03T00:00:00Z',
        size: '0',
        isReplicated: false
      };

      mockedAxios.post.mockResolvedValueOnce({ data: mockApiResponse });

      const result = await snapshotService.createSnapshot('tank/data', 'snap-new');

      expect(mockedAxios.post).toHaveBeenCalledTimes(1);
      expect(mockedAxios.post).toHaveBeenCalledWith(
        expect.any(String),
        { dataset: 'tank/data', name: 'snap-new' }
      );
      expect(result.dataSource).toBe(DataSourceType.LIVE);
      expect(result.name).toBe('snap-new');
    });

    it('should throw error when API call fails in strict live mode', async () => {
      mockedAxios.post.mockRejectedValueOnce(new Error('Network error'));

      await expect(snapshotService.createSnapshot('tank/data', 'snap-new')).rejects.toThrow(
        'Failed to create snapshot'
      );
      expect(mockedAxios.post).toHaveBeenCalledTimes(1);
    });
  });

  describe('deleteSnapshot', () => {
    it('should succeed when API call succeeds', async () => {
      mockedAxios.delete.mockResolvedValueOnce({});

      await expect(snapshotService.deleteSnapshot('tank/data@snap1')).resolves.not.toThrow();
      expect(mockedAxios.delete).toHaveBeenCalledTimes(1);
      expect(mockedAxios.delete).toHaveBeenCalledWith(expect.stringContaining('tank/data@snap1'));
    });

    it('should throw error when API call fails in strict live mode', async () => {
      mockedAxios.delete.mockRejectedValueOnce(new Error('Network error'));

      await expect(snapshotService.deleteSnapshot('tank/data@snap1')).rejects.toThrow(
        'Failed to delete snapshot'
      );
      expect(mockedAxios.delete).toHaveBeenCalledTimes(1);
    });
  });

  describe('rollbackSnapshot', () => {
    it('should succeed when API call succeeds', async () => {
      mockedAxios.post.mockResolvedValueOnce({});

      await expect(snapshotService.rollbackSnapshot('tank/data@snap1')).resolves.not.toThrow();
      expect(mockedAxios.post).toHaveBeenCalledTimes(1);
      expect(mockedAxios.post).toHaveBeenCalledWith(expect.stringContaining('tank/data@snap1/rollback'));
    });

    it('should throw error when API call fails in strict live mode', async () => {
      mockedAxios.post.mockRejectedValueOnce(new Error('Network error'));

      await expect(snapshotService.rollbackSnapshot('tank/data@snap1')).rejects.toThrow(
        'Failed to rollback snapshot'
      );
      expect(mockedAxios.post).toHaveBeenCalledTimes(1);
    });
  });
}); 
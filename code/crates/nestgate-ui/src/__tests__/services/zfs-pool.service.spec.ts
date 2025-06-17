import { ZfsPoolService } from '../../services/zfs-pool.service';

// Mock fetch API
global.fetch = jest.fn();
const mockFetch = global.fetch as jest.Mock;

describe('ZfsPoolService', () => {
  beforeEach(() => {
    // Reset all mocks before each test
    jest.clearAllMocks();
    mockFetch.mockClear();
  });

  describe('Pools API', () => {
    it('should fetch all ZFS pools', async () => {
      // Mock successful response
      const mockPools = [
        {
          id: 'pool1',
          name: 'tank',
          health: 'ONLINE',
          size: 8000000000000,
          free: 6000000000000,
          used: 2000000000000
        },
        {
          id: 'pool2',
          name: 'backup',
          health: 'ONLINE',
          size: 4000000000000,
          free: 3500000000000,
          used: 500000000000
        }
      ];
      
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({ data: mockPools, error: null })
      });
      
      // Call the method
      const result = await ZfsPoolService.getPools();
      
      // Verify fetch was called with correct URL
      expect(mockFetch).toHaveBeenCalledWith('http://localhost:3000/api/pools');
      
      // Verify returned data
      expect(result).toEqual(mockPools);
      expect(result.length).toBe(2);
      expect(result[0].name).toBe('tank');
    });

    it('should handle pool fetch errors gracefully', async () => {
      // Mock error response
      mockFetch.mockRejectedValueOnce(new Error('Network error'));
      
      // Call the method
      const result = await ZfsPoolService.getPools();
      
      // Verify default mock data is returned
      expect(result.length).toBe(2);
      expect(result[0].name).toBe('tank');
    });

    it('should fetch a specific ZFS pool', async () => {
      // Mock successful response
      const mockPool = {
        id: 'pool1',
        name: 'tank',
        health: 'ONLINE',
        size: 8000000000000,
        free: 6000000000000,
        used: 2000000000000
      };
      
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({ data: mockPool, error: null })
      });
      
      // Call the method
      const result = await ZfsPoolService.getPool('pool1');
      
      // Verify fetch was called with correct URL
      expect(mockFetch).toHaveBeenCalledWith('http://localhost:3000/api/pools/pool1');
      
      // Verify returned data
      expect(result).toEqual(mockPool);
      expect(result.name).toBe('tank');
    });
  });

  describe('Datasets API', () => {
    it('should fetch datasets for a pool', async () => {
      // Mock successful response
      const mockDatasets = [
        {
          id: 'ds1',
          name: 'pool1/home',
          mountpoint: '/mnt/pool1/home',
          available: 5000000000000,
          used: 1000000000000
        },
        {
          id: 'ds2',
          name: 'pool1/var',
          mountpoint: '/mnt/pool1/var',
          available: 5000000000000,
          used: 500000000000
        }
      ];
      
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({ data: mockDatasets, error: null })
      });
      
      // Call the method
      const result = await ZfsPoolService.getDatasets('pool1');
      
      // Verify fetch was called with correct URL
      expect(mockFetch).toHaveBeenCalledWith('http://localhost:3000/api/pools/pool1/datasets');
      
      // Verify returned data
      expect(result).toEqual(mockDatasets);
      expect(result.length).toBe(2);
      expect(result[0].name).toBe('pool1/home');
    });

    it('should fetch a specific dataset', async () => {
      // Mock successful response
      const mockDataset = {
        id: 'ds1',
        name: 'pool1/home',
        mountpoint: '/mnt/pool1/home',
        available: 5000000000000,
        used: 1000000000000
      };
      
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({ data: mockDataset, error: null })
      });
      
      // Call the method
      const result = await ZfsPoolService.getDataset('ds1');
      
      // Verify fetch was called with correct URL
      expect(mockFetch).toHaveBeenCalledWith('http://localhost:3000/api/datasets/ds1');
      
      // Verify returned data
      expect(result).toEqual(mockDataset);
      expect(result.name).toBe('pool1/home');
    });

    it('should update dataset record size', async () => {
      // Mock successful response
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({ data: null, error: null })
      });
      
      // Call the method
      await ZfsPoolService.updateDatasetRecordSize('ds1', 128);
      
      // Verify fetch was called with correct URL and body
      expect(mockFetch).toHaveBeenCalledWith(
        'http://localhost:3000/api/datasets/property',
        {
          method: 'PUT',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({
            dataset_id: 'ds1',
            property: 'recordsize',
            value: '128K'
          })
        }
      );
    });

    it('should handle dataset update errors', async () => {
      // Mock error response
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({ data: null, error: 'Permission denied' })
      });
      
      // Call the method and expect it to throw
      await expect(ZfsPoolService.updateDatasetRecordSize('ds1', 128))
        .rejects.toThrow('API error: Permission denied');
    });
  });

  describe('Snapshots API', () => {
    it('should fetch snapshots for a dataset', async () => {
      // Mock successful response
      const mockSnapshots = [
        {
          id: 'snap1',
          name: 'ds1@daily-2023-01-01',
          created: '2023-01-01T00:00:00Z',
          used: 50000000
        },
        {
          id: 'snap2',
          name: 'ds1@daily-2023-01-02',
          created: '2023-01-02T00:00:00Z',
          used: 60000000
        }
      ];
      
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({ data: mockSnapshots, error: null })
      });
      
      // Call the method
      const result = await ZfsPoolService.getSnapshots('ds1');
      
      // Verify fetch was called with correct URL
      expect(mockFetch).toHaveBeenCalledWith('http://localhost:3000/api/datasets/ds1/snapshots');
      
      // Verify returned data
      expect(result).toEqual(mockSnapshots);
      expect(result.length).toBe(2);
      expect(result[0].name).toBe('ds1@daily-2023-01-01');
    });

    it('should create a snapshot', async () => {
      // Mock successful response
      const mockSnapshot = {
        id: 'snap3',
        name: 'ds1@manual-2023-05-15',
        created: '2023-05-15T10:00:00Z',
        used: 0
      };
      
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({ data: mockSnapshot, error: null })
      });
      
      // Call the method
      await ZfsPoolService.createSnapshot({
        datasetId: 'ds1',
        name: 'manual-2023-05-15',
        recursive: true
      });
      
      // Verify fetch was called with correct URL and body
      expect(mockFetch).toHaveBeenCalledWith(
        'http://localhost:3000/api/snapshots',
        {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({
            dataset_id: 'ds1',
            name: 'manual-2023-05-15',
            recursive: true
          })
        }
      );
    });

    it('should delete a snapshot', async () => {
      // Mock successful response
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({ data: null, error: null })
      });
      
      // Call the method
      await ZfsPoolService.deleteSnapshot('snap1');
      
      // Verify fetch was called with correct URL
      expect(mockFetch).toHaveBeenCalledWith(
        'http://localhost:3000/api/snapshots/snap1',
        {
          method: 'DELETE'
        }
      );
    });

    it('should handle snapshot deletion errors', async () => {
      // Mock error response
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({ data: null, error: 'Cannot delete snapshot: in use' })
      });
      
      // Call the method and expect it to throw
      await expect(ZfsPoolService.deleteSnapshot('snap1'))
        .rejects.toThrow('API error: Cannot delete snapshot: in use');
    });
  });
}); 
import { SnapshotService, ZfsSnapshot } from '../storage/snapshot.service';
import axios from 'axios';

// Mock axios
jest.mock('axios');
const mockedAxios = axios as jest.Mocked<typeof axios>;

describe('SnapshotService', () => {
  let snapshotService: SnapshotService;

  beforeEach(() => {
    snapshotService = SnapshotService.getInstance();
    jest.clearAllMocks();
  });

  describe('getInstance', () => {
    it('should return a singleton instance', () => {
      const instance1 = SnapshotService.getInstance();
      const instance2 = SnapshotService.getInstance();
      expect(instance1).toBe(instance2);
    });
  });

  describe('getSnapshots', () => {
    it('should return mock snapshots in development mode', async () => {
      mockedAxios.get.mockRejectedValueOnce(new Error('API not available'));
      
      const snapshots = await snapshotService.getSnapshots();
      
      expect(Array.isArray(snapshots)).toBe(true);
      expect(snapshots.length).toBeGreaterThan(0);
      expect(snapshots[0]).toHaveProperty('id');
      expect(snapshots[0]).toHaveProperty('name');
      expect(snapshots[0]).toHaveProperty('dataset');
      expect(snapshots[0]).toHaveProperty('created');
      expect(snapshots[0]).toHaveProperty('size');
    });

    it('should handle API errors properly', async () => {
      const error = new Error('API error');
      mockedAxios.get.mockRejectedValueOnce(error);
      
      const consoleSpy = jest.spyOn(console, 'error').mockImplementation();
      
      const snapshots = await snapshotService.getSnapshots();
      
      expect(consoleSpy).toHaveBeenCalled();
      expect(Array.isArray(snapshots)).toBe(true);
      
      consoleSpy.mockRestore();
    });
  });

  describe('getSnapshot', () => {
    it('should return a specific snapshot by ID', async () => {
      // Mock the API request
      mockedAxios.get.mockRejectedValueOnce(new Error('API not available'));
      
      // We'll use one of the mock snapshot IDs
      const id = '1';
      
      const snapshot = await snapshotService.getSnapshot(id);
      
      expect(snapshot).toBeDefined();
      expect(snapshot.id).toBe(id);
      expect(snapshot.name).toBe('tank/data@daily-2023-01-15');
      expect(snapshot.dataset).toBe('tank/data');
    });

    it('should throw an error when snapshot is not found', async () => {
      // Mock the API request
      mockedAxios.get.mockRejectedValueOnce(new Error('API not available'));
      
      const id = 'nonexistent-id';
      
      await expect(snapshotService.getSnapshot(id)).rejects.toThrow(
        `Snapshot with ID ${id} not found`
      );
    });
  });

  describe('createSnapshot', () => {
    it('should create a new snapshot with the provided data', async () => {
      const newSnapshot: ZfsSnapshot = {
        id: 'new-id',
        name: 'tank/data@snapshot1',
        dataset: 'tank/data',
        created: new Date().toISOString(),
        size: '1.5 GB',
        isReplicated: false
      };
      
      mockedAxios.post.mockResolvedValueOnce({ data: newSnapshot });
      
      const result = await snapshotService.createSnapshot('tank/data', 'snapshot1');
      
      expect(result).toEqual(newSnapshot);
      expect(mockedAxios.post).toHaveBeenCalledWith(
        expect.any(String),
        expect.objectContaining({
          dataset: 'tank/data',
          name: 'snapshot1'
        })
      );
    });

    it('should handle error when creating snapshot fails', async () => {
      const error = new Error('Failed to create snapshot');
      mockedAxios.post.mockRejectedValueOnce(error);
      
      const spy = jest.spyOn(console, 'error').mockImplementation();
      
      await expect(
        snapshotService.createSnapshot('tank/data', 'error-snapshot')
      ).rejects.toThrow('Failed to create snapshot');
      
      expect(spy).toHaveBeenCalled();
      spy.mockRestore();
    });
  });

  describe('deleteSnapshot', () => {
    it('should delete a snapshot with the given ID', async () => {
      const id = '1';
      
      mockedAxios.delete.mockResolvedValueOnce({ status: 200 });
      
      const result = await snapshotService.deleteSnapshot(id);
      
      expect(result).toBe(true);
      expect(mockedAxios.delete).toHaveBeenCalledWith(expect.stringContaining(id));
    });

    it('should handle error when deleting snapshot fails', async () => {
      const id = '2';
      const error = new Error('Failed to delete snapshot');
      
      mockedAxios.delete.mockRejectedValueOnce(error);
      const spy = jest.spyOn(console, 'error').mockImplementation();
      
      await expect(snapshotService.deleteSnapshot(id)).rejects.toThrow('Failed to delete snapshot');
      
      expect(spy).toHaveBeenCalled();
      spy.mockRestore();
    });
  });

  describe('rollbackSnapshot', () => {
    it('should rollback to the snapshot with the given ID', async () => {
      const id = '1';
      
      mockedAxios.post.mockResolvedValueOnce({ status: 200 });
      
      const result = await snapshotService.rollbackSnapshot(id);
      
      expect(result).toBe(true);
      expect(mockedAxios.post).toHaveBeenCalledWith(expect.stringContaining(id));
    });

    it('should handle error when rollback fails', async () => {
      const id = '1';
      const error = new Error('Failed to rollback snapshot');
      const spy = jest.spyOn(console, 'error').mockImplementation();
      
      // Use axios mock to reject the request
      mockedAxios.post.mockRejectedValueOnce(error);
      
      await expect(snapshotService.rollbackSnapshot(id)).rejects.toThrow('Failed to rollback snapshot');
      
      expect(spy).toHaveBeenCalled();
      spy.mockRestore();
    });
  });

  describe('getDatasetSnapshots', () => {
    it('should return snapshots for a specific dataset', async () => {
      // Mock the API request
      mockedAxios.get.mockRejectedValueOnce(new Error('API not available'));
      
      const datasetName = 'tank/data';
      
      const snapshots = await snapshotService.getDatasetSnapshots(datasetName);
      
      expect(Array.isArray(snapshots)).toBe(true);
      expect(snapshots.length).toBeGreaterThan(0);
      expect(snapshots[0].dataset).toBe(datasetName);
    });

    it('should return empty array for non-existent dataset', async () => {
      // Mock the API request
      mockedAxios.get.mockRejectedValueOnce(new Error('API not available'));
      
      const nonExistentDataset = 'non-existent-dataset';
      
      const snapshots = await snapshotService.getDatasetSnapshots(nonExistentDataset);
      
      expect(Array.isArray(snapshots)).toBe(true);
      expect(snapshots.length).toBe(0);
    });

    it('should handle error when fetching dataset snapshots fails', async () => {
      const datasetName = 'tank/data';
      const error = new Error('Failed to fetch snapshots');
      
      mockedAxios.get.mockRejectedValueOnce(error);
      
      // Mock getSnapshots to also fail
      jest.spyOn(snapshotService, 'getSnapshots').mockRejectedValueOnce(error);
      
      const spy = jest.spyOn(console, 'error').mockImplementation();
      
      await expect(snapshotService.getDatasetSnapshots(datasetName)).rejects.toThrow();
      
      expect(spy).toHaveBeenCalled();
      spy.mockRestore();
    });
  });
}); 
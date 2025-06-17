import { ZfsPoolService, ZfsPool, ZfsDataset } from '../../services/zfs-pool.service';
import axios from 'axios';

// Mock axios
jest.mock('axios');
const mockedAxios = axios as jest.Mocked<typeof axios>;

describe('ZfsPoolService', () => {
  let originalConsoleLog: any;
  
  beforeEach(() => {
    // Save original console.log
    originalConsoleLog = console.log;
    // Mock console.log for testing
    console.log = jest.fn();
    // Clear all mocks
    jest.clearAllMocks();
  });
  
  afterEach(() => {
    // Restore original console.log
    console.log = originalConsoleLog;
  });
  
  describe('getInstance', () => {
    it('should return the same instance on multiple calls', () => {
      const instance1 = ZfsPoolService.getInstance();
      const instance2 = ZfsPoolService.getInstance();
      expect(instance1).toBe(instance2);
    });
  });
  
  describe('getPools', () => {
    it('should fetch pools from API', async () => {
      const mockPools: ZfsPool[] = [
        {
          id: 'pool1',
          name: 'pool1',
          status: 'ONLINE',
          size: '1.8T',
          allocated: '1.2T',
          free: '600G',
          health: 'ONLINE',
          devices: [
            { name: 'sda1', status: 'ONLINE', type: 'disk' }
          ]
        }
      ];
      
      mockedAxios.get.mockResolvedValueOnce({ data: mockPools });
      
      const result = await ZfsPoolService.getPools();
      
      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
      expect(result).toEqual(mockPools);
    });
    
    it('should return mock pools on API error', async () => {
      mockedAxios.get.mockRejectedValueOnce(new Error('Network error'));
      
      const result = await ZfsPoolService.getPools();
      
      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
      expect(result.length).toBeGreaterThan(0);
      expect(result[0].isMock).toBe(true);
    });
  });
  
  describe('getDatasets', () => {
    it('should fetch datasets from API', async () => {
      const mockDatasets: ZfsDataset[] = [
        {
          id: 'pool1/dataset1',
          name: 'dataset1',
          pool: 'pool1',
          type: 'filesystem',
          used: '200G',
          available: '800G',
          referenced: '200G',
          recordsize: '128K',
          compression: 'lz4',
          compressratio: '1.5x',
          mounted: true,
          mountpoint: '/mnt/pool1/dataset1',
          quota: 'none',
          reservation: 'none',
          readonly: false
        }
      ];
      
      mockedAxios.get.mockResolvedValueOnce({ data: mockDatasets });
      
      const result = await ZfsPoolService.getDatasets('pool1');
      
      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
      expect(result).toEqual(mockDatasets);
    });
    
    it('should return mock datasets on API error', async () => {
      mockedAxios.get.mockRejectedValueOnce(new Error('Network error'));
      
      const result = await ZfsPoolService.getDatasets('pool1');
      
      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
      expect(result.length).toBeGreaterThan(0);
      expect(result[0].isMock).toBe(true);
    });
  });
  
  describe('updateDatasetRecordSize', () => {
    it('should update dataset record size via API', async () => {
      const mockUpdatedDataset: ZfsDataset = {
        id: 'pool1/dataset1',
        name: 'dataset1',
        pool: 'pool1',
        type: 'filesystem',
        used: '200G',
        available: '800G',
        referenced: '200G',
        recordsize: '1M',
        compression: 'lz4',
        compressratio: '1.5x',
        mounted: true,
        mountpoint: '/mnt/pool1/dataset1',
        quota: 'none',
        reservation: 'none',
        readonly: false
      };
      
      mockedAxios.patch.mockResolvedValueOnce({ data: mockUpdatedDataset });
      
      const result = await ZfsPoolService.updateDatasetRecordSize('pool1/dataset1', '1M');
      
      expect(mockedAxios.patch).toHaveBeenCalledTimes(1);
      expect(mockedAxios.patch).toHaveBeenCalledWith(
        expect.stringContaining('pool1/dataset1'),
        { recordsize: '1M' }
      );
      expect(result).toEqual(mockUpdatedDataset);
    });
    
    it('should handle API errors gracefully', async () => {
      mockedAxios.patch.mockRejectedValueOnce(new Error('API error'));
      
      const result = await ZfsPoolService.updateDatasetRecordSize('pool1/dataset1', '1M');
      
      expect(mockedAxios.patch).toHaveBeenCalledTimes(1);
      expect(result).toBeNull();
    });
  });
  
  describe('logServiceStatus', () => {
    it('should log messages with timestamp', () => {
      ZfsPoolService.logServiceStatus('Test message');
      
      expect(console.log).toHaveBeenCalledTimes(1);
      expect(console.log).toHaveBeenCalledWith(
        expect.stringMatching(/\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\] Test message/)
      );
    });
    
    it('should work with different message types', () => {
      ZfsPoolService.logServiceStatus('String message');
      ZfsPoolService.logServiceStatus('Object message');
      ZfsPoolService.logServiceStatus('Array message');
      
      expect(console.log).toHaveBeenCalledTimes(3);
    });
  });
}); 
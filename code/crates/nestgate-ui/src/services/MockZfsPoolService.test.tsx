import { ZfsPoolService, ZfsPool } from '../services/zfs-pool.service';
import axios from 'axios';

// Mock axios
jest.mock('axios');
const mockedAxios = axios as jest.Mocked<typeof axios>;

describe('ZfsPoolService', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  test('getPools returns pools from API', async () => {
    // Setup
    const mockPools: Partial<ZfsPool>[] = [
      {
        id: 'tank',
        name: 'tank',
        health: 'ONLINE',
        size: 1000000000000, // 1TB
        used: 700000000000,  // 700GB
        free: 300000000000,  // 300GB
        devices: [
          { name: 'sda1', status: 'ONLINE', type: 'disk' }
        ]
      }
    ];
    
    mockedAxios.get.mockResolvedValueOnce({ data: mockPools });
    
    // Act
    const result = await ZfsPoolService.getPools();
    
    // Assert
    expect(mockedAxios.get).toHaveBeenCalledTimes(1);
    expect(result).toEqual(mockPools);
  });

  test('getPools returns mock data when API fails', async () => {
    // Setup
    mockedAxios.get.mockRejectedValueOnce(new Error('Network error'));
    
    // Act
    const result = await ZfsPoolService.getPools();
    
    // Assert
    expect(mockedAxios.get).toHaveBeenCalledTimes(1);
    expect(result.length).toBeGreaterThan(0);
    expect(result[0].isMock).toBe(true);
  });

  test('logServiceStatus formats log messages with timestamp', () => {
    // Setup
    const originalConsoleLog = console.log;
    console.log = jest.fn();
    
    // Act
    ZfsPoolService.logServiceStatus('Test message');
    
    // Assert
    expect(console.log).toHaveBeenCalledTimes(1);
    expect(console.log).toHaveBeenCalledWith(
      expect.stringMatching(/\[ZfsPoolService \d{4}-\d{2}-\d{2}.*\] Test message/)
    );
    
    // Cleanup
    console.log = originalConsoleLog;
  });

  test('getInstance returns singleton instance', () => {
    // Act
    const instance1 = ZfsPoolService.getInstance();
    const instance2 = ZfsPoolService.getInstance();
    
    // Assert
    expect(instance1).toBe(instance2);
  });
}); 
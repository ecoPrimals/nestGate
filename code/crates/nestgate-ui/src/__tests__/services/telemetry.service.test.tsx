// Create a new test file for telemetry service using mocking

import '@testing-library/jest-dom';

// Import the mock directly
import { TelemetryService } from '../../services/__mocks__/telemetry.service';

// Force Jest to use the mock implementation
jest.mock('../../services/telemetry.service');

describe('TelemetryService Mock', () => {
  describe('Static Methods', () => {
    it('should provide system status data', async () => {
      const result = await TelemetryService.getSystemStatus();
      
      expect(result).toHaveProperty('cpuUsage');
      expect(result).toHaveProperty('memoryUsage');
      expect(result).toHaveProperty('uptime');
      expect(result).toHaveProperty('load');
      expect(result).toHaveProperty('diskHealth');
      expect(result).toHaveProperty('services');
      
      expect(result.cpuUsage).toBe(25);
      expect(result.memoryUsage).toBe(50);
      expect(result.diskHealth.length).toBe(2);
      expect(result.services.length).toBe(3);
    });
    
    it('should provide storage status data', async () => {
      const result = await TelemetryService.getStorageStatus();
      
      expect(result).toHaveProperty('total');
      expect(result).toHaveProperty('used');
      expect(result).toHaveProperty('free');
      expect(result).toHaveProperty('usedPercentage');
      
      expect(result.total).toBe(1000000000);
      expect(result.used).toBe(250000000);
      expect(result.free).toBe(750000000);
      expect(result.usedPercentage).toBe(25);
    });
    
    it('should provide performance metrics data', async () => {
      const result = await TelemetryService.getPerformanceMetrics();
      
      expect(result).toHaveProperty('read');
      expect(result).toHaveProperty('write');
      expect(result).toHaveProperty('iops');
      expect(result).toHaveProperty('throughput');
      expect(result).toHaveProperty('latency');
      expect(result).toHaveProperty('history');
      
      expect(result.read).toBe(100);
      expect(result.write).toBe(50);
      expect(result.iops).toBe(150);
      expect(result.history.length).toBe(3);
    });
    
    it('should provide network status data', async () => {
      const result = await TelemetryService.getNetworkStatus();
      
      expect(result).toHaveProperty('interfaces');
      expect(result).toHaveProperty('throughput');
      
      expect(result.interfaces.length).toBe(2);
      expect(result.throughput).toHaveProperty('in');
      expect(result.throughput).toHaveProperty('out');
      
      expect(result.interfaces[0].status).toBe('up');
      expect(result.interfaces[1].status).toBe('down');
    });
    
    it('should provide alert data', async () => {
      const result = await TelemetryService.getAlerts();
      
      expect(Array.isArray(result)).toBe(true);
      expect(result.length).toBe(2);
      
      expect(result[0]).toHaveProperty('id');
      expect(result[0]).toHaveProperty('severity');
      expect(result[0]).toHaveProperty('message');
      expect(result[0]).toHaveProperty('timestamp');
      
      expect(result[0].severity).toBe('info');
      expect(result[1].severity).toBe('warning');
    });
  });
}); 
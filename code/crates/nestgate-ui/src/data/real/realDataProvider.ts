/**
 * Real Data Provider
 * 
 * Implementation of the data provider interface using real system data.
 * This is the only implementation that should be used in production.
 */

import { DataProvider } from '../interfaces';
import axios from 'axios';

/**
 * Error thrown when data cannot be fetched
 */
export class DataUnavailableError extends Error {
  constructor(message: string, public originalError?: unknown) {
    super(message);
    this.name = 'DataUnavailableError';
  }
}

/**
 * Creates a real data provider that fetches data from the API
 */
export function createRealDataProvider(): DataProvider {
  // Base API URL, could be configured or use environment variables
  const apiBaseUrl = '/api';
  
  // Create axios instance with common configuration
  const api = axios.create({
    baseURL: apiBaseUrl,
    headers: {
      'Content-Type': 'application/json'
    },
    timeout: 10000 // 10 seconds timeout
  });
  
  return {
    // ZFS Pool related methods
    async getZfsPools() {
      try {
        const response = await api.get('/zfs/pools');
        return response.data.data;
      } catch (error) {
        throw new DataUnavailableError('Failed to fetch ZFS pools', error);
      }
    },
    
    async getDatasets(poolName: string) {
      try {
        const response = await api.get(`/zfs/pools/${poolName}/datasets`);
        return response.data.data;
      } catch (error) {
        throw new DataUnavailableError(`Failed to fetch datasets for pool ${poolName}`, error);
      }
    },
    
    async getSnapshots(datasetName: string) {
      try {
        const response = await api.get(`/zfs/datasets/${datasetName}/snapshots`);
        return response.data.data;
      } catch (error) {
        throw new DataUnavailableError(`Failed to fetch snapshots for dataset ${datasetName}`, error);
      }
    },
    
    // System status related methods
    async getSystemStatus() {
      try {
        const response = await api.get('/system/status');
        return response.data.data;
      } catch (error) {
        throw new DataUnavailableError('Failed to fetch system status', error);
      }
    },
    
    async getDiskHealth() {
      try {
        const response = await api.get('/system/disks/health');
        return response.data.data;
      } catch (error) {
        throw new DataUnavailableError('Failed to fetch disk health information', error);
      }
    },
    
    async getServicesStatus() {
      try {
        const response = await api.get('/system/services');
        return response.data.data;
      } catch (error) {
        throw new DataUnavailableError('Failed to fetch services status', error);
      }
    },
    
    // Performance metrics related methods
    async getPerformanceMetrics() {
      try {
        const response = await api.get('/system/performance');
        return response.data.data;
      } catch (error) {
        throw new DataUnavailableError('Failed to fetch performance metrics', error);
      }
    }
  };
}

export default createRealDataProvider; 
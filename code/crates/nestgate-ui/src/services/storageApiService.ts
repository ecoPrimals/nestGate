/**
 * Storage API Service - Real backend integration
 * 
 * Connects React UI components to the actual NestGate storage APIs
 */

// Import existing types from components
import { StorageTier } from '../components/storage/TierCard';
import { MigrationJob, EventFilter, FsEvent } from './storage/tieredStorageService';

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}

export interface StorageAllocation {
  allocation_id: string;
  path: string;
  tier: string;
  quota?: number;
  expires_at?: string;
  access_url: string;
}

export interface StorageRequest {
  node_id: string;
  data_type: 'Models' | 'Datasets' | 'Results' | 'Scratch' | 'Metadata';
  preferred_tier?: string;
  size_hint?: number;
  metadata?: any;
}

export interface DatasetInfo {
  id: string;
  name: string;
  tier: string;
  mountpoint: string;
  available: number;
  used: number;
  compression: string;
  recordsize: string;
  readonly: boolean;
}

export interface MigrationJobStatus {
  id: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
  progress: number;
  source_tier: string;
  target_tier: string;
  paths: string[];
  created_at: string;
  updated_at: string;
  error_message?: string;
}

/**
 * Storage API Service Class
 */
export class StorageApiService {
  private static readonly BASE_URL = '/api/v1';

  /**
   * Get all storage tiers with current usage stats
   */
  static async getTiers(): Promise<StorageTier[]> {
    const response = await fetch(`${this.BASE_URL}/storage/tiers`);
    const result: ApiResponse<StorageTier[]> = await response.json();
    
    if (!result.success) {
      throw new Error(result.error || 'Failed to fetch storage tiers');
    }
    
    return result.data || [];
  }

  /**
   * Get usage statistics for a specific tier
   */
  static async getTierUsage(tierId: string): Promise<StorageTier['usage']> {
    const response = await fetch(`${this.BASE_URL}/storage/tiers/${tierId}/usage`);
    const result = await response.json();
    
    if (!result.success) {
      throw new Error(result.error || 'Failed to fetch tier usage');
    }
    
    return result.data;
  }

  /**
   * Update properties for a storage tier
   */
  static async updateTierProperties(
    tierId: string, 
    properties: Record<string, any>
  ): Promise<void> {
    const response = await fetch(`${this.BASE_URL}/storage/tiers/${tierId}/properties`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(properties),
    });
    
    const result = await response.json();
    
    if (!result.success) {
      throw new Error(result.error || 'Failed to update tier properties');
    }
  }

  /**
   * Get all datasets across all tiers
   */
  static async getDatasets(): Promise<DatasetInfo[]> {
    const response = await fetch(`${this.BASE_URL}/storage/datasets`);
    const result: ApiResponse<DatasetInfo[]> = await response.json();
    
    if (!result.success) {
      throw new Error(result.error || 'Failed to fetch datasets');
    }
    
    return result.data || [];
  }

  /**
   * Create a new dataset
   */
  static async createDataset(dataset: Partial<DatasetInfo>): Promise<DatasetInfo> {
    const response = await fetch(`${this.BASE_URL}/storage/datasets`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(dataset),
    });
    
    const result: ApiResponse<DatasetInfo> = await response.json();
    
    if (!result.success) {
      throw new Error(result.error || 'Failed to create dataset');
    }
    
    return result.data!;
  }

  /**
   * Delete a dataset
   */
  static async deleteDataset(datasetId: string): Promise<void> {
    const response = await fetch(`${this.BASE_URL}/storage/datasets/${datasetId}`, {
      method: 'DELETE',
    });
    
    const result = await response.json();
    
    if (!result.success) {
      throw new Error(result.error || 'Failed to delete dataset');
    }
  }

  /**
   * Start a data migration job
   */
  static async startMigration(job: MigrationJob): Promise<MigrationJobStatus> {
    const response = await fetch(`${this.BASE_URL}/storage/migrate`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(job),
    });
    
    const result: ApiResponse<MigrationJobStatus> = await response.json();
    
    if (!result.success) {
      throw new Error(result.error || 'Failed to start migration');
    }
    
    return result.data!;
  }

  /**
   * Get all migration jobs
   */
  static async getMigrationJobs(): Promise<MigrationJobStatus[]> {
    const response = await fetch(`${this.BASE_URL}/storage/migrate/jobs`);
    const result: ApiResponse<MigrationJobStatus[]> = await response.json();
    
    if (!result.success) {
      throw new Error(result.error || 'Failed to fetch migration jobs');
    }
    
    return result.data || [];
  }

  /**
   * Get migration job status
   */
  static async getMigrationJobStatus(jobId: string): Promise<MigrationJobStatus> {
    const response = await fetch(`${this.BASE_URL}/storage/migrate/jobs/${jobId}`);
    const result: ApiResponse<MigrationJobStatus> = await response.json();
    
    if (!result.success) {
      throw new Error(result.error || 'Failed to fetch migration job status');
    }
    
    return result.data!;
  }

  /**
   * Cancel a migration job
   */
  static async cancelMigration(jobId: string): Promise<void> {
    const response = await fetch(`${this.BASE_URL}/storage/migrate/jobs/${jobId}`, {
      method: 'DELETE',
    });
    
    const result = await response.json();
    
    if (!result.success) {
      throw new Error(result.error || 'Failed to cancel migration');
    }
  }

  /**
   * Allocate storage space (for MCP nodes)
   */
  static async allocateStorage(request: StorageRequest): Promise<StorageAllocation> {
    const response = await fetch(`${this.BASE_URL}/storage/allocate`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(request),
    });
    
    const result: ApiResponse<StorageAllocation> = await response.json();
    
    if (!result.success) {
      throw new Error(result.error || 'Failed to allocate storage');
    }
    
    return result.data!;
  }

  /**
   * Get storage allocations for a node
   */
  static async getStorageAllocations(nodeId: string): Promise<StorageAllocation[]> {
    const response = await fetch(`${this.BASE_URL}/storage/allocations/${nodeId}`);
    const result: ApiResponse<StorageAllocation[]> = await response.json();
    
    if (!result.success) {
      throw new Error(result.error || 'Failed to fetch storage allocations');
    }
    
    return result.data || [];
  }

  /**
   * Get usage statistics for a node
   */
  static async getNodeUsage(nodeId: string): Promise<any> {
    const response = await fetch(`${this.BASE_URL}/usage/${nodeId}`);
    const result = await response.json();
    
    if (!result.success) {
      throw new Error(result.error || 'Failed to fetch node usage');
    }
    
    return result.data;
  }

  /**
   * Create WebSocket connection for real-time events
   */
  static createEventStream(
    tierId?: string,
    filter?: EventFilter
  ): WebSocket {
    const params = new URLSearchParams();
    if (tierId) params.set('tier', tierId);
    if (filter) params.set('filter', JSON.stringify(filter));
    
    const wsUrl = `ws://${window.location.host}/api/v1/storage/events?${params}`;
    return new WebSocket(wsUrl);
  }

  /**
   * Test API connectivity
   */
  static async healthCheck(): Promise<boolean> {
    try {
      const response = await fetch(`${this.BASE_URL}/health`);
      return response.ok;
    } catch {
      return false;
    }
  }
}

export default StorageApiService; 
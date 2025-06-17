import apiClient from '../api-client';
import { StorageTier } from '../../components/storage/TierCard';

export interface EventFilter {
  id: string;
  name: string;
  includeDirectories: boolean;
  includeHidden: boolean;
  extensions: string[];
  patterns: string[];
  eventTypes: string[];
}

export interface FsEvent {
  id: string;
  tierId: string;
  kind: string;
  path: string;
  isDirectory: boolean;
  timestamp: string;
}

export interface MigrationJob {
  id: string;
  sourceTierId: string;
  targetTierId: string;
  paths: string[];
  status: "pending" | "in-progress" | "completed" | "failed";
  progress: number;
  startTime: string;
  endTime?: string;
  error?: string;
}

class TieredStorageService {
  /**
   * Get all storage tiers
   * @returns Promise with array of storage tiers
   */
  async getTiers(): Promise<StorageTier[]> {
    try {
      const response = await apiClient.get('/storage/tiers');
      return response.data;
    } catch (error) {
      console.error('Error fetching storage tiers:', error);
      throw error;
    }
  }

  /**
   * Get a specific storage tier by ID
   * @param tierId The ID of the tier to fetch
   * @returns Promise with the storage tier
   */
  async getTier(tierId: string): Promise<StorageTier> {
    try {
      const response = await apiClient.get(`/storage/tiers/${tierId}`);
      return response.data;
    } catch (error) {
      console.error(`Error fetching tier ${tierId}:`, error);
      throw error;
    }
  }

  /**
   * Get filesystem events for a specific tier
   * @param tierId The ID of the tier to get events for
   * @param filter Optional filter to apply to events
   * @returns Promise with array of filesystem events
   */
  async getEvents(tierId: string, filter?: Partial<EventFilter>): Promise<FsEvent[]> {
    try {
      const response = await apiClient.get(`/storage/tiers/${tierId}/events`, {
        params: filter,
      });
      return response.data;
    } catch (error) {
      console.error(`Error fetching events for tier ${tierId}:`, error);
      throw error;
    }
  }

  /**
   * Update a ZFS property for a tier
   * @param tierId The ID of the tier to update
   * @param property The property name to update
   * @param value The new value for the property
   * @returns Promise with the updated tier
   */
  async updateTierProperty(tierId: string, property: string, value: any): Promise<StorageTier> {
    try {
      const response = await apiClient.patch(`/storage/tiers/${tierId}/properties`, {
        [property]: value,
      });
      return response.data;
    } catch (error) {
      console.error(`Error updating property ${property} for tier ${tierId}:`, error);
      throw error;
    }
  }

  /**
   * Start a migration job
   * @param migrationJob The migration job to start
   * @returns Promise with the created migration job
   */
  async startMigration(migrationJob: Partial<MigrationJob>): Promise<MigrationJob> {
    try {
      const response = await apiClient.post('/storage/migrate', migrationJob);
      return response.data;
    } catch (error) {
      console.error('Error starting migration job:', error);
      throw error;
    }
  }

  /**
   * Get active migration jobs
   * @returns Promise with array of migration jobs
   */
  async getActiveMigrations(): Promise<MigrationJob[]> {
    try {
      const response = await apiClient.get('/storage/migrate/active');
      return response.data;
    } catch (error) {
      console.error('Error fetching active migration jobs:', error);
      throw error;
    }
  }

  /**
   * Get migration history
   * @param limit Optional limit on number of jobs to return
   * @returns Promise with array of completed migration jobs
   */
  async getMigrationHistory(limit?: number): Promise<MigrationJob[]> {
    try {
      const response = await apiClient.get('/storage/migrate/history', {
        params: { limit },
      });
      return response.data;
    } catch (error) {
      console.error('Error fetching migration history:', error);
      throw error;
    }
  }

  /**
   * Update event filter for a tier
   * @param tierId The ID of the tier to update
   * @param filter The new filter configuration
   * @returns Promise with the updated filter
   */
  async updateEventFilter(tierId: string, filter: Partial<EventFilter>): Promise<EventFilter> {
    try {
      const response = await apiClient.put(`/storage/tiers/${tierId}/filters`, filter);
      return response.data;
    } catch (error) {
      console.error(`Error updating filter for tier ${tierId}:`, error);
      throw error;
    }
  }
}

// Export a singleton instance
export const tieredStorageService = new TieredStorageService();
export default tieredStorageService; 
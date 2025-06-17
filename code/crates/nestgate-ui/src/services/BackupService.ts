import axios, { AxiosResponse } from 'axios';
import { API_BASE_URL } from '../constants';
import { BackupJobStatus, BackupRetention, BackupTarget, BackupTargetType, BackupJob } from '../types/backup';
import { shouldUseMockData, getApiRetryAttempts, getApiRetryInterval, getApiTimeout, DataSourceType, isStrictLiveMode } from '../utils/env';

// Additional types for backup functionality
export interface BackupSnapshot {
  id: string;
  name: string;
  jobId: string;
  targetId: string;
  source: string;
  size: number;
  created: string;
  snapshotType: string;
  dataSource?: DataSourceType;
}

export interface RestoreJob {
  id: string;
  snapshotId: string;
  destination: string;
  overwrite: boolean;
  status: 'idle' | 'running' | 'completed' | 'failed' | 'paused';
  description?: string;
  startTime?: string;
  endTime?: string;
  created: string;
  modified: string;
  dataSource?: DataSourceType;
}

export class BackupService {
  private static instance: BackupService;
  private baseUrl = `${API_BASE_URL}/api/backup`;
  private mockMode: boolean;
  private isConnected: boolean = false;
  private connectionAttempts: number = 0;
  private maxRetries: number;
  private retryInterval: number; // ms
  private apiTimeout: number;
  
  // Mock data for development mode
  private mockTargets: BackupTarget[] = [];
  private mockJobs: BackupJob[] = [];
  private mockSnapshots: BackupSnapshot[] = [];
  private mockRestoreJobs: RestoreJob[] = [];
  
  // Check if we're in strict live mode
  private isStrictLiveMode(): boolean {
    return isStrictLiveMode();
  }
  
  // Placeholder data for strict live mode
  private getPlaceholderTarget(): BackupTarget {
    return {
      id: 'placeholder',
      name: 'To be added',
      targetType: { 
        type: 'Local', 
        path: '/placeholder' 
      },
      description: 'Live target implementation in progress',
      created: new Date().toISOString(),
      modified: new Date().toISOString(),
      dataSource: DataSourceType.PLACEHOLDER
    };
  }
  
  private getPlaceholderJob(): BackupJob {
    return {
      id: 'placeholder',
      name: 'To be added',
      source: '/placeholder',
      targetId: 'placeholder',
      schedule: '0 0 * * *',
      retention: {
        daily: 7,
        weekly: 4,
        monthly: 12,
        hourly: null,
        yearly: null
      },
      status: 'Idle',
      description: 'Live backup job implementation in progress',
      lastRun: null,
      nextRun: null,
      created: new Date().toISOString(),
      modified: new Date().toISOString(),
      dataSource: DataSourceType.PLACEHOLDER
    };
  }
  
  private getPlaceholderSnapshot(): BackupSnapshot {
    return {
      id: 'placeholder',
      name: 'To be added',
      jobId: 'placeholder',
      targetId: 'placeholder',
      source: '/placeholder',
      size: 0,
      created: new Date().toISOString(),
      snapshotType: 'manual',
      dataSource: DataSourceType.PLACEHOLDER
    };
  }
  
  private getPlaceholderRestoreJob(): RestoreJob {
    return {
      id: 'placeholder',
      snapshotId: 'placeholder',
      destination: '/placeholder',
      overwrite: false,
      status: 'idle',
      description: 'Live restore job implementation in progress',
      startTime: undefined,
      endTime: undefined,
      created: new Date().toISOString(),
      modified: new Date().toISOString(),
      dataSource: DataSourceType.PLACEHOLDER
    };
  }

  // Private constructor to enforce singleton pattern
  private constructor() {
    // Set API settings from environment
    this.maxRetries = getApiRetryAttempts();
    this.retryInterval = getApiRetryInterval();
    this.apiTimeout = getApiTimeout();
    this.mockMode = shouldUseMockData('backup');
    
    // Initialize mock data if in mock mode
    if (this.mockMode && !this.isStrictLiveMode()) {
      this.initializeMockData();
    }
    
    // Initial connection check
    this.checkConnection();
    
    console.log(`BackupService initialized: mockMode=${this.mockMode}`);
  }

  // Gets the singleton instance
  public static getInstance(): BackupService {
    if (!BackupService.instance) {
      BackupService.instance = new BackupService();
    }
    return BackupService.instance;
  }

  // Check API connection
  private async checkConnection(): Promise<boolean> {
    if (this.mockMode) {
      this.isConnected = true;
      return true;
    }

    try {
      const response = await axios.get(`${this.baseUrl}/health`, { timeout: this.apiTimeout });
      this.isConnected = response.status === 200;
      this.connectionAttempts = 0;
      return this.isConnected;
    } catch (error) {
      this.isConnected = false;
      console.error('Backup API connection failed:', error);
      return false;
    }
  }

  // Retry mechanism for API calls
  private async withRetry<T>(apiCall: () => Promise<T>): Promise<T> {
    if (this.mockMode && this.isStrictLiveMode()) {
      throw new Error('Operation requires live data and is not currently implemented');
    }
    
    if (this.mockMode) {
      return apiCall();
    }

    let attempts = 0;
    while (attempts < this.maxRetries) {
      try {
        return await apiCall();
      } catch (error) {
        attempts++;
        if (attempts >= this.maxRetries) {
          throw error;
        }
        console.warn(`Retrying API call (${attempts}/${this.maxRetries})...`);
        await new Promise(resolve => setTimeout(resolve, this.retryInterval));
      }
    }
    throw new Error('Maximum retry attempts reached');
  }

  // Targets
  async getTargets(): Promise<BackupTarget[]> {
    if (this.mockMode) {
      // Return placeholder in strict live mode
      if (this.isStrictLiveMode()) {
        return Promise.resolve([this.getPlaceholderTarget()]);
      }
      return Promise.resolve([...this.mockTargets]);
    }

    return this.withRetry(async () => {
      try {
        const response = await axios.get(`${this.baseUrl}/targets`);
        return response.data;
      } catch (error) {
        console.error('Error fetching backup targets:', error);
        
        // Return placeholder in strict live mode instead of throwing
        if (this.isStrictLiveMode()) {
          return [this.getPlaceholderTarget()];
        }
        
        throw error;
      }
    });
  }

  async getTarget(id: string): Promise<BackupTarget> {
    if (this.mockMode) {
      const target = this.mockTargets.find(t => t.id === id);
      if (!target) {
        throw new Error(`Target not found: ${id}`);
      }
      return Promise.resolve({ ...target });
    }

    return this.withRetry(async () => {
      try {
        const response = await axios.get(`${this.baseUrl}/targets/${id}`);
        return response.data;
      } catch (error) {
        console.error(`Error fetching backup target ${id}:`, error);
        throw error;
      }
    });
  }

  async createTarget(target: Omit<BackupTarget, 'id' | 'created' | 'modified'>): Promise<BackupTarget> {
    if (this.mockMode) {
      // Throw error in strict live mode
      if (this.isStrictLiveMode()) {
        throw new Error('Creating backup targets requires live data and is not currently implemented');
      }
      
      const newTarget: BackupTarget = {
        ...target,
        id: Math.random().toString(36).substring(2, 11),
        created: new Date().toISOString(),
        modified: new Date().toISOString(),
        dataSource: DataSourceType.MOCK
      };
      this.mockTargets.push(newTarget);
      return Promise.resolve({...newTarget});
    }

    return this.withRetry(async () => {
      try {
        const response = await axios.post(`${this.baseUrl}/targets`, target);
        return response.data;
      } catch (error) {
        console.error('Error creating backup target:', error);
        throw error;
      }
    });
  }

  async updateTarget(target: BackupTarget): Promise<BackupTarget> {
    if (this.mockMode) {
      const index = this.mockTargets.findIndex(t => t.id === target.id);
      if (index === -1) {
        throw new Error(`Target not found: ${target.id}`);
      }
      
      const updatedTarget = {
        ...target,
        modified: new Date().toISOString()
      };
      
      this.mockTargets[index] = updatedTarget;
      return Promise.resolve({ ...updatedTarget });
    }

    return this.withRetry(async () => {
      try {
        const response = await axios.put(`${this.baseUrl}/targets/${target.id}`, target);
        return response.data;
      } catch (error) {
        console.error(`Error updating backup target ${target.id}:`, error);
        throw error;
      }
    });
  }

  async deleteTarget(id: string): Promise<void> {
    if (this.mockMode) {
      const index = this.mockTargets.findIndex(t => t.id === id);
      if (index === -1) {
        throw new Error(`Target not found: ${id}`);
      }
      
      this.mockTargets.splice(index, 1);
      return Promise.resolve();
    }

    return this.withRetry(async () => {
      try {
        await axios.delete(`${this.baseUrl}/targets/${id}`);
      } catch (error) {
        console.error(`Error deleting backup target ${id}:`, error);
        throw error;
      }
    });
  }

  // Jobs
  async getJobs(): Promise<BackupJob[]> {
    if (this.mockMode) {
      // Return placeholder in strict live mode
      if (this.isStrictLiveMode()) {
        return Promise.resolve([this.getPlaceholderJob()]);
      }
      return Promise.resolve([...this.mockJobs]);
    }

    return this.withRetry(async () => {
      try {
        const response = await axios.get(`${this.baseUrl}/jobs`);
        return response.data;
      } catch (error) {
        console.error('Error fetching backup jobs:', error);
        
        // Return placeholder in strict live mode instead of throwing
        if (this.isStrictLiveMode()) {
          return [this.getPlaceholderJob()];
        }
        
        throw error;
      }
    });
  }

  async getJob(id: string): Promise<BackupJob> {
    if (this.mockMode) {
      const job = this.mockJobs.find(j => j.id === id);
      if (!job) {
        throw new Error(`Job not found: ${id}`);
      }
      return Promise.resolve({ ...job });
    }

    return this.withRetry(async () => {
      try {
        const response = await axios.get(`${this.baseUrl}/jobs/${id}`);
        return response.data;
      } catch (error) {
        console.error(`Error fetching backup job ${id}:`, error);
        throw error;
      }
    });
  }

  async createJob(job: Omit<BackupJob, 'id' | 'status' | 'lastRun' | 'nextRun' | 'created' | 'modified'>): Promise<BackupJob> {
    if (this.mockMode) {
      // Verify target exists
      const target = this.mockTargets.find(t => t.id === job.targetId);
      if (!target) {
        throw new Error(`Target not found: ${job.targetId}`);
      }
      
      const newJob: BackupJob = {
        id: Date.now().toString(),
        ...job,
        status: 'Idle',
        lastRun: null,
        nextRun: null,
        created: new Date().toISOString(),
        modified: new Date().toISOString(),
        dataSource: DataSourceType.MOCK
      };
      
      this.mockJobs.push(newJob);
      return Promise.resolve({ ...newJob });
    }

    return this.withRetry(async () => {
      try {
        const response = await axios.post(`${this.baseUrl}/jobs`, job);
        return response.data;
      } catch (error) {
        console.error('Error creating backup job:', error);
        throw error;
      }
    });
  }

  async updateJob(job: BackupJob): Promise<BackupJob> {
    if (this.mockMode) {
      const index = this.mockJobs.findIndex(j => j.id === job.id);
      if (index === -1) {
        throw new Error(`Job not found: ${job.id}`);
      }
      
      // Verify target exists
      const target = this.mockTargets.find(t => t.id === job.targetId);
      if (!target) {
        throw new Error(`Target not found: ${job.targetId}`);
      }
      
      const updatedJob = {
        ...job,
        modified: new Date().toISOString()
      };
      
      this.mockJobs[index] = updatedJob;
      return Promise.resolve({ ...updatedJob });
    }

    return this.withRetry(async () => {
      try {
        const response = await axios.put(`${this.baseUrl}/jobs/${job.id}`, job);
        return response.data;
      } catch (error) {
        console.error(`Error updating backup job ${job.id}:`, error);
        throw error;
      }
    });
  }

  async deleteJob(id: string): Promise<void> {
    if (this.mockMode) {
      const index = this.mockJobs.findIndex(j => j.id === id);
      if (index === -1) {
        throw new Error(`Job not found: ${id}`);
      }
      
      this.mockJobs.splice(index, 1);
      return Promise.resolve();
    }

    return this.withRetry(async () => {
      try {
        await axios.delete(`${this.baseUrl}/jobs/${id}`);
      } catch (error) {
        console.error(`Error deleting backup job ${id}:`, error);
        throw error;
      }
    });
  }

  async runJob(id: string): Promise<void> {
    if (this.mockMode) {
      const job = this.mockJobs.find(j => j.id === id);
      if (!job) {
        throw new Error(`Job not found: ${id}`);
      }
      
      // Update job status
      job.status = 'Running';
      
      // Simulate job completion after 2 seconds
      setTimeout(() => {
        job.status = 'Completed';
        job.lastRun = new Date().toISOString();
        
        // Create a snapshot
        const snapshot: BackupSnapshot = {
          id: Date.now().toString(),
          name: `${job.name}-${new Date().toISOString().replace(/[-:]/g, '').slice(0, 15)}`,
          jobId: job.id,
          targetId: job.targetId,
          source: job.source,
          size: Math.floor(Math.random() * 1024 * 1024 * 500), // Random size up to 500MB
          created: new Date().toISOString(),
          snapshotType: 'manual',
          dataSource: DataSourceType.MOCK
        };
        
        this.mockSnapshots.push(snapshot);
      }, 2000);
      
      return Promise.resolve();
    }

    return this.withRetry(async () => {
      try {
        await axios.post(`${this.baseUrl}/jobs/${id}/run`);
      } catch (error) {
        console.error(`Error running backup job ${id}:`, error);
        throw error;
      }
    });
  }

  // Snapshots
  async getSnapshots(): Promise<BackupSnapshot[]> {
    if (this.mockMode) {
      // Return placeholder in strict live mode
      if (this.isStrictLiveMode()) {
        return Promise.resolve([this.getPlaceholderSnapshot()]);
      }
      return Promise.resolve([...this.mockSnapshots]);
    }
    
    return this.withRetry(async () => {
      try {
        const response = await axios.get(`${this.baseUrl}/snapshots`);
        return response.data;
      } catch (error) {
        console.error('Error fetching backup snapshots:', error);
        
        // Return placeholder in strict live mode instead of throwing
        if (this.isStrictLiveMode()) {
          return [this.getPlaceholderSnapshot()];
        }
        
        throw error;
      }
    });
  }

  async getSnapshot(id: string): Promise<BackupSnapshot> {
    if (this.mockMode) {
      const snapshot = this.mockSnapshots.find(s => s.id === id);
      if (!snapshot) {
        throw new Error(`Snapshot not found: ${id}`);
      }
      return Promise.resolve({ ...snapshot });
    }
    
    return this.withRetry(async () => {
      try {
        const response = await axios.get(`${this.baseUrl}/snapshots/${id}`);
        return response.data;
      } catch (error) {
        console.error(`Error fetching backup snapshot ${id}:`, error);
        throw error;
      }
    });
  }

  async getSnapshotsForJob(jobId: string): Promise<BackupSnapshot[]> {
    if (this.mockMode) {
      return Promise.resolve(this.mockSnapshots.filter(s => s.jobId === jobId));
    }
    
    return this.withRetry(async () => {
      try {
        const response = await axios.get(`${this.baseUrl}/jobs/${jobId}/snapshots`);
        return response.data;
      } catch (error) {
        console.error(`Error fetching snapshots for job ${jobId}:`, error);
        throw error;
      }
    });
  }

  async deleteSnapshot(id: string): Promise<void> {
    if (this.mockMode) {
      const index = this.mockSnapshots.findIndex(s => s.id === id);
      if (index === -1) {
        throw new Error(`Snapshot not found: ${id}`);
      }
      
      // Check if any restore jobs are using this snapshot
      const usingJob = this.mockRestoreJobs.find(j => j.snapshotId === id && ['idle', 'running'].includes(j.status));
      if (usingJob) {
        throw new Error(`Cannot delete snapshot used by restore job: ${usingJob.id}`);
      }
      
      this.mockSnapshots.splice(index, 1);
      return Promise.resolve();
    }
    
    return this.withRetry(async () => {
      try {
        await axios.delete(`${this.baseUrl}/snapshots/${id}`);
      } catch (error) {
        console.error(`Error deleting backup snapshot ${id}:`, error);
        throw error;
      }
    });
  }

  // Restore jobs
  async getRestoreJobs(): Promise<RestoreJob[]> {
    if (this.mockMode) {
      // Return placeholder in strict live mode
      if (this.isStrictLiveMode()) {
        return Promise.resolve([this.getPlaceholderRestoreJob()]);
      }
      return Promise.resolve([...this.mockRestoreJobs]);
    }
    
    return this.withRetry(async () => {
      try {
        const response = await axios.get(`${this.baseUrl}/restore`);
        return response.data;
      } catch (error) {
        console.error('Error fetching restore jobs:', error);
        
        // Return placeholder in strict live mode instead of throwing
        if (this.isStrictLiveMode()) {
          return [this.getPlaceholderRestoreJob()];
        }
        
        throw error;
      }
    });
  }

  async getRestoreJob(id: string): Promise<RestoreJob> {
    if (this.mockMode) {
      const job = this.mockRestoreJobs.find(j => j.id === id);
      if (!job) {
        throw new Error(`Restore job not found: ${id}`);
      }
      return Promise.resolve({ ...job });
    }
    
    return this.withRetry(async () => {
      try {
        const response = await axios.get(`${this.baseUrl}/restore/${id}`);
        return response.data;
      } catch (error) {
        console.error(`Error fetching restore job ${id}:`, error);
        throw error;
      }
    });
  }

  async createRestoreJob(job: Omit<RestoreJob, 'id' | 'status' | 'startTime' | 'endTime' | 'created' | 'modified'>): Promise<RestoreJob> {
    if (this.mockMode) {
      // Verify snapshot exists
      const snapshot = this.mockSnapshots.find(s => s.id === job.snapshotId);
      if (!snapshot) {
        throw new Error(`Snapshot not found: ${job.snapshotId}`);
      }
      
      const newJob: RestoreJob = {
        id: Date.now().toString(),
        ...job,
        status: 'idle',
        created: new Date().toISOString(),
        modified: new Date().toISOString(),
        dataSource: DataSourceType.MOCK
      };
      
      this.mockRestoreJobs.push(newJob);
      return Promise.resolve({ ...newJob });
    }
    
    return this.withRetry(async () => {
      try {
        const response = await axios.post(`${this.baseUrl}/restore`, job);
        return response.data;
      } catch (error) {
        console.error('Error creating restore job:', error);
        throw error;
      }
    });
  }

  async runRestoreJob(id: string): Promise<void> {
    if (this.mockMode) {
      const job = this.mockRestoreJobs.find(j => j.id === id);
      if (!job) {
        throw new Error(`Restore job not found: ${id}`);
      }
      
      // Check if job is already running
      if (job.status === 'running') {
        throw new Error(`Restore job ${id} is already running`);
      }
      
      // Update job status
      job.status = 'running';
      job.startTime = new Date().toISOString();
      job.endTime = undefined;
      
      // Simulate job completion after 3 seconds
      setTimeout(() => {
        job.status = 'completed';
        job.endTime = new Date().toISOString();
      }, 3000);
      
      return Promise.resolve();
    }
    
    return this.withRetry(async () => {
      try {
        await axios.post(`${this.baseUrl}/restore/${id}/run`);
      } catch (error) {
        console.error(`Error running restore job ${id}:`, error);
        throw error;
      }
    });
  }

  async cancelRestoreJob(id: string): Promise<void> {
    if (this.mockMode) {
      const job = this.mockRestoreJobs.find(j => j.id === id);
      if (!job) {
        throw new Error(`Restore job not found: ${id}`);
      }
      
      // Check if job is running
      if (job.status !== 'running') {
        throw new Error(`Restore job ${id} is not running`);
      }
      
      // Update job status
      job.status = 'failed';
      job.endTime = new Date().toISOString();
      
      return Promise.resolve();
    }
    
    return this.withRetry(async () => {
      try {
        await axios.post(`${this.baseUrl}/restore/${id}/cancel`);
      } catch (error) {
        console.error(`Error canceling restore job ${id}:`, error);
        throw error;
      }
    });
  }

  async deleteRestoreJob(id: string): Promise<void> {
    if (this.mockMode) {
      const index = this.mockRestoreJobs.findIndex(j => j.id === id);
      if (index === -1) {
        throw new Error(`Restore job not found: ${id}`);
      }
      
      // Check if job is running
      if (this.mockRestoreJobs[index].status === 'running') {
        throw new Error(`Cannot delete running restore job: ${id}`);
      }
      
      this.mockRestoreJobs.splice(index, 1);
      return Promise.resolve();
    }
    
    return this.withRetry(async () => {
      try {
        await axios.delete(`${this.baseUrl}/restore/${id}`);
      } catch (error) {
        console.error(`Error deleting restore job ${id}:`, error);
        throw error;
      }
    });
  }

  // Connection status
  isApiConnected(): boolean {
    return this.isConnected;
  }

  // Force refresh connection status
  async refreshConnection(): Promise<boolean> {
    return this.checkConnection();
  }

  // Initialize mock data for development
  private initializeMockData(): void {
    // Only initialize if not in strict mode
    if (this.isStrictLiveMode()) {
      return;
    }
    
    this.mockTargets = [
      {
        id: '1',
        name: 'Local Backup',
        targetType: { 
          type: 'Local', 
          path: '/mnt/backup' 
        },
        description: 'Local backup storage',
        created: new Date().toISOString(),
        modified: new Date().toISOString(),
        dataSource: DataSourceType.MOCK
      }
    ];
    
    this.mockJobs = [
      {
        id: '1',
        name: 'Daily Backup',
        source: '/data',
        targetId: '1',
        schedule: '0 0 * * *',
        retention: {
          daily: 7,
          weekly: 4,
          monthly: 12,
          hourly: null,
          yearly: null
        },
        status: 'Idle',
        description: 'Daily backup of all data',
        lastRun: null,
        nextRun: null,
        created: new Date().toISOString(),
        modified: new Date().toISOString(),
        dataSource: DataSourceType.MOCK
      }
    ];
    
    this.mockSnapshots = [
              {
          id: '1',
          name: 'Daily Backup-20230101-000000',
          jobId: '1',
          targetId: '1',
          source: '/data',
          size: 1024 * 1024 * 100, // 100MB
          created: new Date().toISOString(),
          snapshotType: 'daily',
          dataSource: DataSourceType.MOCK
        }
    ];
    
    this.mockRestoreJobs = [
      {
        id: '1',
        snapshotId: '1',
        destination: '/tmp/restore',
        overwrite: true,
        status: 'completed',
        description: 'Test restore',
        startTime: new Date(Date.now() - 3600000).toISOString(), // 1 hour ago
        endTime: new Date().toISOString(),
        created: new Date(Date.now() - 3600000).toISOString(),
        modified: new Date().toISOString(),
        dataSource: DataSourceType.MOCK
      }
    ];
  }
}

// Create and export the singleton instance
const backupServiceInstance = BackupService.getInstance();
export default backupServiceInstance; 
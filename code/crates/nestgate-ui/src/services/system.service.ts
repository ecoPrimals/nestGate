import axios from 'axios';
import { API_BASE_URL } from '../constants';
import { DataSourceType } from '../utils/env';
import { DataUnavailableError } from '../utils/errors';

/**
 * System version information
 */
export interface SystemVersion {
  version: string;
  buildDate: string;
  gitCommit: string;
  platform: string;
  arch: string;
}

/**
 * System update information
 */
export interface SystemUpdate {
  version: string;
  releaseDate: string;
  description: string;
  releaseNotes: string;
  downloadUrl: string;
  size: string;
  isSecurityUpdate: boolean;
  isRecommended: boolean;
  requiresReboot: boolean;
}

/**
 * System resource limits
 */
export interface ResourceLimits {
  cpuCores: number;
  totalMemoryMB: number;
  totalDiskSpaceGB: number;
  maxZpools: number;
}

/**
 * System service status
 */
export interface ServiceStatus {
  name: string;
  displayName: string;
  status: 'running' | 'stopped' | 'failed' | 'starting' | 'stopping';
  enabled: boolean;
  description: string;
  lastStarted?: string;
  restartCount: number;
  dataSource: DataSourceType;
}

/**
 * Service responsible for system management operations
 */
export class SystemService {
  private static instance: SystemService;
  private static readonly API_URL = `${API_BASE_URL}/api/system`;
  private static readonly SERVICE_NAME = 'SystemService';

  /**
   * Gets the singleton instance of SystemService
   */
  public static getInstance(): SystemService {
    if (!SystemService.instance) {
      SystemService.instance = new SystemService();
    }
    return SystemService.instance;
  }

  /**
   * Get system version information
   */
  public async getVersion(): Promise<SystemVersion> {
    try {
      const response = await axios.get(`${SystemService.API_URL}/version`);
      return response.data;
    } catch (error) {
      console.error('Error fetching system version:', error);
      throw new DataUnavailableError('Unable to fetch system version information', {
        originalError: error instanceof Error ? error : undefined,
        serviceName: SystemService.SERVICE_NAME
      });
    }
  }

  /**
   * Check for system updates
   */
  public async checkForUpdates(): Promise<SystemUpdate | null> {
    try {
      const response = await axios.get(`${SystemService.API_URL}/updates/check`);
      return response.data;
    } catch (error) {
      console.error('Error checking for updates:', error);
      throw new DataUnavailableError('Unable to check for system updates', {
        originalError: error instanceof Error ? error : undefined,
        serviceName: SystemService.SERVICE_NAME
      });
    }
  }

  /**
   * Install system update
   */
  public async installUpdate(version: string): Promise<boolean> {
    try {
      await axios.post(`${SystemService.API_URL}/updates/install`, { version });
      return true;
    } catch (error) {
      console.error(`Error installing update ${version}:`, error);
      throw new DataUnavailableError(`Unable to install system update ${version}`, {
        originalError: error instanceof Error ? error : undefined,
        serviceName: SystemService.SERVICE_NAME
      });
    }
  }

  /**
   * Get system resource limits
   */
  public async getResourceLimits(): Promise<ResourceLimits> {
    try {
      const response = await axios.get(`${SystemService.API_URL}/resources/limits`);
      return response.data;
    } catch (error) {
      console.error('Error fetching resource limits:', error);
      throw new DataUnavailableError('Unable to fetch system resource limits', {
        originalError: error instanceof Error ? error : undefined,
        serviceName: SystemService.SERVICE_NAME
      });
    }
  }

  /**
   * Get statuses of all system services
   */
  public async getServiceStatuses(): Promise<ServiceStatus[]> {
    try {
      const response = await axios.get(`${SystemService.API_URL}/services/status`);
      return response.data.map((service: ServiceStatus) => ({
        ...service,
        dataSource: DataSourceType.LIVE
      }));
    } catch (error) {
      console.error('Error fetching service statuses:', error);
      throw new DataUnavailableError('Unable to fetch service statuses', {
        originalError: error instanceof Error ? error : undefined,
        serviceName: SystemService.SERVICE_NAME
      });
    }
  }

  /**
   * Get status of a specific service
   */
  public async getServiceStatus(serviceName: string): Promise<ServiceStatus | null> {
    try {
      const response = await axios.get(`${SystemService.API_URL}/services/${serviceName}`);
      return response.data;
    } catch (error) {
      console.error(`Error fetching status for service ${serviceName}:`, error);
      throw new DataUnavailableError(`Unable to fetch status for service ${serviceName}`, {
        originalError: error instanceof Error ? error : undefined,
        serviceName: SystemService.SERVICE_NAME
      });
    }
  }

  /**
   * Start a system service
   */
  public async startService(serviceName: string): Promise<boolean> {
    try {
      await axios.post(`${SystemService.API_URL}/services/${serviceName}/start`);
      return true;
    } catch (error) {
      console.error(`Error starting service ${serviceName}:`, error);
      throw new DataUnavailableError(`Unable to start service ${serviceName}`, {
        originalError: error instanceof Error ? error : undefined,
        serviceName: SystemService.SERVICE_NAME
      });
    }
  }

  /**
   * Stop a system service
   */
  public async stopService(serviceName: string): Promise<boolean> {
    try {
      await axios.post(`${SystemService.API_URL}/services/${serviceName}/stop`);
      return true;
    } catch (error) {
      console.error(`Error stopping service ${serviceName}:`, error);
      throw new DataUnavailableError(`Unable to stop service ${serviceName}`, {
        originalError: error instanceof Error ? error : undefined,
        serviceName: SystemService.SERVICE_NAME
      });
    }
  }

  /**
   * Restart a system service
   */
  public async restartService(serviceName: string): Promise<boolean> {
    try {
      await axios.post(`${SystemService.API_URL}/services/${serviceName}/restart`);
      return true;
    } catch (error) {
      console.error(`Error restarting service ${serviceName}:`, error);
      throw new DataUnavailableError(`Unable to restart service ${serviceName}`, {
        originalError: error instanceof Error ? error : undefined,
        serviceName: SystemService.SERVICE_NAME
      });
    }
  }

  /**
   * Enable a system service to start on boot
   */
  public async enableService(serviceName: string): Promise<boolean> {
    try {
      await axios.post(`${SystemService.API_URL}/services/${serviceName}/enable`);
      return true;
    } catch (error) {
      console.error(`Error enabling service ${serviceName}:`, error);
      throw new DataUnavailableError(`Unable to enable service ${serviceName}`, {
        originalError: error instanceof Error ? error : undefined,
        serviceName: SystemService.SERVICE_NAME
      });
    }
  }

  /**
   * Disable a system service from starting on boot
   */
  public async disableService(serviceName: string): Promise<boolean> {
    try {
      await axios.post(`${SystemService.API_URL}/services/${serviceName}/disable`);
      return true;
    } catch (error) {
      console.error(`Error disabling service ${serviceName}:`, error);
      throw new DataUnavailableError(`Unable to disable service ${serviceName}`, {
        originalError: error instanceof Error ? error : undefined,
        serviceName: SystemService.SERVICE_NAME
      });
    }
  }

  /**
   * Reboot the system
   */
  public async rebootSystem(): Promise<boolean> {
    try {
      await axios.post(`${SystemService.API_URL}/reboot`);
      return true;
    } catch (error) {
      console.error('Error rebooting system:', error);
      throw new DataUnavailableError('Unable to reboot the system', {
        originalError: error instanceof Error ? error : undefined,
        serviceName: SystemService.SERVICE_NAME
      });
    }
  }

  /**
   * Shutdown the system
   */
  public async shutdownSystem(): Promise<boolean> {
    try {
      await axios.post(`${SystemService.API_URL}/shutdown`);
      return true;
    } catch (error) {
      console.error('Error shutting down system:', error);
      throw new DataUnavailableError('Unable to shutdown the system', {
        originalError: error instanceof Error ? error : undefined,
        serviceName: SystemService.SERVICE_NAME
      });
    }
  }
} 
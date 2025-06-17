/**
 * System interfaces
 * 
 * These interfaces define the data structures for system-related entities.
 */

import { DataSourceType } from '../utils/env-helper';

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
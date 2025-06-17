/**
 * Feature Flags Module
 * 
 * Provides centralized feature flag management for NestGate
 */

import { getConfig } from '../config';

/**
 * Feature flag enum
 */
export enum Feature {
  // Core features
  ZFS_MANAGEMENT = 'zfsManagement',
  PERFORMANCE_MONITORING = 'performanceMonitoring',
  DISK_MANAGEMENT = 'diskManagement',
  
  // Advanced features
  REMOTE_BACKUP = 'remoteBackup',
  REPLICATION = 'replication',
  ENCRYPTION = 'encryption',
  
  // UI features
  DARK_MODE = 'darkMode',
  ADVANCED_CHARTS = 'advancedCharts',
  TECHNICAL_PREVIEW = 'technicalPreview'
}

/**
 * Type defining feature status
 */
export type FeatureStatus = 'enabled' | 'disabled' | 'preview';

/**
 * Feature configuration object type
 */
export interface FeatureConfig {
  [key: string]: { 
    status: FeatureStatus;
    description: string;
  };
}

/**
 * Default feature configuration
 */
const defaultFeatures: FeatureConfig = {
  // Core features - enabled by default
  [Feature.ZFS_MANAGEMENT]: { 
    status: 'enabled',
    description: 'ZFS pool and dataset management'
  },
  [Feature.PERFORMANCE_MONITORING]: { 
    status: 'enabled',
    description: 'System performance monitoring'
  },
  [Feature.DISK_MANAGEMENT]: { 
    status: 'enabled',
    description: 'Physical disk management'
  },
  
  // Advanced features - may be in preview
  [Feature.REMOTE_BACKUP]: { 
    status: 'preview',
    description: 'Remote backup capabilities'
  },
  [Feature.REPLICATION]: { 
    status: 'preview',
    description: 'ZFS replication features'
  },
  [Feature.ENCRYPTION]: { 
    status: 'preview',
    description: 'Data encryption features'
  },
  
  // UI features
  [Feature.DARK_MODE]: { 
    status: 'enabled',
    description: 'Dark mode UI theme'
  },
  [Feature.ADVANCED_CHARTS]: { 
    status: 'enabled',
    description: 'Advanced performance charts'
  },
  [Feature.TECHNICAL_PREVIEW]: { 
    status: 'disabled',
    description: 'Technical preview features'
  }
};

/**
 * Check if a feature is enabled
 */
export function isFeatureEnabled(feature: Feature): boolean {
  // Get current configuration
  const featureConfig = getFeatureConfig();
  
  // Check if the feature exists and is enabled
  if (featureConfig[feature]) {
    return featureConfig[feature].status === 'enabled' || 
           featureConfig[feature].status === 'preview';
  }
  
  return false;
}

/**
 * Get the feature configuration for all features
 */
export function getFeatureConfig(): FeatureConfig {
  return defaultFeatures;
}

/**
 * Get the description of a feature
 */
export function getFeatureDescription(feature: Feature): string {
  const featureConfig = getFeatureConfig();
  return featureConfig[feature]?.description || 'Unknown feature';
}

/**
 * Get the status of a feature
 */
export function getFeatureStatus(feature: Feature): FeatureStatus {
  const featureConfig = getFeatureConfig();
  return featureConfig[feature]?.status || 'disabled';
}

export default {
  isFeatureEnabled,
  getFeatureConfig,
  getFeatureDescription,
  getFeatureStatus,
  Feature
}; 
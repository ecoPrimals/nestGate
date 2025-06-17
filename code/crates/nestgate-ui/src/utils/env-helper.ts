/**
 * Environment Helper
 * 
 * Utilities for working with environment variables and runtime configuration
 */

/**
 * Types of data sources available in the application
 */
export enum DataSourceType {
  LIVE = 'LIVE',           // Real data from a live system
  PLACEHOLDER = 'PLACEHOLDER'  // Placeholder data when data is unavailable
}

/**
 * Get the current data source type
 */
export function getDataSourceType(): DataSourceType {
  return DataSourceType.LIVE;
}

/**
 * Format a port number for display
 * Handles undefined ports gracefully
 */
export function formatPort(port?: number | string): string {
  if (port === undefined || port === null) {
    return 'not set';
  }
  return String(port);
}

/**
 * Force environment variables to use live mode
 * This ensures we're always using live data
 */
export function forceLiveEnvironment(): void {
  if (typeof process !== 'undefined' && process.env) {
    // Force real services
    process.env.REACT_APP_STRICT_DATA_MODE = 'true';
    process.env.REACT_APP_USE_REAL_DISKS = 'true';
  }
}

/**
 * Get the retry interval for API calls in milliseconds
 * @param defaultValue Default value if not set
 */
export function getApiRetryInterval(defaultValue = 1000): number {
  const value = process.env.REACT_APP_API_RETRY_INTERVAL;
  return value ? parseInt(value, 10) : defaultValue;
}

/**
 * Get the number of retry attempts for API calls
 * @param defaultValue Default value if not set
 */
export function getApiRetryAttempts(defaultValue = 3): number {
  const value = process.env.REACT_APP_API_RETRY_ATTEMPTS;
  return value ? parseInt(value, 10) : defaultValue;
}

/**
 * Get the timeout for API calls in milliseconds
 * @param defaultValue Default value if not set
 */
export function getApiTimeout(defaultValue = 10000): number {
  const value = process.env.REACT_APP_API_TIMEOUT;
  return value ? parseInt(value, 10) : defaultValue;
}

// Initialize environment
forceLiveEnvironment(); 
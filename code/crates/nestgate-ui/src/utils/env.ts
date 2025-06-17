/**
 * Environment configuration utilities
 * Provides helpers for determining application environment and configuration
 */

/**
 * Enum for data source types
 */
export enum DataSourceType {
  LIVE = 'live',           // Data from a live system
  MOCK = 'mock',           // Mock data for development/testing
  TEST = 'test',           // Test data
  PLACEHOLDER = 'placeholder' // Placeholder indicating a feature being developed
}

/**
 * Check if strict live mode is enabled
 * 
 * @returns True if strict live mode is enabled
 */
export function isStrictLiveMode(): boolean {
  return process.env.REACT_APP_STRICT_DATA_MODE === 'true';
}

/**
 * Check if we should use mock data for a specific service
 * @param service The service name to check
 * @returns Boolean indicating if we should use mock data
 */
export function shouldUseMockData(service?: string): boolean {
  // Never use mock data in strict mode
  if (isStrictLiveMode()) {
    return false;
  }
  
  // Default to using real data
  return false;
}

/**
 * Get the current data source type for a service
 * @param service The service name to check
 * @returns The data source type enum value
 */
export function getDataSourceType(service?: string): DataSourceType {
  if (isStrictLiveMode()) {
    return DataSourceType.LIVE;
  }
  
  // In non-strict mode, determine based on service
  if (service && shouldUseMockData(service)) {
    return DataSourceType.MOCK;
  }
  
  return DataSourceType.LIVE;
}

/**
 * Check if we're in a development environment
 * @returns Boolean indicating if we're in development
 */
export function isDevelopment(): boolean {
  return process.env.NODE_ENV === 'development';
}

/**
 * Check if we're in a production environment
 * @returns Boolean indicating if we're in production
 */
export function isProduction(): boolean {
  return process.env.NODE_ENV === 'production';
}

/**
 * Get the API base URL
 * @returns The API base URL
 */
export function getApiBaseUrl(): string {
  return process.env.REACT_APP_API_BASE_URL || '';
}

/**
 * Get WebSocket URL with fallback
 */
export function getWebSocketUrl(): string {
  // No hardcoded fallback - must come from Port Manager
  return process.env.REACT_APP_WEBSOCKET_URL || '';
}

/**
 * Get the number of retry attempts for API calls
 * @returns The number of retry attempts (default: 3)
 */
export function getApiRetryAttempts(): number {
  const attempts = parseInt(process.env.REACT_APP_API_RETRY_ATTEMPTS || '3', 10);
  return isNaN(attempts) ? 3 : attempts;
}

/**
 * Get the retry interval for API calls in milliseconds
 * @returns The retry interval in ms (default: 2000)
 */
export function getApiRetryInterval(): number {
  const interval = parseInt(process.env.REACT_APP_API_RETRY_INTERVAL || '2000', 10);
  return isNaN(interval) ? 2000 : interval;
}

/**
 * Get the API request timeout in milliseconds
 * @returns The timeout in ms (default: 10000)
 */
export function getApiTimeout(): number {
  const timeout = parseInt(process.env.REACT_APP_API_TIMEOUT || '10000', 10);
  return isNaN(timeout) ? 10000 : timeout;
} 
/**
 * Configuration module for NestGate
 * 
 * Centralizes configuration settings and environment variable handling
 */

import { DataSourceType } from './utils/env';
import { configService } from './services/config.service';

/**
 * Configuration options for the application
 */
export interface AppConfig {
  /**
   * Base URI for API requests
   */
  API_URI: string;
  
  /**
   * Refresh interval for data (in milliseconds)
   */
  REFRESH_INTERVAL: number;
  
  /**
   * Whether strict live mode is enabled
   */
  STRICT_DATA_MODE: boolean;
  
  /**
   * Whether to use real disk data
   */
  USE_REAL_DISKS: boolean;
  
  /**
   * API base URL
   */
  API_BASE_URL: string;
  
  /**
   * WebSocket base URL
   */
  WS_BASE_URL: string;
  
  /**
   * File monitor URL
   */
  FILE_MONITOR_URL: string;
  
  /**
   * Port manager URL
   */
  PORT_MANAGER_URL: string;
  
  /**
   * UI port
   */
  UI_PORT: string;
  
  /**
   * API port
   */
  API_PORT: string;
  
  /**
   * Server port
   */
  SERVER_PORT: string;
  
  /**
   * Whether strict mode is enabled
   */
  STRICT_MODE: boolean;
  
  /**
   * Whether to use mock data
   */
  USE_MOCK_DATA: boolean;
  
  /**
   * Whether to show mock indicator
   */
  SHOW_MOCK_INDICATOR: boolean;
}

// No hardcoded endpoints - everything comes from dynamic discovery
// Port Manager will be discovered via service discovery mechanism
export const PORT_MANAGER_URL = ""; // Will be discovered dynamically

// Static configuration ONLY contains port manager URL - everything else is dynamic
export const API_BASE_URL = process.env.REACT_APP_API_BASE_URL || ""; // Will be set by port manager
export const WS_BASE_URL = process.env.REACT_APP_WS_URL || ""; // Must come from port manager
export const FILE_MONITOR_URL = process.env.REACT_APP_FILE_MONITOR_URL || ""; // Must come from port manager

// Port information - will be overridden by port manager
export const UI_PORT = process.env.PORT || '3000';
export const API_PORT = ""; // Must come from port manager
export const SERVER_PORT = ""; // Must come from port manager

/**
 * Get a typed environment variable
 * 
 * @param name - The environment variable name
 * @param defaultValue - Default value if not found
 * @returns The typed environment variable value
 */
function getEnvVariable<T>(name: string, defaultValue: T): T {
  const envValue = process.env[name] || (typeof window !== 'undefined' && window.ENV && window.ENV[name]);
  
  if (envValue === undefined) {
    return defaultValue;
  }
  
  try {
    // Try to parse as JSON if it's a string
    if (typeof envValue === 'string') {
      if (envValue.toLowerCase() === 'true') return true as unknown as T;
      if (envValue.toLowerCase() === 'false') return false as unknown as T;
      if (!isNaN(Number(envValue))) return Number(envValue) as unknown as T;
      try {
        return JSON.parse(envValue) as T;
      } catch (e) {
        return envValue as unknown as T;
      }
    }
    return envValue as unknown as T;
  } catch (error) {
    console.warn(`Error parsing environment variable ${name}:`, error);
    return defaultValue;
  }
}

/**
 * Unified app configuration (static - will be enhanced with dynamic config)
 */
const APP_CONFIG: AppConfig = {
  // Legacy config
  API_URI: getEnvVariable('API_URI', API_BASE_URL),
  REFRESH_INTERVAL: getEnvVariable('REFRESH_INTERVAL', 5000),
  STRICT_DATA_MODE: getEnvVariable('STRICT_DATA_MODE', true),
  USE_REAL_DISKS: getEnvVariable('USE_REAL_DISKS', true),
  
  // New config
  API_BASE_URL,
  WS_BASE_URL,
  FILE_MONITOR_URL,
  PORT_MANAGER_URL,
  UI_PORT,
  API_PORT,
  SERVER_PORT,
  STRICT_MODE: process.env.REACT_APP_STRICT_DATA_MODE === 'true',
  USE_MOCK_DATA: process.env.REACT_APP_USE_MOCK_ALL === 'true',
  SHOW_MOCK_INDICATOR: process.env.REACT_APP_SHOW_MOCK_INDICATOR === 'true',
};

/**
 * Check if strict live mode is enabled
 * 
 * @returns True if strict live mode is enabled
 */
export function isStrictLiveMode(): boolean {
  return APP_CONFIG.STRICT_DATA_MODE === true;
}

/**
 * Get the current data source type
 * 
 * @returns The data source type (always LIVE in strict mode)
 */
export function getDataSourceType(): DataSourceType {
  return DataSourceType.LIVE;
}

/**
 * Get the application configuration
 * 
 * @returns The application configuration
 */
export function getConfig(): AppConfig {
  return { ...APP_CONFIG };
}

/**
 * Get the WebSocket URL (dynamic)
 * @returns Full WebSocket URL from port manager or fallback
 */
export const getWebSocketUrl = async (): Promise<string> => {
  try {
    return await configService.getWebSocketUrl();
  } catch (error) {
    console.warn('Using fallback WebSocket URL:', WS_BASE_URL);
  return WS_BASE_URL;
  }
};

/**
 * Get the API base URL (dynamic)
 * @returns API base URL from port manager or fallback
 */
export const getApiBaseUrl = async (): Promise<string> => {
  try {
    return await configService.getApiBaseUrl();
  } catch (error) {
    console.warn('Using fallback API URL:', API_BASE_URL);
    return API_BASE_URL;
  }
};

/**
 * Initialize dynamic configuration
 * Call this early in the app lifecycle
 */
export const initializeConfig = async (): Promise<void> => {
  try {
    console.log('🔧 Initializing dynamic configuration...');
    const dynamicConfig = await configService.fetchConfig();
    
    // Update the global APP_CONFIG with dynamic values
    APP_CONFIG.API_URI = dynamicConfig.apiBaseUrl;
    APP_CONFIG.API_BASE_URL = dynamicConfig.apiBaseUrl;
    APP_CONFIG.WS_BASE_URL = dynamicConfig.websocketUrl;
    APP_CONFIG.PORT_MANAGER_URL = dynamicConfig.portManagerUrl;
    
    console.log('✅ Dynamic configuration initialized:', {
      apiBaseUrl: APP_CONFIG.API_URI,
      websocketUrl: APP_CONFIG.WS_BASE_URL,
      portManagerUrl: APP_CONFIG.PORT_MANAGER_URL
    });
  } catch (error) {
    console.error('❌ Dynamic configuration failed - Port manager required:', error);
    throw new Error('NestGate requires the port manager to be running. Please start the port manager first.');
  }
};

// Make the ENV object available for TypeScript in browser
declare global {
  interface Window {
    ENV?: Record<string, any>;
  }
}

// Export the unified configuration - default export
export default APP_CONFIG; 
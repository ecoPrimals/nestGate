/**
 * NestGate UI Configuration
 * 
 * Updated to work with the orchestrator-based architecture.
 * All configuration is now fetched from the orchestrator service registry.
 */

import { configService } from './services/config.service';

// Application configuration interface
export interface AppConfig {
  // API Configuration - fetched from orchestrator
  API_URI: string;
  API_BASE_URL: string;
  
  // WebSocket Configuration - fetched from orchestrator  
  WS_BASE_URL: string;
  
  // Orchestrator Configuration
  ORCHESTRATOR_URL: string;
  
  // UI Configuration
  APP_NAME: string;
  APP_VERSION: string;
  SERVER_PORT: string;
  API_PORT: string;
  UI_PORT: string;
  
  // Feature flags
  ENABLE_DEBUG: boolean;
  ENABLE_MOCK_DATA: boolean;
  STRICT_DATA_MODE: boolean;
}

// Default configuration (will be updated by orchestrator)
export const APP_CONFIG: AppConfig = {
  // These will be populated from orchestrator service registry
  API_URI: '',
  API_BASE_URL: '',
  WS_BASE_URL: '',
  ORCHESTRATOR_URL: '',
  
  // Static configuration
  APP_NAME: 'NestGate',
  APP_VERSION: '2.0.0',
  SERVER_PORT: '3000',
  API_PORT: '8092',
  UI_PORT: '3000',
  
  // Feature flags from environment
  ENABLE_DEBUG: process.env.NODE_ENV === 'development',
  ENABLE_MOCK_DATA: process.env.REACT_APP_ENABLE_MOCK_DATA === 'true',
  STRICT_DATA_MODE: process.env.REACT_APP_STRICT_DATA_MODE === 'true',
};

// Export individual config values for backward compatibility
export let API_BASE_URL = '';
export let WS_BASE_URL = '';

// No hardcoded endpoints - everything comes from orchestrator discovery
export const ORCHESTRATOR_URL = "";

/**
 * Get configuration object (backward compatibility)
 */
export const getConfig = () => APP_CONFIG;

/**
 * Get API base URL from orchestrator
 */
export const getApiBaseUrl = async (): Promise<string> => {
  try {
    const config = await configService.getConfig();
    return config.apiBaseUrl;
  } catch (error) {
    console.error('Failed to get API URL from orchestrator:', error);
    throw new Error('Cannot connect to NestGate orchestrator. Please ensure the orchestrator is running.');
  }
};

/**
 * Get WebSocket URL from orchestrator
 */
export const getWebSocketUrl = async (): Promise<string> => {
  try {
    const config = await configService.getConfig();
    return config.websocketUrl;
  } catch (error) {
    console.error('Failed to get WebSocket URL from orchestrator:', error);
    // Return a placeholder that will fail gracefully
    return 'ws://localhost:3000/';
  }
};

/**
 * Get orchestrator URL
 */
export const getOrchestratorUrl = async (): Promise<string> => {
  try {
    const config = await configService.getConfig();
    return config.orchestratorUrl;
  } catch (error) {
    console.error('Failed to get orchestrator URL:', error);
    throw new Error('Orchestrator not accessible');
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
    APP_CONFIG.ORCHESTRATOR_URL = dynamicConfig.orchestratorUrl;
    
    // Update exported constants for backward compatibility
    API_BASE_URL = dynamicConfig.apiBaseUrl;
    WS_BASE_URL = dynamicConfig.websocketUrl;
    
    console.log('✅ Dynamic configuration initialized:', {
      apiBaseUrl: APP_CONFIG.API_URI,
      websocketUrl: APP_CONFIG.WS_BASE_URL,
      orchestratorUrl: APP_CONFIG.ORCHESTRATOR_URL
    });
  } catch (error) {
    console.error('❌ Dynamic configuration failed - Orchestrator required:', error);
    throw new Error('NestGate requires the orchestrator to be running. Please start the orchestrator first.');
  }
};

/**
 * Validate configuration
 */
export const validateConfig = (): boolean => {
  const required = [
    'API_URI',
    'API_BASE_URL', 
    'WS_BASE_URL',
    'ORCHESTRATOR_URL'
  ];
  
  for (const key of required) {
    if (!APP_CONFIG[key as keyof AppConfig]) {
      console.error(`Missing required configuration: ${key}`);
      return false;
    }
  }
  
  return true;
};

/**
 * Get service information from orchestrator
 */
export const getServiceInfo = async (serviceId: string) => {
  try {
    return await configService.getServiceEndpoint(serviceId);
  } catch (error) {
    console.error(`Failed to get service info for ${serviceId}:`, error);
    return null;
  }
};

/**
 * Get all services from orchestrator
 */
export const getAllServices = async () => {
  try {
    const config = await configService.getConfig();
    return config.services;
  } catch (error) {
    console.error('Failed to get services from orchestrator:', error);
    return {};
  }
};

/**
 * Refresh configuration from orchestrator
 */
export const refreshConfig = async (): Promise<void> => {
  try {
    console.log('🔄 Refreshing configuration from orchestrator...');
    configService.clearCache();
    await initializeConfig();
    console.log('✅ Configuration refreshed successfully');
  } catch (error) {
    console.error('❌ Failed to refresh configuration:', error);
    throw error;
  }
};

export default APP_CONFIG; 
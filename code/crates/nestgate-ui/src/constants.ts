/**
 * NestGate UI Constants
 */

// API configuration
const apiPort = process.env.REACT_APP_API_PORT || process.env.API_PORT || '3002';
export const API_BASE_URL = process.env.REACT_APP_API_URL || `http://localhost:${apiPort}`;

// Token storage keys
export const AUTH_TOKEN_KEY = 'nestgate_auth_token';
export const REFRESH_TOKEN_KEY = 'nestgate_refresh_token';

// Application settings
export const APP_NAME = 'NestGate';
export const APP_VERSION = '0.1.0';

// Dashboard constants
export const DASHBOARD_REFRESH_INTERVAL = 30000; // 30 seconds

// System metrics thresholds
export const METRICS = {
  STORAGE: {
    WARNING_THRESHOLD: 80, // 80% usage
    CRITICAL_THRESHOLD: 90, // 90% usage
  },
  CPU: {
    WARNING_THRESHOLD: 70, // 70% usage
    CRITICAL_THRESHOLD: 85, // 85% usage
  },
  MEMORY: {
    WARNING_THRESHOLD: 70, // 70% usage
    CRITICAL_THRESHOLD: 85, // 85% usage
  },
  TEMPERATURE: {
    WARNING_THRESHOLD: 60, // 60°C
    CRITICAL_THRESHOLD: 75, // 75°C
  }
};

/**
 * Application constants
 */

// API endpoints
export const API_ENDPOINTS = {
  AUTH: '/api/auth',
  USERS: '/api/users',
  SETTINGS: '/api/settings',
  NETWORK: '/api/network',
  STORAGE: '/api/storage',
  SNAPSHOTS: '/api/snapshots',
  BACKUP: '/api/backup',
  SYSTEM: '/api/system',
  TELEMETRY: '/api/telemetry',
};

// WebSocket events
export const WS_EVENTS = {
  CONNECT: 'connect',
  DISCONNECT: 'disconnect',
  ERROR: 'error',
  NOTIFICATION: 'notification',
  SYSTEM_METRICS: 'system_metrics',
  STORAGE_METRICS: 'storage_metrics',
};

// Local storage keys
export const STORAGE_KEYS = {
  AUTH_TOKEN: 'auth_token',
  USER_PREFERENCES: 'user_preferences',
  THEME: 'theme',
  SIDEBAR_COLLAPSED: 'sidebar_collapsed',
  LAST_ROUTE: 'last_route',
  NOTIFICATION_SETTINGS: 'notification_settings',
};

// Environment settings
export const ENV = {
  // Define which environment we're in
  IS_PRODUCTION: process.env.NODE_ENV === 'production',
  IS_DEVELOPMENT: process.env.NODE_ENV === 'development',
  IS_TEST: process.env.NODE_ENV === 'test',
  
  // API config
  API_BASE_URL: process.env.REACT_APP_API_BASE_URL || '',
  API_TIMEOUT: parseInt(process.env.REACT_APP_API_TIMEOUT || '10000', 10),
  
  // WebSocket config
  WEBSOCKET_URL: process.env.REACT_APP_WEBSOCKET_URL || '',
  
  // Feature flags
  ENABLE_BACKUP_FEATURE: process.env.REACT_APP_ENABLE_BACKUP_FEATURE !== 'false',
  ENABLE_REMOTE_ACCESS: process.env.REACT_APP_ENABLE_REMOTE_ACCESS === 'true',
  ENABLE_AI_WORKLOAD: process.env.REACT_APP_ENABLE_AI_WORKLOAD === 'true',
  
  // UI customization
  PRODUCT_NAME: process.env.REACT_APP_PRODUCT_NAME || 'NestGate',
  PRODUCT_VERSION: process.env.REACT_APP_PRODUCT_VERSION || '1.0.0',
};

export default {
  API_ENDPOINTS,
  WS_EVENTS,
  STORAGE_KEYS,
  ENV,
}; 
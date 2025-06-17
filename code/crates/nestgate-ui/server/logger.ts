/**
 * NestGate Logger
 * 
 * Centralized logging system for the NestGate application
 */

import fs from 'fs';
import path from 'path';
import { format } from 'util';

// Log levels
export enum LogLevel {
  DEBUG = 'DEBUG',
  INFO = 'INFO',
  WARN = 'WARN',
  ERROR = 'ERROR',
  SECURITY = 'SECURITY'
}

// Configure logging
const LOG_DIR = path.join(process.cwd(), 'logs');
const LOG_FILE = path.join(LOG_DIR, 'nestgate.log');
const SECURITY_LOG_FILE = path.join(LOG_DIR, 'security.log');
const MAX_LOG_SIZE = 10 * 1024 * 1024; // 10MB

// Make sure log directory exists
try {
  if (!fs.existsSync(LOG_DIR)) {
    fs.mkdirSync(LOG_DIR, { recursive: true });
  }
} catch (error) {
  console.error('Failed to create log directory:', error);
}

/**
 * Format a log message
 */
function formatLogMessage(level: LogLevel, message: string, context?: any): string {
  const timestamp = new Date().toISOString();
  let logMessage = `[${timestamp}] [${level}] ${message}`;
  
  if (context) {
    if (typeof context === 'object') {
      try {
        logMessage += ` - ${JSON.stringify(context)}`;
      } catch (error) {
        logMessage += ` - ${format(context)}`;
      }
    } else {
      logMessage += ` - ${context}`;
    }
  }
  
  return logMessage;
}

/**
 * Write to a log file
 */
function writeToFile(filePath: string, message: string): void {
  try {
    // Check if file exists and get size
    let fileSize = 0;
    if (fs.existsSync(filePath)) {
      const stats = fs.statSync(filePath);
      fileSize = stats.size;
    }
    
    // Rotate log if necessary
    if (fileSize > MAX_LOG_SIZE) {
      const backupPath = `${filePath}.${Date.now()}.bak`;
      fs.renameSync(filePath, backupPath);
    }
    
    // Append to log file
    fs.appendFileSync(filePath, message + '\n');
  } catch (error) {
    console.error('Failed to write to log file:', error);
  }
}

/**
 * Log a message
 */
export function log(level: LogLevel, message: string, context?: any): void {
  const logMessage = formatLogMessage(level, message, context);
  
  // Always log to console
  switch (level) {
    case LogLevel.DEBUG:
      console.debug(logMessage);
      break;
    case LogLevel.INFO:
      console.info(logMessage);
      break;
    case LogLevel.WARN:
      console.warn(logMessage);
      break;
    case LogLevel.ERROR:
    case LogLevel.SECURITY:
      console.error(logMessage);
      break;
  }
  
  // Log to file
  writeToFile(LOG_FILE, logMessage);
  
  // Also log security issues to security log
  if (level === LogLevel.SECURITY) {
    writeToFile(SECURITY_LOG_FILE, logMessage);
  }
}

/**
 * Log a debug message
 */
export function debug(message: string, context?: any): void {
  log(LogLevel.DEBUG, message, context);
}

/**
 * Log an info message
 */
export function info(message: string, context?: any): void {
  log(LogLevel.INFO, message, context);
}

/**
 * Log a warning message
 */
export function warn(message: string, context?: any): void {
  log(LogLevel.WARN, message, context);
}

/**
 * Log an error message
 */
export function error(message: string, context?: any): void {
  log(LogLevel.ERROR, message, context);
}

/**
 * Log a security-related message
 */
export function security(message: string, context?: any): void {
  log(LogLevel.SECURITY, message, context);
}

/**
 * Log an attempted access to mock data in strict mode
 */
export function logMockDataAccess(req: any, details?: string): void {
  const context = {
    method: req.method,
    path: req.path,
    ip: req.ip,
    headers: req.headers,
    query: req.query,
    body: req.body ? JSON.stringify(req.body).substring(0, 200) : undefined,
    details
  };
  
  security('Attempted access to mock data in strict mode', context);
}

export default {
  debug,
  info,
  warn,
  error,
  security,
  logMockDataAccess
}; 
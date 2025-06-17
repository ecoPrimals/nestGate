/**
 * Common type definitions for the application
 */

/**
 * Service status type
 */
export type ServiceStatus = 'starting' | 'running' | 'stopped' | 'error';

/**
 * Port range configuration
 */
export interface PortRange {
  start: number;
  end: number;
}

/**
 * Service information
 */
export interface ServiceInfo {
  id: string;
  type: string;
  port: number;
  pid?: number;
  status: ServiceStatus;
  error?: Error;
  startTime?: Date;
  logFile?: string;
}

/**
 * Event data types for services
 */
export interface ServiceStartedEvent {
  serviceId: string;
  port: number;
}

export interface ServiceStoppedEvent {
  serviceId: string;
}

export interface ServiceErrorEvent {
  serviceId: string;
  error: Error;
}

export interface ServiceLogEvent {
  serviceId: string;
  data: string;
  isError?: boolean;
} 
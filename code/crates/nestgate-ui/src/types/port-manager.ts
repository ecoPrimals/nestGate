/**
 * Port Manager Type Definitions
 * 
 * These type definitions are used for the NestGate port management system.
 */

export enum ServiceType {
  SERVER = 'server',
  API = 'api',
  UI = 'ui',
  FILE_MONITOR = 'file-monitor',
  OTHER = 'other'
}

export interface PortRequest {
  serviceId: string;
  serviceType: ServiceType;
  preferredPort?: number;
}

export interface PortResponse {
  port: number;
  serviceId: string;
  serviceType: ServiceType;
  allocated: boolean;
  timestamp: string;
}

export interface PortInfo {
  port: number;
  serviceId: string;
  serviceType: ServiceType;
  status: 'active' | 'inactive' | 'unknown';
  uptime?: number;
  lastSeen?: string;
}

export interface PortManagerClient {
  allocatePort(request: PortRequest): Promise<number>;
  deallocatePort(port: number): Promise<boolean>;
  getPortInfo(port: number): Promise<PortInfo>;
  getActivePorts(): Promise<PortInfo[]>;
}

export interface PortManagerHook {
  isConnected: boolean;
  allocatedPort: number | null;
  error: Error | null;
  portInfo: PortInfo | null;
  allocatePort: (request?: PortRequest) => Promise<number>;
  deallocatePort: () => Promise<boolean>;
} 
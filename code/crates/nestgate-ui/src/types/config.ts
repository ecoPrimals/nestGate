/**
 * Configuration Types for NestGate UI
 * 
 * Updated to work with the orchestrator-based architecture
 */

export interface ServiceInfo {
  name: string;
  endpoint: string;
  status: string;
  capabilities: string[];
}

export interface OrchestratorEndpoint {
  host: string;
  port: number;
  base_url: string;
}

export interface ClientConfig {
  orchestrator: OrchestratorEndpoint;
  services: Record<string, ServiceInfo>;
  api_port?: number;
  ui_port?: number;
  websocket_port?: number;
}

export interface DynamicConfig {
  apiBaseUrl: string;
  websocketUrl: string;
  orchestratorUrl: string;
  services: Record<string, string>; // service type -> endpoint URL
} 
/**
 * Configuration Service
 * 
 * Manages dynamic configuration by connecting to the NestGate orchestrator
 */

import { DynamicConfig, ClientConfig } from '../types/config';

interface OrchestratorDiscovery {
  orchestrator: {
  host: string;
  port: number;
    health_endpoint: string;
    services_endpoint: string;
    config_endpoint: string;
  };
  version: string;
  discovery_method: string;
}

class ConfigService {
  private config: DynamicConfig | null = null;
  private orchestratorUrl: string | null = null;

  constructor() {
    // Orchestrator URL will be discovered dynamically
    this.orchestratorUrl = null;
  }

  /**
   * Discover NestGate Orchestrator location
   */
  private async discoverOrchestrator(): Promise<string> {
    // First try environment variable if provided
    if (process.env.REACT_APP_ORCHESTRATOR_URL) {
      return process.env.REACT_APP_ORCHESTRATOR_URL;
    }

    try {
      // Try to read discovery file
      const response = await fetch('/nestgate-discovery.json');
      if (response.ok) {
        const discovery: OrchestratorDiscovery = await response.json();
        const orchestratorUrl = `http://${discovery.orchestrator.host}:${discovery.orchestrator.port}`;
        console.log('🔍 Found orchestrator via discovery file:', orchestratorUrl);
        return orchestratorUrl;
      }
    } catch (error) {
      console.warn('Could not read discovery file, trying default location...');
    }

    // Fallback to default orchestrator location
    const defaultUrl = 'http://localhost:8090';
    console.log('🔧 Using default orchestrator URL:', defaultUrl);
    return defaultUrl;
  }

  /**
   * Fetch configuration from orchestrator
   */
  public async fetchConfig(): Promise<DynamicConfig> {
    try {
      if (!this.orchestratorUrl) {
        this.orchestratorUrl = await this.discoverOrchestrator();
      }

      // Test orchestrator health
      const healthResponse = await fetch(`${this.orchestratorUrl}/health`);
      if (!healthResponse.ok) {
        throw new Error(`Orchestrator health check failed: ${healthResponse.status}`);
      }

      // Fetch configuration from orchestrator
      const configResponse = await fetch(`${this.orchestratorUrl}/api/config`);
      if (!configResponse.ok) {
        throw new Error(`Failed to fetch config: ${configResponse.status}`);
      }

      const orchestratorConfig = await configResponse.json();
      
      // Build dynamic configuration
      const config: DynamicConfig = {
        apiBaseUrl: orchestratorConfig.services.api || `${this.orchestratorUrl}/api`,
        websocketUrl: orchestratorConfig.websocket_url || `ws://localhost:8090/ws`,
        orchestratorUrl: this.orchestratorUrl,
        services: orchestratorConfig.services || {}
      };

      this.config = config;
      console.log('✅ Configuration loaded from orchestrator:', config);
      return config;
      
    } catch (error) {
      console.error('❌ Failed to fetch dynamic configuration:', error);
      throw new Error(`Orchestrator not found. Please ensure the NestGate orchestrator is running and accessible.`);
    }
  }

  /**
   * Get cached configuration or fetch if not available
   */
  public async getConfig(): Promise<DynamicConfig> {
    if (this.config) {
      return this.config;
    }
    return this.fetchConfig();
  }

  /**
   * Get WebSocket URL from configuration
   */
  public async getWebSocketUrl(): Promise<string> {
    const config = await this.getConfig();
    return config.websocketUrl;
  }

  /**
   * Get API base URL from configuration
   */
  public async getApiBaseUrl(): Promise<string> {
    const config = await this.getConfig();
    return config.apiBaseUrl;
  }

  /**
   * Get service endpoint by type
   */
  public async getServiceEndpoint(serviceType: string): Promise<string | null> {
    const config = await this.getConfig();
    return config.services[serviceType] || null;
  }

  /**
   * Clear cached configuration (force refresh)
   */
  public clearCache(): void {
    this.config = null;
    this.orchestratorUrl = null;
  }
}

// Export singleton instance
export const configService = new ConfigService();
/**
 * Configuration Service - Fetches dynamic configuration from Port Manager
 */

export interface ServiceEndpoint {
  name: string;
  service_type: string;
  port: number;
  status: string;
  base_url: string;
  websocket_url: string;
  api_path?: string;
}

export interface PortManagerEndpoint {
  host: string;
  port: number;
  base_url: string;
}

export interface ClientConfig {
  port_manager: PortManagerEndpoint;
  services: Record<string, ServiceEndpoint>;
  api_port?: number;
  ui_port?: number;
  websocket_port?: number;
}

export interface DynamicConfig {
  apiBaseUrl: string;
  websocketUrl: string;
  portManagerUrl: string;
  services: Record<string, ServiceEndpoint>;
}

class ConfigService {
  private config: DynamicConfig | null = null;
  private portManagerUrl: string | null = null;

  constructor() {
    // Port Manager URL will be discovered dynamically
    this.portManagerUrl = null;
  }

  /**
   * Discover Port Manager location from discovery file or environment
   */
  private async discoverPortManager(): Promise<string> {
    // First try environment variable if provided
    if (process.env.REACT_APP_PORT_MANAGER_URL) {
      return process.env.REACT_APP_PORT_MANAGER_URL;
    }

    try {
      // Try to read discovery file (this would work in development when both UI and Port Manager are local)
      const discoveryResponse = await fetch('/api/port-manager-discovery');
      if (discoveryResponse.ok) {
        const discoveryData = await discoveryResponse.json();
        if (discoveryData.port_manager) {
          return discoveryData.port_manager.url;
        }
      }
    } catch (error) {
      console.log('Could not read discovery file, trying fallback ports...');
    }

    // Fallback: try common ports to discover running Port Manager
    const commonPorts = [8080, 8081, 8082, 9000, 9001, 9002];
    
    for (const port of commonPorts) {
      try {
        const testUrl = `http://localhost:${port}`;
        const response = await fetch(`${testUrl}/health`, {
          method: 'GET',
          timeout: 2000,
        } as RequestInit);
        
        if (response.ok) {
          const healthData = await response.json();
          // Verify this is actually the Port Manager by checking the health response
          if (healthData.status === 'healthy') {
            console.log(`🔍 Port Manager discovered at: ${testUrl}`);
            return testUrl;
          }
        }
      } catch (error) {
        // Port not available or not Port Manager, continue searching
        continue;
      }
    }
    
    throw new Error('Port Manager not found. Please ensure the Port Manager is running and accessible.');
  }

  /**
   * Fetch configuration from port manager
   */
  async fetchConfig(): Promise<DynamicConfig> {
    try {
      // Discover Port Manager if not already known
      if (!this.portManagerUrl) {
        this.portManagerUrl = await this.discoverPortManager();
      }

      const response = await fetch(`${this.portManagerUrl}/client-config`);
      
      if (!response.ok) {
        throw new Error(`Failed to fetch config: ${response.status} ${response.statusText}`);
      }

      const result = await response.json();
      
      if (!result.success) {
        throw new Error(`Config API error: ${result.error || 'Unknown error'}`);
      }

      const clientConfig: ClientConfig = result.data;
      
      // Transform to our internal format - NO HARDCODED FALLBACKS
      if (!clientConfig.api_port) {
        throw new Error('No API port available from port manager');
      }
      
      this.config = {
        apiBaseUrl: `http://localhost:${clientConfig.api_port}/api`,
        websocketUrl: this.determineWebSocketUrl(clientConfig),
        portManagerUrl: clientConfig.port_manager.base_url,
        services: clientConfig.services,
      };

      console.log('🔧 Dynamic configuration loaded:', this.config);
      return this.config;
      
    } catch (error) {
      console.error('❌ Failed to fetch dynamic configuration:', error);
      // Reset Port Manager URL to force rediscovery on next attempt
      this.portManagerUrl = null;
      throw error;
    }
  }

  /**
   * Get current configuration (fetch if not loaded)
   */
  async getConfig(): Promise<DynamicConfig> {
    if (!this.config) {
      await this.fetchConfig();
    }
    return this.config!;
  }

  /**
   * Get API base URL
   */
  async getApiBaseUrl(): Promise<string> {
    const config = await this.getConfig();
    return config.apiBaseUrl;
  }

  /**
   * Get WebSocket URL
   */
  async getWebSocketUrl(): Promise<string> {
    const config = await this.getConfig();
    return config.websocketUrl;
  }

  /**
   * Get service endpoint by ID
   */
  async getServiceEndpoint(serviceId: string): Promise<ServiceEndpoint | null> {
    const config = await this.getConfig();
    return config.services[serviceId] || null;
  }

  /**
   * Get all services
   */
  async getServices(): Promise<Record<string, ServiceEndpoint>> {
    const config = await this.getConfig();
    return config.services;
  }

  /**
   * Refresh configuration from port manager
   */
  async refresh(): Promise<DynamicConfig> {
    this.config = null;
    return this.fetchConfig();
  }

  private determineWebSocketUrl(clientConfig: ClientConfig): string {
    // Use websocket_port if available, otherwise use api_port for combined service
    if (clientConfig.websocket_port) {
      return `ws://localhost:${clientConfig.websocket_port}`;
    } else if (clientConfig.api_port) {
      // WebSocket is on the same port as API at /ws endpoint
      return `ws://localhost:${clientConfig.api_port}/ws`;
    } else {
      throw new Error('No WebSocket or API port available from port manager');
    }
  }
}

// Export singleton instance
export const configService = new ConfigService();
export default configService; 
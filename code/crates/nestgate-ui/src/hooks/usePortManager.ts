import { useState, useEffect, useCallback, useRef } from 'react';

// Port Manager API Types
export interface SystemMetrics {
  timestamp: number;
  total_services: number;
  running_services: number;
  failed_services: number;
  allocated_ports: number;
  active_processes: number;
  uptime_seconds: number;
  memory_usage: number;
  cpu_usage: number;
  disk_usage: number;
  network_io: {
    bytes_received: number;
    bytes_transmitted: number;
    packets_received: number;
    packets_transmitted: number;
  };
}

export interface ServiceMetrics {
  service_id: string;
  service_name: string;
  status: string;
  uptime_seconds: number;
  restart_count: number;
  memory_usage: number;
  cpu_usage: number;
  response_time_ms?: number;
  error_count: number;
  health_status: string;
  last_health_check?: number;
}

export interface HealthCheckResult {
  service_id: string;
  timestamp: string;
  status: 'Healthy' | 'Unhealthy' | 'Unknown' | { Failed: string };
  response_time_ms: number;
  details?: string;
  check_type: string;
}

export interface ServiceInstance {
  definition: {
    id: string;
    name: string;
    service_type: string;
    startup_command: string;
    shutdown_command?: string;
    working_directory?: string;
    environment: Record<string, string>;
    preferred_port?: number;
    port_range?: [number, number];
    dependencies: string[];
    health_checks: any[];
    auto_restart: boolean;
    max_restart_attempts: number;
    created_at: string;
    updated_at: string;
  };
  status: string;
  port?: number;
  pid?: number;
  started_at?: string;
  last_health_check?: string;
  restart_count: number;
  is_healthy: boolean;
}

export interface ConnectionStats {
  total_connections: number;
  active_connections: number;
  failed_connections: number;
  bytes_transferred: number;
  avg_response_time: number;
}

export interface DashboardData {
  system_metrics: SystemMetrics;
  service_metrics: Record<string, ServiceMetrics>;
  health_status: Record<string, HealthCheckResult>;
  services: ServiceInstance[];
  port_allocations: Record<string, number>;
  connection_stats: ConnectionStats;
  timestamp: number;
}

interface UsePortManagerState {
  dashboardData: DashboardData | null;
  systemMetrics: SystemMetrics | null;
  serviceMetrics: Record<string, ServiceMetrics>;
  healthStatus: Record<string, HealthCheckResult>;
  services: ServiceInstance[];
  portAllocations: Record<string, number>;
  connectionStats: ConnectionStats | null;
  isLoading: boolean;
  error: string | null;
  lastUpdate: number;
}

interface UsePortManagerActions {
  refreshData: () => Promise<void>;
  startService: (serviceId: string) => Promise<void>;
  stopService: (serviceId: string) => Promise<void>;
  restartService: (serviceId: string) => Promise<void>;
  registerService: (service: any) => Promise<void>;
  unregisterService: (serviceId: string) => Promise<void>;
}

const PORT_MANAGER_BASE_URL = 'http://localhost:8080';

export function usePortManager(): UsePortManagerState & UsePortManagerActions {
  const [state, setState] = useState<UsePortManagerState>({
    dashboardData: null,
    systemMetrics: null,
    serviceMetrics: {},
    healthStatus: {},
    services: [],
    portAllocations: {},
    connectionStats: null,
    isLoading: true,
    error: null,
    lastUpdate: 0,
  });

  const eventSourceRef = useRef<EventSource | null>(null);
  const intervalRef = useRef<any>(null);

  // Helper function to make API calls
  const apiCall = useCallback(async (endpoint: string, options?: RequestInit) => {
    const response = await fetch(`${PORT_MANAGER_BASE_URL}${endpoint}`, {
      headers: {
        'Content-Type': 'application/json',
        ...options?.headers,
      },
      ...options,
    });

    if (!response.ok) {
      throw new Error(`API call failed: ${response.status} ${response.statusText}`);
    }

    const data = await response.json();
    
    if (!data.success) {
      throw new Error(data.error || 'API call failed');
    }

    return data.data;
  }, []);

  // Fetch dashboard data
  const refreshData = useCallback(async () => {
    try {
      setState(prev => ({ ...prev, isLoading: true, error: null }));
      
      const dashboardData: DashboardData = await apiCall('/dashboard');
      
      setState(prev => ({
        ...prev,
        dashboardData,
        systemMetrics: dashboardData.system_metrics,
        serviceMetrics: dashboardData.service_metrics,
        healthStatus: dashboardData.health_status,
        services: dashboardData.services,
        portAllocations: dashboardData.port_allocations,
        connectionStats: dashboardData.connection_stats,
        lastUpdate: Date.now(),
        isLoading: false,
        error: null,
      }));
    } catch (error) {
      console.error('Failed to fetch dashboard data:', error);
      setState(prev => ({
        ...prev,
        isLoading: false,
        error: error instanceof Error ? error.message : 'Unknown error',
      }));
    }
  }, [apiCall]);

  // Service control actions
  const startService = useCallback(async (serviceId: string) => {
    try {
      await apiCall(`/services/${serviceId}/start`, { method: 'POST' });
      await refreshData();
    } catch (error) {
      console.error(`Failed to start service ${serviceId}:`, error);
      throw error;
    }
  }, [apiCall, refreshData]);

  const stopService = useCallback(async (serviceId: string) => {
    try {
      await apiCall(`/services/${serviceId}/stop`, { method: 'POST' });
      await refreshData();
    } catch (error) {
      console.error(`Failed to stop service ${serviceId}:`, error);
      throw error;
    }
  }, [apiCall, refreshData]);

  const restartService = useCallback(async (serviceId: string) => {
    try {
      await apiCall(`/services/${serviceId}/restart`, { method: 'POST' });
      await refreshData();
    } catch (error) {
      console.error(`Failed to restart service ${serviceId}:`, error);
      throw error;
    }
  }, [apiCall, refreshData]);

  const registerService = useCallback(async (service: any) => {
    try {
      await apiCall('/services', {
        method: 'POST',
        body: JSON.stringify(service),
      });
      await refreshData();
    } catch (error) {
      console.error('Failed to register service:', error);
      throw error;
    }
  }, [apiCall, refreshData]);

  const unregisterService = useCallback(async (serviceId: string) => {
    try {
      await apiCall(`/services/${serviceId}`, { method: 'DELETE' });
      await refreshData();
    } catch (error) {
      console.error(`Failed to unregister service ${serviceId}:`, error);
      throw error;
    }
  }, [apiCall, refreshData]);

  // Set up real-time updates
  useEffect(() => {
    // Initial data fetch
    refreshData();

    // Set up polling as fallback
    intervalRef.current = setInterval(refreshData, 30000); // 30 seconds

    // Set up Server-Sent Events for real-time updates
    const setupSSE = () => {
      try {
        eventSourceRef.current = new EventSource(`${PORT_MANAGER_BASE_URL}/stream/services`);
        
        eventSourceRef.current.onmessage = (event) => {
          try {
            const services = JSON.parse(event.data);
            setState(prev => ({
              ...prev,
              services,
              lastUpdate: Date.now(),
            }));
          } catch (error) {
            console.error('Failed to parse SSE data:', error);
          }
        };

        eventSourceRef.current.onerror = (error) => {
          console.error('SSE connection error:', error);
          eventSourceRef.current?.close();
          // Retry SSE connection after 5 seconds
          setTimeout(setupSSE, 5000);
        };
      } catch (error) {
        console.error('Failed to setup SSE:', error);
      }
    };

    setupSSE();

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
      if (eventSourceRef.current) {
        eventSourceRef.current.close();
      }
    };
  }, [refreshData]);

  return {
    ...state,
    refreshData,
    startService,
    stopService,
    restartService,
    registerService,
    unregisterService,
  };
} 
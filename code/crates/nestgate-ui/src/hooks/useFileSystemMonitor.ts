import { useState, useEffect, useCallback } from 'react';
import axios, { AxiosInstance } from 'axios';

// Define service types previously imported from port-manager.ts
export enum ServiceType {
  UI = 'UI',
  API = 'API',
  SERVER = 'SERVER',
  WEBSOCKET = 'WEBSOCKET',
  FileSystem = 'FileSystem'
}

export interface FileSystemEvent {
  id: string;
  path: string;
  event_type: 'create' | 'modify' | 'remove' | 'rename';
  timestamp: string;
  is_directory: boolean;
  metadata?: Record<string, any>;
}

export interface WatchedDirectory {
  id: string;
  path: string;
  recursive: boolean;
  added_at: string;
}

export interface FileSystemMonitorHookOptions {
  baseUrl?: string;
  preferredPort?: number;
  autoConnect?: boolean;
  serviceId?: string;
}

export function useFileSystemMonitor(options: FileSystemMonitorHookOptions = {}) {
  const {
    baseUrl = 'http://localhost:8080', // Port Manager URL (only hardcoded endpoint)
    preferredPort = 9500,
    autoConnect = true,
    serviceId = 'fs-monitor-client'
  } = options;

  const [api, setApi] = useState<AxiosInstance | null>(null);
  const [isConnected, setIsConnected] = useState<boolean>(false);
  const [events, setEvents] = useState<FileSystemEvent[]>([]);
  const [watchedDirectories, setWatchedDirectories] = useState<WatchedDirectory[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<Error | null>(null);
  const [servicePort, setServicePort] = useState<number | null>(null);
  const [portManagerConnected, setPortManagerConnected] = useState<boolean>(false);
  const [portManagerError, setPortManagerError] = useState<Error | null>(null);

  // Connect to the Rust port manager
  const connectToPortManager = useCallback(async () => {
    if (!autoConnect) return;
    
    setIsLoading(true);
    setError(null);
    setPortManagerError(null);
    
    try {
      // Check if port manager is available
      await axios.get(`${baseUrl}/health`);
      setPortManagerConnected(true);
      
      // Get or register service with the Rust port manager
      const response = await axios.post(`${baseUrl}/services`, {
        id: serviceId,
        service_type: ServiceType.FileSystem,
        preferred_port: preferredPort
      });
      
      if (response.data && response.data.port) {
        setServicePort(response.data.port);
      }
    } catch (err) {
      setPortManagerConnected(false);
      setPortManagerError(err instanceof Error ? err : new Error('Failed to connect to port manager'));
      setError(err instanceof Error ? err : new Error('Failed to allocate port from port manager'));
    } finally {
      setIsLoading(false);
    }
  }, [autoConnect, baseUrl, serviceId, preferredPort]);

  // Initialize API client when we have a port
  useEffect(() => {
    if (servicePort) {
      const fsMonitorBaseURL = `http://localhost:${servicePort}`;
      setApi(axios.create({
        baseURL: fsMonitorBaseURL,
        timeout: 10000
      }));
    } else {
      setApi(null);
    }
  }, [servicePort]);

  // Connect to port manager when component mounts
  useEffect(() => {
    connectToPortManager();
  }, [connectToPortManager]);

  // Check connection to filesystem monitor
  useEffect(() => {
    let mounted = true;
    
    const checkConnection = async () => {
      if (!api) {
        if (mounted) setIsConnected(false);
        return;
      }
      
      try {
        await api.get('/health');
        if (mounted) setIsConnected(true);
      } catch (err) {
        if (mounted) {
          setIsConnected(false);
          setError(err instanceof Error ? err : new Error('Failed to connect to filesystem monitor'));
        }
      }
    };
    
    checkConnection();
    const interval = setInterval(checkConnection, 10000); // Check every 10 seconds
    
    return () => {
      mounted = false;
      clearInterval(interval);
    };
  }, [api]);

  // Get watched directories
  const fetchWatchedDirectories = useCallback(async () => {
    if (!api || !isConnected) {
      return [];
    }
    
    setIsLoading(true);
    setError(null);
    
    try {
      const response = await api.get('/watches');
      const dirs = response.data.watches || [];
      setWatchedDirectories(dirs);
      return dirs;
    } catch (err) {
      setError(err instanceof Error ? err : new Error('Failed to fetch watched directories'));
      return [];
    } finally {
      setIsLoading(false);
    }
  }, [api, isConnected]);

  // Get events
  const fetchEvents = useCallback(async (limit: number = 50) => {
    if (!api || !isConnected) {
      return [];
    }
    
    setIsLoading(true);
    setError(null);
    
    try {
      const response = await api.get('/events', {
        params: { limit }
      });
      const newEvents = response.data.events || [];
      setEvents(newEvents);
      return newEvents;
    } catch (err) {
      setError(err instanceof Error ? err : new Error('Failed to fetch events'));
      return [];
    } finally {
      setIsLoading(false);
    }
  }, [api, isConnected]);

  // Add directory to watch
  const addWatch = useCallback(async (path: string, recursive: boolean = true) => {
    if (!api || !isConnected) {
      setError(new Error('Not connected to filesystem monitor'));
      return null;
    }
    
    setIsLoading(true);
    setError(null);
    
    try {
      const response = await api.post('/watches', {
        path,
        recursive
      });
      
      // Refresh the list of watched directories
      await fetchWatchedDirectories();
      
      return response.data;
    } catch (err) {
      setError(err instanceof Error ? err : new Error('Failed to add watch'));
      return null;
    } finally {
      setIsLoading(false);
    }
  }, [api, isConnected, fetchWatchedDirectories]);

  // Remove directory watch
  const removeWatch = useCallback(async (watchId: string) => {
    if (!api || !isConnected) {
      setError(new Error('Not connected to filesystem monitor'));
      return false;
    }
    
    setIsLoading(true);
    setError(null);
    
    try {
      await api.delete(`/watches/${watchId}`);
      
      // Refresh the list of watched directories
      await fetchWatchedDirectories();
      
      return true;
    } catch (err) {
      setError(err instanceof Error ? err : new Error('Failed to remove watch'));
      return false;
    } finally {
      setIsLoading(false);
    }
  }, [api, isConnected, fetchWatchedDirectories]);

  // Initialize by fetching watched directories and events when connected
  useEffect(() => {
    if (isConnected) {
      fetchWatchedDirectories();
      fetchEvents();
    }
  }, [isConnected, fetchWatchedDirectories, fetchEvents]);

  // Clean up on unmount
  useEffect(() => {
    return () => {
      if (serviceId && servicePort) {
        // Try to deregister service on unmount
        axios.delete(`${baseUrl}/services/${serviceId}`).catch(() => {
          // Ignore errors on unmount
        });
      }
    };
  }, [baseUrl, serviceId, servicePort]);

  const handleMessage = (event: MessageEvent<unknown>) => {
    // Implementation of handleMessage function
  };

  return {
    isConnected,
    events,
    watchedDirectories,
    isLoading,
    error,
    allocatedPort: servicePort,
    portManagerConnected,
    portManagerError,
    fetchEvents,
    fetchWatchedDirectories,
    addWatch,
    removeWatch
  };
} 
/**
 * Filesystem Monitor Client
 * 
 * Client library for interacting with the NestGate Filesystem Monitor
 */

import axios, { AxiosError, AxiosInstance } from 'axios';
import {
  ApiResponse,
  EventFilter,
  EventKind,
  FsEvent,
  WatchInfo,
  WatchRequest
} from './types';

/**
 * Configuration for the Filesystem Monitor client
 */
export interface FsMonitorClientConfig {
  /** Base URL for the Filesystem Monitor API */
  baseUrl: string;
  
  /** WebSocket URL for real-time events */
  wsUrl?: string;
  
  /** Request timeout in milliseconds */
  timeout?: number;
  
  /** Auto-reconnect if connection fails */
  autoReconnect?: boolean;
  
  /** Max reconnect attempts */
  maxReconnectAttempts?: number;
}

/**
 * Client for the NestGate Filesystem Monitor
 */
export class FsMonitorClient {
  private api: AxiosInstance;
  private config: FsMonitorClientConfig;
  private isConnected: boolean = false;
  private reconnectAttempts: number = 0;
  
  /**
   * Create a new FsMonitorClient
   * @param config Client configuration
   */
  constructor(config: FsMonitorClientConfig) {
    this.config = {
      timeout: 5000,
      autoReconnect: true,
      maxReconnectAttempts: 5,
      wsUrl: config.baseUrl.replace(/^http/, 'ws') + '/ws',
      ...config,
    };
    
    this.api = axios.create({
      baseURL: this.config.baseUrl,
      timeout: this.config.timeout,
      headers: {
        'Content-Type': 'application/json',
      },
    });
    
    // Add response interceptor for error handling
    this.api.interceptors.response.use(
      (response) => response,
      (error: AxiosError) => this.handleApiError(error)
    );
  }
  
  /**
   * Handle API errors, including reconnection logic
   */
  private async handleApiError(error: AxiosError) {
    // Connection errors
    if (!error.response) {
      this.isConnected = false;
      console.error('Filesystem Monitor connection error:', error.message);
      
      if (this.config.autoReconnect && this.reconnectAttempts < (this.config.maxReconnectAttempts || 5)) {
        this.reconnectAttempts++;
        console.log(`Attempting to reconnect (${this.reconnectAttempts}/${this.config.maxReconnectAttempts})...`);
        
        // Wait before reconnecting
        await new Promise(resolve => setTimeout(resolve, 1000 * this.reconnectAttempts));
        
        // Try request again
        return this.api.request(error.config!);
      }
    }
    
    return Promise.reject(error);
  }
  
  /**
   * Check if the client is connected to the Filesystem Monitor
   */
  public async connect(): Promise<boolean> {
    try {
      await this.api.get<ApiResponse<string>>('/health');
      this.isConnected = true;
      this.reconnectAttempts = 0;
      return true;
    } catch (error) {
      this.isConnected = false;
      return false;
    }
  }
  
  /**
   * Get WebSocket URL for real-time filesystem events
   */
  public getWebSocketUrl(): string {
    return this.config.wsUrl || (this.config.baseUrl.replace(/^http/, 'ws') + '/ws');
  }
  
  /**
   * Watch a directory for changes
   */
  public async watchDirectory(request: WatchRequest): Promise<boolean> {
    const response = await this.api.post<ApiResponse<null>>('/watches', request);
    return response.data.success;
  }
  
  /**
   * Stop watching a directory
   */
  public async unwatchDirectory(path: string): Promise<boolean> {
    const response = await this.api.delete<ApiResponse<null>>(`/watches/${encodeURIComponent(path)}`);
    return response.data.success;
  }
  
  /**
   * Get all watched directories
   */
  public async getWatchedDirectories(): Promise<string[]> {
    const response = await this.api.get<ApiResponse<string[]>>('/watches');
    return response.data.data || [];
  }
  
  /**
   * Create an event filter
   */
  public createEventFilter(options: Partial<EventFilter> = {}): EventFilter {
    return {
      includeDirectories: true,
      includeHidden: false,
      ...options
    };
  }
} 
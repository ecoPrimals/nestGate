import axios from 'axios';
import { API_BASE_URL } from '../constants';
import { DataSourceType, isStrictLiveMode } from '../utils/env';

/**
 * Notification importance level
 */
export enum NotificationLevel {
  INFO = 'info',
  WARNING = 'warning',
  ERROR = 'error',
  SUCCESS = 'success'
}

/**
 * Notification source/category
 */
export enum NotificationSource {
  SYSTEM = 'system',
  STORAGE = 'storage',
  SECURITY = 'security',
  USER = 'user',
  BACKUP = 'backup',
  REPLICATION = 'replication',
  NETWORK = 'network'
}

/**
 * Notification data structure
 */
export interface Notification {
  id: string;
  title: string;
  message: string;
  level: NotificationLevel;
  source: NotificationSource;
  timestamp: string;
  read: boolean;
  dismissed: boolean;
  actionRequired: boolean;
  actionUrl?: string;
  isMock?: boolean;
  dataSource: DataSourceType;
}

/**
 * Service responsible for system notifications
 */
export class NotificationService {
  private static instance: NotificationService;
  private static readonly API_URL = `${API_BASE_URL}/api/notifications`;
  private listeners: Array<(notifications: Notification[]) => void> = [];
  private cachedNotifications: Notification[] = [];
  private pollingInterval: NodeJS.Timeout | null = null;

  private constructor() {
    // Initialize with mock data in development mode
    this.cachedNotifications = this.getMockNotifications();
  }

  /**
   * Gets the singleton instance of NotificationService
   */
  public static getInstance(): NotificationService {
    if (!NotificationService.instance) {
      NotificationService.instance = new NotificationService();
    }
    return NotificationService.instance;
  }

  /**
   * Start polling for notifications
   */
  public startPolling(intervalMs: number = 30000): void {
    if (this.pollingInterval) {
      clearInterval(this.pollingInterval);
    }
    
    // Initial fetch
    this.fetchNotifications();
    
    // Start polling
    this.pollingInterval = setInterval(() => {
      this.fetchNotifications();
    }, intervalMs);
  }

  /**
   * Stop polling for notifications
   */
  public stopPolling(): void {
    if (this.pollingInterval) {
      clearInterval(this.pollingInterval);
      this.pollingInterval = null;
    }
  }

  /**
   * Get all notifications
   */
  public async getNotifications(): Promise<Notification[]> {
    try {
      // Try to get real notifications from API
      const response = await axios.get(`${NotificationService.API_URL}/notifications`);
      
      return response.data.map((notification: Notification) => ({
        ...notification,
        dataSource: DataSourceType.LIVE
      }));
    } catch (error) {
      console.error('Error fetching notifications:', error);
      
      // In strict live mode, return placeholders
      if (isStrictLiveMode()) {
        return this.getPlaceholderNotifications();
      }
      
      // Otherwise fall back to mock data
      return this.getMockNotifications();
    }
  }

  /**
   * Get unread notifications count
   */
  public async getUnreadCount(): Promise<number> {
    const notifications = await this.getNotifications();
    return notifications.filter(notification => !notification.read).length;
  }

  /**
   * Mark a notification as read
   */
  public async markAsRead(id: string): Promise<void> {
    try {
      await axios.put(`${NotificationService.API_URL}/${id}/read`);
      // Update local cache
      this.updateNotificationStatus(id, { read: true });
    } catch (error) {
      console.error(`Error marking notification ${id} as read:`, error);
      // Update local cache anyway for development
      this.updateNotificationStatus(id, { read: true });
    }
  }

  /**
   * Mark a notification as dismissed
   */
  public async dismissNotification(id: string): Promise<void> {
    try {
      await axios.put(`${NotificationService.API_URL}/${id}/dismiss`);
      // Update local cache
      this.updateNotificationStatus(id, { dismissed: true });
    } catch (error) {
      console.error(`Error dismissing notification ${id}:`, error);
      // Update local cache anyway for development
      this.updateNotificationStatus(id, { dismissed: true });
    }
  }

  /**
   * Mark all notifications as read
   */
  public async markAllAsRead(): Promise<void> {
    try {
      await axios.put(`${NotificationService.API_URL}/read-all`);
      // Update local cache
      this.cachedNotifications = this.cachedNotifications.map(notification => ({
        ...notification,
        read: true
      }));
      this.notifyListeners();
    } catch (error) {
      console.error('Error marking all notifications as read:', error);
      // Update local cache anyway for development
      this.cachedNotifications = this.cachedNotifications.map(notification => ({
        ...notification,
        read: true
      }));
      this.notifyListeners();
    }
  }

  /**
   * Dismiss all notifications
   */
  public async dismissAll(): Promise<void> {
    try {
      await axios.put(`${NotificationService.API_URL}/dismiss-all`);
      // Update local cache
      this.cachedNotifications = this.cachedNotifications.map(notification => ({
        ...notification,
        dismissed: true
      }));
      this.notifyListeners();
    } catch (error) {
      console.error('Error dismissing all notifications:', error);
      // Update local cache anyway for development
      this.cachedNotifications = this.cachedNotifications.map(notification => ({
        ...notification,
        dismissed: true
      }));
      this.notifyListeners();
    }
  }

  /**
   * Add a notification listener
   */
  public addListener(callback: (notifications: Notification[]) => void): () => void {
    this.listeners.push(callback);
    
    // Immediately call with current data
    callback(this.cachedNotifications);
    
    // Return unsubscribe function
    return () => {
      this.listeners = this.listeners.filter(listener => listener !== callback);
    };
  }

  /**
   * Fetch notifications from server
   */
  private async fetchNotifications(): Promise<void> {
    try {
      const notifications = await this.getNotifications();
      this.notifyListeners();
    } catch (error) {
      console.error('Error in notification polling:', error);
    }
  }

  /**
   * Update notification status in local cache
   */
  private updateNotificationStatus(id: string, updates: Partial<Notification>): void {
    this.cachedNotifications = this.cachedNotifications.map(notification => {
      if (notification.id === id) {
        return { ...notification, ...updates };
      }
      return notification;
    });
    this.notifyListeners();
  }

  /**
   * Notify all listeners of changes
   */
  private notifyListeners(): void {
    this.listeners.forEach(listener => {
      listener(this.cachedNotifications);
    });
  }

  /**
   * Get mock notifications for development
   */
  private getMockNotifications(): Notification[] {
    return [
      {
        id: '1',
        title: 'System update available',
        message: 'NestGate v2.5.0 is available for installation. This update includes security fixes and new features.',
        level: NotificationLevel.INFO,
        source: NotificationSource.SYSTEM,
        timestamp: new Date(Date.now() - 3600000).toISOString(), // 1 hour ago
        read: false,
        dismissed: false,
        actionRequired: true,
        actionUrl: '/system/updates',
        isMock: true,
        dataSource: DataSourceType.MOCK
      },
      {
        id: '2',
        title: 'Storage pool degraded',
        message: 'Pool "tank" is in degraded state due to a failing disk. Replace the disk as soon as possible.',
        level: NotificationLevel.ERROR,
        source: NotificationSource.STORAGE,
        timestamp: new Date(Date.now() - 7200000).toISOString(), // 2 hours ago
        read: true,
        dismissed: false,
        actionRequired: true,
        actionUrl: '/storage/pools/tank',
        isMock: true,
        dataSource: DataSourceType.MOCK
      },
      {
        id: '3',
        title: 'Scheduled backup completed',
        message: 'Weekly backup of "documents" dataset completed successfully.',
        level: NotificationLevel.SUCCESS,
        source: NotificationSource.BACKUP,
        timestamp: new Date(Date.now() - 86400000).toISOString(), // 1 day ago
        read: true,
        dismissed: true,
        actionRequired: false,
        isMock: true,
        dataSource: DataSourceType.MOCK
      },
      {
        id: '4',
        title: 'Security alert',
        message: 'Multiple failed login attempts detected from IP 192.168.1.105.',
        level: NotificationLevel.WARNING,
        source: NotificationSource.SECURITY,
        timestamp: new Date(Date.now() - 43200000).toISOString(), // 12 hours ago
        read: false,
        dismissed: false,
        actionRequired: true,
        actionUrl: '/security/logs',
        isMock: true,
        dataSource: DataSourceType.MOCK
      }
    ];
  }

  /**
   * Get placeholder notifications for strict live mode
   */
  private getPlaceholderNotifications(): Notification[] {
    return [
      {
        id: 'placeholder-1',
        title: 'Notifications Coming Soon',
        message: 'The notification system is being implemented with live data. This is a placeholder notification.',
        level: NotificationLevel.INFO,
        source: NotificationSource.SYSTEM,
        timestamp: new Date().toISOString(),
        read: false,
        dismissed: false,
        actionRequired: false,
        dataSource: DataSourceType.PLACEHOLDER
      }
    ];
  }
} 
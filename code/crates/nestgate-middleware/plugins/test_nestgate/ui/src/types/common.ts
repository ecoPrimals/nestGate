export interface StorageSystem {
  id: string;
  name: string;
  type: string;
  status: 'online' | 'offline' | 'error';
  capacity: {
    total: number;
    used: number;
    available: number;
  };
  lastUpdated: string;
}

export interface SystemSettings {
  id: string;
  name: string;
  value: string | number | boolean;
  category: string;
  description?: string;
}

export interface SystemMetrics {
  id: string;
  timestamp: string;
  metrics: {
    [key: string]: number;
  };
}

export interface User {
  id: string;
  username: string;
  email: string;
  role: 'admin' | 'user';
  lastLogin?: string;
}

export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  pageSize: number;
  totalPages: number;
} 
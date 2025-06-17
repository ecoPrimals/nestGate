import { DataSourceType } from '../utils/env';

export type BackupJobStatus = 'Idle' | 'Running' | 'Completed' | 'Failed' | 'Paused';

export type BackupTargetType = 
  | { type: 'Local'; path: string }
  | { type: 'RemoteSsh'; host: string; port: number; user: string; path: string; keyFile?: string }
  | { type: 'Nfs'; server: string; export: string; mountPoint: string };

export interface BackupTarget {
  id: string;
  name: string;
  targetType: BackupTargetType;
  description?: string;
  created: string;
  modified: string;
  dataSource?: DataSourceType;
}

export interface BackupRetention {
  hourly: number | null;
  daily: number | null;
  weekly: number | null;
  monthly: number | null;
  yearly: number | null;
}

export interface BackupJob {
  id: string;
  name: string;
  source: string;
  targetId: string;
  schedule: string;
  retention: BackupRetention;
  status: BackupJobStatus;
  description?: string;
  lastRun: string | null;
  nextRun: string | null;
  created: string;
  modified: string;
  dataSource?: DataSourceType;
} 
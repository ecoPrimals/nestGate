export interface GeneralSettings {
  systemName: string;
  timezone: string;
  language: string;
}

export interface NetworkSettings {
  ipAddress: string;
  subnetMask: string;
  gateway: string;
  dnsServers: string;
  dhcp: boolean;
}

export interface UserSettings {
  adminUsername: string;
  adminPassword: string;
  confirmPassword: string;
  email: string;
}

export interface SecuritySettings {
  enableTwoFactor: boolean;
  sessionTimeout: number;
  failedLoginAttempts: number;
  remoteAccess: boolean;
}

export interface NotificationSettings {
  emailNotifications: boolean;
  systemAlerts: boolean;
  updateNotifications: boolean;
  notificationEmail: string;
}

export interface BackupSettings {
  automaticBackups: boolean;
  backupFrequency: 'daily' | 'weekly' | 'monthly';
  backupLocation: string;
  backupsToKeep: number;
}

export interface SettingsFormProps<T> {
  onSave: (values: T) => void;
  initialValues?: Partial<T>;
}

export type SettingsSections = 'general' | 'network' | 'user' | 'security' | 'notifications' | 'backup'; 
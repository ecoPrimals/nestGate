import React, { useState } from 'react';
import { 
  Box, 
  Typography, 
  Tabs, 
  Tab,
  Snackbar,
  Alert
} from '@mui/material';
import {
  Settings as SettingsIcon,
  Person as PersonIcon,
  Public as NetworkIcon,
  Security as SecurityIcon,
  Notifications as NotificationsIcon,
  Backup as BackupIcon,
} from '@mui/icons-material';
import {
  GeneralSettingsTab,
  NetworkSettingsTab,
  UserSettingsTab,
  SecuritySettingsTab,
  NotificationSettingsTab,
  BackupSettingsTab,
} from './settings/components';
import { 
  GeneralSettings, 
  NetworkSettings, 
  UserSettings, 
  SecuritySettings, 
  NotificationSettings, 
  BackupSettings
} from './settings/types';

const ModernSettings: React.FC = () => {
  const [activeTab, setActiveTab] = useState<number>(0);
  const [snackbar, setSnackbar] = useState<{
    open: boolean;
    message: string;
    severity: 'success' | 'error';
  }>({
    open: false,
    message: '',
    severity: 'success',
  });

  const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
    setActiveTab(newValue);
  };

  const showNotification = (message: string, severity: 'success' | 'error' = 'success') => {
    setSnackbar({ open: true, message, severity });
  };

  const handleCloseSnackbar = () => {
    setSnackbar(prev => ({ ...prev, open: false }));
  };

  const handleGeneralSave = async (values: GeneralSettings) => {
    console.log('General settings:', values);
    // TODO: Implement actual save logic
  };

  const handleNetworkSave = async (values: NetworkSettings) => {
    console.log('Network settings:', values);
    // TODO: Implement actual save logic
  };

  const handleUserSave = async (values: UserSettings) => {
    console.log('User settings:', values);
    // TODO: Implement actual save logic
  };

  const handleSecuritySave = async (values: SecuritySettings) => {
    console.log('Security settings:', values);
    // TODO: Implement actual save logic
  };

  const handleNotificationSave = async (values: NotificationSettings) => {
    console.log('Notification settings:', values);
    // TODO: Implement actual save logic
  };

  const handleBackupSave = async (values: BackupSettings) => {
    console.log('Backup settings:', values);
    // TODO: Implement actual save logic
  };

  const tabs = [
    { value: 0, label: 'General', icon: <SettingsIcon /> },
    { value: 1, label: 'Network', icon: <NetworkIcon /> },
    { value: 2, label: 'User Management', icon: <PersonIcon /> },
    { value: 3, label: 'Security', icon: <SecurityIcon /> },
    { value: 4, label: 'Notifications', icon: <NotificationsIcon /> },
    { value: 5, label: 'Backup & Restore', icon: <BackupIcon /> },
  ];

  return (
    <Box sx={{ padding: 3 }}>
      <Box sx={{ marginBottom: 3 }}>
        <Typography variant="h4" component="h1" gutterBottom>
          System Settings
        </Typography>
        <Typography variant="body1" color="text.secondary">
          Configure system settings and preferences. Changes will be applied immediately.
        </Typography>
      </Box>

      <Box sx={{ borderBottom: 1, borderColor: 'divider', marginBottom: 3 }}>
        <Tabs 
          value={activeTab}
          onChange={handleTabChange}
          variant="scrollable"
          scrollButtons="auto"
        >
          {tabs.map((tab) => (
            <Tab 
              key={tab.value}
              value={tab.value}
              icon={tab.icon}
              label={tab.label}
              iconPosition="start"
            />
          ))}
        </Tabs>
      </Box>

      <Box sx={{ display: activeTab === 0 ? 'block' : 'none' }}>
        <GeneralSettingsTab
          onSave={handleGeneralSave}
          onSuccess={(message) => showNotification(message)}
          onError={(message) => showNotification(message, 'error')}
        />
      </Box>

      <Box sx={{ display: activeTab === 1 ? 'block' : 'none' }}>
        <NetworkSettingsTab
          onSave={handleNetworkSave}
          onSuccess={(message) => showNotification(message)}
          onError={(message) => showNotification(message, 'error')}
        />
      </Box>

      <Box sx={{ display: activeTab === 2 ? 'block' : 'none' }}>
        <UserSettingsTab
          onSave={handleUserSave}
          onSuccess={(message) => showNotification(message)}
          onError={(message) => showNotification(message, 'error')}
        />
      </Box>

      <Box sx={{ display: activeTab === 3 ? 'block' : 'none' }}>
        <SecuritySettingsTab
          onSave={handleSecuritySave}
          onSuccess={(message) => showNotification(message)}
          onError={(message) => showNotification(message, 'error')}
        />
      </Box>

      <Box sx={{ display: activeTab === 4 ? 'block' : 'none' }}>
        <NotificationSettingsTab
          onSave={handleNotificationSave}
          onSuccess={(message) => showNotification(message)}
          onError={(message) => showNotification(message, 'error')}
        />
      </Box>

      <Box sx={{ display: activeTab === 5 ? 'block' : 'none' }}>
        <BackupSettingsTab
          onSave={handleBackupSave}
          onSuccess={(message) => showNotification(message)}
          onError={(message) => showNotification(message, 'error')}
        />
      </Box>

      <Snackbar
        open={snackbar.open}
        autoHideDuration={6000}
        onClose={handleCloseSnackbar}
        anchorOrigin={{ vertical: 'bottom', horizontal: 'right' }}
      >
        <Alert 
          onClose={handleCloseSnackbar} 
          severity={snackbar.severity}
          sx={{ width: '100%' }}
        >
          {snackbar.message}
        </Alert>
      </Snackbar>
    </Box>
  );
};

export default ModernSettings; 
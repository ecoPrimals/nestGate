import React, { useState } from 'react';
import { 
  Typography, 
  Tabs, 
  Tab,
  TextField,
  Button, 
  Select, 
  MenuItem,
  FormControl,
  InputLabel,
  Switch, 
  FormControlLabel,
  Card, 
  CardContent,
  CardHeader,
  Divider, 
  Box,
  Stack,
  Snackbar,
  Alert,
  InputAdornment,
  Checkbox,
  FormGroup,
  Tooltip,
  CircularProgress,
} from '@mui/material';
import {
  Save as SaveIcon,
  Refresh as RefreshIcon,
  Help as HelpIcon,
  Upload as UploadIcon,
  Schedule as ScheduleIcon,
  Lock as LockIcon,
  Person as UserIcon,
  Email as MailIcon,
  CloudSync as CloudSyncIcon,
  Notifications as BellIcon,
  Settings as SettingIcon,
} from '@mui/icons-material';
import dayjs from 'dayjs';
import TabPanel from '../components/common/TabPanel';

interface GeneralFormData {
  systemName: string;
  timezone: string;
  language: string;
  autoUpdate: boolean;
  updateSchedule: string;
}

interface NetworkFormData {
  hostname: string;
  ipConfiguration: string;
  ipAddress: string;
  subnetMask: string;
  gateway: string;
  dns: string;
  enableSsh: boolean;
  sshPort: number;
  enableHttps: boolean;
  httpPort: number;
  httpsPort: number;
}

interface BackupFormData {
  enableAutoSnapshots: boolean;
  snapshotFrequency: string;
  snapshotTime: string;
  snapshotRetention: number;
  snapshotDatasets: string[];
  enableReplication: boolean;
  replicationType: string;
  replicationTarget: string;
  replicationSchedule: string;
}

interface UserFormData {
  adminEmail: string;
  currentPassword: string;
  newPassword: string;
  confirmPassword: string;
  enableNotifications: boolean;
  notificationTypes: string[];
}

const SettingsNew: React.FC = () => {
  const [activeTab, setActiveTab] = useState(0);
  const [isLoading, setIsLoading] = useState(false);
  
  const [snackbar, setSnackbar] = useState<{
    open: boolean;
    message: string;
    severity: 'success' | 'error';
  }>({
    open: false,
    message: '',
    severity: 'success',
  });

  // Form state for each tab
  const [generalForm, setGeneralForm] = useState<GeneralFormData>({
    systemName: 'NestGate-01',
    timezone: 'UTC',
    language: 'en-US',
    autoUpdate: true,
    updateSchedule: 'weekly',
  });

  const [networkForm, setNetworkForm] = useState<NetworkFormData>({
    hostname: 'nestgate.local',
    ipConfiguration: 'dhcp',
    ipAddress: '',
    subnetMask: '',
    gateway: '',
    dns: '',
    enableSsh: true,
    sshPort: 22,
    enableHttps: true,
    httpPort: 80,
    httpsPort: 443,
  });

  const [backupForm, setBackupForm] = useState<BackupFormData>({
    enableAutoSnapshots: true,
    snapshotFrequency: 'daily',
    snapshotTime: '00:00',
    snapshotRetention: 30,
    snapshotDatasets: [],
    enableReplication: false,
    replicationType: 'push',
    replicationTarget: '',
    replicationSchedule: 'daily',
  });

  const [userForm, setUserForm] = useState<UserFormData>({
    adminEmail: 'admin@example.com',
    currentPassword: '',
    newPassword: '',
    confirmPassword: '',
    enableNotifications: true,
    notificationTypes: ['system', 'storage', 'security'],
  });

  // Form errors state
  const [generalErrors, setGeneralErrors] = useState<Record<string, string>>({});
  const [networkErrors, setNetworkErrors] = useState<Record<string, string>>({});
  const [backupErrors, setBackupErrors] = useState<Record<string, string>>({});
  const [userErrors, setUserErrors] = useState<Record<string, string>>({});

  const showNotification = (message: string, severity: 'success' | 'error' = 'success') => {
    setSnackbar({ open: true, message, severity });
  };

  const handleCloseSnackbar = () => {
    setSnackbar(prev => ({ ...prev, open: false }));
  };

  const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
    setActiveTab(newValue);
  };
  
  // Handle form submission
  const handleSubmit = async (formName: string, formData: any, validator: () => boolean) => {
    if (!validator()) {
      return;
    }

    setIsLoading(true);
    
    try {
      // Simulate API call - in Tauri, this would be a Tauri command
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      console.log(`${formName} form submitted:`, formData);
      
      showNotification(`${formName} settings have been updated successfully.`);
    } catch (error) {
      showNotification(`Failed to update ${formName} settings.`, 'error');
    } finally {
      setIsLoading(false);
    }
  };

  // Validation functions
  const validateGeneral = (): boolean => {
    const errors: Record<string, string> = {};
    
    if (!generalForm.systemName.trim()) {
      errors.systemName = 'Please enter a system name';
    }
    
    if (!generalForm.timezone) {
      errors.timezone = 'Please select a timezone';
    }
    
    if (!generalForm.language) {
      errors.language = 'Please select a language';
    }
    
    if (generalForm.autoUpdate && !generalForm.updateSchedule) {
      errors.updateSchedule = 'Please select an update schedule';
    }

    setGeneralErrors(errors);
    return Object.keys(errors).length === 0;
  };

  const validateNetwork = (): boolean => {
    const errors: Record<string, string> = {};
    
    if (!networkForm.hostname.trim()) {
      errors.hostname = 'Please enter a hostname';
    }
    
    if (!networkForm.ipConfiguration) {
      errors.ipConfiguration = 'Please select IP configuration';
    }
    
    if (networkForm.ipConfiguration === 'static') {
      if (!networkForm.ipAddress.trim()) {
        errors.ipAddress = 'Please enter an IP address';
      }
      if (!networkForm.subnetMask.trim()) {
        errors.subnetMask = 'Please enter a subnet mask';
      }
      if (!networkForm.gateway.trim()) {
        errors.gateway = 'Please enter a gateway';
      }
      if (!networkForm.dns.trim()) {
        errors.dns = 'Please enter DNS servers';
      }
    }

    setNetworkErrors(errors);
    return Object.keys(errors).length === 0;
  };

  const validateBackup = (): boolean => {
    const errors: Record<string, string> = {};
    
    if (backupForm.enableAutoSnapshots) {
      if (!backupForm.snapshotFrequency) {
        errors.snapshotFrequency = 'Please select a snapshot frequency';
      }
      if (!backupForm.snapshotTime) {
        errors.snapshotTime = 'Please select a snapshot time';
      }
      if (!backupForm.snapshotRetention) {
        errors.snapshotRetention = 'Please enter retention period';
      }
      if (!backupForm.snapshotDatasets.length) {
        errors.snapshotDatasets = 'Please select at least one dataset';
      }
    }
    
    if (backupForm.enableReplication) {
      if (!backupForm.replicationType) {
        errors.replicationType = 'Please select a replication type';
      }
      if (!backupForm.replicationTarget.trim()) {
        errors.replicationTarget = 'Please enter a replication target';
      }
      if (!backupForm.replicationSchedule) {
        errors.replicationSchedule = 'Please select a schedule';
      }
    }

    setBackupErrors(errors);
    return Object.keys(errors).length === 0;
  };

  const validateUser = (): boolean => {
    const errors: Record<string, string> = {};
    
    if (!userForm.adminEmail.trim()) {
      errors.adminEmail = 'Please enter an email address';
    } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(userForm.adminEmail)) {
      errors.adminEmail = 'Please enter a valid email address';
    }
    
    if (userForm.newPassword && userForm.newPassword !== userForm.confirmPassword) {
      errors.confirmPassword = 'Passwords do not match';
    }
    
    if (userForm.enableNotifications && !userForm.notificationTypes.length) {
      errors.notificationTypes = 'Please select at least one notification type';
    }

    setUserErrors(errors);
    return Object.keys(errors).length === 0;
  };

  // Form change handlers
  const handleGeneralChange = (field: keyof GeneralFormData, value: any) => {
    setGeneralForm(prev => ({ ...prev, [field]: value }));
    if (generalErrors[field]) {
      setGeneralErrors(prev => ({ ...prev, [field]: '' }));
    }
  };

  const handleNetworkChange = (field: keyof NetworkFormData, value: any) => {
    setNetworkForm(prev => ({ ...prev, [field]: value }));
    if (networkErrors[field]) {
      setNetworkErrors(prev => ({ ...prev, [field]: '' }));
    }
  };

  const handleBackupChange = (field: keyof BackupFormData, value: any) => {
    setBackupForm(prev => ({ ...prev, [field]: value }));
    if (backupErrors[field]) {
      setBackupErrors(prev => ({ ...prev, [field]: '' }));
    }
  };

  const handleUserChange = (field: keyof UserFormData, value: any) => {
    setUserForm(prev => ({ ...prev, [field]: value }));
    if (userErrors[field]) {
      setUserErrors(prev => ({ ...prev, [field]: '' }));
    }
  };

  return (
    <Box sx={{ padding: 3 }}>
      <Typography variant="h4" component="h1" gutterBottom>
        System Settings (New)
      </Typography>
      <Typography variant="body1" color="text.secondary" paragraph>
        Configure system-wide settings for your NestGate storage system.
      </Typography>
      
      <Box sx={{ borderBottom: 1, borderColor: 'divider', marginBottom: 3 }}>
        <Tabs value={activeTab} onChange={handleTabChange} aria-label="settings tabs">
          <Tab icon={<SettingIcon />} label="General" iconPosition="start" />
          <Tab icon={<CloudSyncIcon />} label="Network" iconPosition="start" />
          <Tab icon={<CloudSyncIcon />} label="Backup & Replication" iconPosition="start" />
          <Tab icon={<UserIcon />} label="User Management" iconPosition="start" />
        </Tabs>
      </Box>

      {/* General Settings Tab */}
      <TabPanel value={activeTab} index={0}>
          <Card>
          <CardHeader title="General Settings" />
          <CardContent>
            <Stack>
              <Typography variant="h6">System Configuration</Typography>
              
              <TextField
                label="System Name"
                value={generalForm.systemName}
                onChange={(e) => handleGeneralChange('systemName', e.target.value)}
                error={!!generalErrors.systemName}
                helperText={generalErrors.systemName}
                placeholder="Enter system name"
                required
                fullWidth
              />
              
              <FormControl fullWidth required>
                <InputLabel>Timezone</InputLabel>
                <Select
                  value={generalForm.timezone}
                  onChange={(e) => handleGeneralChange('timezone', e.target.value)}
                  label="Timezone"
                  error={!!generalErrors.timezone}
                >
                  <MenuItem value="UTC">UTC</MenuItem>
                  <MenuItem value="America/New_York">Eastern Time (ET)</MenuItem>
                  <MenuItem value="America/Chicago">Central Time (CT)</MenuItem>
                  <MenuItem value="America/Denver">Mountain Time (MT)</MenuItem>
                  <MenuItem value="America/Los_Angeles">Pacific Time (PT)</MenuItem>
                  <MenuItem value="Europe/London">London</MenuItem>
                  <MenuItem value="Europe/Paris">Paris</MenuItem>
                  <MenuItem value="Asia/Tokyo">Tokyo</MenuItem>
                </Select>
                {generalErrors.timezone && (
                  <Typography variant="caption" color="error" sx={{ ml: 1.75, mt: 0.5 }}>
                    {generalErrors.timezone}
                  </Typography>
                )}
              </FormControl>
              
              <FormControl fullWidth required>
                <InputLabel>Language</InputLabel>
                <Select
                  value={generalForm.language}
                  onChange={(e) => handleGeneralChange('language', e.target.value)}
                  label="Language"
                  error={!!generalErrors.language}
                >
                  <MenuItem value="en-US">English (US)</MenuItem>
                  <MenuItem value="es-ES">Spanish</MenuItem>
                  <MenuItem value="fr-FR">French</MenuItem>
                  <MenuItem value="de-DE">German</MenuItem>
                  <MenuItem value="ja-JP">Japanese</MenuItem>
                </Select>
                {generalErrors.language && (
                  <Typography variant="caption" color="error" sx={{ ml: 1.75, mt: 0.5 }}>
                    {generalErrors.language}
                  </Typography>
                )}
              </FormControl>
              
              <Divider />
              
              <Typography variant="h6">Updates</Typography>
              
              <FormControlLabel
                control={
                  <Switch
                    checked={generalForm.autoUpdate}
                    onChange={(e) => handleGeneralChange('autoUpdate', e.target.checked)}
                  />
                }
                label="Automatic Updates"
              />
              
              {generalForm.autoUpdate && (
                <FormControl fullWidth>
                  <InputLabel>Update Schedule</InputLabel>
                  <Select
                    value={generalForm.updateSchedule}
                    onChange={(e) => handleGeneralChange('updateSchedule', e.target.value)}
                    label="Update Schedule"
                    error={!!generalErrors.updateSchedule}
                  >
                    <MenuItem value="daily">Daily</MenuItem>
                    <MenuItem value="weekly">Weekly</MenuItem>
                    <MenuItem value="monthly">Monthly</MenuItem>
                </Select>
                  {generalErrors.updateSchedule && (
                    <Typography variant="caption" color="error" sx={{ ml: 1.75, mt: 0.5 }}>
                      {generalErrors.updateSchedule}
                    </Typography>
                  )}
                </FormControl>
              )}
              
              <Stack direction="row">
                <Button
                  variant="contained"
                  startIcon={isLoading ? <CircularProgress size={20} /> : <SaveIcon />}
                  onClick={() => handleSubmit('General', generalForm, validateGeneral)}
                  disabled={isLoading}
                >
                    Save Changes
                  </Button>
                <Button
                  variant="outlined"
                  startIcon={<RefreshIcon />}
                  onClick={() => setGeneralForm({
                    systemName: 'NestGate-01',
                    timezone: 'UTC',
                    language: 'en-US',
                    autoUpdate: true,
                    updateSchedule: 'weekly',
                  })}
                  disabled={isLoading}
                >
                    Reset
                  </Button>
              </Stack>
            </Stack>
          </CardContent>
          </Card>
      </TabPanel>

      {/* Simplified tabs for other settings - adding basic structure */}
      <TabPanel value={activeTab} index={1}>
          <Card>
          <CardHeader title="Network Configuration" />
          <CardContent>
            <Typography variant="body1" color="text.secondary">
              Network settings would be configured here. This is a simplified version of the SettingsNew component.
            </Typography>
          </CardContent>
        </Card>
      </TabPanel>

      <TabPanel value={activeTab} index={2}>
        <Card>
          <CardHeader title="Backup & Replication" />
          <CardContent>
            <Typography variant="body1" color="text.secondary">
              Backup and replication settings would be configured here.
            </Typography>
          </CardContent>
        </Card>
      </TabPanel>

      <TabPanel value={activeTab} index={3}>
        <Card>
          <CardHeader title="User Management" />
          <CardContent>
            <Typography variant="body1" color="text.secondary">
              User management settings would be configured here.
            </Typography>
          </CardContent>
        </Card>
      </TabPanel>

      {/* Snackbar for notifications */}
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

export default SettingsNew; 
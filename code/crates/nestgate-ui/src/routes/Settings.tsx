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

const Settings: React.FC = () => {
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
    
    if (networkForm.enableSsh && !networkForm.sshPort) {
      errors.sshPort = 'Please enter SSH port';
    }
    
    if (networkForm.enableHttps && !networkForm.httpsPort) {
      errors.httpsPort = 'Please enter HTTPS port';
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

  // Reset form handlers
  const resetGeneralForm = () => {
    setGeneralForm({
                systemName: 'NestGate-01',
                timezone: 'UTC',
                language: 'en-US',
                autoUpdate: true,
      updateSchedule: 'weekly',
    });
    setGeneralErrors({});
  };

  const resetNetworkForm = () => {
    setNetworkForm({
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
    setNetworkErrors({});
  };

  const resetBackupForm = () => {
    setBackupForm({
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
    setBackupErrors({});
  };

  const resetUserForm = () => {
    setUserForm({
      adminEmail: 'admin@example.com',
      currentPassword: '',
      newPassword: '',
      confirmPassword: '',
      enableNotifications: true,
      notificationTypes: ['system', 'storage', 'security'],
    });
    setUserErrors({});
  };

  return (
    <Box sx={{ padding: 3 }}>
      <Typography variant="h4" component="h1" gutterBottom>
        System Settings
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
                  onClick={resetGeneralForm}
                  disabled={isLoading}
                >
                    Reset
                  </Button>
              </Stack>
            </Stack>
          </CardContent>
          </Card>
      </TabPanel>

      {/* Network Settings Tab */}
      <TabPanel value={activeTab} index={1}>
          <Card>
          <CardHeader title="Network Configuration" />
          <CardContent>
            <Stack>
              <Typography variant="h6">Network Configuration</Typography>
              
              <TextField
                label="Hostname"
                value={networkForm.hostname}
                onChange={(e) => handleNetworkChange('hostname', e.target.value)}
                error={!!networkErrors.hostname}
                helperText={networkErrors.hostname}
                placeholder="Enter hostname"
                required
                fullWidth
              />
              
              <FormControl fullWidth required>
                <InputLabel>IP Configuration</InputLabel>
                <Select
                  value={networkForm.ipConfiguration}
                  onChange={(e) => handleNetworkChange('ipConfiguration', e.target.value)}
                label="IP Configuration"
                  error={!!networkErrors.ipConfiguration}
              >
                  <MenuItem value="dhcp">DHCP</MenuItem>
                  <MenuItem value="static">Static IP</MenuItem>
                </Select>
                {networkErrors.ipConfiguration && (
                  <Typography variant="caption" color="error" sx={{ ml: 1.75, mt: 0.5 }}>
                    {networkErrors.ipConfiguration}
                  </Typography>
                )}
              </FormControl>
              
              {networkForm.ipConfiguration === 'static' && (
                <Stack>
                  <TextField
                        label="IP Address"
                    value={networkForm.ipAddress}
                    onChange={(e) => handleNetworkChange('ipAddress', e.target.value)}
                    error={!!networkErrors.ipAddress}
                    helperText={networkErrors.ipAddress}
                    placeholder="192.168.1.100"
                    required
                    fullWidth
                  />
                  
                  <TextField
                        label="Subnet Mask"
                    value={networkForm.subnetMask}
                    onChange={(e) => handleNetworkChange('subnetMask', e.target.value)}
                    error={!!networkErrors.subnetMask}
                    helperText={networkErrors.subnetMask}
                    placeholder="255.255.255.0"
                    required
                    fullWidth
                  />
                  
                  <TextField
                        label="Gateway"
                    value={networkForm.gateway}
                    onChange={(e) => handleNetworkChange('gateway', e.target.value)}
                    error={!!networkErrors.gateway}
                    helperText={networkErrors.gateway}
                    placeholder="192.168.1.1"
                    required
                    fullWidth
                  />
                  
                  <TextField
                        label="DNS Servers"
                    value={networkForm.dns}
                    onChange={(e) => handleNetworkChange('dns', e.target.value)}
                    error={!!networkErrors.dns}
                    helperText={networkErrors.dns}
                    placeholder="8.8.8.8, 8.8.4.4"
                    required
                    fullWidth
                  />
                </Stack>
              )}
              
              <Divider />
              
              <Typography variant="h6">Services</Typography>
              
              <FormControlLabel
                control={
                  <Switch
                    checked={networkForm.enableSsh}
                    onChange={(e) => handleNetworkChange('enableSsh', e.target.checked)}
                  />
                }
                label="Enable SSH"
              />
              
              {networkForm.enableSsh && (
                <TextField
                label="SSH Port"
                  type="number"
                  value={networkForm.sshPort}
                  onChange={(e) => handleNetworkChange('sshPort', parseInt(e.target.value) || 22)}
                  error={!!networkErrors.sshPort}
                  helperText={networkErrors.sshPort}
                  inputProps={{ min: 1, max: 65535 }}
                  fullWidth
                />
              )}
              
              <FormControlLabel
                control={
                  <Switch
                    checked={networkForm.enableHttps}
                    onChange={(e) => handleNetworkChange('enableHttps', e.target.checked)}
                  />
                }
                label="Enable HTTPS"
              />
              
              <TextField
                label="HTTP Port"
                type="number"
                value={networkForm.httpPort}
                onChange={(e) => handleNetworkChange('httpPort', parseInt(e.target.value) || 80)}
                inputProps={{ min: 1, max: 65535 }}
                fullWidth
              />
              
              {networkForm.enableHttps && (
                <TextField
                label="HTTPS Port"
                  type="number"
                  value={networkForm.httpsPort}
                  onChange={(e) => handleNetworkChange('httpsPort', parseInt(e.target.value) || 443)}
                  error={!!networkErrors.httpsPort}
                  helperText={networkErrors.httpsPort}
                  inputProps={{ min: 1, max: 65535 }}
                  fullWidth
                />
              )}
              
              <Stack direction="row">
                <Button
                  variant="contained"
                  startIcon={isLoading ? <CircularProgress size={20} /> : <SaveIcon />}
                  onClick={() => handleSubmit('Network', networkForm, validateNetwork)}
                  disabled={isLoading}
                >
                    Save Changes
                  </Button>
                <Button
                  variant="outlined"
                  startIcon={<RefreshIcon />}
                  onClick={resetNetworkForm}
                  disabled={isLoading}
                >
                    Reset
                  </Button>
              </Stack>
            </Stack>
          </CardContent>
          </Card>
      </TabPanel>

      {/* Backup & Replication Tab */}
      <TabPanel value={activeTab} index={2}>
          <Card>
          <CardHeader title="Backup & Replication" />
          <CardContent>
            <Stack>
              <Typography variant="h6">Automated Snapshots</Typography>
              
              <FormControlLabel
                control={
                  <Switch
                    checked={backupForm.enableAutoSnapshots}
                    onChange={(e) => handleBackupChange('enableAutoSnapshots', e.target.checked)}
                  />
                }
                label="Enable Automated Snapshots"
              />
              
              {backupForm.enableAutoSnapshots && (
                <Stack>
                  <FormControl fullWidth>
                    <InputLabel>Snapshot Frequency</InputLabel>
                    <Select
                      value={backupForm.snapshotFrequency}
                      onChange={(e) => handleBackupChange('snapshotFrequency', e.target.value)}
                label="Snapshot Frequency"
                      error={!!backupErrors.snapshotFrequency}
                    >
                      <MenuItem value="hourly">Hourly</MenuItem>
                      <MenuItem value="daily">Daily</MenuItem>
                      <MenuItem value="weekly">Weekly</MenuItem>
                      <MenuItem value="monthly">Monthly</MenuItem>
                </Select>
                    {backupErrors.snapshotFrequency && (
                      <Typography variant="caption" color="error" sx={{ ml: 1.75, mt: 0.5 }}>
                        {backupErrors.snapshotFrequency}
                      </Typography>
                    )}
                  </FormControl>
                  
                  <TextField
                label="Snapshot Time"
                    type="time"
                    value={backupForm.snapshotTime}
                    onChange={(e) => handleBackupChange('snapshotTime', e.target.value)}
                    error={!!backupErrors.snapshotTime}
                    helperText={backupErrors.snapshotTime}
                    InputLabelProps={{
                      shrink: true,
                    }}
                    fullWidth
                  />
                  
                  <TextField
                label="Snapshot Retention (days)"
                    type="number"
                    value={backupForm.snapshotRetention}
                    onChange={(e) => handleBackupChange('snapshotRetention', parseInt(e.target.value) || 30)}
                    error={!!backupErrors.snapshotRetention}
                    helperText={backupErrors.snapshotRetention}
                    inputProps={{ min: 1, max: 365 }}
                    fullWidth
                  />
                  
                  <FormControl fullWidth>
                    <InputLabel>Datasets to Snapshot</InputLabel>
                    <Select
                      multiple
                      value={backupForm.snapshotDatasets}
                      onChange={(e) => handleBackupChange('snapshotDatasets', e.target.value)}
                label="Datasets to Snapshot"
                      error={!!backupErrors.snapshotDatasets}
                    >
                      <MenuItem value="tank/data">tank/data</MenuItem>
                      <MenuItem value="tank/apps">tank/apps</MenuItem>
                      <MenuItem value="tank/home">tank/home</MenuItem>
                </Select>
                    {backupErrors.snapshotDatasets && (
                      <Typography variant="caption" color="error" sx={{ ml: 1.75, mt: 0.5 }}>
                        {backupErrors.snapshotDatasets}
                      </Typography>
                    )}
                  </FormControl>
                </Stack>
              )}
              
              <Divider />
              
              <Typography variant="h6">Replication</Typography>
              
              <FormControlLabel
                control={
                  <Switch
                    checked={backupForm.enableReplication}
                    onChange={(e) => handleBackupChange('enableReplication', e.target.checked)}
                  />
                }
                label="Enable Replication"
              />
              
              {backupForm.enableReplication && (
                <Stack>
                  <FormControl fullWidth>
                    <InputLabel>Replication Type</InputLabel>
                    <Select
                      value={backupForm.replicationType}
                      onChange={(e) => handleBackupChange('replicationType', e.target.value)}
                label="Replication Type"
                      error={!!backupErrors.replicationType}
                    >
                      <MenuItem value="push">Push (to remote)</MenuItem>
                      <MenuItem value="pull">Pull (from remote)</MenuItem>
                </Select>
                    {backupErrors.replicationType && (
                      <Typography variant="caption" color="error" sx={{ ml: 1.75, mt: 0.5 }}>
                        {backupErrors.replicationType}
                      </Typography>
                    )}
                  </FormControl>
                  
                  <TextField
                label="Replication Target"
                    value={backupForm.replicationTarget}
                    onChange={(e) => handleBackupChange('replicationTarget', e.target.value)}
                    error={!!backupErrors.replicationTarget}
                    helperText={backupErrors.replicationTarget}
                    placeholder="hostname or IP address"
                    required
                    fullWidth
                  />
                  
                  <FormControl fullWidth>
                    <InputLabel>Replication Schedule</InputLabel>
                    <Select
                      value={backupForm.replicationSchedule}
                      onChange={(e) => handleBackupChange('replicationSchedule', e.target.value)}
                label="Replication Schedule"
                      error={!!backupErrors.replicationSchedule}
                    >
                      <MenuItem value="hourly">Hourly</MenuItem>
                      <MenuItem value="daily">Daily</MenuItem>
                      <MenuItem value="weekly">Weekly</MenuItem>
                      <MenuItem value="monthly">Monthly</MenuItem>
                </Select>
                    {backupErrors.replicationSchedule && (
                      <Typography variant="caption" color="error" sx={{ ml: 1.75, mt: 0.5 }}>
                        {backupErrors.replicationSchedule}
                      </Typography>
                    )}
                  </FormControl>
                </Stack>
              )}
              
              <Stack direction="row">
                <Button
                  variant="contained"
                  startIcon={isLoading ? <CircularProgress size={20} /> : <SaveIcon />}
                  onClick={() => handleSubmit('Backup', backupForm, validateBackup)}
                  disabled={isLoading}
                >
                    Save Changes
                  </Button>
                <Button
                  variant="outlined"
                  startIcon={<RefreshIcon />}
                  onClick={resetBackupForm}
                  disabled={isLoading}
                >
                    Reset
                  </Button>
              </Stack>
            </Stack>
          </CardContent>
          </Card>
      </TabPanel>

      {/* User Management Tab */}
      <TabPanel value={activeTab} index={3}>
          <Card>
          <CardHeader title="User Management" />
          <CardContent>
            <Stack>
              <Typography variant="h6">Admin Account</Typography>
              
              <TextField
                label="Email Address"
                type="email"
                value={userForm.adminEmail}
                onChange={(e) => handleUserChange('adminEmail', e.target.value)}
                error={!!userErrors.adminEmail}
                helperText={userErrors.adminEmail}
                InputProps={{
                  startAdornment: (
                    <InputAdornment position="start">
                      <MailIcon />
                    </InputAdornment>
                  ),
                }}
                placeholder="Email address"
                required
                fullWidth
              />
              
              <TextField
                label="Current Password"
                type="password"
                value={userForm.currentPassword}
                onChange={(e) => handleUserChange('currentPassword', e.target.value)}
                InputProps={{
                  startAdornment: (
                    <InputAdornment position="start">
                      <LockIcon />
                    </InputAdornment>
                  ),
                }}
                placeholder="Current password"
                fullWidth
              />
              
              <TextField
                label="New Password"
                type="password"
                value={userForm.newPassword}
                onChange={(e) => handleUserChange('newPassword', e.target.value)}
                InputProps={{
                  startAdornment: (
                    <InputAdornment position="start">
                      <LockIcon />
                    </InputAdornment>
                  ),
                }}
                placeholder="New password"
                fullWidth
              />
              
              <TextField
                label="Confirm Password"
                type="password"
                value={userForm.confirmPassword}
                onChange={(e) => handleUserChange('confirmPassword', e.target.value)}
                error={!!userErrors.confirmPassword}
                helperText={userErrors.confirmPassword}
                InputProps={{
                  startAdornment: (
                    <InputAdornment position="start">
                      <LockIcon />
                    </InputAdornment>
                  ),
                }}
                placeholder="Confirm password"
                fullWidth
              />
              
              <Divider />
              
              <Typography variant="h6">Notifications</Typography>
              
              <FormControlLabel
                control={
                  <Switch
                    checked={userForm.enableNotifications}
                    onChange={(e) => handleUserChange('enableNotifications', e.target.checked)}
                  />
                }
                label="Enable Email Notifications"
              />
              
              {userForm.enableNotifications && (
                <FormGroup>
                  <Typography variant="body2" gutterBottom>
                    Notification Types
                  </Typography>
                  {[
                    { value: 'system', label: 'System Alerts' },
                    { value: 'storage', label: 'Storage Alerts' },
                    { value: 'security', label: 'Security Alerts' },
                    { value: 'updates', label: 'Update Notifications' },
                    { value: 'replication', label: 'Replication Status' },
                  ].map((option) => (
                    <FormControlLabel
                      key={option.value}
                      control={
                        <Checkbox
                          checked={userForm.notificationTypes.includes(option.value)}
                          onChange={(e) => {
                            const newTypes = e.target.checked
                              ? [...userForm.notificationTypes, option.value]
                              : userForm.notificationTypes.filter(type => type !== option.value);
                            handleUserChange('notificationTypes', newTypes);
                          }}
                        />
                      }
                      label={option.label}
                    />
                  ))}
                  {userErrors.notificationTypes && (
                    <Typography variant="caption" color="error">
                      {userErrors.notificationTypes}
                    </Typography>
                  )}
                </FormGroup>
              )}
              
              <Stack direction="row">
                <Button
                  variant="contained"
                  startIcon={isLoading ? <CircularProgress size={20} /> : <SaveIcon />}
                  onClick={() => handleSubmit('User', userForm, validateUser)}
                  disabled={isLoading}
                >
                    Save Changes
                  </Button>
                <Button
                  variant="outlined"
                  startIcon={<RefreshIcon />}
                  onClick={resetUserForm}
                  disabled={isLoading}
                >
                    Reset
                  </Button>
              </Stack>
            </Stack>
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

export default Settings; 
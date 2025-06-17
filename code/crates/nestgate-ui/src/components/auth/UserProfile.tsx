import React, { useState, useContext } from 'react';
import {
  Card,
  CardContent,
  CardHeader,
  Avatar,
  TextField,
  Button,
  Tabs,
  Tab,
  Typography,
  Switch,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Divider,
  Alert,
  Box,
  Stack,
  CircularProgress,
  FormControlLabel,
  LinearProgress,
  Snackbar,
  IconButton,
  InputAdornment,
} from '@mui/material';
import {
  Person as UserIcon,
  Email as MailIcon,
  Lock as LockIcon,
  Save as SaveIcon,
  Visibility,
  VisibilityOff,
  Settings as SettingsIcon,
  Security as SecurityIcon,
  Sync as SyncIcon
} from '@mui/icons-material';
import { AuthContext } from '../../contexts/AuthContext';
import { UserPreferences } from '../../services/auth.service';
import TabPanel from '../common/TabPanel';

const UserProfile: React.FC = () => {
  const {
    user,
    updateProfile,
    changePassword,
    validatePassword,
    checkPasswordStrength,
    isLoading
  } = useContext(AuthContext);

  const [activeTab, setActiveTab] = useState(0);
  const [profileSubmitting, setProfileSubmitting] = useState(false);
  const [passwordSubmitting, setPasswordSubmitting] = useState(false);
  const [passwordFeedback, setPasswordFeedback] = useState<{ score: number; feedback: string } | null>(null);
  const [passwordErrors, setPasswordErrors] = useState<string[]>([]);
  const [showPassword, setShowPassword] = useState(false);
  const [showNewPassword, setShowNewPassword] = useState(false);
  const [showConfirmPassword, setShowConfirmPassword] = useState(false);
  
  // Form state
  const [profileData, setProfileData] = useState({
    email: user?.email || '',
    theme: user?.preferences?.theme || 'system',
    notifications: user?.preferences?.notifications !== false,
    language: user?.preferences?.language || 'en',
    dashboardLayout: user?.preferences?.dashboardLayout || 'default'
  });

  const [passwordData, setPasswordData] = useState({
    currentPassword: '',
    newPassword: '',
    confirmPassword: ''
  });

  const [snackbar, setSnackbar] = useState<{
    open: boolean;
    message: string;
    severity: 'success' | 'error';
  }>({
    open: false,
    message: '',
    severity: 'success',
  });

  if (!user) {
    return (
      <Card>
        <CardContent>
          <Box sx={{ textAlign: 'center', padding: '40px 0' }}>
            <Typography color="text.secondary">You need to log in to view your profile</Typography>
          </Box>
        </CardContent>
      </Card>
    );
  }

  const showNotification = (message: string, severity: 'success' | 'error' = 'success') => {
    setSnackbar({ open: true, message, severity });
  };

  const handleCloseSnackbar = () => {
    setSnackbar(prev => ({ ...prev, open: false }));
  };

  // Handle profile update
  const handleProfileUpdate = async () => {
    setProfileSubmitting(true);
    try {
      const success = await updateProfile({
        email: profileData.email,
        preferences: {
          theme: profileData.theme,
          notifications: profileData.notifications,
          language: profileData.language,
          dashboardLayout: profileData.dashboardLayout
        }
      });
      if (success) {
        showNotification('Profile updated successfully');
      } else {
        showNotification('Failed to update profile', 'error');
      }
    } catch (error) {
      showNotification('An error occurred while updating profile', 'error');
      console.error('Profile update error:', error);
    } finally {
      setProfileSubmitting(false);
    }
  };

  // Handle password change
  const handlePasswordChange = async () => {
    setPasswordSubmitting(true);
    try {
      // Validate password
      const validation = validatePassword(passwordData.newPassword);
      if (!validation.valid) {
        setPasswordErrors(validation.errors);
        return;
      }

      // Check passwords match
      if (passwordData.newPassword !== passwordData.confirmPassword) {
        setPasswordErrors(['New password and confirmation do not match']);
        return;
      }

      // Submit password change
      const success = await changePassword(passwordData.currentPassword, passwordData.newPassword);
      if (success) {
        showNotification('Password changed successfully');
        setPasswordData({ currentPassword: '', newPassword: '', confirmPassword: '' });
        setPasswordErrors([]);
        setPasswordFeedback(null);
      } else {
        showNotification('Failed to change password', 'error');
      }
    } catch (error: any) {
      showNotification(error.message || 'An error occurred while changing password', 'error');
      console.error('Password change error:', error);
    } finally {
      setPasswordSubmitting(false);
    }
  };

  // Check password strength on input
  const handlePasswordInput = (value: string) => {
    setPasswordData(prev => ({ ...prev, newPassword: value }));
    
    if (value) {
      const strength = checkPasswordStrength(value);
      setPasswordFeedback(strength);
      
      // Also validate against policy
      const validation = validatePassword(value);
      setPasswordErrors(validation.valid ? [] : validation.errors);
    } else {
      setPasswordFeedback(null);
      setPasswordErrors([]);
    }
  };

  // Render password strength indicator
  const renderPasswordStrength = () => {
    if (!passwordFeedback) return null;
    
    const { score, feedback } = passwordFeedback;
    
    // Calculate color based on score
    let color = 'error'; // Red for weak
    if (score >= 4) color = 'success'; // Green for strong
    else if (score >= 3) color = 'warning'; // Yellow/orange for medium
    
    return (
      <Box sx={{ marginTop: 1 }}>
        <Typography variant="body2" color="text.secondary">
          Strength: <Typography component="span" color={`${color}.main`}>{feedback}</Typography>
        </Typography>
        <LinearProgress 
          variant="determinate" 
          value={(score / 6) * 100} 
          color={color as any}
          sx={{ marginTop: 0.5, height: 4, borderRadius: 2 }}
        />
      </Box>
    );
  };

  const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
    setActiveTab(newValue);
  };

  return (
    <Box>
      {isLoading && <LinearProgress />}
      <Card>
        <CardHeader
          avatar={<UserIcon />}
          title="User Profile"
        />
        <CardContent>
          <Stack direction="row" spacing={2} sx={{ marginBottom: 3 }}>
            <Avatar sx={{ width: 64, height: 64 }}>
              <UserIcon />
            </Avatar>
            <Box>
              <Typography variant="h5">{user.username}</Typography>
              <Typography color="text.secondary">{user.role}</Typography>
            {user.lastLogin && (
                <Typography variant="body2" color="text.secondary">
                  Last login: {new Date(user.lastLogin).toLocaleString()}
                </Typography>
              )}
            </Box>
          </Stack>

          <Box sx={{ borderBottom: 1, borderColor: 'divider', marginBottom: 3 }}>
            <Tabs value={activeTab} onChange={handleTabChange}>
              <Tab icon={<UserIcon />} label="Profile" />
              <Tab icon={<LockIcon />} label="Security" />
              <Tab icon={<SettingsIcon />} label="Preferences" />
            </Tabs>
          </Box>

          <TabPanel value={activeTab} index={0}>
            <Stack spacing={3}>
              <Typography variant="h6">Profile Information</Typography>
              
              <TextField
                label="Email"
                type="email"
                value={profileData.email}
                onChange={(e) => setProfileData(prev => ({ ...prev, email: e.target.value }))}
                InputProps={{
                  startAdornment: (
                    <InputAdornment position="start">
                      <MailIcon />
                    </InputAdornment>
                  ),
                }}
                fullWidth
              />

              <Button
                variant="contained"
                startIcon={profileSubmitting ? <CircularProgress size={20} /> : <SaveIcon />}
                onClick={handleProfileUpdate}
                disabled={profileSubmitting}
              >
                Update Profile
              </Button>
            </Stack>
          </TabPanel>

          <TabPanel value={activeTab} index={1}>
            <Stack spacing={3}>
              <Typography variant="h6">Change Password</Typography>
              
              {passwordErrors.length > 0 && (
                <Alert severity="error">
                  <ul style={{ margin: 0, paddingLeft: 20 }}>
                    {passwordErrors.map((error, index) => (
                      <li key={index}>{error}</li>
                    ))}
                  </ul>
                </Alert>
              )}

              <TextField
                label="Current Password"
                type={showPassword ? 'text' : 'password'}
                value={passwordData.currentPassword}
                onChange={(e) => setPasswordData(prev => ({ ...prev, currentPassword: e.target.value }))}
                InputProps={{
                  startAdornment: (
                    <InputAdornment position="start">
                      <LockIcon />
                    </InputAdornment>
                  ),
                  endAdornment: (
                    <InputAdornment position="end">
                      <IconButton
                        onClick={() => setShowPassword(!showPassword)}
                        edge="end"
                      >
                        {showPassword ? <VisibilityOff /> : <Visibility />}
                      </IconButton>
                    </InputAdornment>
                  ),
                }}
                fullWidth
                required
              />

              <TextField
                label="New Password"
                type={showNewPassword ? 'text' : 'password'}
                value={passwordData.newPassword}
                onChange={(e) => handlePasswordInput(e.target.value)}
                InputProps={{
                  startAdornment: (
                    <InputAdornment position="start">
                      <LockIcon />
                    </InputAdornment>
                  ),
                  endAdornment: (
                    <InputAdornment position="end">
                      <IconButton
                        onClick={() => setShowNewPassword(!showNewPassword)}
                        edge="end"
                      >
                        {showNewPassword ? <VisibilityOff /> : <Visibility />}
                      </IconButton>
                    </InputAdornment>
                  ),
                }}
                fullWidth
                required
              />
              
              {renderPasswordStrength()}
              
              <TextField
                label="Confirm New Password"
                type={showConfirmPassword ? 'text' : 'password'}
                value={passwordData.confirmPassword}
                onChange={(e) => setPasswordData(prev => ({ ...prev, confirmPassword: e.target.value }))}
                InputProps={{
                  startAdornment: (
                    <InputAdornment position="start">
                      <LockIcon />
                    </InputAdornment>
                  ),
                  endAdornment: (
                    <InputAdornment position="end">
                      <IconButton
                        onClick={() => setShowConfirmPassword(!showConfirmPassword)}
                        edge="end"
                      >
                        {showConfirmPassword ? <VisibilityOff /> : <Visibility />}
                      </IconButton>
                    </InputAdornment>
                  ),
                }}
                fullWidth
                required
              />

                <Button 
                variant="contained"
                startIcon={passwordSubmitting ? <CircularProgress size={20} /> : <SecurityIcon />}
                onClick={handlePasswordChange}
                disabled={passwordSubmitting}
                >
                  Change Password
                </Button>
            </Stack>
          </TabPanel>

          <TabPanel value={activeTab} index={2}>
            <Stack spacing={3}>
              <Typography variant="h6">User Preferences</Typography>
              
              <FormControl fullWidth>
                <InputLabel>Theme</InputLabel>
                <Select
                  value={profileData.theme}
                  onChange={(e) => setProfileData(prev => ({ ...prev, theme: e.target.value }))}
                  label="Theme"
                >
                  <MenuItem value="light">Light</MenuItem>
                  <MenuItem value="dark">Dark</MenuItem>
                  <MenuItem value="system">System</MenuItem>
                </Select>
              </FormControl>

              <FormControl fullWidth>
                <InputLabel>Language</InputLabel>
                <Select
                  value={profileData.language}
                  onChange={(e) => setProfileData(prev => ({ ...prev, language: e.target.value }))}
                  label="Language"
                >
                  <MenuItem value="en">English</MenuItem>
                  <MenuItem value="es">Spanish</MenuItem>
                  <MenuItem value="fr">French</MenuItem>
                  <MenuItem value="de">German</MenuItem>
                </Select>
              </FormControl>

              <FormControl fullWidth>
                <InputLabel>Dashboard Layout</InputLabel>
                <Select
                  value={profileData.dashboardLayout}
                  onChange={(e) => setProfileData(prev => ({ ...prev, dashboardLayout: e.target.value }))}
                  label="Dashboard Layout"
                >
                  <MenuItem value="default">Default</MenuItem>
                  <MenuItem value="compact">Compact</MenuItem>
                  <MenuItem value="detailed">Detailed</MenuItem>
                </Select>
              </FormControl>

              <FormControlLabel
                control={
                  <Switch
                    checked={profileData.notifications}
                    onChange={(e) => setProfileData(prev => ({ ...prev, notifications: e.target.checked }))}
                  />
                }
                label="Enable Notifications"
              />

              <Button
                variant="contained"
                startIcon={profileSubmitting ? <CircularProgress size={20} /> : <SaveIcon />}
                onClick={handleProfileUpdate}
                disabled={profileSubmitting}
              >
                Save Preferences
              </Button>
            </Stack>
          </TabPanel>
        </CardContent>
      </Card>

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

export default UserProfile; 
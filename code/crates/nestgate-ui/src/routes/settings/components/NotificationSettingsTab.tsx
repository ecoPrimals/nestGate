import React, { useState } from 'react';
import {
  Card,
  CardContent,
  TextField,
  Button,
  FormControlLabel,
  Switch,
  Box,
  CircularProgress,
} from '@mui/material';
import { Save as SaveIcon } from '@mui/icons-material';
import { NotificationSettings, SettingsFormProps } from '../types';
import { useSettingsForm } from '../hooks/useSettingsForm';

const NotificationSettingsTab: React.FC<SettingsFormProps<NotificationSettings> & {
  onSuccess?: (message: string) => void;
  onError?: (message: string) => void;
}> = ({ onSave, initialValues, onSuccess, onError }) => {
  const [formData, setFormData] = useState<NotificationSettings>({
    emailNotifications: initialValues?.emailNotifications || false,
    systemAlerts: initialValues?.systemAlerts || false,
    updateNotifications: initialValues?.updateNotifications || false,
    notificationEmail: initialValues?.notificationEmail || '',
  });

  const [errors, setErrors] = useState<{ notificationEmail?: string }>({});

  const { loading, handleSave } = useSettingsForm({
    sectionName: 'Notification',
    onSubmit: async (values) => onSave(values),
    onSuccess,
    onError,
  });

  const validate = (): boolean => {
    const newErrors: { notificationEmail?: string } = {};

    if (formData.notificationEmail && !/\S+@\S+\.\S+/.test(formData.notificationEmail)) {
      newErrors.notificationEmail = 'Please enter a valid email address';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = () => {
    if (validate()) {
      handleSave(formData);
    }
  };

  const handleSwitchChange = (field: keyof NotificationSettings, checked: boolean) => {
    setFormData(prev => ({ ...prev, [field]: checked }));
  };

  const handleEmailChange = (value: string) => {
    setFormData(prev => ({ ...prev, notificationEmail: value }));
    if (errors.notificationEmail) {
      setErrors(prev => ({ ...prev, notificationEmail: undefined }));
    }
  };

  return (
    <Card>
      <CardContent>
        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 3 }}>
          <FormControlLabel
            control={
              <Switch
                checked={formData.emailNotifications}
                onChange={(e) => handleSwitchChange('emailNotifications', e.target.checked)}
              />
            }
            label="Email Notifications"
          />

          <FormControlLabel
            control={
              <Switch
                checked={formData.systemAlerts}
                onChange={(e) => handleSwitchChange('systemAlerts', e.target.checked)}
              />
            }
            label="System Alerts"
          />

          <FormControlLabel
            control={
              <Switch
                checked={formData.updateNotifications}
                onChange={(e) => handleSwitchChange('updateNotifications', e.target.checked)}
              />
            }
            label="Update Notifications"
          />

          <TextField
            label="Notification Email"
            type="email"
            value={formData.notificationEmail}
            onChange={(e) => handleEmailChange(e.target.value)}
            placeholder="admin@example.com"
            error={!!errors.notificationEmail}
            helperText={errors.notificationEmail}
            fullWidth
          />

          <Box>
            <Button
              variant="contained"
              startIcon={loading ? <CircularProgress size={20} /> : <SaveIcon />}
              onClick={handleSubmit}
              disabled={loading}
            >
              Save Notification Settings
            </Button>
          </Box>
        </Box>
      </CardContent>
    </Card>
  );
};

export default NotificationSettingsTab; 
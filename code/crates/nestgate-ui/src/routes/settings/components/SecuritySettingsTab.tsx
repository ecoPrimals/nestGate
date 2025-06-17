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
import { SecuritySettings, SettingsFormProps } from '../types';
import { useSettingsForm } from '../hooks/useSettingsForm';

const SecuritySettingsTab: React.FC<SettingsFormProps<SecuritySettings> & {
  onSuccess?: (message: string) => void;
  onError?: (message: string) => void;
}> = ({ onSave, initialValues, onSuccess, onError }) => {
  const [formData, setFormData] = useState<SecuritySettings>({
    enableTwoFactor: initialValues?.enableTwoFactor || false,
    sessionTimeout: initialValues?.sessionTimeout || 30,
    failedLoginAttempts: initialValues?.failedLoginAttempts || 5,
    remoteAccess: initialValues?.remoteAccess || false,
  });

  const { loading, handleSave } = useSettingsForm({
    sectionName: 'Security',
    onSubmit: async (values) => onSave(values),
    onSuccess,
    onError,
  });

  const handleSubmit = () => {
    handleSave(formData);
  };

  const handleSwitchChange = (field: keyof SecuritySettings, checked: boolean) => {
    setFormData(prev => ({ ...prev, [field]: checked }));
  };

  const handleNumberChange = (field: keyof SecuritySettings, value: string) => {
    const numValue = parseInt(value) || 0;
    setFormData(prev => ({ ...prev, [field]: numValue }));
  };

  return (
    <Card>
      <CardContent>
        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 3 }}>
          <FormControlLabel
            control={
              <Switch
                checked={formData.enableTwoFactor}
                onChange={(e) => handleSwitchChange('enableTwoFactor', e.target.checked)}
              />
            }
            label="Enable Two-Factor Authentication"
          />

          <TextField
            label="Session Timeout (minutes)"
            type="number"
            value={formData.sessionTimeout}
            onChange={(e) => handleNumberChange('sessionTimeout', e.target.value)}
            inputProps={{ min: 1, max: 1440 }}
            placeholder="30"
            fullWidth
          />

          <TextField
            label="Failed Login Attempts Before Lockout"
            type="number"
            value={formData.failedLoginAttempts}
            onChange={(e) => handleNumberChange('failedLoginAttempts', e.target.value)}
            inputProps={{ min: 1, max: 10 }}
            placeholder="5"
            fullWidth
          />

          <FormControlLabel
            control={
              <Switch
                checked={formData.remoteAccess}
                onChange={(e) => handleSwitchChange('remoteAccess', e.target.checked)}
              />
            }
            label="Allow Remote Access"
          />

          <Box>
            <Button
              variant="contained"
              startIcon={loading ? <CircularProgress size={20} /> : <SaveIcon />}
              onClick={handleSubmit}
              disabled={loading}
            >
              Save Security Settings
            </Button>
          </Box>
        </Box>
      </CardContent>
    </Card>
  );
};

export default SecuritySettingsTab; 
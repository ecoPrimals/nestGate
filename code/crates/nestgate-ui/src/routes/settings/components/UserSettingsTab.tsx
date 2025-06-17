import React, { useState } from 'react';
import {
  Card,
  CardContent,
  TextField,
  Button,
  Box,
  CircularProgress,
} from '@mui/material';
import { Save as SaveIcon } from '@mui/icons-material';
import { UserSettings, SettingsFormProps } from '../types';
import { useSettingsForm } from '../hooks/useSettingsForm';

const UserSettingsTab: React.FC<SettingsFormProps<UserSettings> & {
  onSuccess?: (message: string) => void;
  onError?: (message: string) => void;
}> = ({ onSave, initialValues, onSuccess, onError }) => {
  const [formData, setFormData] = useState<UserSettings>({
    adminUsername: initialValues?.adminUsername || '',
    adminPassword: initialValues?.adminPassword || '',
    confirmPassword: initialValues?.confirmPassword || '',
    email: initialValues?.email || '',
  });

  const [errors, setErrors] = useState<Partial<Record<keyof UserSettings, string>>>({});

  const { loading, handleSave } = useSettingsForm({
    sectionName: 'User',
    onSubmit: async (values) => onSave(values),
    onSuccess,
    onError,
  });

  const validate = (): boolean => {
    const newErrors: Partial<Record<keyof UserSettings, string>> = {};

    if (formData.adminPassword && formData.adminPassword !== formData.confirmPassword) {
      newErrors.confirmPassword = 'Passwords do not match';
    }

    if (formData.email && !/\S+@\S+\.\S+/.test(formData.email)) {
      newErrors.email = 'Please enter a valid email address';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = () => {
    if (validate()) {
      handleSave(formData);
    }
  };

  const handleChange = (field: keyof UserSettings, value: string) => {
    setFormData(prev => ({ ...prev, [field]: value }));
    // Clear error when user starts typing
    if (errors[field]) {
      setErrors(prev => ({ ...prev, [field]: undefined }));
    }
  };

  return (
    <Card>
      <CardContent>
        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 3 }}>
          <TextField
            label="Admin Username"
            value={formData.adminUsername}
            onChange={(e) => handleChange('adminUsername', e.target.value)}
            placeholder="admin"
            fullWidth
          />

          <TextField
            label="Admin Password"
            type="password"
            value={formData.adminPassword}
            onChange={(e) => handleChange('adminPassword', e.target.value)}
            placeholder="Enter new password"
            helperText="Leave blank to keep current password"
            fullWidth
          />

          <TextField
            label="Confirm Password"
            type="password"
            value={formData.confirmPassword}
            onChange={(e) => handleChange('confirmPassword', e.target.value)}
            placeholder="Confirm new password"
            error={!!errors.confirmPassword}
            helperText={errors.confirmPassword}
            fullWidth
          />

          <TextField
            label="Email"
            type="email"
            value={formData.email}
            onChange={(e) => handleChange('email', e.target.value)}
            placeholder="admin@example.com"
            error={!!errors.email}
            helperText={errors.email}
            fullWidth
          />

          <Box>
            <Button
              variant="contained"
              startIcon={loading ? <CircularProgress size={20} /> : <SaveIcon />}
              onClick={handleSubmit}
              disabled={loading}
            >
              Save User Settings
            </Button>
          </Box>
        </Box>
      </CardContent>
    </Card>
  );
};

export default UserSettingsTab; 
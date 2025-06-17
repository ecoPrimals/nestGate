import React, { useState } from 'react';
import {
  Card,
  CardContent,
  TextField,
  Button,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Box,
  CircularProgress,
} from '@mui/material';
import { Save as SaveIcon } from '@mui/icons-material';
import { GeneralSettings, SettingsFormProps } from '../types';
import { useSettingsForm } from '../hooks/useSettingsForm';

const GeneralSettingsTab: React.FC<SettingsFormProps<GeneralSettings> & {
  onSuccess?: (message: string) => void;
  onError?: (message: string) => void;
}> = ({ onSave, initialValues, onSuccess, onError }) => {
  const [formData, setFormData] = useState<GeneralSettings>({
    systemName: initialValues?.systemName || '',
    timezone: initialValues?.timezone || '',
    language: initialValues?.language || '',
  });

  const [errors, setErrors] = useState<Partial<Record<keyof GeneralSettings, string>>>({});

  const { loading, handleSave } = useSettingsForm({
    sectionName: 'General',
    onSubmit: async (values) => onSave(values),
    onSuccess,
    onError,
  });

  const validate = (): boolean => {
    const newErrors: Partial<Record<keyof GeneralSettings, string>> = {};

    if (!formData.systemName.trim()) {
      newErrors.systemName = 'System name is required';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = () => {
    if (validate()) {
      handleSave(formData);
    }
  };

  const handleChange = (field: keyof GeneralSettings, value: string) => {
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
            label="System Name"
            value={formData.systemName}
            onChange={(e) => handleChange('systemName', e.target.value)}
            error={!!errors.systemName}
            helperText={errors.systemName}
            placeholder="Enter system name"
            required
            fullWidth
          />

          <FormControl fullWidth>
            <InputLabel>Time Zone</InputLabel>
            <Select
              value={formData.timezone}
              onChange={(e) => handleChange('timezone', e.target.value)}
              label="Time Zone"
            >
              <MenuItem value="utc">UTC</MenuItem>
              <MenuItem value="est">Eastern Time (EST)</MenuItem>
              <MenuItem value="pst">Pacific Time (PST)</MenuItem>
              <MenuItem value="cst">Central Time (CST)</MenuItem>
            </Select>
          </FormControl>

          <FormControl fullWidth>
            <InputLabel>Language</InputLabel>
            <Select
              value={formData.language}
              onChange={(e) => handleChange('language', e.target.value)}
              label="Language"
            >
              <MenuItem value="en">English</MenuItem>
              <MenuItem value="es">Spanish</MenuItem>
              <MenuItem value="fr">French</MenuItem>
              <MenuItem value="de">German</MenuItem>
            </Select>
          </FormControl>

          <Box>
            <Button
              variant="contained"
              startIcon={loading ? <CircularProgress size={20} /> : <SaveIcon />}
              onClick={handleSubmit}
              disabled={loading}
            >
              Save Settings
            </Button>
          </Box>
        </Box>
      </CardContent>
    </Card>
  );
};

export default GeneralSettingsTab; 
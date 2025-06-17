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
import { NetworkSettings, SettingsFormProps } from '../types';
import { useSettingsForm } from '../hooks/useSettingsForm';

const NetworkSettingsTab: React.FC<SettingsFormProps<NetworkSettings> & {
  onSuccess?: (message: string) => void;
  onError?: (message: string) => void;
}> = ({ onSave, initialValues, onSuccess, onError }) => {
  const [formData, setFormData] = useState<NetworkSettings>({
    ipAddress: initialValues?.ipAddress || '',
    subnetMask: initialValues?.subnetMask || '',
    gateway: initialValues?.gateway || '',
    dnsServers: initialValues?.dnsServers || '',
    dhcp: initialValues?.dhcp || false,
  });

  const [errors, setErrors] = useState<Partial<Record<keyof NetworkSettings, string>>>({});

  const { loading, handleSave } = useSettingsForm({
    sectionName: 'Network',
    onSubmit: async (values) => onSave(values),
    onSuccess,
    onError,
  });

  const validate = (): boolean => {
    const newErrors: Partial<Record<keyof NetworkSettings, string>> = {};

    if (!formData.dhcp && !formData.ipAddress.trim()) {
      newErrors.ipAddress = 'IP address is required when DHCP is disabled';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = () => {
    if (validate()) {
      handleSave(formData);
    }
  };

  const handleTextChange = (field: keyof NetworkSettings, value: string) => {
    setFormData(prev => ({ ...prev, [field]: value }));
    // Clear error when user starts typing
    if (errors[field]) {
      setErrors(prev => ({ ...prev, [field]: undefined }));
    }
  };

  const handleSwitchChange = (field: keyof NetworkSettings, checked: boolean) => {
    setFormData(prev => ({ ...prev, [field]: checked }));
  };

  return (
    <Card>
      <CardContent>
        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 3 }}>
          <FormControlLabel
            control={
              <Switch
                checked={formData.dhcp}
                onChange={(e) => handleSwitchChange('dhcp', e.target.checked)}
              />
            }
            label="Enable DHCP"
          />

          <TextField
            label="IP Address"
            value={formData.ipAddress}
            onChange={(e) => handleTextChange('ipAddress', e.target.value)}
            error={!!errors.ipAddress}
            helperText={errors.ipAddress}
            placeholder="192.168.1.1"
            disabled={formData.dhcp}
            required={!formData.dhcp}
            fullWidth
          />

          <TextField
            label="Subnet Mask"
            value={formData.subnetMask}
            onChange={(e) => handleTextChange('subnetMask', e.target.value)}
            placeholder="255.255.255.0"
            disabled={formData.dhcp}
            fullWidth
          />

          <TextField
            label="Default Gateway"
            value={formData.gateway}
            onChange={(e) => handleTextChange('gateway', e.target.value)}
            placeholder="192.168.1.1"
            disabled={formData.dhcp}
            fullWidth
          />

          <TextField
            label="DNS Servers"
            value={formData.dnsServers}
            onChange={(e) => handleTextChange('dnsServers', e.target.value)}
            placeholder="8.8.8.8, 8.8.4.4"
            helperText="Separate multiple DNS servers with commas"
            fullWidth
          />

          <Box>
            <Button
              variant="contained"
              startIcon={loading ? <CircularProgress size={20} /> : <SaveIcon />}
              onClick={handleSubmit}
              disabled={loading}
            >
              Save Network Settings
            </Button>
          </Box>
        </Box>
      </CardContent>
    </Card>
  );
};

export default NetworkSettingsTab; 
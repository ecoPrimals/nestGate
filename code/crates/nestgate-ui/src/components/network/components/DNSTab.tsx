import React, { useState } from 'react';
import {
  Card,
  CardHeader,
  CardContent,
  TextField,
  Button,
  Box,
  Typography,
  CircularProgress,
  Alert
} from '@mui/material';
import { Save as SaveIcon } from '@mui/icons-material';
import { DNSSettings } from '../types';

interface DNSTabProps {
  dnsSettings: DNSSettings;
  isLoading: boolean;
  onSaveDNSSettings: (values: DNSSettings) => Promise<boolean>;
}

const DNSTab: React.FC<DNSTabProps> = ({
  dnsSettings,
  isLoading,
  onSaveDNSSettings
}) => {
  const [values, setValues] = useState<DNSSettings>(dnsSettings);
  const [saveSuccess, setSaveSuccess] = useState<boolean | null>(null);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setValues(prev => ({ ...prev, [name]: value }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const success = await onSaveDNSSettings(values);
    setSaveSuccess(success);
    
    // Clear success message after 3 seconds
    if (success) {
      setTimeout(() => {
        setSaveSuccess(null);
      }, 3000);
    }
  };

  // Helper function to validate IP address format
  const isValidIP = (ip: string): boolean => {
    // Allow empty strings
    if (!ip) return true;
    
    // IPv4 regex pattern
    const ipv4Pattern = /^(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})$/;
    if (!ipv4Pattern.test(ip)) return false;
    
    // Check each octet is between 0-255
    const octets = ip.split('.');
    for (const octet of octets) {
      const num = parseInt(octet, 10);
      if (num < 0 || num > 255) return false;
    }
    
    return true;
  };

  // Validation states
  const primaryDNSValid = isValidIP(values.primaryDNS);
  const secondaryDNSValid = isValidIP(values.secondaryDNS);
  const formValid = primaryDNSValid && secondaryDNSValid;

  return (
    <Card>
      <CardHeader title="DNS Settings" />
      <CardContent>
        <form onSubmit={handleSubmit}>
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 3 }}>
            <Typography variant="body2" color="text.secondary" paragraph>
              Configure DNS server settings for name resolution on your network.
            </Typography>
            
            {saveSuccess !== null && (
              <Alert severity={saveSuccess ? "success" : "error"} sx={{ mb: 2 }}>
                {saveSuccess 
                  ? "DNS settings saved successfully" 
                  : "Failed to save DNS settings"}
              </Alert>
            )}
            
            <TextField
              label="Primary DNS Server"
              name="primaryDNS"
              value={values.primaryDNS}
              onChange={handleChange}
              fullWidth
              placeholder="e.g., 8.8.8.8"
              helperText={!primaryDNSValid ? "Invalid IP address format" : ""}
              error={!primaryDNSValid}
            />
            
            <TextField
              label="Secondary DNS Server"
              name="secondaryDNS"
              value={values.secondaryDNS}
              onChange={handleChange}
              fullWidth
              placeholder="e.g., 8.8.4.4"
              helperText={!secondaryDNSValid ? "Invalid IP address format" : ""}
              error={!secondaryDNSValid}
            />
            
            <Box sx={{ display: 'flex', justifyContent: 'flex-end', mt: 2 }}>
              <Button
                type="submit"
                variant="contained"
                color="primary"
                startIcon={isLoading ? <CircularProgress size={20} color="inherit" /> : <SaveIcon />}
                disabled={isLoading || !formValid}
              >
                Save DNS Settings
              </Button>
            </Box>
          </Box>
        </form>
      </CardContent>
    </Card>
  );
};

export default DNSTab; 
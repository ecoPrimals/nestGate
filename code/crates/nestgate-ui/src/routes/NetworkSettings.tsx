import React from 'react';
import { Typography, Box } from '@mui/material';
import NetworkConfiguration from '../components/network/NetworkConfiguration';

const NetworkSettings: React.FC = () => {
  return (
    <Box sx={{ padding: 3 }}>
      <Typography variant="h4" component="h1" gutterBottom>
        Network Management
      </Typography>
      <Typography variant="body1" color="text.secondary" sx={{ marginBottom: 3 }}>
        Configure and manage network interfaces, DNS settings, and firewall rules to optimize your NestGate NAS network performance and security.
      </Typography>
      
      <NetworkConfiguration />
    </Box>
  );
};

export default NetworkSettings; 
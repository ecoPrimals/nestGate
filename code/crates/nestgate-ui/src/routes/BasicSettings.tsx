import React, { useState } from 'react';
import { Typography, Tabs, Tab, Card, CardContent, Box } from '@mui/material';
import { Settings as SettingIcon, CloudSync as CloudSyncIcon, Person as UserIcon } from '@mui/icons-material';
import TabPanel from '../components/common/TabPanel';

const BasicSettings: React.FC = () => {
  const [activeTab, setActiveTab] = useState(0);

  const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
    setActiveTab(newValue);
  };

  return (
    <Box sx={{ padding: 3 }}>
      <Typography variant="h4" component="h1" gutterBottom>
        System Settings
      </Typography>
      <Typography variant="body1" color="text.secondary" sx={{ marginBottom: 3 }}>
        Configure system-wide settings for your NestGate storage system.
      </Typography>
      
      <Box sx={{ borderBottom: 1, borderColor: 'divider', marginBottom: 2 }}>
        <Tabs value={activeTab} onChange={handleTabChange} aria-label="settings tabs">
          <Tab 
            icon={<SettingIcon />}
            label="General" 
            iconPosition="start"
          />
          <Tab 
            icon={<CloudSyncIcon />}
            label="Network" 
            iconPosition="start"
          />
          <Tab 
            icon={<UserIcon />}
            label="User Management" 
            iconPosition="start"
          />
        </Tabs>
      </Box>
      
      <TabPanel value={activeTab} index={0}>
        <Card>
          <CardContent>
            <Typography variant="h6" gutterBottom>
              System Configuration
            </Typography>
            <Typography variant="body1">
              General settings content would go here
            </Typography>
          </CardContent>
        </Card>
      </TabPanel>
      
      <TabPanel value={activeTab} index={1}>
        <Card>
          <CardContent>
            <Typography variant="h6" gutterBottom>
              Network Configuration
            </Typography>
            <Typography variant="body1">
              Network settings content would go here
            </Typography>
          </CardContent>
        </Card>
      </TabPanel>
      
      <TabPanel value={activeTab} index={2}>
        <Card>
          <CardContent>
            <Typography variant="h6" gutterBottom>
              User Management
            </Typography>
            <Typography variant="body1">
              User management content would go here
            </Typography>
          </CardContent>
        </Card>
      </TabPanel>
    </Box>
  );
};

export default BasicSettings; 
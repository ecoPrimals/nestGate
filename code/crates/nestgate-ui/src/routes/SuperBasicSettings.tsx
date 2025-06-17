import React, { useState } from 'react';
import { Tabs, Tab, Box, Typography, TextField, Button, Paper } from '@mui/material';
import TabPanel from '../components/common/TabPanel';

// Simple component - using MUI components
const SimpleSettingsForm: React.FC<{
  title: string;
  onSave: (values: any) => void;
}> = ({ title, onSave }) => {
  const [formValue, setFormValue] = useState('');
  
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSave({ value: formValue });
  };
  
  return (
    <Paper variant="outlined" sx={{ padding: 3 }}>
      <Typography variant="h6" gutterBottom>
        {title}
      </Typography>
      <form onSubmit={handleSubmit}>
        <TextField
          fullWidth
          label="Setting Name"
              value={formValue}
              onChange={(e) => setFormValue(e.target.value)}
          margin="normal"
          variant="outlined"
        />
        <Button 
          type="submit"
          variant="contained"
          color="primary"
          sx={{ marginTop: 2 }}
        >
          Save Changes
        </Button>
      </form>
    </Paper>
  );
};

const SuperBasicSettings: React.FC = () => {
  const [currentTab, setCurrentTab] = useState(0);

  const handleSave = (section: string, values: any) => {
    console.log(`${section} settings saved:`, values);
  };

  const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
    setCurrentTab(newValue);
  };

  return (
    <Box>
      <Typography variant="h4" component="h1" gutterBottom>
        System Settings
      </Typography>
      <Typography variant="body1" paragraph>
        Configure system-wide settings for your NestGate storage system.
      </Typography>
      
      <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
        <Tabs value={currentTab} onChange={handleTabChange} aria-label="settings tabs">
          <Tab label="General" />
          <Tab label="Network" />
          <Tab label="Backup & Replication" />
          <Tab label="User Management" />
        </Tabs>
      </Box>
      
      <TabPanel value={currentTab} index={0}>
        <SimpleSettingsForm 
          title="System Configuration" 
          onSave={(values) => handleSave('General', values)} 
        />
      </TabPanel>
      
      <TabPanel value={currentTab} index={1}>
        <SimpleSettingsForm 
          title="Network Configuration" 
          onSave={(values) => handleSave('Network', values)} 
        />
      </TabPanel>
      
      <TabPanel value={currentTab} index={2}>
        <SimpleSettingsForm 
          title="Backup Configuration" 
          onSave={(values) => handleSave('Backup', values)} 
        />
      </TabPanel>
      
      <TabPanel value={currentTab} index={3}>
        <SimpleSettingsForm 
          title="User Settings" 
          onSave={(values) => handleSave('User', values)} 
        />
      </TabPanel>
    </Box>
  );
};

export default SuperBasicSettings; 
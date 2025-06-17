import React, { useState } from 'react';
import { Tabs, Tab, Typography, Card, Box } from '@mui/material';
import BackupJobManagement from './BackupJobManagement';
import BackupTargetManagement from './BackupTargetManagement';
import RestoreWorkflow from './RestoreWorkflow';
import TabPanel from '../common/TabPanel';

const BackupManagement: React.FC = () => {
  const [activeTab, setActiveTab] = useState(0);

  const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
    setActiveTab(newValue);
  };

  return (
    <Box sx={{ padding: 3 }}>
      <Typography variant="h4" component="h1" gutterBottom>
        Backup Management
      </Typography>
      <Typography variant="body1" color="text.secondary" sx={{ marginBottom: 3 }}>
        Manage your backup jobs, targets, and restore data from previous backups.
      </Typography>

      <Card>
        <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
          <Tabs value={activeTab} onChange={handleTabChange} aria-label="backup management tabs">
            <Tab label="Backup Jobs" />
            <Tab label="Backup Targets" />
            <Tab label="Restore Data" />
          </Tabs>
        </Box>
        
        <TabPanel value={activeTab} index={0}>
          <BackupJobManagement />
        </TabPanel>
        
        <TabPanel value={activeTab} index={1}>
          <BackupTargetManagement />
        </TabPanel>
        
        <TabPanel value={activeTab} index={2}>
          <RestoreWorkflow />
        </TabPanel>
      </Card>
    </Box>
  );
};

export default BackupManagement; 
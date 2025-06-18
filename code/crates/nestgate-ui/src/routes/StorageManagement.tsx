import React, { useState } from 'react';
import {
  Box,
  Typography,
  Tabs,
  Tab,
  Container,
  Paper,
  Alert,
} from '@mui/material';
import {
  Storage as StorageIcon,
  Dashboard as DashboardIcon,
  Timeline as TimelineIcon,
  Settings as SettingsIcon,
} from '@mui/icons-material';

// Import our new components
import PoolManagementDashboard from '../components/storage/PoolManagementDashboard';
import TieredStorageManager from '../components/storage/TieredStorageManager';
import ZFSPoolMonitor from '../components/monitoring/ZFSPoolMonitor';

// Tab panel component
interface TabPanelProps {
  children?: React.ReactNode;
  index: number;
  value: number;
}

function TabPanel(props: TabPanelProps) {
  const { children, value, index, ...other } = props;

  return (
    <div
      role="tabpanel"
      hidden={value !== index}
      id={`storage-tabpanel-${index}`}
      aria-labelledby={`storage-tab-${index}`}
      {...other}
    >
      {value === index && (
        <Box sx={{ py: 3 }}>
          {children}
        </Box>
      )}
    </div>
  );
}

function a11yProps(index: number) {
  return {
    id: `storage-tab-${index}`,
    'aria-controls': `storage-tabpanel-${index}`,
  };
}

const StorageManagement: React.FC = () => {
  const [tabValue, setTabValue] = useState(0);

  const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
    setTabValue(newValue);
  };

  return (
    <Container maxWidth="xl" sx={{ py: 2 }}>
      {/* Header */}
      <Box mb={3}>
        <Typography variant="h4" component="h1" gutterBottom>
          Storage Management
        </Typography>
        <Typography variant="body1" color="text.secondary">
          Comprehensive ZFS storage management with pool creation, tiered storage, and monitoring.
        </Typography>
      </Box>

      {/* Navigation Tabs */}
      <Paper sx={{ mb: 3 }}>
        <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
          <Tabs 
            value={tabValue} 
            onChange={handleTabChange} 
            aria-label="storage management tabs"
            variant="scrollable"
            scrollButtons="auto"
          >
            <Tab 
              label="Pool Management" 
              icon={<StorageIcon />} 
              iconPosition="start"
              {...a11yProps(0)} 
            />
            <Tab 
              label="Tiered Storage" 
              icon={<DashboardIcon />} 
              iconPosition="start"
              {...a11yProps(1)} 
            />
            <Tab 
              label="Monitoring" 
              icon={<TimelineIcon />} 
              iconPosition="start"
              {...a11yProps(2)} 
            />
            <Tab 
              label="Advanced Settings" 
              icon={<SettingsIcon />} 
              iconPosition="start"
              {...a11yProps(3)} 
            />
          </Tabs>
        </Box>

        {/* Pool Management Tab */}
        <TabPanel value={tabValue} index={0}>
          <Alert severity="info" sx={{ mb: 2 }}>
            <Typography variant="body2">
              <strong>Pool Management:</strong> Create, configure, and manage ZFS pools with advanced features including 
              RAID configurations, encryption, and performance optimization.
            </Typography>
          </Alert>
          <PoolManagementDashboard />
        </TabPanel>

        {/* Tiered Storage Tab */}
        <TabPanel value={tabValue} index={1}>
          <Alert severity="info" sx={{ mb: 2 }}>
            <Typography variant="body2">
              <strong>Tiered Storage:</strong> Manage hot, warm, and cold storage tiers with automated data migration 
              and intelligent tier assignment based on access patterns.
            </Typography>
          </Alert>
          <TieredStorageManager />
        </TabPanel>

        {/* Monitoring Tab */}
        <TabPanel value={tabValue} index={2}>
          <Alert severity="info" sx={{ mb: 2 }}>
            <Typography variant="body2">
              <strong>Real-time Monitoring:</strong> Monitor ZFS pool health, performance metrics, and system status 
              with live updates and detailed analytics.
            </Typography>
          </Alert>
          <ZFSPoolMonitor showAllPools={true} showDetailed={true} />
        </TabPanel>

        {/* Advanced Settings Tab */}
        <TabPanel value={tabValue} index={3}>
          <Alert severity="warning" sx={{ mb: 2 }}>
            <Typography variant="body2">
              <strong>Advanced Settings:</strong> Advanced ZFS configuration options. 
              These settings can significantly impact system performance and data integrity.
            </Typography>
          </Alert>
          
          <Box sx={{ p: 3, textAlign: 'center' }}>
            <Typography variant="h6" gutterBottom>
              Advanced ZFS Settings
            </Typography>
            <Typography variant="body2" color="text.secondary">
              Advanced configuration options will be available in the next update.
              This includes ZFS send/receive, replication settings, and system-wide optimizations.
            </Typography>
          </Box>
        </TabPanel>
      </Paper>
    </Container>
  );
};

export default StorageManagement; 
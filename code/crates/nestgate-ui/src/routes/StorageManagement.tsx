import React, { useState } from 'react';
import {
  Box,
  Typography,
  Tabs,
  Tab,
  Container,
  Paper,
  Alert,
  Chip,
} from '@mui/material';
import {
  Storage as StorageIcon,
  Dashboard as DashboardIcon,
  Layers as LayersIcon,
  Monitor as MonitorIcon,
  Settings as SettingsIcon,
  Folder as FolderIcon,
} from '@mui/icons-material';

// Import our new components
import PoolManagementDashboard from '../components/storage/PoolManagementDashboard';
import TieredStorageManager from '../components/storage/TieredStorageManager';
import ZFSPoolMonitor from '../components/monitoring/ZFSPoolMonitor';
import DatasetManager from '../components/storage/DatasetManager';
import DatasetCreationWizard from '../components/storage/DatasetCreationWizard';

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
        <Box sx={{ p: 3 }}>
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
  const [datasetWizardOpen, setDatasetWizardOpen] = useState(false);
  const [selectedPool, setSelectedPool] = useState<string | undefined>();

  const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
    setTabValue(newValue);
  };

  const handleDatasetCreated = (datasetName: string) => {
    console.log('Dataset created:', datasetName);
    // TODO: Refresh dataset list or show success notification
  };

  const handlePoolSelect = (poolName: string) => {
    setSelectedPool(poolName);
    // Switch to dataset management tab when a pool is selected
    setTabValue(1);
  };

  return (
    <Container maxWidth="xl" sx={{ py: 2 }}>
      {/* Header */}
      <Box mb={3}>
        <Typography variant="h4" component="h1" gutterBottom>
          Storage Management
        </Typography>
        <Typography variant="body1" color="text.secondary">
          Comprehensive ZFS storage management with pool creation, dataset management, and tiered storage
        </Typography>
      </Box>

      {/* Status Alert */}
      <Alert severity="info" sx={{ mb: 3 }}>
        <Box display="flex" alignItems="center" gap={1}>
          <Typography variant="body2">
            <strong>UI Half Marathon Progress:</strong> Pool Management Complete
          </Typography>
          <Chip label="Day 2: Dataset Management" size="small" color="primary" />
        </Box>
      </Alert>

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
              label="Dataset Management" 
              icon={<FolderIcon />} 
              iconPosition="start"
              {...a11yProps(1)} 
            />
            <Tab 
              label="Tiered Storage" 
              icon={<LayersIcon />} 
              iconPosition="start"
              {...a11yProps(2)} 
            />
            <Tab 
              label="Monitoring" 
              icon={<MonitorIcon />} 
              iconPosition="start"
              {...a11yProps(3)} 
            />
            <Tab 
              label="Advanced Settings" 
              icon={<SettingsIcon />} 
              iconPosition="start"
              {...a11yProps(4)} 
            />
          </Tabs>
        </Box>

        {/* Pool Management Tab */}
        <TabPanel value={tabValue} index={0}>
          <Box>
            <Typography variant="h5" gutterBottom>
              ZFS Pool Management
            </Typography>
            <Typography variant="body2" color="text.secondary" paragraph>
              Create, manage, and monitor ZFS storage pools. Configure RAID levels, 
              manage pool properties, and perform maintenance operations.
            </Typography>
            <PoolManagementDashboard onPoolSelect={handlePoolSelect} />
          </Box>
        </TabPanel>

        {/* Dataset Management Tab */}
        <TabPanel value={tabValue} index={1}>
          <Box>
            <Typography variant="h5" gutterBottom>
              Dataset Management
            </Typography>
            <Typography variant="body2" color="text.secondary" paragraph>
              Create and manage ZFS datasets with hierarchical organization, 
              quotas, compression, and tiered storage assignment.
            </Typography>
            <DatasetManager 
              poolName={selectedPool}
              onDatasetSelect={(dataset) => console.log('Selected dataset:', dataset)}
            />
          </Box>
        </TabPanel>

        {/* Tiered Storage Tab */}
        <TabPanel value={tabValue} index={2}>
          <Box>
            <Typography variant="h5" gutterBottom>
              Tiered Storage Management
            </Typography>
            <Typography variant="body2" color="text.secondary" paragraph>
              Configure storage tiers, manage data placement policies, and optimize 
              storage performance across hot, warm, and cold tiers.
            </Typography>
            <TieredStorageManager />
          </Box>
        </TabPanel>

        {/* Monitoring Tab */}
        <TabPanel value={tabValue} index={3}>
          <Box>
            <Typography variant="h5" gutterBottom>
              Storage Monitoring
            </Typography>
            <Typography variant="body2" color="text.secondary" paragraph>
              Monitor pool health, performance metrics, and storage utilization 
              across all ZFS pools and datasets.
            </Typography>
            <ZFSPoolMonitor />
          </Box>
        </TabPanel>

        {/* Advanced Settings Tab */}
        <TabPanel value={tabValue} index={4}>
          <Box>
            <Typography variant="h5" gutterBottom>
              Advanced Storage Settings
            </Typography>
            <Typography variant="body2" color="text.secondary" paragraph>
              Configure advanced ZFS features, performance tuning, and system-wide 
              storage policies.
            </Typography>
            <Alert severity="info">
              <Typography variant="body2">
                Advanced settings panel coming soon. This will include ZFS performance tuning, 
                system-wide policies, and advanced feature configuration.
              </Typography>
            </Alert>
          </Box>
        </TabPanel>
      </Paper>

      {/* Dataset Creation Wizard */}
      <DatasetCreationWizard
        open={datasetWizardOpen}
        onClose={() => setDatasetWizardOpen(false)}
        onDatasetCreated={handleDatasetCreated}
        defaultPool={selectedPool}
      />
    </Container>
  );
};

export default StorageManagement; 
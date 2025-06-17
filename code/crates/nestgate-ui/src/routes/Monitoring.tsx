import React, { useState } from 'react';
import { 
  Box, 
  Card, 
  CardContent,
  Tabs, 
  Tab,
  Typography,
  Button,
  Divider,
} from '@mui/material';
import {
  Dashboard as DashboardIcon,
  Refresh as RefreshIcon,
  Settings as SettingsIcon,
  ShowChart as LineChartIcon,
  Storage as HddIcon,
  Dataset as DatabaseIcon,
  Folder as FolderOpenIcon
} from '@mui/icons-material';

// Import monitoring components
import SystemMonitor from '../components/monitoring/SystemMonitor';
import DiskHealthMonitor from '../components/monitoring/DiskHealthMonitor';
import ZFSPoolMonitor from '../components/monitoring/ZFSPoolMonitor';
import FileSystemMonitor from '../components/monitoring/FileSystemMonitor';
import NotificationCenter from '../components/notifications/NotificationCenter';
import TabPanel from '../components/common/TabPanel';

const Monitoring: React.FC = () => {
  const [activeTab, setActiveTab] = useState(0);
  const [refreshKey, setRefreshKey] = useState(0);

  // Function to force refresh all monitoring components
  const handleRefresh = () => {
    setRefreshKey(prev => prev + 1);
  };

  const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
    setActiveTab(newValue);
  };

  return (
    <Box sx={{ padding: 3 }}>
      <Box sx={{ marginBottom: 3 }}>
        <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 2 }}>
          <Box>
            <Typography variant="h4" component="h1" sx={{ display: 'flex', alignItems: 'center', gap: 1, marginBottom: 1 }}>
              <DashboardIcon />
              System Monitoring
            </Typography>
            <Typography variant="body1" color="text.secondary">
                Real-time monitoring of system performance and health
            </Typography>
          </Box>
          <Box sx={{ display: 'flex', gap: 1, alignItems: 'center' }}>
              <Button 
              startIcon={<RefreshIcon />} 
                onClick={handleRefresh}
              variant="outlined"
              >
                Refresh All
              </Button>
            <Button 
              startIcon={<SettingsIcon />}
              variant="outlined"
            >
                Settings
              </Button>
              <NotificationCenter />
          </Box>
        </Box>
        <Divider />
      </Box>

      <Box sx={{ borderBottom: 1, borderColor: 'divider', marginBottom: 2 }}>
      <Tabs 
          value={activeTab} 
          onChange={handleTabChange}
          aria-label="monitoring tabs"
        >
          <Tab 
            icon={<LineChartIcon />}
            label="System" 
            iconPosition="start"
          />
          <Tab 
            icon={<HddIcon />}
            label="Disks" 
            iconPosition="start"
          />
          <Tab 
            icon={<DatabaseIcon />}
            label="ZFS Pools" 
            iconPosition="start"
          />
          <Tab 
            icon={<FolderOpenIcon />}
            label="File System" 
            iconPosition="start"
          />
        </Tabs>
        <Box sx={{ display: 'flex', justifyContent: 'flex-end', padding: 1 }}>
          <Typography variant="caption" color="text.secondary">
            Auto-refresh: 30s
          </Typography>
        </Box>
      </Box>

      <TabPanel value={activeTab} index={0}>
        <Box>
              <SystemMonitor key={`system-${refreshKey}`} showDetailedStats={true} />
        </Box>
      </TabPanel>

      <TabPanel value={activeTab} index={1}>
        <Box>
              <DiskHealthMonitor key={`disk-${refreshKey}`} showDetailed={true} />
        </Box>
      </TabPanel>

      <TabPanel value={activeTab} index={2}>
        <Box>
              <ZFSPoolMonitor key={`zfs-${refreshKey}`} showAllPools={true} showDetailed={true} />
        </Box>
      </TabPanel>

      <TabPanel value={activeTab} index={3}>
        <Box>
              <FileSystemMonitor key={`filesystem-${refreshKey}`} showDetailed={true} />
        </Box>
      </TabPanel>

      {/* Display full dashboard when on system tab */}
      {activeTab === 0 && (
        <Box sx={{ marginTop: 3 }}>
          <Typography variant="h5" component="h2" sx={{ marginBottom: 2 }}>
            Quick Status Overview
          </Typography>
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
            <Box sx={{ display: 'flex', gap: 2, flexWrap: 'wrap' }}>
              <Box sx={{ flex: 1, minWidth: 300 }}>
                <Card>
                  <CardContent>
                    <Typography variant="h6" component="h3" gutterBottom>
                      Disk Health Summary
                    </Typography>
                <DiskHealthMonitor showDetailed={false} />
                  </CardContent>
              </Card>
              </Box>
              <Box sx={{ flex: 1, minWidth: 300 }}>
                <Card>
                  <CardContent>
                    <Typography variant="h6" component="h3" gutterBottom>
                      ZFS Pools Summary
                    </Typography>
                <ZFSPoolMonitor showDetailed={false} />
                  </CardContent>
              </Card>
              </Box>
            </Box>
            <Box>
              <Card>
                <CardContent>
                  <Typography variant="h6" component="h3" gutterBottom>
                    File System Activity
                  </Typography>
                <FileSystemMonitor showDetailed={false} />
                </CardContent>
              </Card>
            </Box>
          </Box>
        </Box>
      )}
    </Box>
  );
};

export default Monitoring; 
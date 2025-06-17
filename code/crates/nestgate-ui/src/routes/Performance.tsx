import React, { useState } from 'react';
import { Typography, Divider, Card, CardContent, Tabs, Tab, Box } from '@mui/material';
import { ShowChart as AreaChartIcon, Dashboard as DashboardIcon, Storage as HddIcon } from '@mui/icons-material';
import { PoolPerformance } from '../components/storage/PoolPerformance';
import TabPanel from '../components/common/TabPanel';

/**
 * Performance monitoring page displaying ZFS pool performance metrics
 */
const Performance: React.FC = () => {
  const [activeTab, setActiveTab] = useState(0);

  const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
    setActiveTab(newValue);
  };

  return (
    <Box sx={{ padding: 3 }}>
      <Typography variant="h4" component="h1" sx={{ display: 'flex', alignItems: 'center', gap: 1, marginBottom: 1 }}>
        <AreaChartIcon />
        Performance Monitoring
      </Typography>
      <Typography variant="body1" color="text.secondary" sx={{ marginBottom: 3 }}>
        Monitor the performance metrics and health of your ZFS storage pools in real-time. 
        Use the controls to select different time ranges and view detailed metrics.
      </Typography>
      
      <Divider sx={{ marginY: 3 }} />
      
      <Box sx={{ borderBottom: 1, borderColor: 'divider', marginBottom: 2 }}>
        <Tabs value={activeTab} onChange={handleTabChange} aria-label="performance tabs">
          <Tab 
            icon={<DashboardIcon />}
            label="Pool Performance" 
            iconPosition="start"
          />
          <Tab 
            icon={<HddIcon />}
            label="Disk Performance" 
            iconPosition="start"
          />
        </Tabs>
      </Box>
      
      <TabPanel value={activeTab} index={0}>
          <PoolPerformance />
      </TabPanel>
      
      <TabPanel value={activeTab} index={1}>
        <Card>
          <CardContent>
            <Typography variant="h6" gutterBottom>
              Disk Performance Monitoring
            </Typography>
            <Typography variant="body1">
              Individual disk performance monitoring will be available in a future update.
              This feature will provide detailed metrics for each physical disk in your system.
            </Typography>
          </CardContent>
          </Card>
      </TabPanel>
    </Box>
  );
};

export default Performance; 
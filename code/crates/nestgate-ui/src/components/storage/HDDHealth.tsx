import React, { useState, useEffect } from 'react';
import { 
  Card, 
  CardContent,
  CardHeader,
  Typography, 
  Button, 
  Tabs, 
  Tab,
  Box,
  Tooltip,
  CircularProgress,
  Badge,
  Alert,
  IconButton,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
  Chip,
  LinearProgress
} from '@mui/material';
import { 
  CheckCircle as CheckCircleIcon, 
  Cancel as CloseCircleIcon, 
  Warning as ExclamationCircleIcon, 
  Refresh as ReloadIcon, 
  Info as InfoCircleIcon, 
  Storage as HddIcon,
  ShowChart as AreaChartIcon,
  AccessTime as ClockCircleIcon
} from '@mui/icons-material';
import { 
  WebSocketService, 
  WebSocketMessage, 
  WebSocketMessageType,
  ConnectionStatus,
  Disk,
  isConnectionStatus,
  isDisks
} from '../../services/websocket.service';
import { DataSourceType } from '../../utils/env';
import StatusChip from '../common/StatusChip';
import EmptyState from '../common/EmptyState';
import TabPanel from '../common/TabPanel';

// Extend the Disk interface with additional SMART status details
interface DiskHealth extends Omit<Disk, 'status'> {
  status: string;
  smartStatus: {
    passed: boolean;
    attributes: SmartAttribute[];
  };
  lastScan: string;
  powerOnHours: number;
  interface: string;
}

interface SmartAttribute {
  id: number;
  name: string;
  value: number;
  worst: number;
  threshold: number;
  status: 'GOOD' | 'WARNING' | 'FAILED';
  raw: string;
}

const HDDHealth: React.FC = () => {
  const [disks, setDisks] = useState<DiskHealth[]>([]);
  const [selectedDisk, setSelectedDisk] = useState<string | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [connected, setConnected] = useState<boolean>(false);
  const [activeTab, setActiveTab] = useState(0);
  
  // Always use LIVE data source for production
  const dataSource = DataSourceType.LIVE;

  useEffect(() => {
    const wsService = WebSocketService.getInstance();
    
    wsService.connect();
    
    // Setup websocket subscriptions
    const connectionUnsubscribe = wsService.subscribe<ConnectionStatus>('connection', (msg: WebSocketMessage<ConnectionStatus>) => {
      setConnected(msg.data.status === 'connected');
      
      if (msg.data.status === 'connected') {
        fetchDiskData();
      }
    });

    const disksUnsubscribe = wsService.subscribe('disks', (msg: WebSocketMessage) => {
      if (isDisks(msg.data)) {
        // Transform the basic disk data to include extended disk health information
        const enhancedDisks: DiskHealth[] = msg.data.map(disk => ({
          ...disk,
          smartStatus: {
            passed: disk.health === 'good',
            attributes: disk.smartAttributes || []
          },
          lastScan: disk.lastScan || new Date().toLocaleString(),
          powerOnHours: disk.powerOnHours || 0,
          interface: disk.interface || 'SATA'
        }));
        
        setDisks(enhancedDisks);
        
        // Select first disk if none selected
        if (!selectedDisk && enhancedDisks.length > 0) {
          setSelectedDisk(enhancedDisks[0].id);
        }
      }
      setLoading(false);
    });
    
    // Monitor for error messages
    const errorUnsubscribe = wsService.subscribe(WebSocketMessageType.ERROR, (msg: WebSocketMessage) => {
      console.error('WebSocket error:', msg.data);
      setLoading(false);
    });
    
    // Initial data fetch
    fetchDiskData();
    
    // Cleanup
    return () => {
      connectionUnsubscribe();
      disksUnsubscribe();
      errorUnsubscribe();
    };
  }, [selectedDisk]);

  const fetchDiskData = () => {
    setLoading(true);
    const wsService = WebSocketService.getInstance();
    wsService.send('get_disks');
    
    // Set a timeout to stop the loading state even if no data arrives
    setTimeout(() => {
      setLoading(false);
    }, 5000);
  };

  const handleRefresh = () => {
    fetchDiskData();
  };

  const getDiskStatusChip = (status: string) => {
    const statusMap: { [key: string]: 'success' | 'warning' | 'error' | 'info' } = {
      'PASSED': 'success',
      'WARNING': 'warning', 
      'FAILED': 'error'
    };
    
    return (
      <StatusChip 
        status={statusMap[status] || 'info'} 
        label={status} 
        size="small"
      />
    );
  };

  const getAttributeStatusChip = (status: string) => {
    const statusMap: { [key: string]: 'success' | 'warning' | 'error' } = {
      'GOOD': 'success',
      'WARNING': 'warning',
      'FAILED': 'error'
    };
    
    return (
      <Chip 
        label={status}
        color={statusMap[status] || 'default'}
        size="small"
        variant="outlined"
      />
    );
  };

  const getSelectedDisk = () => {
    return disks.find(disk => disk.id === selectedDisk);
  };

  const getTemperatureColor = (temp: number) => {
    if (temp >= 50) return 'error';
    if (temp >= 45) return 'warning';
    return 'success';
  };

  const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
    setActiveTab(newValue);
  };

  const disk = getSelectedDisk();

  return (
    <Box sx={{ padding: 3 }}>
      <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, marginBottom: 1 }}>
        <HddIcon />
        <Typography variant="h4" component="h1">
          Disk Health Monitoring
        </Typography>
      </Box>
      
      <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, marginBottom: 3 }}>
        <Typography variant="body2" color="text.secondary">
          Data Source:
        </Typography>
        {connected ? (
          <StatusChip status="success" label="Live Data" size="small" />
        ) : (
          <StatusChip status="error" label="Connection Error" size="small" />
        )}
        <Typography variant="body2" color="text.secondary">
          {connected ? 
            'ⓘ Connected to ZFS management server' : 
            'Connection to ZFS management server failed. Please check your network connection.'
          }
        </Typography>
      </Box>
      
      {!connected && (
        <Alert
          severity="error" 
          sx={{ marginBottom: 2 }}
        >
          <Typography variant="subtitle2">Connection Error</Typography>
          <Typography variant="body2">
            Unable to connect to the storage system. Please check your network connection or server status.
          </Typography>
        </Alert>
      )}
      
      <Card sx={{ marginBottom: 2 }}>
        <CardHeader 
        title="Disk Overview" 
          action={
          <Button 
              startIcon={<ReloadIcon />}
            onClick={handleRefresh} 
              disabled={loading}
              variant="outlined"
              size="small"
          >
              {loading ? 'Loading...' : 'Refresh Data'}
          </Button>
        }
        />
        <CardContent>
          {loading ? (
            <Box sx={{ display: 'flex', justifyContent: 'center', padding: 3 }}>
              <CircularProgress />
            </Box>
          ) : (
            <TableContainer component={Paper} variant="outlined">
              <Table size="small">
                <TableHead>
                  <TableRow>
                    <TableCell>Disk ID</TableCell>
                    <TableCell>Status</TableCell>
                    <TableCell>Model</TableCell>
                    <TableCell>Size</TableCell>
                    <TableCell>Temperature</TableCell>
                    <TableCell>Action</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {disks.map((diskItem) => (
                    <TableRow key={diskItem.id}>
                      <TableCell>{diskItem.id}</TableCell>
                      <TableCell>
                        {getDiskStatusChip(diskItem.status)}
                      </TableCell>
                      <TableCell>{diskItem.model}</TableCell>
                      <TableCell>{diskItem.size}</TableCell>
                      <TableCell>
                        <Typography 
                          variant="body2" 
                          color={getTemperatureColor(diskItem.temperature) + '.main'}
                          sx={{ fontWeight: 600 }}
                        >
                          {diskItem.temperature}°C
                        </Typography>
                      </TableCell>
                      <TableCell>
                        <Button 
                          variant={selectedDisk === diskItem.id ? 'contained' : 'outlined'}
                          size="small"
                          onClick={() => setSelectedDisk(diskItem.id)}
                        >
                          View Details
                        </Button>
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </TableContainer>
          )}
        </CardContent>
      </Card>
      
      {disk && (
        <Card>
          <CardHeader title={`Disk Details: ${disk.id}`} />
          <CardContent>
            <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
              <Tabs value={activeTab} onChange={handleTabChange}>
                <Tab 
                  icon={<InfoCircleIcon />}
                  label="Overview" 
                  iconPosition="start"
                />
                <Tab 
                  icon={<AreaChartIcon />}
                  label="SMART Attributes" 
                  iconPosition="start"
                />
              </Tabs>
            </Box>
            
            <TabPanel value={activeTab} index={0}>
              <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 2, marginY: 2 }}>
                <Card variant="outlined" sx={{ minWidth: 200, textAlign: 'center' }}>
                  <CardContent>
                    <Typography variant="subtitle2" color="text.secondary" gutterBottom>
                      Status
                    </Typography>
                    {getDiskStatusChip(disk.status)}
                  </CardContent>
                  </Card>
                
                <Card variant="outlined" sx={{ minWidth: 200, textAlign: 'center' }}>
                  <CardContent>
                    <Typography variant="subtitle2" color="text.secondary" gutterBottom>
                      Temperature
                    </Typography>
                    <Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 1 }}>
                      <CircularProgress 
                        variant="determinate" 
                        value={Math.min(100, (disk.temperature / 60) * 100)}
                        color={getTemperatureColor(disk.temperature)}
                        size={60}
                      />
                      <Typography variant="h6" color={getTemperatureColor(disk.temperature) + '.main'}>
                        {disk.temperature}°C
                      </Typography>
                    </Box>
                  </CardContent>
                  </Card>
                
                <Card variant="outlined" sx={{ minWidth: 200, textAlign: 'center' }}>
                  <CardContent>
                    <Typography variant="subtitle2" color="text.secondary" gutterBottom>
                      Power On Time
                    </Typography>
                    <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'center', gap: 1 }}>
                      <ClockCircleIcon color="action" />
                      <Typography variant="h6">
                         {disk.powerOnHours.toLocaleString()}
                       </Typography>
                       <Typography variant="body2" color="text.secondary">
                         hours
                       </Typography>
                     </Box>
                   </CardContent>
                  </Card>
                 
                 <Card variant="outlined" sx={{ minWidth: 200, textAlign: 'center' }}>
                   <CardContent>
                     <Typography variant="subtitle2" color="text.secondary" gutterBottom>
                       Last SMART Test
                     </Typography>
                     <Typography variant="body2">
                       {disk.lastScan}
                     </Typography>
                   </CardContent>
                  </Card>
               </Box>
               
               <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 2, marginTop: 2 }}>
                 <Card variant="outlined" sx={{ flex: 1, minWidth: 300 }}>
                   <CardContent>
                     <Typography variant="h6" gutterBottom>
                       Disk Information
                     </Typography>
                     <Typography variant="body2" paragraph>
                       <strong>Model:</strong> {disk.model}
                     </Typography>
                     <Typography variant="body2" paragraph>
                       <strong>Serial Number:</strong> {disk.serial}
                     </Typography>
                     <Typography variant="body2" paragraph>
                       <strong>Size:</strong> {disk.size}
                     </Typography>
                     <Typography variant="body2">
                       <strong>Interface:</strong> {disk.interface}
                     </Typography>
                   </CardContent>
                  </Card>
                 
                 <Card variant="outlined" sx={{ flex: 1, minWidth: 300 }}>
                   <CardContent>
                     <Typography variant="h6" gutterBottom>
                       SMART Status
                     </Typography>
                     <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, marginBottom: 2 }}>
                       {disk.smartStatus.passed ? (
                         <>
                           <CheckCircleIcon color="success" />
                           <Typography variant="body2" color="success.main">
                             SMART Passed
                           </Typography>
                         </>
                       ) : (
                         <>
                           <CloseCircleIcon color="error" />
                           <Typography variant="body2" color="error.main">
                             SMART Failed
                           </Typography>
                         </>
                       )}
                     </Box>
                     <Typography variant="body2" color="text.secondary">
                       Regular SMART monitoring helps identify disk issues before they lead to data loss.
                     </Typography>
                   </CardContent>
                  </Card>
               </Box>
             </TabPanel>
            
             <TabPanel value={activeTab} index={1}>
              {disk.smartStatus.attributes.length > 0 ? (
                 <TableContainer component={Paper} variant="outlined">
                   <Table size="small">
                     <TableHead>
                       <TableRow>
                         <TableCell>ID</TableCell>
                         <TableCell>Name</TableCell>
                         <TableCell>Value</TableCell>
                         <TableCell>Worst</TableCell>
                         <TableCell>Threshold</TableCell>
                         <TableCell>Status</TableCell>
                         <TableCell>Raw Value</TableCell>
                       </TableRow>
                     </TableHead>
                     <TableBody>
                       {disk.smartStatus.attributes.map((attr) => (
                         <TableRow key={attr.id}>
                           <TableCell>{attr.id}</TableCell>
                           <TableCell>{attr.name}</TableCell>
                           <TableCell>{attr.value}</TableCell>
                           <TableCell>{attr.worst}</TableCell>
                           <TableCell>{attr.threshold}</TableCell>
                           <TableCell>
                             {getAttributeStatusChip(attr.status)}
                           </TableCell>
                           <TableCell>{attr.raw}</TableCell>
                         </TableRow>
                       ))}
                     </TableBody>
                   </Table>
                 </TableContainer>
               ) : (
                 <EmptyState 
                   title="No SMART attributes available"
                   icon={<AreaChartIcon />}
                 />
               )}
               
               <Box sx={{ marginTop: 3 }}>
                 <Typography variant="h6" gutterBottom>
                   Understanding SMART Attributes
                 </Typography>
                 <Typography variant="body2" component="div">
                  <ul>
                    <li><strong>Value:</strong> Current normalized value (higher is better)</li>
                    <li><strong>Worst:</strong> Worst normalized value recorded</li>
                    <li><strong>Threshold:</strong> Minimum acceptable value (below this indicates failure)</li>
                    <li><strong>Raw Value:</strong> Actual value reported by the drive</li>
                  </ul>
                 </Typography>
               </Box>
             </TabPanel>
           </CardContent>
        </Card>
      )}
     </Box>
  );
};

export default HDDHealth; 
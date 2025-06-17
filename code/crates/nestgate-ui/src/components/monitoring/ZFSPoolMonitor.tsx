import React, { useEffect, useState, useRef } from 'react';
import { 
  Card,
  CardHeader,
  CardContent,
  Typography,
  Box,
  Alert,
  Tabs,
  Tab,
  Divider,
  CircularProgress,
  Chip,
  TableContainer,
  Table,
  TableHead,
  TableBody,
  TableRow,
  TableCell,
  Paper,
  Badge,
  Tooltip,
  LinearProgress
} from '@mui/material';
import {
  Dashboard as DashboardIcon,
  Sync as SyncIcon,
  CheckCircle as CheckCircleIcon,
  Error as ErrorIcon,
  Cancel as CancelIcon,
  Storage as StorageIcon,
  Warning as WarningIcon,
  ArrowUpward as ArrowUpIcon,
  ArrowDownward as ArrowDownIcon,
  AccountTree as BranchesIcon
} from '@mui/icons-material';
import { 
  WebSocketService, 
  WebSocketMessageType, 
  ZFSMetrics,
  WebSocketMessage
} from '../../services/websocket.service';
import { formatBytes, formatPercent } from '../../utils/format.utils';
import StatusChip from '../common/StatusChip';

interface ZFSPoolMonitorProps {
  poolName?: string; // Optional pool name to show only one pool
  showAllPools?: boolean;
  showDetailed?: boolean;
}

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
      id={`zfs-pool-tabpanel-${index}`}
      aria-labelledby={`zfs-pool-tab-${index}`}
      {...other}
    >
      {value === index && (
        <Box sx={{ pt: 2 }}>
          {children}
        </Box>
      )}
    </div>
  );
}

const ZFSPoolMonitor: React.FC<ZFSPoolMonitorProps> = ({
  poolName,
  showAllPools = true,
  showDetailed = true
}) => {
  const [pools, setPools] = useState<Map<string, ZFSMetrics>>(new Map());
  const [loading, setLoading] = useState<boolean>(true);
  const [connectionError, setConnectionError] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<number>(0);
  
  const wsServiceRef = useRef<WebSocketService>(WebSocketService.getInstance());
  const unsubscribeRef = useRef<(() => void) | null>(null);

  // Connect to WebSocket and subscribe to ZFS metrics
  useEffect(() => {
    const connectToWebSocket = async () => {
      setLoading(true);
      setConnectionError(null);
      
      try {
        const wsService = wsServiceRef.current;
        
        await wsService.connect();
        
        // Subscribe to ZFS pool metrics
        const unsubscribe = wsService.subscribe(
          WebSocketMessageType.ZFS_METRICS,
          (message: WebSocketMessage<ZFSMetrics>) => {
            // If poolName is specified, only update that pool
            if (poolName && message.data.poolName !== poolName) {
              return;
            }
            
            setPools(prevPools => {
              const newPools = new Map(prevPools);
              newPools.set(message.data.poolName, message.data);
              return newPools;
            });
            setLoading(false);
          }
        );
        
        // Store the unsubscribe function
        unsubscribeRef.current = unsubscribe;
        
      } catch (error) {
        console.error('Failed to connect to WebSocket:', error);
        setConnectionError('Failed to connect to the ZFS monitoring service');
        setLoading(false);
      }
    };
    
    connectToWebSocket();
    
    // Cleanup on unmount
    return () => {
      if (unsubscribeRef.current) {
        unsubscribeRef.current();
      }
    };
  }, [poolName]);

  const getStatusColor = (status: string): "success" | "warning" | "error" | "default" => {
    switch (status) {
      case 'online': return 'success';
      case 'degraded': return 'warning';
      case 'faulted': return 'error';
      case 'unavailable': return 'error';
      case 'offline': return 'default';
      default: return 'default';
    }
  };

  const getHealthIcon = (health: string) => {
    switch (health) {
      case 'online': 
        return <CheckCircleIcon color="success" />;
      case 'degraded': 
        return <WarningIcon color="warning" />;
      case 'faulted': 
      case 'unavailable':
        return <CancelIcon color="error" />;
      default: 
        return <WarningIcon color="disabled" />;
    }
  };

  const getCapacityColor = (percent: number): "success" | "warning" | "error" => {
    if (percent >= 90) return 'error';
    if (percent >= 75) return 'warning';
    return 'success';
  };

  const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
    setActiveTab(newValue);
  };

  // Transform pools Map to array
  const poolsArray = Array.from(pools.values());

  // Calculate health statistics
  const healthStats = {
    total: poolsArray.length,
    healthy: poolsArray.filter(pool => pool.status === 'online').length,
    degraded: poolsArray.filter(pool => pool.status === 'degraded').length,
    faulted: poolsArray.filter(pool => 
      pool.status === 'faulted' || pool.status === 'unavailable'
    ).length
  };

  if (loading && poolsArray.length === 0) {
    return (
      <Card>
        <CardHeader title="ZFS Pool Monitoring" />
        <CardContent sx={{ textAlign: 'center', py: 5 }}>
          <Box display="flex" flexDirection="column" alignItems="center" gap={2}>
            <CircularProgress size={48} />
            <Typography>Loading ZFS pool metrics...</Typography>
          </Box>
        </CardContent>
      </Card>
    );
  }

  if (connectionError) {
    return (
      <Alert
        severity="error"
        action={
          <SyncIcon 
            fontSize="small" 
            sx={{ cursor: 'pointer' }} 
            onClick={() => window.location.reload()}
          />
        }
      >
        <Typography variant="subtitle1" fontWeight="bold">Connection Error</Typography>
        <Typography variant="body2">{connectionError}</Typography>
      </Alert>
    );
  }

  // If we're showing a single pool and it's loaded
  if (poolName && pools.has(poolName)) {
    const pool = pools.get(poolName)!;
    const usedPercent = (pool.capacityUsed / pool.capacityTotal) * 100;
    
    return (
      <Card>
        <CardHeader 
          title={
            <Box display="flex" alignItems="center" gap={1}>
              <StorageIcon />
              <Typography variant="h6">ZFS Pool: {pool.poolName}</Typography>
              {loading && <CircularProgress size={20} sx={{ ml: 1 }} />}
            </Box>
          }
        />
        <Box sx={{ borderBottom: 1, borderColor: 'divider', px: 2 }}>
          <Tabs value={activeTab} onChange={handleTabChange} aria-label="zfs pool tabs">
            <Tab label="Overview" id="zfs-pool-tab-0" aria-controls="zfs-pool-tabpanel-0" />
            <Tab label="VDevs" id="zfs-pool-tab-1" aria-controls="zfs-pool-tabpanel-1" />
            <Tab label="Performance" id="zfs-pool-tab-2" aria-controls="zfs-pool-tabpanel-2" />
          </Tabs>
        </Box>
        
        <CardContent>
          <TabPanel value={activeTab} index={0}>
            <Box display="flex" flexWrap="wrap" gap={2}>
              {/* Status */}
              <Box sx={{ flex: '1 1 20%', minWidth: { xs: '100%', sm: '45%', md: '22%' } }}>
                <Paper elevation={0} sx={{ p: 2, height: '100%' }}>
                  <Typography variant="subtitle1" gutterBottom>Status</Typography>
                  <Box sx={{ textAlign: 'center' }}>
                    <StatusChip 
                      status={pool.status as any} 
                      sx={{ width: '100%', mb: 1 }} 
                    />
                    {pool.status !== 'online' && (
                      <Alert severity="warning" sx={{ mt: 1 }}>
                        This pool requires attention
                      </Alert>
                    )}
                  </Box>
                </Paper>
              </Box>
              
              {/* Capacity */}
              <Box sx={{ flex: '1 1 20%', minWidth: { xs: '100%', sm: '45%', md: '22%' } }}>
                <Paper elevation={0} sx={{ p: 2, height: '100%' }}>
                  <Typography variant="subtitle1" gutterBottom>Capacity</Typography>
                  <Typography variant="body2" color="text.secondary">
                    {formatPercent(pool.capacityUsed, pool.capacityTotal)} Used
                  </Typography>
                  <Typography variant="h6" color={getCapacityColor(usedPercent)}>
                    {formatBytes(pool.capacityUsed)}
                    <Typography component="span" variant="body2" color="text.secondary">
                      {' '} / {formatBytes(pool.capacityTotal)}
                    </Typography>
                  </Typography>
                  <LinearProgress 
                    variant="determinate" 
                    value={Math.round(usedPercent)} 
                    color={getCapacityColor(usedPercent)}
                    sx={{ mt: 1, height: 6, borderRadius: 3 }}
                  />
                </Paper>
              </Box>
              
              {/* Error Summary */}
              <Box sx={{ flex: '1 1 20%', minWidth: { xs: '100%', sm: '45%', md: '22%' } }}>
                <Paper elevation={0} sx={{ p: 2, height: '100%' }}>
                  <Typography variant="subtitle1" gutterBottom>Error Summary</Typography>
                  <Box display="flex" justifyContent="space-between">
                    <Box textAlign="center" flex="1">
                      <Typography variant="body2" color="text.secondary">Read</Typography>
                      <Typography 
                        variant="h6" 
                        color={pool.diskErrors.read > 0 ? 'error.main' : 'success.main'}
                      >
                        {pool.diskErrors.read}
                      </Typography>
                    </Box>
                    <Box textAlign="center" flex="1">
                      <Typography variant="body2" color="text.secondary">Write</Typography>
                      <Typography 
                        variant="h6" 
                        color={pool.diskErrors.write > 0 ? 'error.main' : 'success.main'}
                      >
                        {pool.diskErrors.write}
                      </Typography>
                    </Box>
                    <Box textAlign="center" flex="1">
                      <Typography variant="body2" color="text.secondary">Checksum</Typography>
                      <Typography 
                        variant="h6" 
                        color={pool.diskErrors.checksum > 0 ? 'error.main' : 'success.main'}
                      >
                        {pool.diskErrors.checksum}
                      </Typography>
                    </Box>
                  </Box>
                </Paper>
              </Box>
              
              {/* Properties */}
              <Box sx={{ flex: '1 1 20%', minWidth: { xs: '100%', sm: '45%', md: '22%' } }}>
                <Paper elevation={0} sx={{ p: 2, height: '100%' }}>
                  <Typography variant="subtitle1" gutterBottom>Properties</Typography>
                  <Typography variant="body2">
                    <strong>Health:</strong> {pool.health}
                  </Typography>
                  <Typography variant="body2">
                    <strong>Deduplication:</strong> {pool.properties?.dedupratio || 'Off'}
                  </Typography>
                  <Typography variant="body2">
                    <strong>Compression:</strong> {pool.properties?.compression || 'Off'}
                  </Typography>
                </Paper>
              </Box>
            </Box>
            
            {pool.status !== 'online' && (
              <Alert
                severity="warning"
                sx={{ mt: 2 }}
              >
                <Typography variant="subtitle2">
                  {`Pool ${pool.poolName} is ${pool.status}`}
                </Typography>
                <Typography variant="body2">
                  This pool may have reliability issues. Check the VDevs tab for more details.
                </Typography>
              </Alert>
            )}
            
            {/* Storage Allocation */}
            <Paper elevation={0} sx={{ p: 2, mt: 2 }}>
              <Typography variant="subtitle1" gutterBottom>Storage Allocation</Typography>
              <Box display="flex" flexWrap="wrap" gap={2}>
                <Box sx={{ flex: '1 1 30%', minWidth: { xs: '100%', sm: '30%' } }}>
                  <Typography variant="body2" color="text.secondary">
                    Free Space
                  </Typography>
                  <Typography variant="h6">
                    {formatBytes(pool.capacityTotal - pool.capacityUsed)}
                  </Typography>
                </Box>
                <Box sx={{ flex: '1 1 30%', minWidth: { xs: '100%', sm: '30%' } }}>
                  <Typography variant="body2" color="text.secondary">
                    Used Space
                  </Typography>
                  <Typography variant="h6">
                    {formatBytes(pool.capacityUsed)}
                  </Typography>
                </Box>
                <Box sx={{ flex: '1 1 30%', minWidth: { xs: '100%', sm: '30%' } }}>
                  <Typography variant="body2" color="text.secondary">
                    Total Size
                  </Typography>
                  <Typography variant="h6">
                    {formatBytes(pool.capacityTotal)}
                  </Typography>
                </Box>
              </Box>
            </Paper>
          </TabPanel>
          
          <TabPanel value={activeTab} index={1}>
            <TableContainer component={Paper} elevation={0}>
              <Table size="small">
                <TableHead>
                  <TableRow>
                    <TableCell>VDev Name</TableCell>
                    <TableCell>Status</TableCell>
                    <TableCell>Read Errors</TableCell>
                    <TableCell>Write Errors</TableCell>
                    <TableCell>Checksum Errors</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {pool.children.map((vdev) => (
                    <React.Fragment key={vdev.name}>
                      <TableRow>
                        <TableCell>
                          <Box display="flex" alignItems="center" gap={1}>
                            <BranchesIcon />
                            <Typography variant="body1" fontWeight="bold">
                              {vdev.name}
                            </Typography>
                          </Box>
                        </TableCell>
                        <TableCell>
                          <StatusChip status={vdev.status as any} />
                        </TableCell>
                        <TableCell>
                          <Typography 
                            color={vdev.errors.read > 0 ? 'error.main' : 'success.main'}
                          >
                            {vdev.errors.read}
                          </Typography>
                        </TableCell>
                        <TableCell>
                          <Typography 
                            color={vdev.errors.write > 0 ? 'error.main' : 'success.main'}
                          >
                            {vdev.errors.write}
                          </Typography>
                        </TableCell>
                        <TableCell>
                          <Typography 
                            color={vdev.errors.checksum > 0 ? 'error.main' : 'success.main'}
                          >
                            {vdev.errors.checksum}
                          </Typography>
                        </TableCell>
                      </TableRow>
                      {/* Device Details Section */}
                      <TableRow>
                        <TableCell colSpan={5} sx={{ py: 0 }}>
                          <Box sx={{ p: 2 }}>
                            <Typography variant="subtitle2" gutterBottom>
                              VDev Details
                            </Typography>
                                <Paper 
                                  elevation={1} 
                                  sx={{ 
                                    p: 2, 
                                    minWidth: { xs: '100%', sm: '45%', md: '30%' } 
                                  }}
                                >
                                  <Typography variant="subtitle2" gutterBottom>
                                {vdev.name}
                                  </Typography>
                                  <Typography variant="body2">
                                <strong>Status:</strong> {vdev.status}
                                  </Typography>
                                  <Typography variant="body2">
                                <strong>Read Errors:</strong> {vdev.errors.read}
                                  </Typography>
                                  <Typography variant="body2">
                                <strong>Write Errors:</strong> {vdev.errors.write}
                                  </Typography>
                                  <Typography variant="body2">
                                <strong>Checksum Errors:</strong> {vdev.errors.checksum}
                                  </Typography>
                                </Paper>
                          </Box>
                        </TableCell>
                      </TableRow>
                    </React.Fragment>
                  ))}
                </TableBody>
              </Table>
            </TableContainer>
          </TabPanel>
          
          <TabPanel value={activeTab} index={2}>
            <Box display="flex" flexWrap="wrap" gap={2}>
              {/* IOPS */}
              <Box sx={{ flex: '1 1 45%', minWidth: { xs: '100%', sm: '45%' } }}>
                <Paper elevation={0} sx={{ p: 2 }}>
                  <Typography variant="subtitle1" gutterBottom>IOPS</Typography>
                  <Box display="flex" flexWrap="wrap" gap={2}>
                    <Box sx={{ flex: '1 1 45%', minWidth: { xs: '100%', sm: '45%' } }}>
                      <Box display="flex" alignItems="center" gap={0.5}>
                        <ArrowDownIcon fontSize="small" color="success" />
                        <Typography variant="body2" color="text.secondary">
                          Read IOPS
                        </Typography>
                      </Box>
                      <Typography variant="h6">
                        {pool.performance?.iops?.read || 0}
                      </Typography>
                    </Box>
                    <Box sx={{ flex: '1 1 45%', minWidth: { xs: '100%', sm: '45%' } }}>
                      <Box display="flex" alignItems="center" gap={0.5}>
                        <ArrowUpIcon fontSize="small" color="primary" />
                        <Typography variant="body2" color="text.secondary">
                          Write IOPS
                        </Typography>
                      </Box>
                      <Typography variant="h6">
                        {pool.performance?.iops?.write || 0}
                      </Typography>
                    </Box>
                  </Box>
                </Paper>
              </Box>
              
              {/* Throughput */}
              <Box sx={{ flex: '1 1 45%', minWidth: { xs: '100%', sm: '45%' } }}>
                <Paper elevation={0} sx={{ p: 2 }}>
                  <Typography variant="subtitle1" gutterBottom>Throughput</Typography>
                  <Box display="flex" flexWrap="wrap" gap={2}>
                    <Box sx={{ flex: '1 1 45%', minWidth: { xs: '100%', sm: '45%' } }}>
                      <Box display="flex" alignItems="center" gap={0.5}>
                        <ArrowDownIcon fontSize="small" color="success" />
                        <Typography variant="body2" color="text.secondary">
                          Read Throughput
                        </Typography>
                      </Box>
                      <Typography variant="h6">
                        {formatBytes(pool.performance?.throughput?.read || 0)}
                      </Typography>
                    </Box>
                    <Box sx={{ flex: '1 1 45%', minWidth: { xs: '100%', sm: '45%' } }}>
                      <Box display="flex" alignItems="center" gap={0.5}>
                        <ArrowUpIcon fontSize="small" color="primary" />
                        <Typography variant="body2" color="text.secondary">
                          Write Throughput
                        </Typography>
                      </Box>
                      <Typography variant="h6">
                        {formatBytes(pool.performance?.throughput?.write || 0)}
                      </Typography>
                    </Box>
                  </Box>
                </Paper>
              </Box>
            </Box>
            
            {/* Latency */}
            <Paper elevation={0} sx={{ p: 2, mt: 2 }}>
              <Typography variant="subtitle1" gutterBottom>Latency</Typography>
              <Box display="flex" flexWrap="wrap" gap={2}>
                <Box sx={{ flex: '1 1 30%', minWidth: { xs: '100%', sm: '30%' } }}>
                  <Typography variant="body2" color="text.secondary">
                    Read Latency
                  </Typography>
                  <Typography variant="h6">
                    {(pool.performance?.latency?.read || 0).toFixed(2)}
                    <Typography component="span" variant="body2">
                      ms
                    </Typography>
                  </Typography>
                </Box>
                <Box sx={{ flex: '1 1 30%', minWidth: { xs: '100%', sm: '30%' } }}>
                  <Typography variant="body2" color="text.secondary">
                    Write Latency
                  </Typography>
                  <Typography variant="h6">
                    {(pool.performance?.latency?.write || 0).toFixed(2)}
                    <Typography component="span" variant="body2">
                      ms
                    </Typography>
                  </Typography>
                </Box>
                <Box sx={{ flex: '1 1 30%', minWidth: { xs: '100%', sm: '30%' } }}>
                  <Typography variant="body2" color="text.secondary">
                    Sync Latency
                  </Typography>
                  <Typography variant="h6">
                    {(pool.performance?.latency?.sync || 0).toFixed(2)}
                    <Typography component="span" variant="body2">
                      ms
                    </Typography>
                  </Typography>
                </Box>
              </Box>
            </Paper>
            
            {/* Cache Hit Ratio */}
            <Paper elevation={0} sx={{ p: 2, mt: 2 }}>
              <Typography variant="subtitle1" gutterBottom>Cache Hit Ratio</Typography>
              <Box display="flex" flexWrap="wrap" gap={2}>
                <Box sx={{ flex: '1 1 45%', minWidth: { xs: '100%', sm: '45%' } }}>
                  <Typography variant="body2" color="text.secondary">
                    ARC Hit Ratio
                  </Typography>
                  <Typography variant="h6">
                    {pool.performance?.cache ? formatPercent(
                      pool.performance.cache.arcHits || 0,
                      (pool.performance.cache.arcHits || 0) + (pool.performance.cache.arcMisses || 0)
                    ) : '0%'}
                  </Typography>
                  <LinearProgress
                    variant="determinate"
                    value={pool.performance?.cache ? Math.round(
                      ((pool.performance.cache.arcHits || 0) / 
                      ((pool.performance.cache.arcHits || 0) + (pool.performance.cache.arcMisses || 0) || 1)) * 100
                    ) : 0}
                    sx={{ mt: 1, height: 6, borderRadius: 3 }}
                  />
                </Box>
                <Box sx={{ flex: '1 1 45%', minWidth: { xs: '100%', sm: '45%' } }}>
                  <Typography variant="body2" color="text.secondary">
                    L2ARC Hit Ratio
                  </Typography>
                  <Typography variant="h6">
                    {pool.performance?.cache ? formatPercent(
                      pool.performance.cache.l2arcHits || 0,
                      (pool.performance.cache.l2arcHits || 0) + (pool.performance.cache.l2arcMisses || 0)
                    ) : '0%'}
                  </Typography>
                  <LinearProgress
                    variant="determinate"
                    value={pool.performance?.cache ? Math.round(
                      ((pool.performance.cache.l2arcHits || 0) / 
                      ((pool.performance.cache.l2arcHits || 0) + (pool.performance.cache.l2arcMisses || 0) || 1)) * 100
                    ) : 0}
                    sx={{ mt: 1, height: 6, borderRadius: 3 }}
                  />
                </Box>
              </Box>
            </Paper>
          </TabPanel>
        </CardContent>
      </Card>
    );
  }

  // For all pools view
  return (
    <Card>
      <CardHeader title="ZFS Pool Health" />
      <CardContent>
        {/* Health summary */}
        <Box display="flex" flexWrap="wrap" gap={2} mb={2}>
          <Box sx={{ flex: '1 1 33%', minWidth: { xs: '100%', sm: '33%' } }}>
            <Typography variant="subtitle1" gutterBottom>Total Pools</Typography>
            <Typography variant="h6">{healthStats.total}</Typography>
          </Box>
          <Box sx={{ flex: '1 1 33%', minWidth: { xs: '100%', sm: '33%' } }}>
            <Typography variant="subtitle1" gutterBottom>Healthy</Typography>
            <Typography variant="h6" color="success.main">{healthStats.healthy}</Typography>
          </Box>
          <Box sx={{ flex: '1 1 33%', minWidth: { xs: '100%', sm: '33%' } }}>
            <Typography variant="subtitle1" gutterBottom>Degraded</Typography>
            <Typography variant="h6" color="warning.main">{healthStats.degraded}</Typography>
          </Box>
          <Box sx={{ flex: '1 1 33%', minWidth: { xs: '100%', sm: '33%' } }}>
            <Typography variant="subtitle1" gutterBottom>Faulted</Typography>
            <Typography variant="h6" color="error.main">{healthStats.faulted}</Typography>
          </Box>
        </Box>

        {/* Warnings if any */}
        {healthStats.faulted > 0 && (
          <Alert severity="error">
            <Typography variant="subtitle1" fontWeight="bold">Critical Pool Issues Detected</Typography>
            <Typography variant="body2">One or more pools are in a faulted or unavailable state. Immediate action recommended.</Typography>
          </Alert>
        )}
        
        {healthStats.degraded > 0 && !healthStats.faulted && (
          <Alert severity="warning">
            <Typography variant="subtitle1" fontWeight="bold">Pool Warnings Detected</Typography>
            <Typography variant="body2">One or more pools are in a degraded state. Please investigate at your convenience.</Typography>
          </Alert>
        )}

        {/* Pools table */}
        <TableContainer component={Paper} elevation={0}>
          <Table size="small">
            <TableHead>
              <TableRow>
                <TableCell>Pool Name</TableCell>
                <TableCell>Status</TableCell>
                <TableCell>Capacity</TableCell>
                <TableCell>Read IOPS</TableCell>
                <TableCell>Write IOPS</TableCell>
                <TableCell>Errors</TableCell>
              </TableRow>
            </TableHead>
            <TableBody>
              {poolsArray.map((pool) => (
                <TableRow key={pool.poolName}>
                  <TableCell>{pool.poolName}</TableCell>
                  <TableCell>
                    <StatusChip status={pool.status as any} />
                  </TableCell>
                  <TableCell>
                    {formatPercent(pool.capacityUsed, pool.capacityTotal)} Used
                  </TableCell>
                  <TableCell>{pool.performance?.iops?.read || 0}</TableCell>
                  <TableCell>{pool.performance?.iops?.write || 0}</TableCell>
                  <TableCell>
                    {pool.diskErrors.read + pool.diskErrors.write + pool.diskErrors.checksum > 0 ? (
                      <Badge badgeContent={pool.diskErrors.read + pool.diskErrors.write + pool.diskErrors.checksum} color="error" />
                    ) : (
                      <CheckCircleIcon color="success" />
                    )}
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </TableContainer>
      </CardContent>
    </Card>
  );
};

export default ZFSPoolMonitor; 
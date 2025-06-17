import React, { useState, useEffect } from 'react';
import { 
  Card, 
  CardContent,
  CardHeader,
  CircularProgress, 
  Button, 
  Alert, 
  Typography, 
  List, 
  ListItem,
  ListItemText,
  ListItemIcon,
  Divider, 
  Box,
  LinearProgress,
  Paper,
  IconButton
} from '@mui/material';
import { 
  Refresh as ReloadIcon, 
  CheckCircle as CheckCircleIcon, 
  Warning as WarningIcon, 
  Cancel as CloseCircleIcon,
  Storage as HddIcon,
  CloudQueue as CloudServerIcon,
  Dashboard as DashboardIcon,
  Computer as DatabaseIcon
} from '@mui/icons-material';

// Import services
import { ZfsPoolService, ZfsPool } from '../../services/zfs-pool.service';
import { NfsService } from '../../services/nfs.service';
import { SmbService } from '../../services/smb.service';
import { TelemetryService, DiskHealth } from '../../services/telemetry.service';
import { formatCapacity } from '../../utils/format';
import { DataSourceType } from '../../utils/env';
import StatusChip from '../common/StatusChip';

// Use ZfsPool type directly instead of duplicating it
type Pool = ZfsPool;

interface SystemHealth {
  status: 'healthy' | 'warning' | 'critical';
  cpu: number;
  memory: number;
  temperature: number;
  diskHealth: Array<{
    device: string;
    status: string;
    temperature: number;
    smart: any;
  }>;
  // Optional fields for backward compatibility
  cpuUsage?: number;
  memoryUsage?: number;
  hostname?: string;
  uptime?: number;
}

interface ServiceStatus {
  status: 'running' | 'stopped' | 'error';
  version: string;
  uptime: string;
  [key: string]: any;
}

interface PerformanceMetrics {
  timestamp: number;
  iops: {
    read: number;
    write: number;
    total: number;
  };
  throughput: {
    read: number;
    write: number;
    total: number;
  };
  latency: {
    read: number;
    write: number;
    average: number;
  };
}

// Statistic component to replace Ant Design's Statistic
interface StatisticProps {
  title: string;
  value: string | number;
  suffix?: string;
  prefix?: React.ReactNode;
  color?: string;
}

const Statistic: React.FC<StatisticProps> = ({ title, value, suffix, prefix, color }) => (
  <Box sx={{ textAlign: 'center', padding: 1 }}>
    <Typography variant="body2" color="text.secondary" gutterBottom>
      {title}
    </Typography>
    <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'center', gap: 0.5 }}>
      {prefix}
      <Typography variant="h6" sx={{ color }}>
        {value}{suffix}
      </Typography>
    </Box>
  </Box>
);

export const NasMetrics: React.FC = () => {
  // State variables
  const [pools, setPools] = useState<Pool[]>([]);
  const [systemHealth, setSystemHealth] = useState<SystemHealth | null>(null);
  const [nfsStatus, setNfsStatus] = useState<ServiceStatus | null>(null);
  const [smbStatus, setSmbStatus] = useState<ServiceStatus | null>(null);
  const [performanceMetrics, setPerformanceMetrics] = useState<PerformanceMetrics | null>(null);
  const [diskHealth, setDiskHealth] = useState<Record<string, DiskHealth>>({});
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [isLiveUpdating, setIsLiveUpdating] = useState<boolean>(true);

  // Calculate storage metrics with safety checks
  const totalCapacity = pools.reduce((sum, pool) => sum + (Number(pool.size) || 0), 0);
  const usedCapacity = pools.reduce((sum, pool) => sum + (Number(pool.used) || 0), 0);
  const freeCapacity = pools.reduce((sum, pool) => sum + (Number(pool.free) || 0), 0);
  const usagePercentage = totalCapacity > 0 ? Math.round((usedCapacity / totalCapacity) * 100) : 0;

  // Setup WebSocket listeners for real-time updates
  useEffect(() => {
    if (isLiveUpdating) {
      // Health metrics listener
      const healthListener = (data: any) => {
        const systemHealth: SystemHealth = {
          status: data.status === 'healthy' ? 'healthy' : 
                  data.status === 'warning' ? 'warning' : 'critical',
          cpu: data.cpu,
          memory: data.memory,
          temperature: data.temperature,
          diskHealth: data.diskHealth,
          cpuUsage: data.cpuUsage,
          memoryUsage: data.memoryUsage,
          hostname: data.hostname,
          uptime: data.uptime
        };
        
        setSystemHealth(systemHealth);
      };
      
      // Performance metrics listener
      const performanceListener = (data: PerformanceMetrics) => {
        setPerformanceMetrics(data);
      };
      
      // Disk health listener
      const diskHealthListener = (data: Record<string, DiskHealth>) => {
        setDiskHealth(data);
      };
      
      // ZFS pool listener
      const poolListener = (data: Pool[]) => {
        setPools(data);
      };
      
      // Register all listeners
      TelemetryService.addListener('SystemHealth', healthListener);
      TelemetryService.addListener('Performance', performanceListener);
      TelemetryService.addListener('DiskHealth', diskHealthListener);
      TelemetryService.addListener('ZfsPool', poolListener);
      
      // Connect to WebSocket
      TelemetryService.connectWebSocket();
      
      // Cleanup function
      return () => {
        TelemetryService.removeListener('SystemHealth', healthListener);
        TelemetryService.removeListener('Performance', performanceListener);
        TelemetryService.removeListener('DiskHealth', diskHealthListener);
        TelemetryService.removeListener('ZfsPool', poolListener);
      };
    }
    
    // Return a no-op cleanup function when not live updating
    return () => {};
  }, [isLiveUpdating]);

  // Load all data initially
  const loadData = async (): Promise<void> => {
    setLoading(true);
    setError(null);
    
    try {
      // Get pools data
      const poolsData = await ZfsPoolService.getPools();
      setPools(poolsData);
      
      // Get system health
      const healthData = await TelemetryService.getSystemHealth();
      const systemHealthData: SystemHealth = {
        status: healthData.status === 'healthy' ? 'healthy' : 
                healthData.status === 'warning' ? 'warning' : 'critical',
        cpu: healthData.cpu,
        memory: healthData.memory,
        temperature: healthData.temperature,
        diskHealth: healthData.diskHealth,
        cpuUsage: healthData.cpuUsage,
        memoryUsage: healthData.memoryUsage,
        hostname: healthData.hostname,
        uptime: healthData.uptime
      };
      setSystemHealth(systemHealthData);
      
      // Get service status
      const nfsData = await NfsService.getStatus();
      setNfsStatus(nfsData);
      
      const smbData = await SmbService.getStatus();
      setSmbStatus(smbData);
      
      // Get performance metrics
      const metricsData = await TelemetryService.getPerformanceMetrics();
      setPerformanceMetrics(metricsData);
      
      // Get disk health
      const diskHealthData = await TelemetryService.getDiskHealth();
      setDiskHealth(diskHealthData);
    } catch (err: any) {
      console.error('Error loading data:', err);
      
      // Check for authentication errors
      if (err.response && err.response.status === 401) {
        setError('Authentication required. Please log in.');
      } else {
        setError('Failed to load system metrics. Please try again.');
      }
    } finally {
      setLoading(false);
    }
  };

  // Load data on component mount
  useEffect(() => {
    loadData();
  }, []);

  const toggleLiveUpdates = () => {
    setIsLiveUpdating(!isLiveUpdating);
  };

  const getHealthIcon = (status: string) => {
    switch (status) {
      case 'healthy':
        return <CheckCircleIcon sx={{ color: 'success.main' }} />;
      case 'warning':
        return <WarningIcon sx={{ color: 'warning.main' }} />;
      case 'critical':
        return <CloseCircleIcon sx={{ color: 'error.main' }} />;
      default:
        return <WarningIcon sx={{ color: 'warning.main' }} />;
    }
  };

  const getStatusColor = (status: string): 'success' | 'warning' | 'error' => {
    switch (status) {
      case 'running':
        return 'success';
      case 'stopped':
        return 'warning';
      case 'error':
        return 'error';
      default:
        return 'warning';
    }
  };

  return (
    <Box sx={{ padding: 3 }}>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 3 }}>
        <Typography variant="h4" component="h1">
          NAS System Metrics
        </Typography>
        <Box sx={{ display: 'flex', gap: 1 }}>
          <Button 
            variant={isLiveUpdating ? "contained" : "outlined"}
            onClick={toggleLiveUpdates}
            color={isLiveUpdating ? "primary" : "inherit"}
          >
            {isLiveUpdating ? "Live" : "Paused"}
          </Button>
          <Button 
            variant="contained"
            startIcon={<ReloadIcon />} 
            onClick={loadData}
            disabled={loading}
          >
            Refresh
          </Button>
        </Box>
      </Box>

      {error && (
        <Alert severity="error" sx={{ marginBottom: 2 }}>
          {error}
        </Alert>
      )}

      {loading ? (
        <Box sx={{ display: 'flex', justifyContent: 'center', padding: 4 }}>
          <CircularProgress />
        </Box>
      ) : (
        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
          <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 2 }}>
          {/* Storage Usage Section */}
            <Box sx={{ flex: '1 1 400px', minWidth: 400 }}>
              <Card sx={{ height: '100%' }}>
                <CardHeader title="Storage Usage" />
                <CardContent>
                  <Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 2 }}>
                    <Box sx={{ position: 'relative', display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
                      <CircularProgress 
                        variant="determinate" 
                        value={usagePercentage} 
                        size={120}
                        thickness={4}
                        color={usagePercentage > 90 ? "error" : "primary"}
                      />
                      <Box sx={{ 
                        position: 'absolute',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                      }}>
                        <Typography variant="h6">
                          {usagePercentage}%
                        </Typography>
                      </Box>
                    </Box>
                    
                    <Box sx={{ display: 'flex', gap: 3, flexWrap: 'wrap', justifyContent: 'center' }}>
                <Statistic title="Total Capacity" value={formatCapacity(totalCapacity)} />
                <Statistic title="Used Space" value={formatCapacity(usedCapacity)} />
                <Statistic title="Free Space" value={formatCapacity(freeCapacity)} />
                    </Box>
                  </Box>
                  
                  <Divider sx={{ marginY: 2 }}>
                    <Typography variant="subtitle2">Storage Pools</Typography>
                  </Divider>
                  
                  <List>
                    {pools.map(pool => (
                      <ListItem key={pool.name} sx={{ paddingX: 0 }}>
                        <ListItemIcon>
                          <HddIcon />
                        </ListItemIcon>
                        <ListItemText
                          primary={
                            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                              <Typography variant="subtitle2">{pool.name}</Typography>
                              <StatusChip 
                                status={pool.health === 'ONLINE' ? 'success' : 'error'} 
                                label={pool.health} 
                        size="small" 
                              />
                            </Box>
                          }
                          secondary={
                            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, marginTop: 1 }}>
                              <Typography variant="body2" color="text.secondary">
                                {formatCapacity(pool.size || 0)}
                              </Typography>
                              <LinearProgress 
                                variant="determinate"
                                value={pool.size > 0 ? Math.round(((pool.used || 0) / pool.size) * 100) : 0}
                                color={pool.size > 0 && ((pool.used || 0) / pool.size) > 0.9 ? "error" : "primary"}
                                sx={{ flex: 1, height: 6, borderRadius: 1 }}
                              />
                            </Box>
                          }
                        />
                      </ListItem>
                    ))}
                  </List>
                </CardContent>
            </Card>
            </Box>

          {/* System Health Section */}
            <Box sx={{ flex: '1 1 400px', minWidth: 400 }}>
              <Card sx={{ height: '100%' }}>
                <CardHeader title="System Health" />
                <CardContent>
              {systemHealth && (
                <>
                      <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, marginBottom: 3 }}>
                    {getHealthIcon(systemHealth.status)}
                        <Typography variant="h6" sx={{ 
                          color: systemHealth.status === 'healthy' ? 'success.main' : 
                                 systemHealth.status === 'warning' ? 'warning.main' : 'error.main'
                        }}>
                      System is {systemHealth.status}
                        </Typography>
                      </Box>

                      <Box sx={{ display: 'flex', gap: 2, flexWrap: 'wrap' }}>
                      <Statistic 
                      title="CPU Usage" 
                        value={systemHealth.cpuUsage || systemHealth.cpu || 0}
                          prefix={<DashboardIcon />}
                        suffix="%"
                          color={(systemHealth.cpuUsage || systemHealth.cpu || 0) > 80 ? 'error.main' : 'success.main'}
                      />
                      <Statistic 
                        title="Memory Usage" 
                        value={systemHealth.memoryUsage || systemHealth.memory || 0}
                          prefix={<DatabaseIcon />}
                        suffix="%"
                          color={(systemHealth.memoryUsage || systemHealth.memory || 0) > 80 ? 'error.main' : 'success.main'}
                      />
                      </Box>
                </>
              )}
                </CardContent>
            </Card>
            </Box>
          </Box>

          <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 2 }}>
          {/* Performance Section */}
            <Box sx={{ flex: '1 1 400px', minWidth: 400 }}>
              <Card sx={{ height: '100%' }}>
                <CardHeader title="Performance Metrics" />
                <CardContent>
              {performanceMetrics && (
                    <Box sx={{ display: 'flex', gap: 2, flexWrap: 'wrap' }}>
                      <Statistic title="Read Throughput" value={performanceMetrics.throughput.read} suffix=" MB/s" />
                      <Statistic title="Write Throughput" value={performanceMetrics.throughput.write} suffix=" MB/s" />
                      <Statistic title="Read Latency" value={performanceMetrics.latency.read} suffix=" ms" />
                      <Statistic title="Write Latency" value={performanceMetrics.latency.write} suffix=" ms" />
                  <Statistic title="IOPS" value={performanceMetrics.iops.total} />
                    </Box>
              )}
                </CardContent>
            </Card>
            </Box>
          
          {/* Service Status Section */}
            <Box sx={{ flex: '1 1 400px', minWidth: 400 }}>
              <Card sx={{ height: '100%' }}>
                <CardHeader title="Service Status" />
                <CardContent>
                  <Box sx={{ display: 'flex', gap: 2, flexWrap: 'wrap' }}>
                    <Box sx={{ flex: '1 1 200px' }}>
                      <Paper variant="outlined" sx={{ padding: 2 }}>
                        <Typography variant="h6" gutterBottom>NFS Service</Typography>
                    {nfsStatus && (
                      <>
                            <StatusChip 
                              status={getStatusColor(nfsStatus.status)} 
                              label={nfsStatus.status.toUpperCase()} 
                              size="small"
                            />
                            <Box sx={{ marginTop: 1 }}>
                              <Typography variant="body2">Version: {nfsStatus.version}</Typography>
                              <Typography variant="body2">Uptime: {nfsStatus.uptime}</Typography>
                            </Box>
                      </>
                    )}
                      </Paper>
                    </Box>
                    <Box sx={{ flex: '1 1 200px' }}>
                      <Paper variant="outlined" sx={{ padding: 2 }}>
                        <Typography variant="h6" gutterBottom>SMB Service</Typography>
                    {smbStatus && (
                      <>
                            <StatusChip 
                              status={getStatusColor(smbStatus.status)} 
                              label={smbStatus.status.toUpperCase()} 
                              size="small"
                            />
                            <Box sx={{ marginTop: 1 }}>
                              <Typography variant="body2">Version: {smbStatus.version}</Typography>
                              <Typography variant="body2">Uptime: {smbStatus.uptime}</Typography>
                            </Box>
                      </>
                    )}
                      </Paper>
                    </Box>
                  </Box>
                </CardContent>
                  </Card>
            </Box>
          </Box>

          {/* Disk Health Section */}
          <Box sx={{ width: '100%' }}>
            <Card>
              <CardHeader title="Disk Health" />
              <CardContent>
                <Box sx={{ display: 'flex', gap: 2, flexWrap: 'wrap' }}>
                {Object.entries(diskHealth).map(([device, info]) => (
                    <Box key={device} sx={{ flex: '1 1 300px', minWidth: 300 }}>
                      <Paper variant="outlined" sx={{ padding: 2 }}>
                        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, marginBottom: 1 }}>
                          <HddIcon />
                          <Typography variant="subtitle1">{info.model}</Typography>
                          <StatusChip 
                            status={info.status === 'healthy' ? 'success' : 
                                info.status === 'warning' ? 'warning' : 'error'}
                            label={info.status.toUpperCase()} 
                            size="small"
                          />
                        </Box>
                        <Box>
                          <Typography variant="body2">Device: {info.device}</Typography>
                          <Typography variant="body2">Serial: {info.serial}</Typography>
                          <Typography variant="body2">Temperature: {info.temperature}°C</Typography>
                          <Typography variant="body2">SMART Attributes: {info.smartAttributes.length}</Typography>
                        {info.smartAttributes.some((attr: { status: string }) => attr.status !== 'healthy') && (
                            <Typography variant="body2" color="warning.main">
                              Warning: Some SMART attributes need attention
                            </Typography>
                          )}
                        </Box>
                      </Paper>
                    </Box>
                  ))}
                </Box>
              </CardContent>
            </Card>
          </Box>
        </Box>
      )}
    </Box>
  );
}; 
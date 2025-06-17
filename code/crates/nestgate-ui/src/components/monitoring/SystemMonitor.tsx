import React, { useState } from 'react';
import { 
  Card, 
  CardContent,
  CardHeader,
  CircularProgress, 
  LinearProgress,
  Typography, 
  Divider, 
  Alert, 
  Box,
  Paper,
  IconButton
} from '@mui/material';
import {
  Dashboard as DashboardIcon,
  Computer as DesktopIcon,
  Storage as DatabaseIcon,
  SwapHoriz as SwapIcon,
  ArrowUpward as ArrowUpIcon,
  ArrowDownward as ArrowDownIcon,
  Sync as SyncIcon,
  FlashOn as ThunderboltIcon,
  Warning as WarningIcon,
  CheckCircle as CheckCircleIcon,
  AccessTime as ClockCircleIcon,
  Refresh as LoadingIcon
} from '@mui/icons-material';
import { 
  WebSocketMessageType, 
  SystemMetrics,
  WebSocketMessage
} from '../../services/websocket.service';
import { formatCapacity as formatBytes, formatUptime as formatDuration } from '../../utils/format';
import useWebSocket, { MessageSubscription } from '../../hooks/useWebSocket';
import { DataSourceBanner } from '../common';

// Custom Statistic component to replace Ant Design's Statistic
interface StatisticProps {
  title?: string;
  value: string | number;
  suffix?: string;
  prefix?: React.ReactNode;
  color?: string;
  size?: 'small' | 'medium' | 'large';
}

const Statistic: React.FC<StatisticProps> = ({ 
  title, 
  value, 
  suffix, 
  prefix, 
  color,
  size = 'medium'
}) => {
  const fontSize = size === 'small' ? '1rem' : size === 'large' ? '1.5rem' : '1.2rem';
  
  return (
    <Box sx={{ textAlign: 'center' }}>
      {title && (
        <Typography variant="body2" color="text.secondary" gutterBottom>
          {title}
        </Typography>
      )}
      <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'center', gap: 0.5 }}>
        {prefix}
        <Typography variant="h6" sx={{ color, fontSize }}>
          {value}{suffix}
        </Typography>
      </Box>
    </Box>
  );
};

interface SystemMonitorProps {
  refreshInterval?: number;
  showDetailedStats?: boolean;
}

const SystemMonitor: React.FC<SystemMonitorProps> = ({
  refreshInterval = 2000,
  showDetailedStats = true
}) => {
  const [systemMetrics, setSystemMetrics] = useState<SystemMetrics | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [connectionError, setConnectionError] = useState<string | null>(null);
  
  // Use the WebSocket hook instead of direct service reference
  const { 
    connected, 
    dataSource, 
    mockReason, 
    isMock
  } = useWebSocket({
    autoConnect: true,
    subscriptions: [
      {
        type: WebSocketMessageType.SYSTEM_METRICS,
        handler: (message: WebSocketMessage<SystemMetrics>) => {
          setSystemMetrics(message.data);
          setLoading(false);
        }
      } as MessageSubscription<SystemMetrics>
    ],
    onError: (error) => {
      setConnectionError(error.message || 'Failed to connect to monitoring service');
    }
  });
  
  // Get status color based on value
  const getStatusColor = (value: number, thresholds: [number, number]): string => {
    const [warningThreshold, criticalThreshold] = thresholds;
    if (value >= criticalThreshold) return '#f44336'; // Red
    if (value >= warningThreshold) return '#ff9800'; // Orange
    return '#4caf50'; // Green
  };
  
  // Format load average to 2 decimal places
  const formatLoadAvg = (load: number): string => {
    return load.toFixed(2);
  };
  
  if (loading) {
    return (
      <Card>
        <CardContent>
          <Box sx={{ textAlign: 'center', padding: 5 }}>
            <CircularProgress size={48} />
            <Typography variant="body1" sx={{ marginTop: 2 }}>
              Loading system metrics...
            </Typography>
          </Box>
        </CardContent>
      </Card>
    );
  }
  
  if (connectionError && !isMock) {
    return (
      <Alert
        severity="error"
        action={
          <IconButton 
            color="inherit" 
            size="small" 
            onClick={() => window.location.reload()}
          >
            <SyncIcon />
          </IconButton>
        }
      >
        <Typography variant="subtitle2">Connection Error</Typography>
        <Typography variant="body2">{connectionError}</Typography>
      </Alert>
    );
  }
  
  // If we're connected but don't have metrics yet
  if (!systemMetrics) {
    return (
      <Card>
        <CardContent>
          <Box sx={{ textAlign: 'center', padding: 3 }}>
            <CircularProgress size={24} />
            <Typography variant="body2" color="text.secondary" sx={{ marginTop: 1 }}>
              Waiting for system metrics...
            </Typography>
          </Box>
        </CardContent>
      </Card>
    );
  }
  
  return (
    <Box className="system-monitor">
      {/* Display mock data banner if using mock data */}
      {isMock && (
        <DataSourceBanner 
          dataSource={dataSource} 
          mockReason={mockReason}
          serviceName="System Monitoring"
        />
      )}
      
      <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 2, marginBottom: 2 }}>
        {/* CPU Usage */}
        <Box sx={{ flex: '1 1 300px', minWidth: 300 }}>
          <Card>
            <CardHeader 
            title={
                <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                  <DesktopIcon />
                  <Typography variant="h6">CPU Usage</Typography>
                </Box>
              }
            />
            <CardContent>
              <Box sx={{ textAlign: 'center' }}>
                <Box sx={{ position: 'relative', display: 'inline-flex', marginBottom: 2 }}>
                  <CircularProgress
                    variant="determinate"
                    value={Math.round(systemMetrics.cpuUsage)}
                    size={120}
                    thickness={4}
                    sx={{ 
                      color: getStatusColor(systemMetrics.cpuUsage, [70, 90]),
                    }}
                  />
                  <Box sx={{
                    position: 'absolute',
                    top: 0,
                    left: 0,
                    bottom: 0,
                    right: 0,
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                  }}>
                    <Typography variant="h6" component="div" color="text.secondary">
                      {`${Math.round(systemMetrics.cpuUsage)}%`}
                    </Typography>
                  </Box>
                </Box>
                <Statistic 
                  value={systemMetrics.cpuUsage.toFixed(1)} 
                  suffix="%" 
                  color={getStatusColor(systemMetrics.cpuUsage, [70, 90])}
                />
              </Box>
            </CardContent>
          </Card>
        </Box>
        
        {/* Memory Usage */}
        <Box sx={{ flex: '1 1 300px', minWidth: 300 }}>
          <Card>
            <CardHeader 
            title={
                <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                  <SwapIcon />
                  <Typography variant="h6">Memory Usage</Typography>
                </Box>
              }
            />
            <CardContent>
              <Box sx={{ textAlign: 'center' }}>
                <Box sx={{ position: 'relative', display: 'inline-flex', marginBottom: 2 }}>
                  <CircularProgress
                    variant="determinate"
                    value={Math.round(systemMetrics.memoryUsage)}
                    size={120}
                    thickness={4}
                    sx={{ 
                      color: getStatusColor(systemMetrics.memoryUsage, [75, 90]),
                    }}
                  />
                  <Box sx={{
                    position: 'absolute',
                    top: 0,
                    left: 0,
                    bottom: 0,
                    right: 0,
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                  }}>
                    <Typography variant="h6" component="div" color="text.secondary">
                      {`${Math.round(systemMetrics.memoryUsage)}%`}
                    </Typography>
                  </Box>
                </Box>
                <Statistic 
                  value={systemMetrics.memoryUsage.toFixed(1)} 
                  suffix="%" 
                  color={getStatusColor(systemMetrics.memoryUsage, [75, 90])}
                />
              </Box>
            </CardContent>
          </Card>
        </Box>
        
        {/* System Uptime */}
        <Box sx={{ flex: '1 1 300px', minWidth: 300 }}>
          <Card>
            <CardHeader 
            title={
                <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                  <ClockCircleIcon />
                  <Typography variant="h6">Uptime</Typography>
                </Box>
              }
            />
            <CardContent>
              <Box sx={{ textAlign: 'center', padding: '10px 0' }}>
                <Typography variant="h4" sx={{ margin: 0 }}>
                {formatDuration(systemMetrics.uptime)}
                </Typography>
                <Typography variant="body2" color="text.secondary">
                Since {new Date(Date.now() - (systemMetrics.uptime * 1000)).toLocaleString()}
                </Typography>
              </Box>
            </CardContent>
          </Card>
        </Box>
        
        {/* Temperature */}
        <Box sx={{ flex: '1 1 300px', minWidth: 300 }}>
          <Card>
            <CardHeader 
            title={
                <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                  <ThunderboltIcon />
                  <Typography variant="h6">System Temperature</Typography>
                </Box>
              }
            />
            <CardContent>
              <Box sx={{ textAlign: 'center' }}>
                <Box sx={{ position: 'relative', display: 'inline-flex', marginBottom: 1 }}>
                  <CircularProgress
                    variant="determinate"
                    value={systemMetrics.temperature}
                    size={120}
                    thickness={4}
                    sx={{ 
                      color: getStatusColor(systemMetrics.temperature, [45, 60]),
                    }}
                  />
                  <Box sx={{
                    position: 'absolute',
                    top: 0,
                    left: 0,
                    bottom: 0,
                    right: 0,
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                  }}>
                    <Typography variant="h6" component="div" color="text.secondary">
                      {`${systemMetrics.temperature}°C`}
                    </Typography>
                  </Box>
                </Box>
                <Typography variant="body2" color="text.secondary">
                  {systemMetrics.temperature < 45 ? 'Normal' : 
                   systemMetrics.temperature < 60 ? 'Warning' : 'Critical'}
                </Typography>
              </Box>
            </CardContent>
          </Card>
        </Box>
      </Box>
      
      {showDetailedStats && (
        <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 2, marginTop: 2 }}>
          {/* System Load Average */}
          <Box sx={{ flex: '1 1 400px', minWidth: 400 }}>
            <Card>
              <CardHeader 
              title={
                  <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                    <DashboardIcon />
                    <Typography variant="h6">Load Average</Typography>
                  </Box>
                }
              />
              <CardContent>
                <Box sx={{ display: 'flex', gap: 2, justifyContent: 'space-around' }}>
                    <Statistic 
                      title="1 min" 
                      value={formatLoadAvg(systemMetrics.loadAverage[0])}
                    color={getStatusColor(systemMetrics.loadAverage[0], [1, 2])}
                    />
                    <Statistic 
                      title="5 min" 
                      value={formatLoadAvg(systemMetrics.loadAverage[1])}
                    color={getStatusColor(systemMetrics.loadAverage[1], [1, 2])}
                    />
                    <Statistic 
                      title="15 min" 
                      value={formatLoadAvg(systemMetrics.loadAverage[2])}
                    color={getStatusColor(systemMetrics.loadAverage[2], [1, 2])}
                  />
                </Box>
                <Divider sx={{ margin: '12px 0' }} />
                <Typography variant="body2" color="text.secondary">
                  Load average represents the system load over time. Values consistently above 1.0 per CPU core indicate high load.
                </Typography>
              </CardContent>
            </Card>
          </Box>
          
          {/* Memory Details */}
          <Box sx={{ flex: '1 1 400px', minWidth: 400 }}>
            <Card>
              <CardHeader 
              title={
                  <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                    <DatabaseIcon />
                    <Typography variant="h6">Memory Details</Typography>
                  </Box>
                }
              />
              <CardContent>
                <Box sx={{ textAlign: 'center', marginBottom: 2 }}>
                    <Statistic 
                      title="Memory Usage" 
                      value={systemMetrics.memoryUsage.toFixed(1)} 
                      suffix="%" 
                    color={getStatusColor(systemMetrics.memoryUsage, [75, 90])}
                    size="large"
                  />
                </Box>
                <Divider sx={{ margin: '12px 0' }} />
                <LinearProgress 
                  variant="determinate"
                  value={systemMetrics.memoryUsage} 
                  color={systemMetrics.memoryUsage > 90 ? "error" : 
                        systemMetrics.memoryUsage > 75 ? "warning" : 
                        "success"} 
                  sx={{ height: 10, borderRadius: 1, marginBottom: 1 }}
              />
                <Typography variant="body2" color="text.secondary">
                  {systemMetrics.memoryUsage < 75 ? 
                    "Memory usage is normal." : 
                   systemMetrics.memoryUsage < 90 ? 
                    "Memory usage is elevated." :
                    "Memory usage is critically high!"}
                </Typography>
              </CardContent>
            </Card>
          </Box>
          
          {/* Network I/O */}
          <Box sx={{ flex: '1 1 400px', minWidth: 400 }}>
            <Card>
              <CardHeader title="Network I/O" />
              <CardContent>
                <Box sx={{ display: 'flex', gap: 3, justifyContent: 'space-around' }}>
                      <Statistic 
                        title="Received" 
                        value={formatBytes(systemMetrics.networkIO.received)} 
                    prefix={<ArrowDownIcon sx={{ color: '#4caf50' }} />}
                    color="#4caf50"
                        suffix="/s"
                      />
                      <Statistic 
                        title="Sent" 
                        value={formatBytes(systemMetrics.networkIO.sent)} 
                    prefix={<ArrowUpIcon sx={{ color: '#2196f3' }} />}
                    color="#2196f3"
                        suffix="/s"
                      />
                </Box>
              </CardContent>
            </Card>
          </Box>
          
          {/* Disk I/O */}
          <Box sx={{ flex: '1 1 400px', minWidth: 400 }}>
            <Card>
              <CardHeader title="Disk I/O" />
              <CardContent>
                <Box sx={{ display: 'flex', gap: 3, justifyContent: 'space-around' }}>
                      <Statistic 
                        title="Read" 
                        value={formatBytes(systemMetrics.diskIO.read)} 
                    prefix={<ArrowDownIcon sx={{ color: '#9c27b0' }} />}
                    color="#9c27b0"
                        suffix="/s"
                      />
                      <Statistic 
                        title="Write" 
                        value={formatBytes(systemMetrics.diskIO.write)} 
                    prefix={<ArrowUpIcon sx={{ color: '#e91e63' }} />}
                    color="#e91e63"
                        suffix="/s"
                      />
                </Box>
              </CardContent>
            </Card>
          </Box>
        </Box>
      )}
      
      {/* Last update timestamp */}
      <Box sx={{ marginTop: 1, textAlign: 'right' }}>
        <Typography variant="body2" color="text.secondary">
          Last updated: {new Date().toLocaleTimeString()}
        </Typography>
      </Box>
    </Box>
  );
};

export default SystemMonitor; 
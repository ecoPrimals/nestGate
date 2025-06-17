import React, { useState } from 'react';
import { 
  Card, 
  CardHeader,
  CardContent,
  Box,
  Typography,
  LinearProgress,
  Divider,
  Paper,
  Stack
} from '@mui/material';
import { 
  Dashboard as DashboardIcon,
  Speed as SpeedIcon,
  Memory as MemoryIcon,
  Storage as StorageIcon,
  AccessTime as ClockIcon,
  ArrowUpward as ArrowUpIcon,
  ArrowDownward as ArrowDownIcon,
  Wifi as NetworkIcon
} from '@mui/icons-material';
import useWebSocket from '../../hooks/useWebSocket';
import { WebSocketMessageType, WebSocketMessage } from '../../services/websocket.service';
import StatusChip from '../common/StatusChip';

interface SystemMetrics {
  cpu: {
    usage: number;
    temperature: number;
    processes: number;
  };
  memory: {
    used: number;
    total: number;
    usage: number;
  };
  disk: {
    used: number;
    total: number;
    usage: number;
    read_rate: number;
    write_rate: number;
  };
  network: {
    upload: number;
    download: number;
    connections: number;
  };
  uptime: number;
}

const SystemMonitor: React.FC = () => {
  const [metrics, setMetrics] = useState<SystemMetrics>({
    cpu: { usage: 0, temperature: 0, processes: 0 },
    memory: { used: 0, total: 1, usage: 0 },
    disk: { used: 0, total: 1, usage: 0, read_rate: 0, write_rate: 0 },
    network: { upload: 0, download: 0, connections: 0 },
    uptime: 0
  });

  // Get connection status and subscribe to telemetry updates
  const { connected } = useWebSocket({
    autoConnect: true,
    subscriptions: [
      {
        type: WebSocketMessageType.TELEMETRY,
        handler: (message: WebSocketMessage<unknown>) => {
          if (message.type === WebSocketMessageType.TELEMETRY && message.data) {
            setMetrics(message.data as SystemMetrics);
          }
        }
      }
    ]
  });

  // Format values for display
  const formatBytes = (bytes: number, decimals = 2): string => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB', 'PB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(decimals)) + ' ' + sizes[i];
  };

  const formatUptime = (seconds: number): string => {
    const days = Math.floor(seconds / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    
    return `${days}d ${hours}h ${minutes}m`;
  };

  // Format network speeds
  const formatSpeed = (bytesPerSec: number): string => {
    return `${formatBytes(bytesPerSec)}/s`;
  };

  // Helper to determine progress color based on usage percentage
  const getProgressColor = (usage: number) => {
    if (usage > 90) return 'error';
    if (usage > 70) return 'warning';
    return 'primary';
  };

  return (
    <Card sx={{ mb: 3 }}>
      <CardHeader
        title={
          <Stack direction="row" spacing={1} alignItems="center">
            <DashboardIcon />
            <Typography variant="h6">System Health</Typography>
            {connected ? (
              <StatusChip status="running" label="LIVE" size="small" />
            ) : (
              <StatusChip status="error" label="DISCONNECTED" size="small" />
            )}
          </Stack>
        }
      />
      <CardContent>
        <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 2 }}>
          {/* CPU Section */}
          <Box sx={{ flex: '1 1 250px', minWidth: { xs: '100%', sm: '45%', md: '45%', lg: '22%' } }}>
            <Paper 
              elevation={0} 
              sx={{ 
                p: 2, 
                height: '100%', 
                transition: 'box-shadow 0.3s',
                '&:hover': {
                  boxShadow: '0 2px 8px rgba(0, 0, 0, 0.09)'
                }
              }}
            >
              <Stack direction="row" spacing={1} alignItems="center" sx={{ mb: 1 }}>
                <SpeedIcon color="primary" />
                <Typography variant="subtitle1">CPU</Typography>
              </Stack>
              
              <Typography variant="h4" gutterBottom>
                {metrics.cpu.usage.toFixed(1)}%
              </Typography>
              
              <LinearProgress 
                variant="determinate" 
                value={metrics.cpu.usage} 
                color={getProgressColor(metrics.cpu.usage)}
                sx={{ mb: 2, height: 6, borderRadius: 3 }}
              />
              
              <Box sx={{ display: 'flex', justifyContent: 'space-between', mt: 1 }}>
                <Typography variant="body2" color="text.secondary">
                  Temp: {metrics.cpu.temperature}°C
                </Typography>
                <Typography variant="body2" color="text.secondary">
                  Processes: {metrics.cpu.processes}
                </Typography>
              </Box>
            </Paper>
          </Box>

          {/* Memory Section */}
          <Box sx={{ flex: '1 1 250px', minWidth: { xs: '100%', sm: '45%', md: '45%', lg: '22%' } }}>
            <Paper 
              elevation={0} 
              sx={{ 
                p: 2, 
                height: '100%', 
                transition: 'box-shadow 0.3s',
                '&:hover': {
                  boxShadow: '0 2px 8px rgba(0, 0, 0, 0.09)'
                }
              }}
            >
              <Stack direction="row" spacing={1} alignItems="center" sx={{ mb: 1 }}>
                <MemoryIcon color="primary" />
                <Typography variant="subtitle1">Memory</Typography>
              </Stack>
              
              <Typography variant="h4" gutterBottom>
                {metrics.memory.usage.toFixed(1)}%
              </Typography>
              
              <LinearProgress 
                variant="determinate" 
                value={metrics.memory.usage} 
                color={getProgressColor(metrics.memory.usage)}
                sx={{ mb: 2, height: 6, borderRadius: 3 }}
              />
              
              <Box sx={{ display: 'flex', justifyContent: 'space-between', mt: 1 }}>
                <Typography variant="body2" color="text.secondary">
                  Used: {formatBytes(metrics.memory.used)}
                </Typography>
                <Typography variant="body2" color="text.secondary">
                  Total: {formatBytes(metrics.memory.total)}
                </Typography>
              </Box>
            </Paper>
          </Box>

          {/* Disk Section */}
          <Box sx={{ flex: '1 1 250px', minWidth: { xs: '100%', sm: '45%', md: '45%', lg: '22%' } }}>
            <Paper 
              elevation={0} 
              sx={{ 
                p: 2, 
                height: '100%', 
                transition: 'box-shadow 0.3s',
                '&:hover': {
                  boxShadow: '0 2px 8px rgba(0, 0, 0, 0.09)'
                }
              }}
            >
              <Stack direction="row" spacing={1} alignItems="center" sx={{ mb: 1 }}>
                <StorageIcon color="primary" />
                <Typography variant="subtitle1">Disk</Typography>
              </Stack>
              
              <Typography variant="h4" gutterBottom>
                {metrics.disk.usage.toFixed(1)}%
              </Typography>
              
              <LinearProgress 
                variant="determinate" 
                value={metrics.disk.usage} 
                color={getProgressColor(metrics.disk.usage)}
                sx={{ mb: 2, height: 6, borderRadius: 3 }}
              />
              
              <Box sx={{ display: 'flex', flexWrap: 'wrap' }}>
                <Box sx={{ width: '50%' }}>
                  <Typography variant="body2" color="text.secondary">
                    Used: {formatBytes(metrics.disk.used)}
                  </Typography>
                </Box>
                <Box sx={{ width: '50%' }}>
                  <Typography variant="body2" color="text.secondary">
                    Total: {formatBytes(metrics.disk.total)}
                  </Typography>
                </Box>
                <Box sx={{ width: '50%' }}>
                  <Typography variant="body2" color="text.secondary">
                    <Stack direction="row" spacing={0.5} alignItems="center">
                      <ArrowDownIcon fontSize="small" />
                      <span>{formatSpeed(metrics.disk.read_rate)}</span>
                    </Stack>
                  </Typography>
                </Box>
                <Box sx={{ width: '50%' }}>
                  <Typography variant="body2" color="text.secondary">
                    <Stack direction="row" spacing={0.5} alignItems="center">
                      <ArrowUpIcon fontSize="small" />
                      <span>{formatSpeed(metrics.disk.write_rate)}</span>
                    </Stack>
                  </Typography>
                </Box>
              </Box>
            </Paper>
          </Box>

          {/* Network Section */}
          <Box sx={{ flex: '1 1 250px', minWidth: { xs: '100%', sm: '45%', md: '45%', lg: '22%' } }}>
            <Paper 
              elevation={0} 
              sx={{ 
                p: 2, 
                height: '100%', 
                transition: 'box-shadow 0.3s',
                '&:hover': {
                  boxShadow: '0 2px 8px rgba(0, 0, 0, 0.09)'
                }
              }}
            >
              <Stack direction="row" spacing={1} alignItems="center" sx={{ mb: 1 }}>
                <NetworkIcon color="primary" />
                <Typography variant="subtitle1">Network</Typography>
              </Stack>
              
              <Typography variant="h4" gutterBottom>
                {metrics.network.connections}
              </Typography>
              <Typography variant="body2" color="text.secondary" gutterBottom>
                connections
              </Typography>
              
              <Divider sx={{ my: 1 }} />
              
              <Box sx={{ display: 'flex', textAlign: 'center' }}>
                <Box sx={{ width: '50%' }}>
                  <Typography variant="body2" color="text.secondary">
                    <Stack direction="row" spacing={0.5} alignItems="center" justifyContent="center">
                      <ArrowDownIcon fontSize="small" />
                      <span>Download</span>
                    </Stack>
                  </Typography>
                  <Typography>{formatSpeed(metrics.network.download)}</Typography>
                </Box>
                <Box sx={{ width: '50%' }}>
                  <Typography variant="body2" color="text.secondary">
                    <Stack direction="row" spacing={0.5} alignItems="center" justifyContent="center">
                      <ArrowUpIcon fontSize="small" />
                      <span>Upload</span>
                    </Stack>
                  </Typography>
                  <Typography>{formatSpeed(metrics.network.upload)}</Typography>
                </Box>
              </Box>
            </Paper>
          </Box>

          {/* Uptime */}
          <Box sx={{ width: '100%' }}>
            <Paper 
              elevation={0} 
              sx={{ 
                p: 1.5, 
                textAlign: 'center',
                backgroundColor: 'rgba(25, 118, 210, 0.08)'
              }}
            >
              <Stack direction="row" spacing={1} alignItems="center" justifyContent="center">
                <ClockIcon color="primary" />
                <Typography>System Uptime: {formatUptime(metrics.uptime)}</Typography>
              </Stack>
            </Paper>
          </Box>
        </Box>
      </CardContent>
    </Card>
  );
};

export default SystemMonitor; 
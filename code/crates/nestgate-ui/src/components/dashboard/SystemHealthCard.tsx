import React from 'react';
import { Card, CardHeader, CardContent, Typography, LinearProgress, Box, List, ListItem, ListItemIcon, ListItemText } from '@mui/material';
import { 
  CheckCircle as CheckCircleIcon, 
  Warning as WarningIcon, 
  Cancel as CloseCircleIcon,
  Computer as DesktopIcon,
  Storage as DatabaseIcon
} from '@mui/icons-material';
import StatusChip from '../common/StatusChip';

interface SystemStatus {
  hostname: string;
  uptime: number;
  load: number[];
  memory: {
    total: number;
    used: number;
    free: number;
  };
  cpu: {
    cores: number;
    usage: number;
  };
  version: string;
}

interface DiskHealth {
  device: string;
  model: string;
  serial: string;
  size: number;
  temperature: number;
  status: string;
  smart: {
    passed: boolean;
    attributes: {
      name: string;
      value: number;
      threshold: number;
      status: string;
    }[];
  };
}

interface SystemHealthCardProps {
  systemStatus?: SystemStatus;
  diskHealth?: DiskHealth[];
}

const SystemHealthCard: React.FC<SystemHealthCardProps> = ({ 
  systemStatus, 
  diskHealth = []
}) => {
  // Format uptime to human readable
  const formatUptime = (uptime: number): string => {
    const days = Math.floor(uptime / 86400);
    const hours = Math.floor((uptime % 86400) / 3600);
    const minutes = Math.floor((uptime % 3600) / 60);
    
    return `${days}d ${hours}h ${minutes}m`;
  };
  
  // Get overall health status
  const getOverallHealth = (): { status: string; statusType: string; icon: React.ReactNode } => {
    // Check if data is loaded
    if (!systemStatus) {
      return {
        status: 'Unknown',
        statusType: 'warning',
        icon: <WarningIcon />
      };
    }
    
    // Check CPU usage
    const highCpuUsage = systemStatus.cpu.usage > 80;
    
    // Check memory usage
    const memoryUsage = systemStatus.memory.used / systemStatus.memory.total * 100;
    const highMemoryUsage = memoryUsage > 85;
    
    // Check disk health
    const hasFailedDisks = diskHealth.some(disk => 
      disk.status.toLowerCase() === 'failed' || 
      disk.status.toLowerCase() === 'critical' ||
      !disk.smart.passed
    );
    
    const hasWarningDisks = diskHealth.some(disk => 
      disk.status.toLowerCase() === 'warning' || 
      disk.temperature > 65
    );
    
    // Determine overall status
    if (hasFailedDisks || highCpuUsage || highMemoryUsage) {
      return {
        status: 'Critical',
        statusType: 'error',
        icon: <CloseCircleIcon />
      };
    } else if (hasWarningDisks || systemStatus.cpu.usage > 70 || memoryUsage > 75) {
      return {
        status: 'Warning',
        statusType: 'warning',
        icon: <WarningIcon />
      };
    } else {
      return {
        status: 'Healthy',
        statusType: 'success',
        icon: <CheckCircleIcon />
      };
    }
  };
  
  const health = getOverallHealth();
  
  // Calculate memory usage
  const memoryUsage = systemStatus 
    ? Math.round(systemStatus.memory.used / systemStatus.memory.total * 100) 
    : 0;
  
  // Get color based on usage
  const getUsageColor = (percentage: number): 'success' | 'warning' | 'error' => {
    if (percentage < 70) return 'success';
    if (percentage < 85) return 'warning';
    return 'error';
  };
  
  // Check for disk warnings
  const diskWarnings = diskHealth
    .filter(disk => disk.status.toLowerCase() !== 'healthy' || disk.temperature > 65 || !disk.smart.passed)
    .map(disk => ({
      device: disk.device,
      model: disk.model,
      issue: !disk.smart.passed 
        ? 'SMART test failed' 
        : disk.temperature > 65 
          ? `High temperature (${disk.temperature}°C)` 
          : `Status: ${disk.status}`
    }));
  
  return (
    <Card>
      <CardHeader 
        title={
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
            <DesktopIcon />
            System Health
          </Box>
        }
      />
      <CardContent>
        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
          <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
            <Typography variant="h6" sx={{ margin: 0 }}>
              Overall Status
            </Typography>
            <StatusChip 
              status={health.statusType} 
              label={health.status}
              size="small"
            />
          </Box>
        
        {systemStatus && (
          <>
              <Box sx={{ display: 'flex', gap: 2, flexWrap: 'wrap' }}>
                <Box sx={{ flex: 1, minWidth: 200 }}>
                  <Typography variant="body2" color="text.secondary" gutterBottom>
                    CPU Usage
                  </Typography>
                  <LinearProgress 
                    variant="determinate"
                    value={systemStatus.cpu.usage} 
                    color={getUsageColor(systemStatus.cpu.usage)}
                    sx={{ height: 8, borderRadius: 1 }}
                  />
                  <Typography variant="caption" color="text.secondary">
                    {Math.round(systemStatus.cpu.usage)}%
                  </Typography>
                </Box>
                
                <Box sx={{ flex: 1, minWidth: 200 }}>
                  <Typography variant="body2" color="text.secondary" gutterBottom>
                    Memory Usage
                  </Typography>
                  <LinearProgress 
                    variant="determinate"
                    value={memoryUsage} 
                    color={getUsageColor(memoryUsage)}
                    sx={{ height: 8, borderRadius: 1 }}
                  />
                  <Typography variant="caption" color="text.secondary">
                    {memoryUsage}%
                  </Typography>
                </Box>
              </Box>
              
              <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
                <Box sx={{ display: 'flex', justifyContent: 'space-between' }}>
                  <Typography variant="body2">Hostname:</Typography>
                  <Typography variant="body2" sx={{ fontWeight: 600 }}>
                    {systemStatus.hostname}
                  </Typography>
                </Box>
                
                <Box sx={{ display: 'flex', justifyContent: 'space-between' }}>
                  <Typography variant="body2">Uptime:</Typography>
                  <Typography variant="body2" sx={{ fontWeight: 600 }}>
                    {formatUptime(systemStatus.uptime)}
                  </Typography>
                </Box>
                
                <Box sx={{ display: 'flex', justifyContent: 'space-between' }}>
                  <Typography variant="body2">System Load:</Typography>
                  <Typography variant="body2" sx={{ fontWeight: 600 }}>
                    {systemStatus.load.join(', ')}
                  </Typography>
                </Box>
                
                <Box sx={{ display: 'flex', justifyContent: 'space-between' }}>
                  <Typography variant="body2">CPU Cores:</Typography>
                  <Typography variant="body2" sx={{ fontWeight: 600 }}>
                    {systemStatus.cpu.cores}
                  </Typography>
                </Box>
                
                <Box sx={{ display: 'flex', justifyContent: 'space-between' }}>
                  <Typography variant="body2">System Version:</Typography>
                  <Typography variant="body2" sx={{ fontWeight: 600 }}>
                    {systemStatus.version}
                  </Typography>
                </Box>
              </Box>
          </>
        )}
        
        {diskWarnings.length > 0 && (
            <Box>
              <Typography variant="h6" sx={{ marginTop: 2, marginBottom: 1 }}>
                Disk Warnings
              </Typography>
              <List dense>
                {diskWarnings.map((item, index) => (
                  <ListItem key={index} disablePadding>
                    <ListItemIcon>
                      <DatabaseIcon color="warning" />
                    </ListItemIcon>
                    <ListItemText
                      primary={
                        <Box sx={{ display: 'flex', gap: 1, alignItems: 'center', flexWrap: 'wrap' }}>
                          <Typography variant="body2" sx={{ fontWeight: 600 }}>
                            {item.device}
                          </Typography>
                          <Typography variant="body2" color="text.secondary">
                            ({item.model})
                          </Typography>
                        </Box>
                      }
                      secondary={
                        <Typography variant="body2" color="error">
                          {item.issue}
                        </Typography>
                      }
                    />
                  </ListItem>
                ))}
              </List>
            </Box>
          )}
        </Box>
      </CardContent>
    </Card>
  );
};

export default SystemHealthCard; 
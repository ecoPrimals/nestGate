import React from 'react';
import {
  Box,
  Card,
  CardContent,
  Typography,
  Chip,
  LinearProgress,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
  IconButton,
  Tooltip,
  Alert,
  CircularProgress,
} from '@mui/material';

import {
  PlayArrow,
  Stop,
  Refresh,
  Computer,
  Memory,
  Storage,
  NetworkCheck,
  Speed,
  HealthAndSafety,
  Error as ErrorIcon,
  CheckCircle,
  Warning,
} from '@mui/icons-material';
import { usePortManager } from '../../hooks/usePortManager';

const SystemDashboard: React.FC = () => {
  const {
    systemMetrics,
    serviceMetrics,
    healthStatus,
    services,
    portAllocations,
    connectionStats,
    isLoading,
    error,
    lastUpdate,
    startService,
    stopService,
    restartService,
    refreshData,
  } = usePortManager();

  const formatBytes = (bytes: number) => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  const formatUptime = (seconds: number) => {
    const days = Math.floor(seconds / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    
    if (days > 0) return `${days}d ${hours}h ${minutes}m`;
    if (hours > 0) return `${hours}h ${minutes}m`;
    return `${minutes}m`;
  };

  const getStatusColor = (status: string) => {
    switch (status.toLowerCase()) {
      case 'running': return 'success';
      case 'stopped': return 'default';
      case 'starting': return 'info';
      case 'stopping': return 'warning';
      case 'failed': return 'error';
      default: return 'default';
    }
  };

  const getHealthColor = (health: any) => {
    if (typeof health === 'string') {
      switch (health.toLowerCase()) {
        case 'healthy': return 'success';
        case 'unhealthy': return 'error';
        case 'unknown': return 'warning';
        default: return 'default';
      }
    }
    if (health && typeof health === 'object' && health.Failed) {
      return 'error';
    }
    return 'default';
  };

  if (error) {
    return (
      <Box p={3}>
        <Alert severity="error" action={
          <IconButton color="inherit" size="small" onClick={refreshData}>
            <Refresh />
          </IconButton>
        }>
          {error}
        </Alert>
      </Box>
    );
  }

  return (
    <Box p={3}>
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Typography variant="h4" component="h1">
          NestGate System Dashboard
        </Typography>
        <Box display="flex" alignItems="center" gap={2}>
          <Typography variant="body2" color="text.secondary">
            Last updated: {lastUpdate ? new Date(lastUpdate).toLocaleTimeString() : 'Never'}
          </Typography>
          <IconButton onClick={refreshData} disabled={isLoading}>
            <Refresh />
          </IconButton>
        </Box>
      </Box>

      {isLoading && !systemMetrics && (
        <Box display="flex" justifyContent="center" p={4}>
          <CircularProgress />
        </Box>
      )}

      {systemMetrics && (
        <>
          {/* System Overview Cards */}
          <Box sx={{ mb: 4 }}>
            <Box>
              <Card>
                <CardContent>
                  <Box display="flex" alignItems="center" gap={2}>
                    <Computer color="primary" />
                    <Box>
                      <Typography variant="h6">
                        {systemMetrics?.total_services || 0}
                      </Typography>
                      <Typography variant="body2" color="text.secondary">
                        Total Services
                      </Typography>
                    </Box>
                  </Box>
                </CardContent>
              </Card>
            </Box>

            <Box>
              <Card>
                <CardContent>
                  <Box display="flex" alignItems="center" gap={2}>
                    <Speed color="success" />
                    <Box>
                      <Typography variant="h6">
                        {systemMetrics?.running_services || 0}
                      </Typography>
                      <Typography variant="body2" color="text.secondary">
                        Running
                      </Typography>
                    </Box>
                  </Box>
                </CardContent>
              </Card>
            </Box>

            <Box>
              <Card>
                <CardContent>
                  <Box display="flex" alignItems="center" gap={2}>
                    <ErrorIcon color="error" />
                    <Box>
                      <Typography variant="h6">
                        {systemMetrics?.failed_services || 0}
                      </Typography>
                      <Typography variant="body2" color="text.secondary">
                        Failed
                      </Typography>
                    </Box>
                  </Box>
                </CardContent>
              </Card>
            </Box>

            <Box>
              <Card>
                <CardContent>
                  <Box display="flex" alignItems="center" gap={2}>
                    <NetworkCheck color="info" />
                    <Box>
                      <Typography variant="h6">
                        {systemMetrics?.allocated_ports || 0}
                      </Typography>
                      <Typography variant="body2" color="text.secondary">
                        Allocated Ports
                      </Typography>
                    </Box>
                  </Box>
                </CardContent>
              </Card>
            </Box>
          </Box>

          {/* System Metrics */}
          <Box sx={{ mb: 4 }}>
            <Box>
              <Card>
                <CardContent>
                  <Typography variant="h6" gutterBottom>
                    System Resources
                  </Typography>
                  {systemMetrics && (
                    <Box>
                      <Box>
                        <Box display="flex" justifyContent="space-between" alignItems="center" mb={1}>
                          <Typography variant="body2" color="text.secondary">
                            CPU Usage
                          </Typography>
                          <Typography variant="body2">
                            {systemMetrics.cpu_usage}%
                          </Typography>
                        </Box>
                        <LinearProgress 
                          variant="determinate" 
                          value={systemMetrics.cpu_usage} 
                          sx={{ mb: 2 }}
                        />
                      </Box>

                      <Box>
                        <Box display="flex" justifyContent="space-between" alignItems="center" mb={1}>
                          <Typography variant="body2" color="text.secondary">
                            Memory Usage
                          </Typography>
                          <Typography variant="body2">
                            {(systemMetrics.memory_usage / (1024 * 1024 * 1024)).toFixed(1)} GB
                          </Typography>
                        </Box>
                        <LinearProgress 
                          variant="determinate" 
                          value={(systemMetrics.memory_usage / (8 * 1024 * 1024 * 1024)) * 100} 
                          sx={{ mb: 2 }}
                        />
                      </Box>

                      <Box>
                        <Box display="flex" justifyContent="space-between" alignItems="center" mb={1}>
                          <Typography variant="body2" color="text.secondary">
                            Disk Usage
                          </Typography>
                          <Typography variant="body2">
                            {systemMetrics.disk_usage}%
                          </Typography>
                        </Box>
                        <LinearProgress 
                          variant="determinate" 
                          value={systemMetrics.disk_usage} 
                        />
                      </Box>
                    </Box>
                  )}
                </CardContent>
              </Card>
            </Box>

            <Box>
              <Card>
                <CardContent>
                  <Typography variant="h6" gutterBottom>
                    Network Statistics
                  </Typography>
                  {connectionStats && (
                    <Box>
                      <Box>
                        <Typography variant="body2" color="text.secondary">
                          Total Connections
                        </Typography>
                        <Typography variant="h6">{connectionStats.total_connections}</Typography>
                      </Box>
                      <Box>
                        <Typography variant="body2" color="text.secondary">
                          Active Connections
                        </Typography>
                        <Typography variant="h6">{connectionStats.active_connections}</Typography>
                      </Box>
                      <Box>
                        <Typography variant="body2" color="text.secondary">
                          Bytes Transferred
                        </Typography>
                        <Typography variant="h6">
                          {(connectionStats.bytes_transferred / (1024 * 1024)).toFixed(1)} MB
                        </Typography>
                      </Box>
                      <Box>
                        <Typography variant="body2" color="text.secondary">
                          Avg Response Time
                        </Typography>
                        <Typography variant="h6">{connectionStats.avg_response_time.toFixed(1)}ms</Typography>
                      </Box>
                    </Box>
                  )}
                </CardContent>
              </Card>
            </Box>
          </Box>

          {/* Services Table */}
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                <Computer sx={{ mr: 1, verticalAlign: 'middle' }} />
                Services
              </Typography>
              
              <TableContainer component={Paper} variant="outlined">
                <Table>
                  <TableHead>
                    <TableRow>
                      <TableCell>Service</TableCell>
                      <TableCell>Status</TableCell>
                      <TableCell>Health</TableCell>
                      <TableCell>Port</TableCell>
                      <TableCell>Uptime</TableCell>
                      <TableCell>Memory</TableCell>
                      <TableCell>CPU</TableCell>
                      <TableCell>Actions</TableCell>
                    </TableRow>
                  </TableHead>
                  <TableBody>
                    {services.map((service) => {
                      const metrics = serviceMetrics[service.definition.id];
                      const health = healthStatus[service.definition.id];
                      
                      return (
                        <TableRow key={service.definition.id}>
                          <TableCell>
                            <Box>
                              <Typography variant="body2" fontWeight="medium">
                                {service.definition.name}
                              </Typography>
                              <Typography variant="caption" color="text.secondary">
                                {service.definition.service_type}
                              </Typography>
                            </Box>
                          </TableCell>
                          <TableCell>
                            <Chip 
                              label={service.status} 
                              color={getStatusColor(service.status) as any}
                              size="small"
                            />
                          </TableCell>
                          <TableCell>
                            {health ? (
                              <Tooltip title={health.details || ''}>
                                <Chip 
                                  label={typeof health.status === 'string' ? health.status : 'Failed'}
                                  color={getHealthColor(health.status) as any}
                                  size="small"
                                  icon={
                                    getHealthColor(health.status) === 'success' ? <CheckCircle /> :
                                    getHealthColor(health.status) === 'error' ? <ErrorIcon /> :
                                    <Warning />
                                  }
                                />
                              </Tooltip>
                            ) : (
                              <Chip label="Unknown" color="default" size="small" />
                            )}
                          </TableCell>
                          <TableCell>
                            {service.port || portAllocations[service.definition.id] || '-'}
                          </TableCell>
                          <TableCell>
                            {metrics ? formatUptime(metrics.uptime_seconds) : '-'}
                          </TableCell>
                          <TableCell>
                            {metrics ? `${metrics.memory_usage.toFixed(1)}%` : '-'}
                          </TableCell>
                          <TableCell>
                            {metrics ? `${metrics.cpu_usage.toFixed(1)}%` : '-'}
                          </TableCell>
                          <TableCell>
                            <Box display="flex" gap={1}>
                              {service.status === 'Running' ? (
                                <>
                                  <Tooltip title="Stop Service">
                                    <IconButton 
                                      size="small" 
                                      onClick={() => stopService(service.definition.id)}
                                      color="error"
                                    >
                                      <Stop />
                                    </IconButton>
                                  </Tooltip>
                                  <Tooltip title="Restart Service">
                                    <IconButton 
                                      size="small" 
                                      onClick={() => restartService(service.definition.id)}
                                      color="warning"
                                    >
                                      <Refresh />
                                    </IconButton>
                                  </Tooltip>
                                </>
                              ) : (
                                <Tooltip title="Start Service">
                                  <IconButton 
                                    size="small" 
                                    onClick={() => startService(service.definition.id)}
                                    color="success"
                                  >
                                    <PlayArrow />
                                  </IconButton>
                                </Tooltip>
                              )}
                            </Box>
                          </TableCell>
                        </TableRow>
                      );
                    })}
                  </TableBody>
                </Table>
              </TableContainer>
            </CardContent>
          </Card>
        </>
      )}
    </Box>
  );
};

export default SystemDashboard; 
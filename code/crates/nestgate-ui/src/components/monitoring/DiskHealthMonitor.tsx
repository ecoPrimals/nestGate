import React, { useEffect, useState, useRef } from 'react';
import { 
  Card, 
  CardHeader,
  CardContent,
  Typography, 
  Box,
  Alert,
  LinearProgress,
  Badge,
  Paper,
  Tooltip,
  TableContainer,
  Table,
  TableHead,
  TableBody,
  TableRow,
  TableCell,
  CircularProgress
} from '@mui/material';
import {
  CheckCircle as CheckCircleIcon,
  Error as ErrorIcon,
  Cancel as CancelIcon,
  Storage as StorageIcon,
  Sync as SyncIcon,
  FlashOn as FlashOnIcon,
  ArrowUpward as ArrowUpIcon,
  ArrowDownward as ArrowDownIcon,
  Warning as WarningIcon
} from '@mui/icons-material';
import { 
  WebSocketService, 
  WebSocketMessageType, 
  DiskMetrics,
  WebSocketMessage
} from '../../services/websocket.service';
import { formatBytes } from '../../utils/format.utils';
import StatusChip from '../common/StatusChip';

interface DiskHealthMonitorProps {
  showDetailed?: boolean;
}

const DiskHealthMonitor: React.FC<DiskHealthMonitorProps> = ({
  showDetailed = true
}) => {
  const [disks, setDisks] = useState<Map<string, DiskMetrics>>(new Map());
  const [loading, setLoading] = useState<boolean>(true);
  const [connectionError, setConnectionError] = useState<string | null>(null);
  
  const wsServiceRef = useRef<WebSocketService>(WebSocketService.getInstance());
  const unsubscribeRef = useRef<(() => void) | null>(null);

  // Connect to WebSocket and subscribe to disk metrics
  useEffect(() => {
    const connectToWebSocket = async () => {
      setLoading(true);
      setConnectionError(null);
      
      try {
        const wsService = wsServiceRef.current;
        
        await wsService.connect();
        
        // Subscribe to disk metrics
        const unsubscribe = wsService.subscribe(
          WebSocketMessageType.DISK_METRICS,
          (message: WebSocketMessage<DiskMetrics>) => {
            setDisks(prevDisks => {
              const newDisks = new Map(prevDisks);
              newDisks.set(message.data.diskId, message.data);
              return newDisks;
            });
            setLoading(false);
          }
        );
        
        // Store the unsubscribe function
        unsubscribeRef.current = unsubscribe;
        
      } catch (error) {
        console.error('Failed to connect to WebSocket:', error);
        setConnectionError('Failed to connect to the disk monitoring service');
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
  }, []);

  const getStatusColor = (status: string): "success" | "warning" | "error" | "default" => {
    switch (status) {
      case 'online': return 'success';
      case 'degraded': return 'warning';
      case 'offline': return 'error';
      default: return 'default';
    }
  };

  const getHealthColor = (health: string): "success" | "warning" | "error" | "default" => {
    switch (health) {
      case 'good': return 'success';
      case 'warning': return 'warning';
      case 'critical': return 'error';
      default: return 'default';
    }
  };

  const getHealthIcon = (health: string) => {
    switch (health) {
      case 'good': 
        return <CheckCircleIcon color="success" />;
      case 'warning': 
        return <WarningIcon color="warning" />;
      case 'critical': 
        return <CancelIcon color="error" />;
      default: 
        return <StorageIcon color="disabled" />;
    }
  };

  const getTemperatureColor = (temp: number): "success" | "warning" | "error" => {
    if (temp >= 60) return 'error';
    if (temp >= 45) return 'warning';
    return 'success';
  };

  // Transform disks Map to array for the table
  const disksArray = Array.from(disks.values());

  // Calculate health statistics
  const healthStats = {
    total: disksArray.length,
    good: disksArray.filter(disk => disk.health === 'good').length,
    warning: disksArray.filter(disk => disk.health === 'warning').length,
    critical: disksArray.filter(disk => disk.health === 'critical').length
  };

  if (loading && disksArray.length === 0) {
    return (
      <Card>
        <CardHeader title="Disk Health" />
        <CardContent sx={{ textAlign: 'center', py: 5 }}>
          <Box display="flex" flexDirection="column" alignItems="center" gap={2}>
            <CircularProgress size={48} />
            <Typography>Loading disk metrics...</Typography>
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

  return (
    <Card>
      <CardHeader
        title={
          <Box display="flex" alignItems="center" gap={1}>
            <StorageIcon />
            <Typography variant="h6">Disk Health Status</Typography>
            {loading && <CircularProgress size={20} sx={{ ml: 1 }} />}
          </Box>
        }
      />
      <CardContent>
        {/* Health summary */}
        <Box display="flex" flexWrap="wrap" gap={2} mb={2}>
          <Box sx={{ flex: '1 1 20%', minWidth: { xs: '100%', sm: '45%', md: '20%' } }}>
            <Typography variant="subtitle1" gutterBottom>Total Disks</Typography>
            <Box display="flex" alignItems="center" gap={1}>
              <StorageIcon />
              <Typography variant="h6">{healthStats.total}</Typography>
            </Box>
          </Box>
          <Box sx={{ flex: '1 1 20%', minWidth: { xs: '100%', sm: '45%', md: '20%' } }}>
            <Typography variant="subtitle1" gutterBottom>Healthy</Typography>
            <Box display="flex" alignItems="center" gap={1}>
              <CheckCircleIcon color="success" />
              <Typography variant="h6" color="success.main">{healthStats.good}</Typography>
            </Box>
          </Box>
          <Box sx={{ flex: '1 1 20%', minWidth: { xs: '100%', sm: '45%', md: '20%' } }}>
            <Typography variant="subtitle1" gutterBottom>Warning</Typography>
            <Box display="flex" alignItems="center" gap={1}>
              <WarningIcon color="warning" />
              <Typography variant="h6" color="warning.main">{healthStats.warning}</Typography>
            </Box>
          </Box>
          <Box sx={{ flex: '1 1 20%', minWidth: { xs: '100%', sm: '45%', md: '20%' } }}>
            <Typography variant="subtitle1" gutterBottom>Critical</Typography>
            <Box display="flex" alignItems="center" gap={1}>
              <CancelIcon color="error" />
              <Typography variant="h6" color="error.main">{healthStats.critical}</Typography>
            </Box>
          </Box>
        </Box>

        {/* Warnings if any */}
        {healthStats.critical > 0 && (
          <Alert
            severity="error"
            sx={{ mb: 2 }}
          >
            <Typography variant="subtitle1" fontWeight="bold">Critical Disk Issues Detected</Typography>
            <Typography variant="body2">One or more disks are in critical state. Immediate action recommended.</Typography>
          </Alert>
        )}
        
        {healthStats.warning > 0 && !healthStats.critical && (
          <Alert
            severity="warning"
            sx={{ mb: 2 }}
          >
            <Typography variant="subtitle1" fontWeight="bold">Disk Warnings Detected</Typography>
            <Typography variant="body2">One or more disks have warnings. Please investigate at your convenience.</Typography>
          </Alert>
        )}

        {/* Detailed disk information */}
        <TableContainer component={Paper} elevation={0}>
          <Table size="small">
            <TableHead>
              <TableRow>
                <TableCell>Disk</TableCell>
                <TableCell>Status</TableCell>
                <TableCell>Health</TableCell>
                <TableCell>Temperature</TableCell>
                {showDetailed && (
                  <>
                    <TableCell>Read IOPS</TableCell>
                    <TableCell>Write IOPS</TableCell>
                    <TableCell>Errors</TableCell>
                  </>
                )}
              </TableRow>
            </TableHead>
            <TableBody>
              {disksArray.map((disk) => (
                <TableRow key={disk.diskId}>
                  <TableCell>
                    <Box display="flex" alignItems="center" gap={1}>
                      <StorageIcon />
                      <Typography fontWeight="bold">{disk.diskId}</Typography>
                    </Box>
                  </TableCell>
                  <TableCell>
                    <StatusChip status={disk.status as any} />
                  </TableCell>
                  <TableCell>
                    <StatusChip 
                      status={disk.health as any} 
                    />
                  </TableCell>
                  <TableCell>
                    <Tooltip title={`${disk.temperature}°C`}>
                      <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                        <LinearProgress
                          variant="determinate"
                          value={Math.min(100, Math.round((disk.temperature / 70) * 100))}
                          color={getTemperatureColor(disk.temperature)}
                          sx={{ width: 80, height: 6, borderRadius: 3 }}
                        />
                        <Typography>{disk.temperature.toFixed(1)}°C</Typography>
                      </Box>
                    </Tooltip>
                  </TableCell>
                  {showDetailed && (
                    <>
                      <TableCell>{disk.iops.read.toFixed(0)}</TableCell>
                      <TableCell>{disk.iops.write.toFixed(0)}</TableCell>
                      <TableCell>
                        <Box display="flex" gap={1}>
                          {disk.readErrors > 0 && (
                            <Badge badgeContent={disk.readErrors} color="error" max={999} />
                          )}
                          {disk.writeErrors > 0 && (
                            <Badge badgeContent={disk.writeErrors} color="primary" max={999} />
                          )}
                          {disk.readErrors === 0 && disk.writeErrors === 0 && (
                            <CheckCircleIcon color="success" fontSize="small" />
                          )}
                        </Box>
                      </TableCell>
                    </>
                  )}
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </TableContainer>

        {/* Expanded information only if requested */}
        {showDetailed && disksArray.length > 0 && (
          <Box mt={3}>
            <Typography variant="h6" gutterBottom>Disk Throughput</Typography>
            <Box display="flex" flexWrap="wrap" gap={2}>
              {disksArray.map(disk => (
                <Paper
                  key={disk.diskId}
                  elevation={1}
                  sx={{ 
                    p: 2, 
                    flex: '1 1 250px', 
                    minWidth: { xs: '100%', sm: '45%', md: '30%', lg: '22%' }
                  }}
                >
                  <Typography variant="subtitle2" gutterBottom>{disk.diskId}</Typography>
                  
                  <Box sx={{ mb: 2 }}>
                    <Box display="flex" alignItems="center" gap={0.5}>
                      <ArrowDownIcon fontSize="small" color="success" />
                      <Typography variant="body2" color="text.secondary">
                        Read Throughput
                      </Typography>
                    </Box>
                    <Typography variant="h6">
                      {formatBytes(disk.throughput.read)}
                    </Typography>
                  </Box>
                  
                  <Box>
                    <Box display="flex" alignItems="center" gap={0.5}>
                      <ArrowUpIcon fontSize="small" color="primary" />
                      <Typography variant="body2" color="text.secondary">
                        Write Throughput
                      </Typography>
                    </Box>
                    <Typography variant="h6">
                      {formatBytes(disk.throughput.write)}
                    </Typography>
                  </Box>
                </Paper>
              ))}
            </Box>
          </Box>
        )}
      </CardContent>
    </Card>
  );
};

export default DiskHealthMonitor; 
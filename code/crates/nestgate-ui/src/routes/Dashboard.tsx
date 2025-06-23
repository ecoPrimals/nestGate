import React, { useState, useEffect } from 'react';
import {
  Box,
  Card,
  CardContent,
  Typography,
  LinearProgress,
  Alert,
  CircularProgress,
  Stack
} from '@mui/material';
import {
  Storage as StorageIcon,
  Warning as WarningIcon,
  CloudQueue as CloudIcon,
} from '@mui/icons-material';
import { WebSocketService, WebSocketMessageType, Service, Disk, PerformanceMetrics } from '../services/websocket.service';
import { WS_BASE_URL } from '../config';
import { getWebSocketUrl } from '../config';

const Dashboard: React.FC = () => {
  const [disks, setDisks] = useState<Disk[]>([]);
  const [services, setServices] = useState<Service[]>([]);
  const [performanceMetrics, setPerformanceMetrics] = useState<PerformanceMetrics>({
    cpu: 0,
    memory: 0,
    disk_read: 0,
    disk_write: 0,
    network_in: 0,
    network_out: 0,
    lastUpdate: new Date().toISOString()
  });
  const [connected, setConnected] = useState(false);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const wsService = WebSocketService.getInstance();
    
    // Connect to the WebSocket server using the URL from config or env
    const wsUrl = getWebSocketUrl() || WS_BASE_URL;
    console.log('Connecting to WebSocket server at:', wsUrl);
    
    wsService.connect();
    
    // Subscribe to connection status
    const unsubscribeConnection = wsService.subscribe(WebSocketMessageType.CONNECTION, (message) => {
      console.log('Connection status:', message.data);
      if (typeof message.data === 'object' && message.data !== null && 'connected' in message.data) {
        setConnected(message.data.connected as boolean);
        if (message.data.connected) {
          setLoading(false);
        }
      }
    });
    
    // Subscribe to disks data
    const unsubscribeDisks = wsService.subscribe(WebSocketMessageType.DISKS, (message) => {
      console.log('Received disks data:', message.data);
      setDisks(message.data as Disk[]);
    });
    
    // Subscribe to services data
    const unsubscribeServices = wsService.subscribe(WebSocketMessageType.SERVICES, (message) => {
      console.log('Received services data:', message.data);
      setServices(message.data as Service[]);
    });
    
    // Subscribe to performance metrics
    const unsubscribePerformance = wsService.subscribe(WebSocketMessageType.PERFORMANCE_METRICS, (message) => {
      console.log('Received performance metrics:', message.data);
      const data = message.data as PerformanceMetrics;
      setPerformanceMetrics({
        ...data,
        lastUpdate: new Date().toISOString()
      });
    });
    
    // Request initial data
    setTimeout(() => {
      wsService.send(WebSocketMessageType.GET_DISKS);
      wsService.send(WebSocketMessageType.GET_SERVICES);
      wsService.send(WebSocketMessageType.GET_PERFORMANCE_METRICS);
    }, 1000);
    
    return () => {
      unsubscribeConnection();
      unsubscribeDisks();
      unsubscribeServices();
      unsubscribePerformance();
      wsService.disconnect();
    };
  }, []);

  return (
    <Box className="dashboard-container">
      {!connected && (
        <Alert
          severity="error"
          sx={{ mb: 2 }}
        >
          Unable to connect to the storage server. Please check your network connection or server status.
        </Alert>
      )}
      
      {loading ? (
        <Box sx={{ textAlign: 'center', p: '50px' }}>
          <CircularProgress size={60} />
          <Typography sx={{ mt: 2 }}>Loading dashboard data...</Typography>
        </Box>
      ) : (
        <>
          <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 2 }}>
            <Box sx={{ flex: '1 1 300px', minWidth: { xs: '100%', sm: '45%', md: '22%' } }}>
              <Card>
                <CardContent>
                  <Typography color="text.secondary" gutterBottom>
                    CPU Usage
                  </Typography>
                  <Typography variant="h4" component="div">
                    {performanceMetrics.cpu}%
                  </Typography>
                  <LinearProgress 
                    variant="determinate" 
                    value={performanceMetrics.cpu} 
                    sx={{ mt: 1 }}
                  />
                </CardContent>
              </Card>
            </Box>
            
            <Box sx={{ flex: '1 1 300px', minWidth: { xs: '100%', sm: '45%', md: '22%' } }}>
              <Card>
                <CardContent>
                  <Typography color="text.secondary" gutterBottom>
                    Memory Usage
                  </Typography>
                  <Typography variant="h4" component="div">
                    {performanceMetrics.memory}%
                  </Typography>
                  <LinearProgress 
                    variant="determinate" 
                    value={performanceMetrics.memory} 
                    sx={{ mt: 1 }}
                  />
                </CardContent>
              </Card>
            </Box>
            
            <Box sx={{ flex: '1 1 300px', minWidth: { xs: '100%', sm: '45%', md: '22%' } }}>
              <Card>
                <CardContent>
                  <Typography color="text.secondary" gutterBottom>
                    Disk Activity
                  </Typography>
                  <Box sx={{ display: 'flex', alignItems: 'center' }}>
                    <StorageIcon sx={{ mr: 1 }} />
                    <Typography variant="h4" component="div">
                      {((performanceMetrics.disk_read + performanceMetrics.disk_write) / 2).toFixed(1)}
                      <Typography variant="body2" component="span" sx={{ ml: 0.5 }}>
                        MB/s
                      </Typography>
                    </Typography>
                  </Box>
                  <Box sx={{ display: 'flex', justifyContent: 'space-between', mt: 1 }}>
                    <Typography variant="body2">
                      Read: {performanceMetrics.disk_read.toFixed(1)} MB/s
                    </Typography>
                    <Typography variant="body2">
                      Write: {performanceMetrics.disk_write.toFixed(1)} MB/s
                    </Typography>
                  </Box>
                </CardContent>
              </Card>
            </Box>
            
            <Box sx={{ flex: '1 1 300px', minWidth: { xs: '100%', sm: '45%', md: '22%' } }}>
              <Card>
                <CardContent>
                  <Typography color="text.secondary" gutterBottom>
                    Network Activity
                  </Typography>
                  <Box sx={{ display: 'flex', alignItems: 'center' }}>
                    <CloudIcon sx={{ mr: 1 }} />
                    <Typography variant="h4" component="div">
                      {((performanceMetrics.network_in + performanceMetrics.network_out) / 2).toFixed(1)}
                      <Typography variant="body2" component="span" sx={{ ml: 0.5 }}>
                        MB/s
                      </Typography>
                    </Typography>
                  </Box>
                  <Box sx={{ display: 'flex', justifyContent: 'space-between', mt: 1 }}>
                    <Typography variant="body2">
                      In: {performanceMetrics.network_in.toFixed(1)} MB/s
                    </Typography>
                    <Typography variant="body2">
                      Out: {performanceMetrics.network_out.toFixed(1)} MB/s
                    </Typography>
                  </Box>
                </CardContent>
              </Card>
            </Box>
          </Box>

          {/* Disk Status Section */}
          <Box sx={{ mt: 2 }}>
            <Card>
              <CardContent>
                <Typography variant="h6" gutterBottom>
                  Disk Status
                </Typography>
                
                {disks.length > 0 ? (
                  <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 2 }}>
                    {disks.map(disk => (
                      <Box 
                        key={disk.id} 
                        sx={{ 
                          flex: '1 1 300px', 
                          minWidth: { xs: '100%', sm: '45%', md: '30%', lg: '22%' } 
                        }}
                      >
                        <Card variant="outlined">
                          <CardContent>
                            <Typography variant="subtitle1" gutterBottom>
                              {disk.model}
                            </Typography>
                            <Stack>
                              <Box>
                                <Typography variant="body2" color="text.secondary">
                                  Temperature
                                </Typography>
                                <Typography variant="body1">
                                  {disk.temperature}°C
                                </Typography>
                              </Box>
                              <Box>
                                <Typography variant="body2" color="text.secondary">
                                  Status
                                </Typography>
                                <Typography variant="body1">
                                  {disk.status}
                                </Typography>
                              </Box>
                              <Box>
                                <Typography variant="body2" color="text.secondary">
                                  Size
                                </Typography>
                                <Typography variant="body1">
                                  {(disk.size / 1099511627776).toFixed(2)} TB
                                </Typography>
                              </Box>
                            </Stack>
                          </CardContent>
                        </Card>
                      </Box>
                    ))}
                  </Box>
                ) : (
                  <Box sx={{ textAlign: 'center', py: 3 }}>
                    <Typography color="text.secondary">
                      No disk data available
                    </Typography>
                  </Box>
                )}
              </CardContent>
            </Card>
          </Box>

          {performanceMetrics.lastUpdate && (
            <Typography 
              variant="caption" 
              sx={{ textAlign: 'right', mt: 2, display: 'block', color: 'text.secondary' }}
            >
              Last Update: {new Date(performanceMetrics.lastUpdate).toLocaleString()}
            </Typography>
          )}
        </>
      )}
    </Box>
  );
}

export default Dashboard;
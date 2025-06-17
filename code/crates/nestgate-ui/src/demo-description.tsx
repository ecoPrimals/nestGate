import React from 'react';
import { Card, CardContent, CardHeader, Typography, Divider, Alert, Box } from '@mui/material';

const DemoDescription: React.FC = () => {
  return (
    <Card sx={{ maxWidth: 800, margin: '20px auto' }}>
      <CardHeader title="NestGate ZFS Dashboard Demo" />
      <CardContent>
      <Alert
          severity="info"
          sx={{ marginBottom: 3 }}
        >
          <Typography variant="body2">
            <strong>Demo Instructions</strong>
            <br />
            Run the demo using 'npm run demo' and navigate to http://localhost:3000 in your browser to see it in action.
          </Typography>
        </Alert>
        
        <Typography variant="h5" gutterBottom>
          Component 1: NasMetrics Dashboard
        </Typography>
        <Typography variant="body1" paragraph>
        The NasMetrics component provides a comprehensive dashboard view of your NAS system with the following sections:
        </Typography>
        
        <Card variant="outlined" sx={{ marginBottom: 2 }}>
          <CardHeader title="Storage Usage" />
          <CardContent>
            <Typography variant="body1" paragraph>
              <Typography component="span" sx={{ fontWeight: 'bold' }}>Key Features:</Typography>
              <Box component="ul" sx={{ paddingLeft: 3, marginTop: 1 }}>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  Total capacity, used space, and free space visualization
                </Typography>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  Usage percentage displayed as a circular progress indicator
                </Typography>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  List of storage pools with health status and capacity details
                </Typography>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  Pool health indicators (ONLINE/DEGRADED/ERROR)
                </Typography>
              </Box>
            </Typography>
          </CardContent>
      </Card>
      
        <Card variant="outlined" sx={{ marginBottom: 2 }}>
          <CardHeader title="System Health" />
          <CardContent>
            <Typography variant="body1" paragraph>
              <Typography component="span" sx={{ fontWeight: 'bold' }}>Key Features:</Typography>
              <Box component="ul" sx={{ paddingLeft: 3, marginTop: 1 }}>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  Overall system health status (Healthy/Warning/Critical)
                </Typography>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  CPU load and memory usage statistics
                </Typography>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  Active system alerts and notifications
                </Typography>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  Visual indicators for health status
                </Typography>
              </Box>
            </Typography>
          </CardContent>
      </Card>
      
        <Card variant="outlined" sx={{ marginBottom: 2 }}>
          <CardHeader title="Services Status" />
          <CardContent>
            <Typography variant="body1" paragraph>
              <Typography component="span" sx={{ fontWeight: 'bold' }}>Key Features:</Typography>
              <Box component="ul" sx={{ paddingLeft: 3, marginTop: 1 }}>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  NFS service status with version, uptime, exports, and client information
                </Typography>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  SMB service status with version, uptime, shares, and connection information
                </Typography>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  Visual indicators for running/stopped services
                </Typography>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  Service details for troubleshooting
                </Typography>
              </Box>
            </Typography>
          </CardContent>
      </Card>
      
        <Card variant="outlined" sx={{ marginBottom: 2 }}>
          <CardHeader title="Performance Metrics" />
          <CardContent>
            <Typography variant="body1" paragraph>
              <Typography component="span" sx={{ fontWeight: 'bold' }}>Key Features:</Typography>
              <Box component="ul" sx={{ paddingLeft: 3, marginTop: 1 }}>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  Read/Write throughput measurements (MB/s)
                </Typography>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  Read/Write latency metrics (ms)
                </Typography>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  IOPS (Input/Output Operations Per Second) monitoring
                </Typography>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  Real-time updates with refresh functionality
                </Typography>
              </Box>
            </Typography>
          </CardContent>
      </Card>
      
        <Divider sx={{ marginY: 3 }} />
        
        <Typography variant="h5" gutterBottom>
          Component 2: Performance Optimizer
        </Typography>
        <Typography variant="body1" paragraph>
        The Performance Optimizer component allows for advanced optimization of ZFS datasets based on workload patterns:
        </Typography>
        
        <Card variant="outlined" sx={{ marginBottom: 2 }}>
          <CardHeader title="Selection Interface" />
          <CardContent>
            <Typography variant="body1" paragraph>
              <Typography component="span" sx={{ fontWeight: 'bold' }}>Key Features:</Typography>
              <Box component="ul" sx={{ paddingLeft: 3, marginTop: 1 }}>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  Storage pool selection dropdown
                </Typography>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  Multiple dataset selection for batch optimization
                </Typography>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  Clear user interface for selecting optimization targets
                </Typography>
              </Box>
            </Typography>
          </CardContent>
      </Card>
      
        <Card variant="outlined" sx={{ marginBottom: 2 }}>
          <CardHeader title="Performance Metrics" />
          <CardContent>
            <Typography variant="body1" paragraph>
              <Typography component="span" sx={{ fontWeight: 'bold' }}>Key Features:</Typography>
              <Box component="ul" sx={{ paddingLeft: 3, marginTop: 1 }}>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  Compression ratio visualization
                </Typography>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  Cache hit ratio analytics
                </Typography>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  Fragmentation level monitoring
                </Typography>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  Visual indicators with progress bars
                </Typography>
              </Box>
            </Typography>
          </CardContent>
      </Card>
      
        <Card variant="outlined" sx={{ marginBottom: 2 }}>
          <CardHeader title="Performance Comparison" />
          <CardContent>
            <Typography variant="body1" paragraph>
              <Typography component="span" sx={{ fontWeight: 'bold' }}>Key Features:</Typography>
              <Box component="ul" sx={{ paddingLeft: 3, marginTop: 1 }}>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  Line chart comparing current vs. optimized performance
                </Typography>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  Metrics for throughput, latency, and IOPS
                </Typography>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  Visual representation of potential improvements
                </Typography>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  Interactive chart elements
                </Typography>
              </Box>
            </Typography>
          </CardContent>
      </Card>
      
        <Card variant="outlined" sx={{ marginBottom: 2 }}>
          <CardHeader title="Optimization Settings" />
          <CardContent>
            <Typography variant="body1" paragraph>
              <Typography component="span" sx={{ fontWeight: 'bold' }}>Key Features:</Typography>
              <Box component="ul" sx={{ paddingLeft: 3, marginTop: 1 }}>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  Record size adjustment controls
                </Typography>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  Current vs. recommended record size comparison
                </Typography>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  "Apply Optimization" button for implementing changes
                </Typography>
                <Typography component="li" variant="body2" sx={{ marginBottom: 0.5 }}>
                  "Reset to Recommended" option to restore defaults
                </Typography>
              </Box>
            </Typography>
          </CardContent>
      </Card>
      
      <Alert
          severity="success"
          sx={{ marginTop: 3 }}
        >
          <Typography variant="body2">
            <strong>Test Coverage</strong>
            <br />
            Both components have comprehensive test coverage with Jest and React Testing Library, ensuring reliability and stability.
          </Typography>
        </Alert>
      </CardContent>
    </Card>
  );
};

export default DemoDescription; 
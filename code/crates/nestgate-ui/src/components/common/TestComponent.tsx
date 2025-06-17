import React, { useEffect, useState } from 'react';
import { Card, CardHeader, CardContent, Typography, Button, Box } from '@mui/material';
import { ZfsPoolService } from '../../services/zfs-pool.service';

const TestComponent: React.FC = () => {
  const [timestamp, setTimestamp] = useState(new Date().toISOString());
  
  useEffect(() => {
    // Log when component is mounted
    ZfsPoolService.logServiceStatus('TestComponent mounted');
    
    // Update timestamp every 5 seconds
    const interval = setInterval(() => {
      const newTimestamp = new Date().toISOString();
      setTimestamp(newTimestamp);
      ZfsPoolService.logServiceStatus(`TestComponent updated: ${newTimestamp}`);
    }, 5000);
    
    return () => {
      clearInterval(interval);
      ZfsPoolService.logServiceStatus('TestComponent unmounted');
    };
  }, []);
  
  const handleRefresh = () => {
    const newTimestamp = new Date().toISOString();
    setTimestamp(newTimestamp);
    ZfsPoolService.logServiceStatus(`TestComponent manually refreshed: ${newTimestamp}`);
  };
  
  return (
    <Card>
      <CardHeader title="Test Component" />
      <CardContent>
        <Typography variant="h6" gutterBottom>
          Hot Module Reload Test
        </Typography>
        <Typography variant="body1" sx={{ marginBottom: 2 }}>
          This component was rendered at: {timestamp}
        </Typography>
        <Box sx={{ marginTop: 2 }}>
          <Button variant="contained" onClick={handleRefresh}>
          Refresh Timestamp
        </Button>
        </Box>
      </CardContent>
    </Card>
  );
};

export default TestComponent; 
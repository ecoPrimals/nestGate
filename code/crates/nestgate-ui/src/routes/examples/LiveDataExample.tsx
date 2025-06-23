import React from 'react';
import { Typography, Box, Divider, Alert, Stack } from '@mui/material';
import StorageDatasetExample from '../../components/storage/StorageDatasetExample';
import { DataSourceIndicator, PlaceholderContent } from '../../components/common';
import { DataSourceType, isStrictLiveMode } from '../../utils/env';

/**
 * Example page that demonstrates the live-only approach with placeholders
 */
const LiveDataExample: React.FC = () => {
  const isStrictMode = isStrictLiveMode();
  
  return (
    <Box sx={{ padding: 3 }}>
      <Typography variant="h4" component="h1" gutterBottom>
        Live Data Implementation Examples
      </Typography>
      
      <Alert
        severity="info"
        sx={{ marginBottom: 3 }}
      >
        <Typography variant="body2">
          <strong>Live-Only Implementation Mode</strong>
          <br />
            This page demonstrates how components should behave in live-only mode.
          The application is currently running with strict live mode{' '}
            <strong>{isStrictMode ? ' enabled' : ' disabled'}</strong>.
        </Typography>
      </Alert>
      
      <Box sx={{ marginBottom: 3 }}>
        <Typography variant="h6" gutterBottom sx={{ fontWeight: 'medium', borderLeft: 3, borderColor: 'primary.main', paddingLeft: 2 }}>
          Data Source Indicators
        </Typography>
        <Divider sx={{ marginBottom: 2 }} />
      </Box>
      
      <Typography variant="body1" paragraph>
        Data source indicators show the origin of data being displayed:
      </Typography>
      
      <Stack direction="row" sx={{ marginBottom: 3 }}>
        <Stack direction="row" alignItems="center">
          <DataSourceIndicator dataSource={DataSourceType.LIVE} />
          <Typography variant="body2">Live data from real system</Typography>
        </Stack>
        
        <Stack direction="row" alignItems="center">
          <DataSourceIndicator dataSource={DataSourceType.PLACEHOLDER} />
          <Typography variant="body2">Feature under development</Typography>
        </Stack>
      </Stack>
      
      <Box sx={{ marginBottom: 3 }}>
        <Typography variant="h6" gutterBottom sx={{ fontWeight: 'medium', borderLeft: 3, borderColor: 'primary.main', paddingLeft: 2 }}>
          Example Components
        </Typography>
        <Divider sx={{ marginBottom: 2 }} />
      </Box>
      
      <Box sx={{ display: 'flex', flexDirection: 'column', gap: 3 }}>
        <Box>
          <StorageDatasetExample />
        </Box>
        
        <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 3 }}>
          <Box sx={{ flex: '1 1 400px', minWidth: 400 }}>
          <PlaceholderContent
            title="Backup Schedule"
            description="Backup scheduling functionality is being implemented with live data integration."
            height={300}
          />
          </Box>
        
          <Box sx={{ flex: '1 1 400px', minWidth: 400 }}>
          <PlaceholderContent
            title="Remote Access Gateway"
            description="Remote access functionality is being integrated with secure authentication."
            height={300}
          />
          </Box>
        </Box>
      </Box>
      
      <Box sx={{ marginTop: 3, marginBottom: 3 }}>
        <Typography variant="h6" gutterBottom sx={{ fontWeight: 'medium', borderLeft: 3, borderColor: 'primary.main', paddingLeft: 2 }}>
          Implementation Guidelines
        </Typography>
        <Divider sx={{ marginBottom: 2 }} />
      </Box>
      
      <Box component="ul" sx={{ paddingLeft: 3 }}>
        <Typography component="li" variant="body1" sx={{ marginBottom: 1 }}>
          All components must support strict live mode with no mock data fallbacks
        </Typography>
        <Typography component="li" variant="body1" sx={{ marginBottom: 1 }}>
          Use <Box component="code" sx={{ 
            backgroundColor: 'grey.100', 
            padding: '2px 6px', 
            borderRadius: 1, 
            fontFamily: 'monospace',
            fontSize: '0.875rem'
          }}>PlaceholderContent</Box> for features without live data implementation
        </Typography>
        <Typography component="li" variant="body1" sx={{ marginBottom: 1 }}>
          Always indicate data source with <Box component="code" sx={{ 
            backgroundColor: 'grey.100', 
            padding: '2px 6px', 
            borderRadius: 1, 
            fontFamily: 'monospace',
            fontSize: '0.875rem'
          }}>DataSourceIndicator</Box>
        </Typography>
        <Typography component="li" variant="body1" sx={{ marginBottom: 1 }}>
          Use <Box component="code" sx={{ 
            backgroundColor: 'grey.100', 
            padding: '2px 6px', 
            borderRadius: 1, 
            fontFamily: 'monospace',
            fontSize: '0.875rem'
          }}>useLiveService</Box> hook for consistent data fetching behavior
        </Typography>
        <Typography component="li" variant="body1" sx={{ marginBottom: 1 }}>
          Handle errors by showing placeholders rather than mock data
        </Typography>
      </Box>
    </Box>
  );
};

export default LiveDataExample; 
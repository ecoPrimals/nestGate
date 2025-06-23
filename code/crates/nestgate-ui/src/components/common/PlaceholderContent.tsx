import React from 'react';
import { Card, Typography, Stack, Box } from '@mui/material';
import { Info as InfoIcon } from '@mui/icons-material';
import DataSourceIndicator from './DataSourceIndicator';
import { DataSourceType } from '../../utils/env';

interface PlaceholderContentProps {
  title?: string;
  description?: string;
  height?: number | string;
  bordered?: boolean;
}

/**
 * A placeholder component for features that are not yet implemented
 * To be used in strict live mode when a feature is pending implementation
 */
const PlaceholderContent: React.FC<PlaceholderContentProps> = ({
  title = 'Feature in development',
  description = 'This feature is currently being implemented with live data integration.',
  height = 'auto',
  bordered = true
}) => {
  return (
    <Card
      variant={bordered ? 'outlined' : 'elevation'}
      sx={{
        height,
        display: 'flex',
        flexDirection: 'column',
        justifyContent: 'center',
        alignItems: 'center',
        textAlign: 'center',
        backgroundColor: '#f0f5ff',
        padding: 3,
      }}
    >
      <Stack alignItems="center">
        <DataSourceIndicator dataSource={DataSourceType.PLACEHOLDER} />
        
        <Typography variant="h5" component="h4" sx={{ marginTop: 2 }}>
          {title}
        </Typography>
        
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <InfoIcon sx={{ color: 'primary.main' }} />
          <Typography variant="body2" color="text.secondary">
            {description}
          </Typography>
        </Box>
      </Stack>
    </Card>
  );
};

export default PlaceholderContent; 
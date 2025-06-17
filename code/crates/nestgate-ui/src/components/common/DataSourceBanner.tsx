import React from 'react';
import { Alert, Box, Typography } from '@mui/material';
import { InfoOutlined, WarningOutlined } from '@mui/icons-material';
import { DataSourceType } from '../../utils/env';
import { MockModeReason } from '../../services/websocket.service';

interface EnvironmentBannerProps {
  serviceName: string;
  isDevelopment?: boolean;
  dataSource?: DataSourceType;
  mockReason?: MockModeReason;
}

/**
 * Banner component that displays relevant environment and service information
 * Only shown in development mode or when using non-live data sources
 */
const EnvironmentBanner: React.FC<EnvironmentBannerProps> = ({
  serviceName,
  isDevelopment = process.env.NODE_ENV === 'development',
  dataSource = DataSourceType.LIVE,
  mockReason = MockModeReason.NONE
}) => {
  // In production with live data, don't show any banner
  if (!isDevelopment && dataSource === DataSourceType.LIVE) {
    return null;
  }

  // For development environment with live data
  if (isDevelopment && dataSource === DataSourceType.LIVE) {
    return (
      <Alert
        severity="info"
        icon={<InfoOutlined />}
        sx={{ mb: 2 }}
      >
        <Box sx={{ display: 'flex', alignItems: 'center' }}>
          <Typography variant="body1" fontWeight="medium">
            {`${serviceName} - Development Mode`}
          </Typography>
        </Box>
        <Typography variant="body2">
          This component is running in development mode with live data.
        </Typography>
      </Alert>
    );
  }

  // For placeholder/test data
  if (dataSource === DataSourceType.PLACEHOLDER || dataSource === DataSourceType.TEST) {
    return (
      <Alert
        severity="warning"
        icon={<WarningOutlined />}
        sx={{ mb: 2 }}
      >
        <Box sx={{ display: 'flex', alignItems: 'center' }}>
          <Typography variant="body1" fontWeight="medium">
            {`${serviceName} - ${dataSource === DataSourceType.PLACEHOLDER ? 'Placeholder' : 'Test'} Data`}
          </Typography>
        </Box>
        <Typography variant="body2">
          {dataSource === DataSourceType.PLACEHOLDER 
            ? "This component is showing placeholder data because live data is unavailable."
            : "This component is showing test data for development purposes."}
        </Typography>
      </Alert>
    );
  }

  // For mock data
  if (dataSource === DataSourceType.MOCK) {
    const reasonText = mockReason === MockModeReason.DELIBERATE
      ? "Mock mode is deliberately enabled for development or testing."
      : "Using mock data because live data is unavailable.";

    return (
      <Alert
        severity="info"
        icon={<InfoOutlined />}
        sx={{ mb: 2 }}
      >
        <Box sx={{ display: 'flex', alignItems: 'center' }}>
          <Typography variant="body1" fontWeight="medium">
            {`${serviceName} - Mock Data`}
          </Typography>
        </Box>
        <Typography variant="body2">
          {reasonText}
        </Typography>
      </Alert>
    );
  }

  return null;
};

export default EnvironmentBanner;
export type { EnvironmentBannerProps as DataSourceBannerProps };
export { EnvironmentBanner as DataSourceBanner }; 
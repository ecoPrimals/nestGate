/**
 * ServiceStatusComponent
 * 
 * A reference component that follows the new approach for fetching service status.
 * It demonstrates proper data fetching, error handling, and loading states.
 */

import React from 'react';
import { useQuery } from '@tanstack/react-query';
import { 
  Box, 
  Typography, 
  Paper, 
  Chip, 
  List, 
  ListItem, 
  ListItemText 
} from '@mui/material';
import {
  CheckCircle as CheckCircleIcon,
  Error as ErrorIcon,
  Pending as PendingIcon,
  Warning as WarningIcon
} from '@mui/icons-material';
import LoadingIndicator from '../common/LoadingIndicator';
import ErrorDisplay from '../common/ErrorDisplay';
import EmptyState from '../common/EmptyState';
import { ServiceStatus } from '../../interfaces/system';
import { SystemService } from '../../services/system.service';

const systemService = SystemService.getInstance();

/**
 * Service status component
 */
const ServiceStatusComponent: React.FC = () => {
  // Use React Query to fetch service statuses
  const {
    data: services,
    isLoading,
    error,
    refetch
  } = useQuery({
    queryKey: ['serviceStatuses'],
    queryFn: () => systemService.getServiceStatuses(),
    retry: 2,
    staleTime: 30000, // 30 seconds
    refetchInterval: 60000 // 1 minute
  });

  // Show loading indicator while fetching data
  if (isLoading) {
    return <LoadingIndicator message="Loading service statuses..." />;
  }

  // Show error state if fetch failed
  if (error) {
    return (
      <ErrorDisplay 
        error={error instanceof Error ? error : new Error(String(error))} 
        onRetry={() => refetch()}
        title="Unable to Load Service Statuses"
        showDetails={true}
      />
    );
  }

  // Show empty state if no services
  if (!services || services.length === 0) {
    return (
      <EmptyState 
        title="No Services Available"
        message="There are currently no system services to display."
        actionLabel="Refresh"
        onAction={() => refetch()}
      />
    );
  }

  // Helper to get status chip color
  const getStatusColor = (status: string): 'success' | 'error' | 'warning' | 'default' => {
    switch (status) {
      case 'running': return 'success';
      case 'failed': return 'error';
      case 'starting':
      case 'stopping': return 'warning';
      default: return 'default';
    }
  };

  // Helper to get status icon
  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'running': return <CheckCircleIcon fontSize="small" color="success" />;
      case 'failed': return <ErrorIcon fontSize="small" color="error" />;
      case 'starting':
      case 'stopping': return <PendingIcon fontSize="small" color="warning" />;
      default: return <WarningIcon fontSize="small" color="disabled" />;
    }
  };

  return (
    <Box sx={{ p: 2 }}>
      <Typography variant="h5" gutterBottom>System Services</Typography>
      
      <List>
        {services.map((service) => (
          <Paper key={service.name} sx={{ mb: 2, p: 2 }}>
            <Box sx={{ display: 'flex', flexWrap: 'wrap', alignItems: 'center', gap: 2 }}>
              <Box sx={{ flex: '1 1 300px', minWidth: { xs: '100%', sm: '300px' } }}>
                <ListItemText
                  primary={service.displayName || service.name}
                  secondary={service.description}
                />
              </Box>
              
              <Box sx={{ flex: '0 0 auto', display: 'flex', gap: 1 }}>
                <Chip
                  icon={getStatusIcon(service.status)}
                  label={service.status.toUpperCase()}
                  color={getStatusColor(service.status)}
                  size="small"
                />
                {service.enabled ? (
                  <Chip label="ENABLED" size="small" color="primary" />
                ) : (
                  <Chip label="DISABLED" size="small" color="default" />
                )}
              </Box>
              
              <Box sx={{ flex: '1 1 200px', minWidth: { xs: '100%', sm: '200px' } }}>
                {service.lastStarted && (
                  <Typography variant="body2" color="text.secondary">
                    Last Started: {new Date(service.lastStarted).toLocaleString()}
                  </Typography>
                )}
                {service.restartCount > 0 && (
                  <Typography variant="body2" color="text.secondary">
                    Restart Count: {service.restartCount}
                  </Typography>
                )}
              </Box>
            </Box>
          </Paper>
        ))}
      </List>
    </Box>
  );
};

export default ServiceStatusComponent; 
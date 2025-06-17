import React from 'react';
import { Box, Grid, Paper, Typography } from '@mui/material';
import { useQuery } from '@tanstack/react-query';
import { apiService } from '@services/api';
import { useApiError } from '@hooks/useApiError';
import { ErrorDisplay } from '@components/ErrorDisplay';
import { AxiosError } from 'axios';
import type { ApiError } from '@hooks/useApiError';

interface StorageSystem {
  id: string;
  name: string;
  status: string;
}

interface SystemMetrics {
  id: string;
  cpu_usage: number;
  memory_usage: number;
  total_memory: number;
  free_memory: number;
  total_disk: number;
  free_disk: number;
  last_update: number;
}

export function Dashboard() {
  const { error, handleError, clearError } = useApiError();

  const { data: storageSystems = [] } = useQuery<StorageSystem[]>({
    queryKey: ['storageSystems'] as const,
    queryFn: async () => {
      try {
        const response = await apiService.get<StorageSystem[]>('/storage-systems');
        return response;
      } catch (error) {
        if (error instanceof AxiosError) {
          handleError(error);
        }
        return [];
      }
    }
  });

  const { data: systemMetrics = [] } = useQuery<SystemMetrics[]>({
    queryKey: ['systemMetrics'] as const,
    queryFn: async () => {
      try {
        const response = await apiService.get<SystemMetrics[]>('/system-metrics');
        return response;
      } catch (error) {
        if (error instanceof AxiosError) {
          handleError(error);
        }
        return [];
      }
    }
  });

  return (
    <Box sx={{ flexGrow: 1, p: 3 }}>
      <Grid container spacing={3}>
        <Grid item xs={12}>
          <Paper sx={{ p: 2 }}>
            <Typography variant="h6" gutterBottom>
              Storage Systems
            </Typography>
            {storageSystems.map((system) => (
              <Typography key={system.id}>
                {system.name} - {system.status}
              </Typography>
            ))}
          </Paper>
        </Grid>
        <Grid item xs={12}>
          <Paper sx={{ p: 2 }}>
            <Typography variant="h6" gutterBottom>
              System Metrics
            </Typography>
            {systemMetrics.map((metric) => (
              <Typography key={metric.id}>
                CPU: {metric.cpu_usage}% | Memory: {metric.memory_usage}%
              </Typography>
            ))}
          </Paper>
        </Grid>
      </Grid>
      <ErrorDisplay message={error?.message || null} onClose={clearError} />
    </Box>
  );
}
import React from 'react';
import { Card, CardHeader, CardContent, CardActions, Typography, LinearProgress, Box, Button, Stack } from '@mui/material';
import { Computer as HddIcon, Settings as SettingsIcon, Info as InfoIcon } from '@mui/icons-material';
import { formatCapacity } from '../../utils/format';
import StatusChip from '../common/StatusChip';

interface Pool {
  id: string;
  name: string;
  health: string;
  size: number;
  used: number;
  free: number;
}

interface PoolStatusCardProps {
  pool: Pool;
}

const PoolStatusCard: React.FC<PoolStatusCardProps> = ({ pool }) => {
  const usagePercentage = Math.round((pool.used / pool.size) * 100);
  
  // Get color based on health status
  const getHealthStatusColor = (status: string): string => {
    switch (status.toLowerCase()) {
      case 'online':
        return 'success';
      case 'degraded':
        return 'warning';
      case 'faulted':
      case 'offline':
        return 'error';
      default:
        return 'default';
    }
  };
  
  // Get color based on usage
  const getUsageColor = (percentage: number): 'success' | 'warning' | 'error' => {
    if (percentage < 70) return 'success';
    if (percentage < 90) return 'warning';
    return 'error';
  };
  
  return (
    <Card>
      <CardHeader 
      title={
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
            <HddIcon />
            <Typography variant="h6" component="span">
              {pool.name}
            </Typography>
          </Box>
        }
        action={
          <StatusChip 
            status={getHealthStatusColor(pool.health)} 
            label={pool.health}
          />
        }
      />
      <CardContent>
        <Stack>
          <Box>
            <Typography variant="body2" color="text.secondary" gutterBottom>
              Usage:
            </Typography>
            <LinearProgress 
              variant="determinate"
              value={usagePercentage}
              color={getUsageColor(usagePercentage)}
              sx={{ 
                height: 8,
                borderRadius: 1,
              }}
            />
            <Typography variant="body2" color="text.secondary" align="center" sx={{ mt: 0.5 }}>
              {usagePercentage}%
            </Typography>
          </Box>
          
          <Stack>
            <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
              <Typography variant="body2">Total Size:</Typography>
              <Typography variant="body2" sx={{ fontWeight: 600 }}>
                {formatCapacity(pool.size)}
              </Typography>
            </Box>
            
            <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
              <Typography variant="body2">Used:</Typography>
              <Typography variant="body2" sx={{ fontWeight: 600 }}>
                {formatCapacity(pool.used)}
              </Typography>
            </Box>
            
            <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
              <Typography variant="body2">Free:</Typography>
              <Typography variant="body2" sx={{ fontWeight: 600 }}>
                {formatCapacity(pool.free)}
              </Typography>
            </Box>
          </Stack>
        </Stack>
      </CardContent>
      <CardActions sx={{ justifyContent: 'space-between', padding: 2 }}>
        <Button startIcon={<InfoIcon />} size="small">
          Details
        </Button>
        <Button startIcon={<SettingsIcon />} size="small">
          Manage
        </Button>
      </CardActions>
    </Card>
  );
};

export default PoolStatusCard; 
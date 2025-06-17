import React from 'react';
import { Card, CardHeader, CardContent, Typography, LinearProgress, Box } from '@mui/material';
import { formatCapacity } from '../../utils/format';

interface StorageUsageCardProps {
  used: number;
  total: number;
  percentage: number;
}

const StorageUsageCard: React.FC<StorageUsageCardProps> = ({ 
  used, 
  total, 
  percentage 
}) => {
  // Determine color based on usage percentage
  const getColorByPercentage = (percent: number): 'success' | 'warning' | 'error' => {
    if (percent < 70) return 'success';
    if (percent < 90) return 'warning';
    return 'error';
  };
  
  const color = getColorByPercentage(percentage);
  
  return (
    <Card>
      <CardHeader title="Storage Usage" />
      <CardContent>
        <Box sx={{ display: 'flex', alignItems: 'center', marginBottom: 3 }}>
          <Typography variant="h4" component="div" sx={{ margin: 0 }}>
            {percentage}%
          </Typography>
          <Box sx={{ flexGrow: 1, marginLeft: 2 }}>
            <LinearProgress 
              variant="determinate"
              value={percentage}
              color={color}
              sx={{ 
                height: 10,
                borderRadius: 1,
              }}
            />
          </Box>
        </Box>
        
        <Box sx={{ display: 'flex', gap: 3 }}>
          <Box sx={{ flex: 1, textAlign: 'center' }}>
            <Typography variant="body2" color="text.secondary">
              Total
            </Typography>
            <Typography variant="h6" component="div">
              {formatCapacity(total)}
            </Typography>
          </Box>
          
          <Box sx={{ flex: 1, textAlign: 'center' }}>
            <Typography variant="body2" color="text.secondary">
              Used
            </Typography>
            <Typography variant="h6" component="div">
              {formatCapacity(used)}
            </Typography>
          </Box>
          
          <Box sx={{ flex: 1, textAlign: 'center' }}>
            <Typography variant="body2" color="text.secondary">
              Free
            </Typography>
            <Typography variant="h6" component="div">
              {formatCapacity(total - used)}
            </Typography>
          </Box>
        </Box>
      </CardContent>
    </Card>
  );
};

export default StorageUsageCard; 
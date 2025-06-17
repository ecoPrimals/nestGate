import React from 'react';
import {
  Card,
  CardContent,
  Typography,
  Box,
  LinearProgress,
  Chip,
  CardActionArea,
} from '@mui/material';
import {
  Storage as StorageIcon,
  Speed as SpeedIcon,
  CompareArrows as CompressIcon,
} from '@mui/icons-material';

export interface StorageTier {
  id: string;
  name: string;
  path: string;
  properties: {
    compression: string;
    recordsize: string;
    atime: string;
    primarycache: string;
    [key: string]: string;
  };
  usage: {
    available: number;
    used: number;
    total: number;
    compressionRatio: number;
  };
  monitoring: {
    enabled: boolean;
    activeEvents: number;
    recentEvents: number;
  };
}

interface TierCardProps {
  tier: StorageTier;
  isSelected?: boolean;
  onClick?: () => void;
}

const TierCard: React.FC<TierCardProps> = ({ tier, isSelected = false, onClick }) => {
  // Calculate usage percentage
  const usagePercent = (tier.usage.used / tier.usage.total) * 100;
  
  // Format bytes to GB with 2 decimal places
  const formatGB = (bytes: number): string => {
    return (bytes / (1024 * 1024 * 1024)).toFixed(2);
  };

  // Get color based on usage
  const getUsageColor = (percent: number): 'success' | 'warning' | 'error' | 'primary' => {
    if (percent < 60) return 'success';
    if (percent < 85) return 'warning';
    return 'error';
  };

  // Get tier display name (capitalized)
  const tierName = tier.name.charAt(0).toUpperCase() + tier.name.slice(1);

  // Determine card border color based on selection state
  const borderColor = isSelected ? 'primary.main' : 'divider';
  const boxShadow = isSelected ? 3 : 1;

  return (
    <Card 
      sx={{ 
        mb: 2, 
        border: 1, 
        borderColor, 
        boxShadow, 
        transition: 'all 0.2s ease-in-out',
        '&:hover': {
          boxShadow: 4,
          borderColor: 'primary.light',
        }
      }}
      className="tier-card"
    >
      <CardActionArea onClick={onClick}>
        <CardContent>
          <Box display="flex" justifyContent="space-between" alignItems="center" mb={2}>
            <Typography variant="h5" component="div" color="primary">
              {tierName} Tier
            </Typography>
            <Box display="flex" alignItems="center">
              <CompressIcon sx={{ mr: 0.5 }} fontSize="small" />
              <Typography variant="body2" color="text.secondary">
                {tier.usage.compressionRatio.toFixed(2)}x
              </Typography>
            </Box>
          </Box>

          <Typography variant="body2" color="text.secondary" gutterBottom>
            {tier.path}
          </Typography>

          <Box mt={2} mb={1}>
            <Box display="flex" justifyContent="space-between" alignItems="center" mb={0.5}>
              <Typography variant="body2">
                Storage Usage
              </Typography>
              <Typography variant="body2" color="text.secondary">
                {formatGB(tier.usage.used)} / {formatGB(tier.usage.total)} GB
              </Typography>
            </Box>
            <LinearProgress 
              variant="determinate" 
              value={usagePercent} 
              color={getUsageColor(usagePercent)}
              sx={{ height: 10, borderRadius: 1 }}
            />
          </Box>

          <Box display="flex" justifyContent="space-between" mt={2}>
            <Chip 
              icon={<StorageIcon />} 
              label={tier.properties.compression} 
              size="small" 
              variant="outlined"
            />
            <Chip 
              icon={<SpeedIcon />} 
              label={tier.properties.recordsize} 
              size="small" 
              variant="outlined"
            />
          </Box>
          
          {tier.monitoring.enabled && tier.monitoring.activeEvents > 0 && (
            <Box mt={2}>
              <Chip 
                label={`${tier.monitoring.activeEvents} active events`} 
                color="primary" 
                size="small" 
                sx={{ width: '100%' }}
              />
            </Box>
          )}
        </CardContent>
      </CardActionArea>
    </Card>
  );
};

export default TierCard; 
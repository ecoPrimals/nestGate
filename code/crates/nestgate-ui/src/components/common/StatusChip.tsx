import React from 'react';
import { Chip, SxProps, Theme } from '@mui/material';

// Type for predefined status options
type StatusType = 'online' | 'offline' | 'warning' | 'error' | 'success' | 'info' | 'loading' | 'default' | string;

interface StatusChipProps {
  status?: StatusType;
  label?: string;
  color?: string;
  size?: 'small' | 'medium';
  variant?: 'filled' | 'outlined';
  icon?: React.ReactElement;
  sx?: SxProps<Theme>;
  onClick?: () => void;
}

/**
 * StatusChip component is a replacement for Ant Design's Tag with 
 * predefined status colors and styling for consistent status indicators.
 */
const StatusChip: React.FC<StatusChipProps> = ({
  status = 'default',
  label,
  color,
  size = 'small',
  variant = 'filled',
  icon,
  sx,
  onClick
}) => {
  // Map status to MUI color
  const getStatusColor = (status: StatusType): string => {
    if (color) return color;
    
    // Map status strings to colors
    const statusColorMap: Record<string, string> = {
      'online': 'success',
      'offline': 'error',
      'warning': 'warning',
      'error': 'error',
      'success': 'success',
      'info': 'info',
      'loading': 'info',
      'up': 'success',
      'down': 'error',
      'active': 'success',
      'inactive': 'default',
      'running': 'success',
      'stopped': 'error',
      'healthy': 'success',
      'critical': 'error',
      'degraded': 'warning',
    };
    
    return statusColorMap[status.toLowerCase()] || 'default';
  };
  
  // Get the display label based on status if not provided
  const displayLabel = label || status.charAt(0).toUpperCase() + status.slice(1);
  
  // Get the color for this status
  const chipColor = getStatusColor(status);
  
  return (
    <Chip
      label={displayLabel}
      color={chipColor as 'success' | 'error' | 'warning' | 'info' | 'default' | 'primary' | 'secondary'}
      size={size}
      variant={variant}
      icon={icon}
      onClick={onClick}
      sx={{
        textTransform: 'capitalize',
        fontWeight: 'medium',
        ...sx
      }}
    />
  );
};

export default StatusChip; 
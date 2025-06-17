/**
 * EmptyState Component
 * 
 * A reusable component for displaying empty state messages when no data is available.
 * Used to replace the previous pattern of showing mock data.
 */

import React from 'react';
import { Box, Typography, Button, Paper } from '@mui/material';
import InboxIcon from '@mui/icons-material/Inbox';

interface EmptyStateProps {
  /**
   * Main title displayed in the empty state
   */
  title: string;
  
  /**
   * Descriptive message about why there's no data
   */
  message?: string;
  
  /**
   * Label for the action button
   */
  actionLabel?: string;
  
  /**
   * Function to call when the action button is clicked
   */
  onAction?: () => void;
  
  /**
   * Custom icon to display instead of the default inbox icon
   */
  icon?: React.ReactNode;
  
  /**
   * Additional CSS styles to apply to the root element
   */
  sx?: React.CSSProperties;
}

/**
 * EmptyState component for displaying when there's no data available
 */
const EmptyState: React.FC<EmptyStateProps> = ({
  title,
  message,
  actionLabel,
  onAction,
  icon,
  sx
}) => {
  return (
    <Paper 
      elevation={0} 
      sx={{ 
        display: 'flex', 
        flexDirection: 'column', 
        alignItems: 'center', 
        justifyContent: 'center', 
        padding: 4,
        backgroundColor: 'background.default',
        borderRadius: 2,
        ...sx
      }}
    >
      <Box sx={{ color: 'text.secondary', fontSize: 64, mb: 2 }}>
        {icon || <InboxIcon fontSize="inherit" />}
      </Box>
      
      <Typography variant="h6" gutterBottom align="center">
        {title}
      </Typography>
      
      {message && (
        <Typography 
          variant="body2" 
          color="text.secondary" 
          align="center"
          sx={{ mb: actionLabel ? 2 : 0 }}
        >
          {message}
        </Typography>
      )}
      
      {actionLabel && onAction && (
        <Button 
          variant="outlined" 
          onClick={onAction}
          sx={{ mt: 2 }}
        >
          {actionLabel}
        </Button>
      )}
    </Paper>
  );
};

export default EmptyState; 
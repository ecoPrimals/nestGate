/**
 * LoadingIndicator Component
 * 
 * A reusable component for displaying loading states when data is being fetched.
 */

import React from 'react';
import { Box, CircularProgress, Typography, Paper } from '@mui/material';

interface LoadingIndicatorProps {
  message?: string;
  size?: 'small' | 'medium' | 'large';
  overlay?: boolean;
  fullHeight?: boolean;
}

/**
 * Size mapping for the loading indicator
 */
const sizeMap = {
  small: {
    spinner: 24,
    fontSize: '0.875rem',
    padding: 2
  },
  medium: {
    spinner: 40,
    fontSize: '1rem',
    padding: 3
  },
  large: {
    spinner: 60,
    fontSize: '1.25rem',
    padding: 4
  }
};

/**
 * LoadingIndicator Component
 */
const LoadingIndicator: React.FC<LoadingIndicatorProps> = ({
  message = 'Loading data...',
  size = 'medium',
  overlay = false,
  fullHeight = false
}) => {
  const sizeProps = sizeMap[size];
  
  const commonContent = (
    <>
      <CircularProgress size={sizeProps.spinner} thickness={4} />
      
      {message && (
        <Typography 
          variant="body2" 
          color="text.secondary" 
          sx={{ 
            mt: 2,
            fontSize: sizeProps.fontSize
          }}
        >
          {message}
        </Typography>
      )}
    </>
  );
  
  // For overlay mode, position absolute over the content
  if (overlay) {
    return (
      <Box
        sx={{
          position: 'absolute',
          top: 0,
          left: 0,
          right: 0,
          bottom: 0,
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
          justifyContent: 'center',
          backgroundColor: 'rgba(255, 255, 255, 0.8)',
          zIndex: 10
        }}
      >
        {commonContent}
      </Box>
    );
  }
  
  // For standard mode
  return (
    <Paper
      elevation={0}
      sx={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        p: sizeProps.padding,
        height: fullHeight ? '100%' : 'auto',
        minHeight: fullHeight ? 200 : 'auto',
        backgroundColor: 'background.default',
        border: 'none'
      }}
    >
      {commonContent}
    </Paper>
  );
};

export default LoadingIndicator; 
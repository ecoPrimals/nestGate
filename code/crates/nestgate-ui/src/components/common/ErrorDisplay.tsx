/**
 * ErrorDisplay Component
 * 
 * A reusable component for displaying error messages with retry options.
 * Used to replace the previous pattern of showing mock data on error.
 */

import React from 'react';
import { Box, Typography, Button, Paper, Alert, AlertTitle } from '@mui/material';
import { ErrorOutline as ErrorIcon, Refresh as RefreshIcon } from '@mui/icons-material';
import { DataUnavailableError, ApiError } from '../../utils/errors';

interface ErrorDisplayProps {
  error: Error | string;
  onRetry?: () => void;
  title?: string;
  showDetails?: boolean;
}

/**
 * ErrorDisplay Component
 */
const ErrorDisplay: React.FC<ErrorDisplayProps> = ({
  error,
  onRetry,
  title = 'Error',
  showDetails = false,
}) => {
  // Get the error message
  const errorMessage = typeof error === 'string' 
    ? error 
    : error instanceof DataUnavailableError 
      ? error.getFullMessage()
      : error.message;
  
  // Check if the error is retryable
  const isRetryable = error instanceof DataUnavailableError 
    ? error.retryable 
    : error instanceof ApiError 
      ? error.retryable 
      : true;

  return (
    <Paper 
      elevation={0} 
      sx={{ 
        p: 3, 
        display: 'flex', 
        flexDirection: 'column', 
        alignItems: 'center',
        backgroundColor: 'background.default',
        borderRadius: 2
      }}
    >
      <ErrorIcon color="error" sx={{ fontSize: 48, mb: 2 }} />
      
      <Typography variant="h6" color="error" gutterBottom>
        {title}
      </Typography>
      
      <Typography variant="body1" color="text.secondary" align="center" sx={{ mb: 3 }}>
        {errorMessage}
      </Typography>
      
      {showDetails && error instanceof DataUnavailableError && error.originalError && (
        <Alert severity="info" sx={{ mb: 2, width: '100%' }}>
          <AlertTitle>Technical Details</AlertTitle>
          {error.originalError.message}
        </Alert>
      )}
      
      {isRetryable && onRetry && (
        <Button 
          variant="contained" 
          color="primary" 
          startIcon={<RefreshIcon />}
          onClick={onRetry}
        >
          Retry
        </Button>
      )}
    </Paper>
  );
};

export default ErrorDisplay; 
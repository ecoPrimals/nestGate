import React from 'react';
import { Alert, Snackbar } from '@mui/material';

interface ErrorDisplayProps {
  message: string | null;
  onClose: () => void;
}

export function ErrorDisplay({ message, onClose }: ErrorDisplayProps) {
  if (!message) return null;

  return (
    <Snackbar
      open={Boolean(message)}
      autoHideDuration={6000}
      onClose={onClose}
      anchorOrigin={{ vertical: 'bottom', horizontal: 'right' }}
    >
      <Alert onClose={onClose} severity="error" sx={{ width: '100%' }}>
        {message}
      </Alert>
    </Snackbar>
  );
} 
import React, { useState } from 'react';
import { Alert, Snackbar } from '@mui/material';
import { AxiosError } from 'axios';

interface ApiError {
  message: string;
  code?: string;
  details?: Record<string, unknown>;
}

export function useApiError() {
  const [error, setError] = useState<ApiError | null>(null);

  const handleError = (error: AxiosError<ApiError>) => {
    const message = error.response?.data?.message || error.message;
    setError({ message });
  };

  const clearError = () => {
    setError(null);
  };

  const ErrorComponent = () => {
    if (!error) return null;

    return (
      <Snackbar
        open={Boolean(error)}
        autoHideDuration={6000}
        onClose={clearError}
        anchorOrigin={{ vertical: 'bottom', horizontal: 'right' }}
      >
        <Alert onClose={clearError} severity="error" sx={{ width: '100%' }}>
          {error.message}
        </Alert>
      </Snackbar>
    );
  };

  return {
    error,
    handleError,
    clearError,
    ErrorComponent,
  };
} 
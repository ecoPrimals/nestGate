import { useState } from 'react';
import { AxiosError } from 'axios';

export interface ApiError {
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

  return {
    error,
    handleError,
    clearError,
  };
} 
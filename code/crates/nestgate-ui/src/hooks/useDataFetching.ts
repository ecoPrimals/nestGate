/**
 * Data Fetching Hook
 * 
 * Provides consistent data fetching with proper error handling.
 * Replaces the previous pattern of falling back to mock data.
 */

import { useQuery, UseQueryOptions, UseQueryResult } from '@tanstack/react-query';
import { DataUnavailableError, ApiError } from '../utils/errors';

/**
 * Custom hook for data fetching with proper error handling
 */
export function useDataFetching<TData = unknown>(
  queryKey: readonly unknown[],
  fetchFn: () => Promise<TData>,
  options: {
    serviceName?: string;
    retry?: number;
    staleTime?: number;
    cacheTime?: number;
    enabled?: boolean;
    onSuccess?: (data: TData) => void;
    onError?: (error: DataUnavailableError) => void;
  } = {}
): UseQueryResult<TData, DataUnavailableError> {
  const {
    serviceName = 'unknown',
    retry = 3,
    staleTime,
    cacheTime,
    enabled = true,
    onSuccess,
    onError
  } = options;

  const queryFn = async (): Promise<TData> => {
    try {
      return await fetchFn();
    } catch (error) {
      if (error instanceof DataUnavailableError) {
        throw error;
      }

      if (error instanceof Error) {
        throw new DataUnavailableError(`Failed to fetch data from ${serviceName}`, {
          originalError: error,
          serviceName,
          retryable: !(error instanceof ApiError) || (error as ApiError).retryable
        });
      }
      
      throw new DataUnavailableError(`Unknown error fetching data from ${serviceName}`, {
        serviceName
      });
    }
  };

  return useQuery(
    queryKey,
    queryFn,
    {
      retry,
      staleTime,
      cacheTime,
      enabled,
      onSuccess,
      onError
    }
  );
}

/**
 * Helper component props for data fetching
 */
export interface DataFetchingComponentProps<TData> {
  /**
   * Data from the query
   */
  data: TData | undefined;
  
  /**
   * Loading state
   */
  isLoading: boolean;
  
  /**
   * Error state
   */
  error: DataUnavailableError | null;
  
  /**
   * Function to refetch data
   */
  refetch: () => void;
} 
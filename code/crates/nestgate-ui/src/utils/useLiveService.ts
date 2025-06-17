import { useState, useEffect } from 'react';
import { DataSourceType, isStrictLiveMode } from './env';

/**
 * Generic hook for handling API data fetching with proper live mode support
 * @param fetchFn The function to fetch data
 * @param dependencies Array of dependencies to trigger refetch
 * @returns Object containing data, loading state, error, and data source type
 */
export function useLiveService<T>(
  fetchFn: () => Promise<T>,
  dependencies: any[] = []
): {
  data: T | null;
  isLoading: boolean;
  error: Error | null;
  dataSource: DataSourceType;
  refetch: () => Promise<void>;
} {
  const [data, setData] = useState<T | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<Error | null>(null);
  const [dataSource, setDataSource] = useState<DataSourceType>(DataSourceType.LIVE);

  const fetchData = async () => {
    setIsLoading(true);
    setError(null);
    
    try {
      const result = await fetchFn();
      setData(result);
      
      // Determine data source type from the response
      if (Array.isArray(result) && result.length > 0 && 'dataSource' in result[0]) {
        setDataSource(result[0].dataSource as DataSourceType);
      } else if (result && typeof result === 'object' && 'dataSource' in result) {
        setDataSource(result.dataSource as DataSourceType);
      } else {
        setDataSource(DataSourceType.LIVE);
      }
    } catch (err) {
      setError(err instanceof Error ? err : new Error(String(err)));
      
      // In strict live mode, we show placeholder
      if (isStrictLiveMode()) {
        setDataSource(DataSourceType.PLACEHOLDER);
      } else {
        setDataSource(DataSourceType.LIVE);
      }
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, dependencies);

  const refetch = async () => {
    await fetchData();
  };

  return { data, isLoading, error, dataSource, refetch };
} 
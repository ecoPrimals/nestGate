/**
 * DataFetchingExample Component
 * 
 * Demonstrates proper data fetching with error handling.
 * Shows how to use React Query with error handling components.
 */

import React from 'react';
import { useQuery } from '@tanstack/react-query';
import { Box, Typography, Card, CardContent } from '@mui/material';
import LoadingIndicator from '../common/LoadingIndicator';
import ErrorDisplay from '../common/ErrorDisplay';
import EmptyState from '../common/EmptyState';
import { DataUnavailableError } from '../../utils/errors';

/**
 * Example data type
 */
interface ExampleData {
  id: number;
  name: string;
  description: string;
}

/**
 * Example API function that fetches data
 */
const fetchExampleData = async (): Promise<ExampleData[]> => {
  try {
    const response = await fetch('/api/examples');
    
    if (!response.ok) {
      throw new Error(`API error: ${response.status}`);
    }
    
    const data = await response.json();
    
    return data;
  } catch (error) {
    // Transform all errors to DataUnavailableError for consistent handling
    if (error instanceof Error) {
      throw new DataUnavailableError('Failed to fetch example data', {
        originalError: error,
        serviceName: 'ExampleService',
        retryable: true
      });
    }
    
    throw new DataUnavailableError('Unknown error fetching example data', {
      serviceName: 'ExampleService',
      retryable: true
    });
  }
};

/**
 * DataFetchingExample Component
 */
const DataFetchingExample: React.FC = () => {
  // Use React Query to fetch data
  const {
    data,
    isLoading,
    error,
    refetch
  } = useQuery<ExampleData[], DataUnavailableError>({
    queryKey: ['exampleData'],
    queryFn: fetchExampleData,
    retry: 2
  });
  
  // Show loading state
  if (isLoading) {
    return <LoadingIndicator message="Loading example data..." />;
  }
  
  // Show error state
  if (error) {
    return (
      <ErrorDisplay 
        error={error} 
        onRetry={() => refetch()}
        title="Unable to Load Data"
        showDetails={true}
      />
    );
  }
  
  // Show empty state if no data
  if (!data || data.length === 0) {
    return (
      <EmptyState 
        title="No Examples Available"
        message="There are currently no examples to display."
        actionLabel="Refresh"
        onAction={() => refetch()}
      />
    );
  }
  
  // Show data
  return (
    <Box sx={{ p: 2 }}>
      <Typography variant="h5" gutterBottom>Example Data</Typography>
      
      {data.map(item => (
        <Card key={item.id} sx={{ mb: 2 }}>
          <CardContent>
            <Typography variant="h6">{item.name}</Typography>
            <Typography variant="body2" color="text.secondary">
              {item.description}
            </Typography>
          </CardContent>
        </Card>
      ))}
    </Box>
  );
};

export default DataFetchingExample; 
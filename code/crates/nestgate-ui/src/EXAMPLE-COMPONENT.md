# Using the New Data Architecture

This document provides examples of how to use the new data architecture that has completely separated mock data from production code.

## Component Example

Here's an example of a component that follows the new pattern:

```tsx
import React from 'react';
import { useQuery } from '@tanstack/react-query';
import { Box, Typography } from '@mui/material';
import LoadingIndicator from '../common/LoadingIndicator';
import ErrorDisplay from '../common/ErrorDisplay';
import EmptyState from '../common/EmptyState';
import { DataUnavailableError } from '../../utils/errors';

// Import the real API service - no mock data imports!
import { apiService } from '../../services/api/apiService';

interface MyComponentProps {
  resourceId: string;
}

const MyComponent: React.FC<MyComponentProps> = ({ resourceId }) => {
  // Use React Query for data fetching with error handling
  const {
    data,
    isLoading,
    error,
    refetch
  } = useQuery({
    queryKey: ['resource', resourceId],
    queryFn: () => apiService.fetchResource(resourceId),
    retry: 2
  });
  
  // Show loading state
  if (isLoading) {
    return <LoadingIndicator message="Loading resource data..." />;
  }
  
  // Show error state - no fallback to mock data
  if (error) {
    return (
      <ErrorDisplay 
        error={error instanceof Error ? error : new Error('Unknown error')} 
        onRetry={() => refetch()}
        title="Unable to Load Resource"
      />
    );
  }
  
  // Show empty state if no data
  if (!data) {
    return (
      <EmptyState 
        title="No Resource Available"
        message="The requested resource is not available."
        actionLabel="Refresh"
        onAction={() => refetch()}
      />
    );
  }
  
  // Render the actual data
  return (
    <Box>
      <Typography variant="h5">{data.name}</Typography>
      <Typography variant="body1">{data.description}</Typography>
    </Box>
  );
};

export default MyComponent;
```

## API Service Example

Here's how to implement an API service with proper error handling:

```tsx
import axios from 'axios';
import { DataUnavailableError } from '../../utils/errors';

// Define the data interface
interface Resource {
  id: string;
  name: string;
  description: string;
}

// Create an API service with proper error handling
export const apiService = {
  async fetchResource(id: string): Promise<Resource> {
    try {
      const response = await axios.get(`/api/resources/${id}`);
      return response.data;
    } catch (error) {
      // Transform all errors to DataUnavailableError for consistent handling
      throw new DataUnavailableError(`Failed to fetch resource with ID ${id}`, {
        originalError: error instanceof Error ? error : undefined,
        serviceName: 'ResourceService',
        retryable: true
      });
    }
  },
  
  async createResource(data: Omit<Resource, 'id'>): Promise<Resource> {
    try {
      const response = await axios.post('/api/resources', data);
      return response.data;
    } catch (error) {
      throw new DataUnavailableError('Failed to create resource', {
        originalError: error instanceof Error ? error : undefined,
        serviceName: 'ResourceService',
        retryable: false // Creation should not be retried automatically
      });
    }
  }
};
```

## Testing Example

Here's how to test a component using the mock data:

```tsx
import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import MyComponent from './MyComponent';

// Import the API service that we'll mock
import { apiService } from '../../services/api/apiService';

// Mock the API service
jest.mock('../../services/api/apiService', () => ({
  apiService: {
    fetchResource: jest.fn()
  }
}));

describe('MyComponent', () => {
  let queryClient: QueryClient;
  
  beforeEach(() => {
    queryClient = new QueryClient({
      defaultOptions: {
        queries: {
          retry: false,
        },
      },
    });
  });
  
  it('renders loading state initially', () => {
    // Mock the API call to never resolve during this test
    (apiService.fetchResource as jest.Mock).mockImplementation(() => new Promise(() => {}));
    
    render(
      <QueryClientProvider client={queryClient}>
        <MyComponent resourceId="123" />
      </QueryClientProvider>
    );
    
    expect(screen.getByText('Loading resource data...')).toBeInTheDocument();
  });
  
  it('renders error state when API fails', async () => {
    // Mock the API call to fail
    (apiService.fetchResource as jest.Mock).mockRejectedValue(new Error('API error'));
    
    render(
      <QueryClientProvider client={queryClient}>
        <MyComponent resourceId="123" />
      </QueryClientProvider>
    );
    
    await waitFor(() => {
      expect(screen.getByText('Unable to Load Resource')).toBeInTheDocument();
    });
  });
  
  it('renders data correctly', async () => {
    // Mock the API call to return test data
    (apiService.fetchResource as jest.Mock).mockResolvedValue({
      id: '123',
      name: 'Test Resource',
      description: 'This is a test resource'
    });
    
    render(
      <QueryClientProvider client={queryClient}>
        <MyComponent resourceId="123" />
      </QueryClientProvider>
    );
    
    await waitFor(() => {
      expect(screen.getByText('Test Resource')).toBeInTheDocument();
      expect(screen.getByText('This is a test resource')).toBeInTheDocument();
    });
  });
});
```

## Key Principles

1. **No Mock Data in Production**: Never import from `__mocks__` in production code
2. **Proper Error Handling**: Use `DataUnavailableError` for consistent error handling
3. **React Query**: Use React Query for data fetching with automatic retries
4. **Component Error States**: Always implement loading, error, and empty states
5. **Testing**: Test components using Jest mocks, not by importing mock data

By following these patterns, we maintain a clean separation between production and test code. 
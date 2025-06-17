import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { CssBaseline, ThemeProvider } from '@mui/material';
import { BrowserRouter } from 'react-router-dom';
import AppRoutes from './routes';
import theme from './theme';
import { getConfig, initializeConfig } from './config';

// Initialize dynamic configuration and start the app
async function startApp() {
  // Initialize dynamic configuration from port manager
  await initializeConfig();
  
  // Get configuration
  const config = getConfig();

  // Log the startup configuration
  console.log('Starting NestGate UI with configuration:');
  console.log('  Environment:', process.env.NODE_ENV);
  console.log('  API URI:', config.API_URI);
  console.log('  Strict Mode:', config.STRICT_DATA_MODE);

  // Create React Query client
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        refetchOnWindowFocus: false,
        retry: 3, // Default retry attempts
        staleTime: 60000, // 1 minute
        cacheTime: 300000, // 5 minutes
      },
    },
  });

  const root = ReactDOM.createRoot(
    document.getElementById('root') as HTMLElement
  );

  root.render(
    <React.StrictMode>
      <QueryClientProvider client={queryClient}>
        <ThemeProvider theme={theme}>
          <BrowserRouter>
            <CssBaseline />
            <AppRoutes />
          </BrowserRouter>
        </ThemeProvider>
      </QueryClientProvider>
    </React.StrictMode>
  );
}

// Start the application
startApp().catch(error => {
  console.error('Failed to start application:', error);
  // Still start the app with fallback configuration
  const config = getConfig();
  console.warn('Starting with fallback configuration');
  
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        refetchOnWindowFocus: false,
        retry: 3,
        staleTime: 60000,
        cacheTime: 300000,
      },
    },
  });

  const root = ReactDOM.createRoot(
    document.getElementById('root') as HTMLElement
  );

  root.render(
    <React.StrictMode>
      <QueryClientProvider client={queryClient}>
        <ThemeProvider theme={theme}>
          <BrowserRouter>
            <CssBaseline />
            <AppRoutes />
          </BrowserRouter>
        </ThemeProvider>
      </QueryClientProvider>
    </React.StrictMode>
  );
}); 
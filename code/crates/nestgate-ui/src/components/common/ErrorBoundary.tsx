import React, { Component, ErrorInfo, ReactNode } from 'react';
import { Alert, Button, Stack, Typography, Box } from '@mui/material';
import { Error as ErrorIcon, Refresh as RefreshIcon } from '@mui/icons-material';

interface ErrorBoundaryProps {
  children: ReactNode;
  fallback?: ReactNode;
  onReset?: () => void;
}

interface ErrorBoundaryState {
  hasError: boolean;
  error: Error | null;
}

/**
 * Error Boundary component that catches JavaScript errors in its child component tree,
 * logs those errors, and displays a fallback UI instead of the component tree that crashed.
 */
class ErrorBoundary extends Component<ErrorBoundaryProps, ErrorBoundaryState> {
  constructor(props: ErrorBoundaryProps) {
    super(props);
    this.state = {
      hasError: false,
      error: null,
    };
  }

  static getDerivedStateFromError(error: Error): ErrorBoundaryState {
    // Update state so the next render will show the fallback UI
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo): void {
    // You can also log the error to an error reporting service
    console.error('ErrorBoundary caught an error:', error, errorInfo);
  }

  resetErrorBoundary = (): void => {
    if (this.props.onReset) {
      this.props.onReset();
    }
    this.setState({ hasError: false, error: null });
  };

  render(): ReactNode {
    if (this.state.hasError) {
      // Custom fallback UI
      if (this.props.fallback) {
        return this.props.fallback;
      }

      // Default fallback UI
      return (
        <Box sx={{ padding: 2 }}>
          <Alert
            severity="error"
            icon={<ErrorIcon />}
            sx={{ marginBottom: 2 }}
          >
            <Typography variant="h6" component="div" sx={{ marginBottom: 1 }}>
              Something went wrong
            </Typography>
            
            <Stack>
              <Typography variant="body2">
                An error occurred in this component. Please try refreshing the page or contact support if the issue persists.
              </Typography>
              
              {this.state.error && (
                <Box
                  component="pre"
                  sx={{
                    backgroundColor: 'grey.100',
                    padding: 1,
                    borderRadius: 1,
                    fontSize: '0.75rem',
                    fontFamily: 'monospace',
                    whiteSpace: 'pre-wrap',
                    wordBreak: 'break-word',
                    maxHeight: 200,
                    overflow: 'auto',
                  }}
                >
                  {this.state.error.toString()}
                </Box>
              )}
              
              <Button
                variant="contained"
                color="primary"
                startIcon={<RefreshIcon />}
                onClick={this.resetErrorBoundary}
                sx={{ alignSelf: 'flex-start' }}
              >
                Try again
              </Button>
            </Stack>
          </Alert>
        </Box>
      );
    }

    return this.props.children;
  }
}

export default ErrorBoundary; 
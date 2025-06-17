import React, { useContext } from 'react';
import { Navigate } from 'react-router-dom';
import { Box, CircularProgress, Typography, Paper } from '@mui/material';
import { Block as BlockIcon } from '@mui/icons-material';
import { Permission, UserService } from '../../services/user.service';
import { AuthContext } from '../../contexts/AuthContext';

interface ProtectedRouteProps {
  children: React.ReactNode;
  requiredPermission?: Permission;
}

export const ProtectedRoute: React.FC<ProtectedRouteProps> = ({ 
  children, 
  requiredPermission 
}) => {
  const { user, isAuthenticated, isLoading } = useContext(AuthContext);
  
  // Show loading spinner while checking authentication
  if (isLoading) {
    return (
      <Box 
        sx={{ 
          display: 'flex', 
          justifyContent: 'center', 
          alignItems: 'center', 
          height: '100vh',
          flexDirection: 'column',
          gap: 2
        }}
      >
        <CircularProgress size={48} />
        <Typography variant="body1" color="text.secondary">
          Loading...
        </Typography>
      </Box>
    );
  }
  
  // Redirect to login if not authenticated
  if (!isAuthenticated) {
    return <Navigate to="/login" />;
  }
  
  // Check for required permission
  if (requiredPermission && user) {
    const hasPermission = UserService.hasPermission(user, requiredPermission);
    
    if (!hasPermission) {
      return (
        <Box 
          sx={{ 
            display: 'flex', 
            justifyContent: 'center', 
            alignItems: 'center', 
            height: '100vh',
            padding: 3
          }}
        >
          <Paper 
            sx={{ 
              padding: 4, 
              textAlign: 'center',
              maxWidth: 400
            }}
          >
            <BlockIcon 
              sx={{ 
                fontSize: 64, 
                color: 'error.main',
                marginBottom: 2
              }} 
            />
            <Typography variant="h4" gutterBottom>
              403
            </Typography>
            <Typography variant="body1" color="text.secondary">
              Sorry, you don't have permission to access this page.
            </Typography>
          </Paper>
        </Box>
      );
    }
  }
  
  // Render children if authenticated and has permission
  return <>{children}</>;
}; 
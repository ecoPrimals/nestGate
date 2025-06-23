import React, { useState, useContext, useEffect } from 'react';
import { 
  Box,
  Card,
  CardContent,
  TextField,
  Button, 
  Typography, 
  Alert, 
  CircularProgress,
  Divider,
  Stack,
  InputAdornment,
} from '@mui/material';
import { 
  Person as UserIcon,
  Lock as LockIcon,
  Login as LoginIcon,
  Security as SafetyIcon,
} from '@mui/icons-material';
import { Navigate, useNavigate } from 'react-router-dom';
import { AuthContext } from '../contexts/AuthContext';

const Login: React.FC = () => {
  const navigate = useNavigate();
  const { login, isAuthenticated, error: authError, isLoading: authLoading } = useContext(AuthContext);
  const [error, setError] = useState<string | null>(null);
  const [credentials, setCredentials] = useState({ username: '', password: '' });

  // If already authenticated, redirect to dashboard
  useEffect(() => {
    if (isAuthenticated) {
      navigate('/dashboard');
    }
  }, [isAuthenticated, navigate]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    
    if (!credentials.username || !credentials.password) {
      setError('Please enter both username and password');
      return;
    }
    
    try {
      // Use the login method from context
      const success = await login(credentials.username, credentials.password);
      
      if (success) {
        navigate('/dashboard');
      } else {
        setError(authError || 'Invalid username or password. Please try again.');
      }
    } catch (err) {
      // Handle any unexpected errors
      setError('An error occurred during login. Please try again.');
      console.error('Login error:', err);
    }
  };

  const handleInputChange = (field: 'username' | 'password') => (
    e: React.ChangeEvent<HTMLInputElement>
  ) => {
    setCredentials(prev => ({ ...prev, [field]: e.target.value }));
    if (error) setError(null);
  };

  // If already authenticated, redirect to dashboard
  if (isAuthenticated) {
    return <Navigate to="/dashboard" />;
  }

  return (
    <Box
      sx={{
      height: '100vh', 
      display: 'flex', 
      justifyContent: 'center', 
      alignItems: 'center',
        background: '#f5f5f5',
        padding: 2,
      }}
    >
      <Card 
        sx={{
          width: { xs: '90%', sm: 450 },
          boxShadow: '0 4px 12px rgba(0, 0, 0, 0.1)',
          borderRadius: 2,
        }}
      >
        <CardContent sx={{ padding: 4 }}>
          <Stack alignItems="center" sx={{ marginBottom: 4 }}>
            <SafetyIcon sx={{ fontSize: 48, color: 'primary.main' }} />
            <Typography variant="h4" component="h1">
              NestGate
            </Typography>
            <Typography variant="body2" color="text.secondary">
              ZFS Storage Management System
            </Typography>
          </Stack>

        {error && (
          <Alert
              severity="error"
              sx={{ marginBottom: 3 }}
              onClose={() => setError(null)}
            >
              <Typography variant="body2">
                <strong>Authentication Error:</strong> {error}
              </Typography>
            </Alert>
          )}

          <form onSubmit={handleSubmit}>
            <TextField
              fullWidth
              label="Username"
              value={credentials.username}
              onChange={handleInputChange('username')}
              margin="normal"
              required
              disabled={authLoading}
              InputProps={{
                startAdornment: (
                  <InputAdornment position="start">
                    <UserIcon />
                  </InputAdornment>
                ),
              }}
            />

            <TextField
              fullWidth
              type="password"
              label="Password"
              value={credentials.password}
              onChange={handleInputChange('password')}
              margin="normal"
              required
              disabled={authLoading}
              InputProps={{
                startAdornment: (
                  <InputAdornment position="start">
                    <LockIcon />
                  </InputAdornment>
                ),
              }}
            />

            <Button 
              type="submit"
              fullWidth
              variant="contained"
              size="large"
              disabled={authLoading || !credentials.username || !credentials.password}
              startIcon={authLoading ? <CircularProgress size={20} /> : <LoginIcon />}
              sx={{
                marginTop: 3,
                marginBottom: 2,
                height: 48,
              }}
            >
              {authLoading ? 'Signing In...' : 'Sign In'}
            </Button>
          </form>

          <Divider sx={{ marginY: 3 }} />
          
          <Stack alignItems="center">
            <Typography variant="body2" color="text.secondary">
              Default credentials for testing:
            </Typography>
            <Typography
              variant="body2"
              sx={{
                fontFamily: 'monospace',
                backgroundColor: 'grey.100',
                padding: '4px 8px',
                borderRadius: 1,
              }}
            >
              Username: admin / Password: admin
            </Typography>
          </Stack>
        </CardContent>
      </Card>
    </Box>
  );
};

export default Login; 
import React, { useState } from 'react';
import {
  Box,
  Card,
  CardContent,
  TextField,
  Button,
  Typography,
  Alert,
  InputAdornment,
  useTheme,
} from '@mui/material';
import {
  Person as UserIcon,
  Lock as LockIcon,
} from '@mui/icons-material';
import { AuthService } from '../../services/auth.service';

interface LoginProps {
  onLoginSuccess: () => void;
}

export const Login: React.FC<LoginProps> = ({ onLoginSuccess }) => {
  const [loading, setLoading] = useState(false);
  const [credentials, setCredentials] = useState({ username: '', password: '' });
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const theme = useTheme();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!credentials.username || !credentials.password) {
      setError('Please fill in all fields');
      return;
    }

    try {
      setLoading(true);
      setError(null);
      await AuthService.login(credentials.username, credentials.password);
      setSuccess('Login successful!');
      setTimeout(() => onLoginSuccess(), 1000);
    } catch (error) {
      setError('Invalid username or password');
    } finally {
      setLoading(false);
    }
  };

  const handleInputChange = (field: 'username' | 'password') => (
    e: React.ChangeEvent<HTMLInputElement>
  ) => {
    setCredentials(prev => ({ ...prev, [field]: e.target.value }));
    if (error) setError(null);
  };

  return (
    <Box
      sx={{
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        minHeight: '100vh',
        backgroundColor: '#f0f2f5',
        padding: 2,
      }}
    >
      <Card
        sx={{
          width: { xs: '90%', sm: 400 },
          boxShadow: '0 8px 24px rgba(0, 0, 0, 0.1)',
          borderRadius: 2,
        }}
      >
        <CardContent sx={{ padding: 4 }}>
          <Box sx={{ textAlign: 'center', marginBottom: 3 }}>
            <Typography variant="h4" component="h1" gutterBottom>
              NestGate Storage Manager
            </Typography>
            <Typography variant="h6" color="text.secondary">
              Sign in to your account
            </Typography>
          </Box>

          {error && (
            <Alert severity="error" sx={{ marginBottom: 2 }}>
              {error}
            </Alert>
          )}

          {success && (
            <Alert severity="success" sx={{ marginBottom: 2 }}>
              {success}
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
              disabled={loading}
              sx={{
                marginTop: 3,
                marginBottom: 2,
                height: 48,
                fontSize: '1rem',
              }}
            >
              {loading ? 'Signing in...' : 'Sign in'}
            </Button>
          </form>

          <Box
            sx={{
              marginTop: 3,
              fontSize: '0.875rem',
              color: 'text.secondary',
              backgroundColor: '#f9f9f9',
              padding: 2,
              borderRadius: 1,
            }}
          >
            <Typography variant="body2" gutterBottom>
              Use one of the following accounts:
            </Typography>
            <Typography variant="body2">
              <strong>Admin:</strong> username: admin, password: admin
            </Typography>
            <Typography variant="body2">
              <strong>ReadOnly:</strong> username: readonly, password: readonly
            </Typography>
          </Box>
        </CardContent>
      </Card>
    </Box>
  );
}; 
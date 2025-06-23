import React, { useEffect, useState, useCallback } from 'react';
import { 
  Dialog, 
  DialogTitle, 
  DialogContent, 
  DialogActions, 
  Button, 
  Stack, 
  Typography, 
  LinearProgress, 
  Box,
  Alert
} from '@mui/material';
import { 
  AccessTime as ClockCircleIcon, 
  Warning as ExclamationCircleIcon, 
  Refresh as LoadingIcon 
} from '@mui/icons-material';
import { AuthService } from '../../services/auth.service';

interface SessionProps {
  onSessionExpired: () => void;
  sessionTimeoutMinutes?: number;
  showWarningBeforeMinutes?: number;
}

export const SessionManager: React.FC<SessionProps> = ({
  onSessionExpired,
  sessionTimeoutMinutes = 30,
  showWarningBeforeMinutes = 5
}) => {
  const [visible, setVisible] = useState(false);
  const [countdown, setCountdown] = useState(0);
  const [activity, setActivity] = useState(Date.now());
  const [refreshing, setRefreshing] = useState(false);
  const [message, setMessage] = useState<{ type: 'success' | 'error'; text: string } | null>(null);

  // Calculate session expiry based on token expiry
  const getSessionExpiry = useCallback(() => {
    const tokenExpiry = AuthService.getTokenExpiry();
    if (!tokenExpiry) {
      return null;
    }
    return tokenExpiry;
  }, []);

  // Set up activity monitoring
  useEffect(() => {
    const activityEvents = ['mousedown', 'keydown', 'scroll', 'touchstart'];
    
    const handleActivity = () => {
      setActivity(Date.now());
      if (visible) {
        // If warning is showing and user is active, extend session
        extendSession();
      }
    };
    
    // Add event listeners for user activity
    activityEvents.forEach(event => {
      window.addEventListener(event, handleActivity);
    });
    
    return () => {
      // Clean up event listeners
      activityEvents.forEach(event => {
        window.removeEventListener(event, handleActivity);
      });
    };
  }, [visible]);

  // Monitor session timeout
  useEffect(() => {
    const checkExpiryInterval = setInterval(() => {
      const sessionExpiry = getSessionExpiry();
      if (!sessionExpiry) {
        // No token expiry found - session might be invalid
        handleSessionExpired();
        return;
      }

      const currentTime = Date.now();
      const timeUntilExpiry = sessionExpiry - currentTime;
      const timeUntilExpiryMinutes = timeUntilExpiry / (1000 * 60);
      
      if (timeUntilExpiry <= 0) {
        // Session expired
        handleSessionExpired();
      } else if (timeUntilExpiryMinutes <= showWarningBeforeMinutes && !visible) {
        // Show warning before expiry
        showWarning();
      }
    }, 10000); // Check every 10 seconds
    
    return () => clearInterval(checkExpiryInterval);
  }, [activity, visible, sessionTimeoutMinutes, showWarningBeforeMinutes, getSessionExpiry]);

  // Countdown timer when warning is visible
  useEffect(() => {
    let timer: NodeJS.Timeout;
    
    if (visible) {
      const sessionExpiry = getSessionExpiry();
      if (sessionExpiry) {
        const timeUntilExpiry = Math.max(0, Math.floor((sessionExpiry - Date.now()) / 1000));
        setCountdown(timeUntilExpiry);
      } else {
        setCountdown(showWarningBeforeMinutes * 60);
      }
      
      timer = setInterval(() => {
        setCountdown(prevCount => {
          if (prevCount <= 1) {
            clearInterval(timer);
            handleSessionExpired();
            return 0;
          }
          return prevCount - 1;
        });
      }, 1000);
    }
    
    return () => {
      if (timer) clearInterval(timer);
    };
  }, [visible, showWarningBeforeMinutes, getSessionExpiry]);

  const showWarning = () => {
    setVisible(true);
  };

  const extendSession = async () => {
    setRefreshing(true);
    try {
      // Call refresh token API
      await AuthService.refreshToken();
      setVisible(false);
      setActivity(Date.now());
      setMessage({ type: 'success', text: 'Session extended successfully' });
      setTimeout(() => setMessage(null), 3000);
    } catch (error) {
      console.error('Failed to extend session:', error);
      setMessage({ type: 'error', text: 'Failed to extend session' });
      setTimeout(() => setMessage(null), 3000);
      handleSessionExpired();
    } finally {
      setRefreshing(false);
    }
  };

  const handleSessionExpired = () => {
    setVisible(false);
    AuthService.logout();
    onSessionExpired();
  };

  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs < 10 ? '0' : ''}${secs}`;
  };

  // Calculate the countdown percentage for progress bar
  const calculateProgress = () => {
    const warningDuration = showWarningBeforeMinutes * 60; // in seconds
    return Math.min(100, Math.max(0, (countdown / warningDuration) * 100));
  };

  return (
    <>
      <Dialog
      open={visible}
        disableEscapeKeyDown
        sx={{ '& .MuiDialog-paper': { minWidth: 400 } }}
      >
        <DialogTitle>
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
            <ClockCircleIcon color="warning" />
            <span>Session Timeout Warning</span>
          </Box>
        </DialogTitle>
        
        <DialogContent>
          <Stack sx={{ paddingY: 2 }}>
            <Box sx={{ textAlign: 'center' }}>
              <ExclamationCircleIcon sx={{ fontSize: 48, color: 'warning.main', marginBottom: 2 }} />
              <Typography variant="h6">
                Your session is about to expire
              </Typography>
            </Box>
            
            <Typography variant="body1">
          Due to inactivity, your session will expire in{' '}
              <Typography component="span" sx={{ fontWeight: 600 }}>
                {formatTime(countdown)}
              </Typography>.
            </Typography>
            
            <LinearProgress 
              variant="determinate" 
              value={calculateProgress()} 
              sx={{ 
                height: 8, 
                borderRadius: 1,
                backgroundColor: 'action.hover',
                '& .MuiLinearProgress-bar': {
                  background: 'linear-gradient(90deg, #ff9800 0%, #4caf50 100%)'
                }
              }}
            />
            
            <Typography variant="body2" color="text.secondary">
          Click "Extend Session" to continue working or "Log Out Now" to end your session.
            </Typography>
          </Stack>
        </DialogContent>
        
        <DialogActions sx={{ padding: 2 }}>
          <Button 
            variant="outlined" 
            color="error" 
            onClick={handleSessionExpired}
          >
            Log Out Now
          </Button>
          <Button 
            variant="contained" 
            onClick={extendSession}
            disabled={refreshing}
            startIcon={refreshing ? <LoadingIcon /> : undefined}
          >
            Extend Session
          </Button>
        </DialogActions>
      </Dialog>

      {/* Message display */}
      {message && (
        <Alert 
          severity={message.type} 
          sx={{ 
            position: 'fixed', 
            top: 24, 
            right: 24, 
            zIndex: 9999 
          }}
          onClose={() => setMessage(null)}
        >
          {message.text}
        </Alert>
      )}
    </>
  );
}; 
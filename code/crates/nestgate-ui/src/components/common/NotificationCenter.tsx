import React, { useState, useEffect } from 'react';
import { 
  Badge, 
  IconButton, 
  Drawer, 
  List, 
  ListItem,
  ListItemText,
  ListItemSecondaryAction,
  Button, 
  Box, 
  Typography, 
  Tabs, 
  Tab, 
  Tooltip, 
  Divider,
  Snackbar,
  Alert
} from '@mui/material';
import {
  Notifications as NotificationsIcon,
  CheckCircle as CheckCircleIcon,
  Delete as DeleteIcon,
  Settings as SettingsIcon,
  Info as InfoIcon,
  Warning as WarningIcon,
  Error as ErrorIcon,
  CheckCircleOutline as SuccessIcon
} from '@mui/icons-material';
import { WebSocketMessage, WebSocketMessageType } from '../../services/websocket.service';
import useWebSocket from '../../hooks/useWebSocket';

export interface Notification {
  id: string;
  type: 'info' | 'warning' | 'error' | 'success';
  message: string;
  description?: string;
  source: string;
  timestamp: number;
  read: boolean;
  action?: {
    text: string;
    callback: () => void;
  };
}

interface TabPanelProps {
  children?: React.ReactNode;
  index: number;
  value: number;
}

function TabPanel(props: TabPanelProps) {
  const { children, value, index, ...other } = props;

  return (
    <div
      role="tabpanel"
      hidden={value !== index}
      id={`notification-tabpanel-${index}`}
      aria-labelledby={`notification-tab-${index}`}
      {...other}
    >
      {value === index && (
        <Box sx={{ pt: 2 }}>
          {children}
        </Box>
      )}
    </div>
  );
}

const NotificationCenter: React.FC = () => {
  const [notifications, setNotifications] = useState<Notification[]>([]);
  const [unreadCount, setUnreadCount] = useState<number>(0);
  const [drawerVisible, setDrawerVisible] = useState<boolean>(false);
  const [activeTab, setActiveTab] = useState<number>(0);
  const [sound, setSound] = useState<boolean>(true);
  const [snackbarOpen, setSnackbarOpen] = useState<boolean>(false);
  const [currentNotification, setCurrentNotification] = useState<Notification | null>(null);
  
  // Use our custom hook for WebSocket integration
  const { connected } = useWebSocket({
    autoConnect: true,
    subscriptions: [
      {
        type: WebSocketMessageType.NOTIFICATION,
        handler: (message: WebSocketMessage<unknown>) => {
          if (message.type === WebSocketMessageType.NOTIFICATION) {
            const newNotification = message.data as Notification;
            addNotification(newNotification);
            if (sound) {
              playNotificationSound(newNotification.type);
            }
          }
        }
      }
    ]
  });

  // Update unread count when notifications change
  useEffect(() => {
    const unread = notifications.filter(n => !n.read).length;
    setUnreadCount(unread);
  }, [notifications]);

  // Load saved settings
  useEffect(() => {
    const savedSound = localStorage.getItem('notification_sound');
    if (savedSound !== null) {
      setSound(savedSound === 'true');
    }
  }, []);
  
  // Add a new notification
  const addNotification = (newNotification: Notification) => {
    setNotifications(prev => {
      // Check if notification with same ID already exists
      const exists = prev.some(n => n.id === newNotification.id);
      if (exists) {
        return prev;
      }
      
      // Add the new notification
      return [
        {
          ...newNotification,
          timestamp: newNotification.timestamp || Date.now(),
          read: false
        },
        ...prev
      ];
    });
    
    // Show snackbar for important notifications
    if (newNotification.type === 'error' || newNotification.type === 'warning') {
      setCurrentNotification(newNotification);
      setSnackbarOpen(true);
    }
  };
  
  // Mark all notifications as read
  const markAllAsRead = () => {
    setNotifications(prev => 
      prev.map(n => ({ ...n, read: true }))
    );
  };
  
  // Mark a single notification as read
  const markAsRead = (id: string) => {
    setNotifications(prev => 
      prev.map(n => n.id === id ? { ...n, read: true } : n)
    );
  };
  
  // Clear all notifications
  const clearAll = () => {
    setNotifications([]);
  };
  
  // Clear a single notification
  const clearNotification = (id: string) => {
    setNotifications(prev => prev.filter(n => n.id !== id));
  };
  
  // Toggle sound setting
  const toggleSound = () => {
    const newSoundSetting = !sound;
    setSound(newSoundSetting);
    localStorage.setItem('notification_sound', newSoundSetting.toString());
  };
  
  // Play a notification sound
  const playNotificationSound = (type: string) => {
    if (!sound) return;
    
    let soundFile: string;
    switch (type) {
      case 'error':
        soundFile = '/sounds/error.mp3';
        break;
      case 'warning':
        soundFile = '/sounds/warning.mp3';
        break;
      case 'success':
        soundFile = '/sounds/success.mp3';
        break;
      default:
        soundFile = '/sounds/notification.mp3';
    }
    
    const audio = new Audio(soundFile);
    audio.play().catch(e => console.error('Error playing notification sound:', e));
  };
  
  // Filter notifications based on active tab
  const getFilteredNotifications = () => {
    switch (activeTab) {
      case 1: // Unread
        return notifications.filter(n => !n.read);
      case 2: // Info
        return notifications.filter(n => n.type === 'info');
      case 3: // Warning
        return notifications.filter(n => n.type === 'warning');
      case 4: // Error
        return notifications.filter(n => n.type === 'error');
      case 5: // Success
        return notifications.filter(n => n.type === 'success');
      default: // All
        return notifications;
    }
  };
  
  // Get notification icon based on type
  const getNotificationIcon = (type: string) => {
    switch (type) {
      case 'info':
        return <InfoIcon color="info" />;
      case 'warning':
        return <WarningIcon color="warning" />;
      case 'error':
        return <ErrorIcon color="error" />;
      case 'success':
        return <SuccessIcon color="success" />;
      default:
        return <InfoIcon color="info" />;
    }
  };
  
  // Handle tab change
  const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
    setActiveTab(newValue);
  };
  
  // Open drawer
  const showDrawer = () => {
    setDrawerVisible(true);
  };
  
  // Close drawer
  const closeDrawer = () => {
    setDrawerVisible(false);
  };
  
  return (
    <>
      <Badge badgeContent={unreadCount} color="error" overlap="circular" max={99}>
        <IconButton
          color={connected ? "primary" : "default"}
          onClick={showDrawer}
          size="large"
        >
          <NotificationsIcon />
        </IconButton>
      </Badge>
      
      <Drawer
        anchor="right"
        open={drawerVisible}
        onClose={closeDrawer}
        sx={{ 
          '& .MuiDrawer-paper': { 
            width: 400,
            maxWidth: '90vw'
          }
        }}
      >
        <Box sx={{ p: 2, display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <Typography variant="h6">Notifications</Typography>
          <Box>
            <Tooltip title={sound ? "Sound On" : "Sound Off"}>
              <IconButton
                color={sound ? "primary" : "default"}
                size="small"
                onClick={toggleSound}
                sx={{ mr: 1 }}
              >
                <SettingsIcon />
              </IconButton>
            </Tooltip>
            <Tooltip title="Mark All as Read">
              <span>
                <IconButton
                  size="small"
                  onClick={markAllAsRead}
                  disabled={!notifications.some(n => !n.read)}
                  sx={{ mr: 1 }}
                >
                  <CheckCircleIcon />
                </IconButton>
              </span>
            </Tooltip>
            <Tooltip title="Clear All">
              <span>
                <IconButton
                  size="small"
                  color="error"
                  onClick={clearAll}
                  disabled={notifications.length === 0}
                >
                  <DeleteIcon />
                </IconButton>
              </span>
            </Tooltip>
          </Box>
        </Box>
        
        <Divider />
        
        <Box sx={{ width: '100%', borderBottom: 1, borderColor: 'divider' }}>
          <Tabs 
            value={activeTab} 
            onChange={handleTabChange}
            variant="scrollable"
            scrollButtons="auto"
          >
            <Tab label="All" id="notification-tab-0" aria-controls="notification-tabpanel-0" />
            <Tab label="Unread" id="notification-tab-1" aria-controls="notification-tabpanel-1" />
            <Tab label="Info" id="notification-tab-2" aria-controls="notification-tabpanel-2" />
            <Tab label="Warning" id="notification-tab-3" aria-controls="notification-tabpanel-3" />
            <Tab label="Error" id="notification-tab-4" aria-controls="notification-tabpanel-4" />
            <Tab label="Success" id="notification-tab-5" aria-controls="notification-tabpanel-5" />
          </Tabs>
        </Box>
        
        <Box sx={{ flex: 1, overflow: 'auto', p: 2 }}>
          <List>
            {getFilteredNotifications().length > 0 ? (
              getFilteredNotifications().map((item) => (
                <ListItem
                  key={item.id}
                  sx={{ 
                    mb: 1,
                    backgroundColor: item.read ? 'transparent' : 'rgba(25, 118, 210, 0.08)',
                    borderRadius: 1,
                    transition: 'all 0.3s',
                    opacity: item.read ? 0.8 : 1,
                    '&:hover': {
                      backgroundColor: 'rgba(25, 118, 210, 0.05)'
                    },
                    animation: item.read ? 'none' : 'fadeIn 0.3s ease-out',
                    '@keyframes fadeIn': {
                      from: {
                        opacity: 0,
                        transform: 'translateY(-10px)'
                      },
                      to: {
                        opacity: 1,
                        transform: 'translateY(0)'
                      }
                    }
                  }}
                >
                  <Box sx={{ display: 'flex', mr: 6, width: '100%' }}>
                    <Box sx={{ mr: 1.5, mt: 0.5 }}>
                      {getNotificationIcon(item.type)}
                    </Box>
                    <Box sx={{ flex: 1 }}>
                      <Typography variant="subtitle2" sx={{ fontWeight: 'bold' }}>
                        {item.message}
                      </Typography>
                      <Typography variant="caption" color="text.secondary" display="block">
                        {new Date(item.timestamp).toLocaleString()}
                      </Typography>
                      <Typography variant="caption" color="text.secondary" display="block">
                        Source: {item.source}
                      </Typography>
                      {item.description && (
                        <Typography variant="body2" sx={{ mt: 0.5 }}>
                          {item.description}
                        </Typography>
                      )}
                      {item.action && (
                        <Button
                          variant="text"
                          size="small"
                          sx={{ mt: 0.5, p: 0 }}
                          onClick={() => {
                            item.action?.callback();
                            markAsRead(item.id);
                          }}
                        >
                          {item.action.text}
                        </Button>
                      )}
                    </Box>
                  </Box>
                  <ListItemSecondaryAction>
                    <IconButton
                      edge="end"
                      size="small"
                      onClick={() => markAsRead(item.id)}
                      disabled={item.read}
                    >
                      <CheckCircleIcon fontSize="small" />
                    </IconButton>
                    <IconButton
                      edge="end"
                      size="small"
                      onClick={() => clearNotification(item.id)}
                    >
                      <DeleteIcon fontSize="small" />
                    </IconButton>
                  </ListItemSecondaryAction>
                </ListItem>
              ))
            ) : (
              <Box 
                sx={{ 
                  display: 'flex', 
                  flexDirection: 'column', 
                  alignItems: 'center', 
                  justifyContent: 'center',
                  py: 4
                }}
              >
                <NotificationsIcon sx={{ fontSize: 48, color: 'text.disabled', mb: 2 }} />
                <Typography color="text.secondary">No notifications</Typography>
              </Box>
            )}
          </List>
        </Box>
      </Drawer>
      
      {/* Snackbar for showing new notifications */}
      <Snackbar
        open={snackbarOpen}
        autoHideDuration={6000}
        onClose={() => setSnackbarOpen(false)}
        anchorOrigin={{ vertical: 'bottom', horizontal: 'right' }}
      >
        <Alert 
          severity={currentNotification?.type as "error" | "warning" | "info" | "success"}
          onClose={() => setSnackbarOpen(false)}
        >
          <Typography variant="subtitle2">{currentNotification?.message}</Typography>
          {currentNotification?.description && (
            <Typography variant="body2">{currentNotification.description}</Typography>
          )}
        </Alert>
      </Snackbar>
    </>
  );
};

export default NotificationCenter; 
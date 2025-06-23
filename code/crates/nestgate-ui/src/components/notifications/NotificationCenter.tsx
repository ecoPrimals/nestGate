import React, { useState, useEffect, useRef, useMemo } from 'react';
import { 
  Badge, 
  Button, 
  Card, 
  Drawer, 
  List, 
  ListItem,
  ListItemText,
  ListItemAvatar,
  ListItemSecondaryAction,
  Typography, 
  Stack, 
  Tabs, 
  Tab,
  Box,
  Tooltip,
  Divider,
  Alert,
  Popover,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  Switch,
  FormControlLabel,
  IconButton,
  Avatar,
  Chip,
  CircularProgress
} from '@mui/material';
import {
  Notifications as BellIcon,
  Cancel as CloseCircleIcon,
  CheckCircle as CheckCircleIcon,
  Warning as ExclamationCircleIcon,
  Info as InfoCircleIcon,
  Sync as SyncIcon,
  Delete as DeleteIcon,
  RemoveRedEye as EyeIcon,
  AccessTime as ClockCircleIcon,
  Settings as SettingsIcon,
  NotificationsActive as NotificationIcon,
  ArrowForward as RightCircleIcon
} from '@mui/icons-material';
import { 
  WebSocketMessageType, 
  WebSocketMessage 
} from '../../services/websocket.service';
import { useLocalStorage } from '../../hooks/useLocalStorage';
import useWebSocket, { MessageSubscription } from '../../hooks/useWebSocket';
import { DataSourceBanner } from '../common';
import EmptyState from '../common/EmptyState';
import StatusChip from '../common/StatusChip';
import TabPanel from '../common/TabPanel';

interface Notification {
  id: string;
  type: 'info' | 'warning' | 'error' | 'success';
  title: string;
  message: string;
  read: boolean;
  timestamp: number;
  category?: string;
  source?: string;
  detail?: string;
  actionable?: boolean;
  action?: {
    label: string;
    url?: string;
    callback?: () => void;
  };
}

interface NotificationCenterProps {
  maxItems?: number;
  buttonSize?: 'small' | 'medium' | 'large';
  showDrawer?: boolean;
  onlyShowBadge?: boolean;
}

const NotificationCenter: React.FC<NotificationCenterProps> = ({
  maxItems = 50,
  buttonSize = 'medium',
  showDrawer = false,
  onlyShowBadge = false
}) => {
  const [notifications, setNotifications] = useState<Notification[]>([]);
  const [drawerVisible, setDrawerVisible] = useState<boolean>(showDrawer);
  const [activeTab, setActiveTab] = useState(0);
  const [loadingNotifications, setLoadingNotifications] = useState<boolean>(true);
  const [settingsVisible, setSettingsVisible] = useState<boolean>(false);
  const [selectedNotification, setSelectedNotification] = useState<Notification | null>(null);
  const [detailsVisible, setDetailsVisible] = useState<boolean>(false);
  const [popoverAnchorEl, setPopoverAnchorEl] = useState<HTMLElement | null>(null);
  
  // Settings
  const [autoReadOnOpen, setAutoReadOnOpen] = useLocalStorage('notifications_autoread', true);
  const [soundEnabled, setSoundEnabled] = useLocalStorage('notifications_sound', true);
  const [showPopup, setShowPopup] = useLocalStorage('notifications_popup', true);
  
  const notificationSound = useRef<HTMLAudioElement | null>(null);

  // Use our websocket hook
  const { 
    connected: isConnected, 
    dataSource,
    mockReason,
    isMock
  } = useWebSocket({
    autoConnect: true,
    subscriptions: [
      {
        type: WebSocketMessageType.NOTIFICATION,
        handler: (message: WebSocketMessage<Notification>) => {
          // Play notification sound if enabled
          if (soundEnabled && notificationSound.current) {
            notificationSound.current.play().catch(err => {
              console.warn('Could not play notification sound:', err);
            });
          }
          
          // Add new notification
          setNotifications(prev => {
            // Add to the beginning of the array
            const updated = [message.data, ...prev];
            // Limit to maxItems
            return updated.slice(0, maxItems);
          });
          
          // Show popup if enabled and notification is important
          if (showPopup && (message.data.type === 'error' || message.data.actionable)) {
            showNotificationPopup(message.data);
          }
          
          setLoadingNotifications(false);
        }
      } as MessageSubscription<Notification>
    ],
    onError: (error) => {
      console.error('Failed to connect to notification service:', error);
      setLoadingNotifications(false);
    }
  });

  // Initialize notification sound
  useEffect(() => {
    notificationSound.current = new Audio('/assets/sounds/notification.mp3');
    setLoadingNotifications(true);
    return () => {
      notificationSound.current = null;
    };
  }, []);

  // Get unread notification count
  const unreadCount = notifications.filter(n => !n.read).length;

  // Show notification popup
  const showNotificationPopup = (notification: Notification) => {
    // Use browser notification if supported and user permits
    if ('Notification' in window && Notification.permission === 'granted') {
      new Notification(notification.title, {
        body: notification.message,
        icon: '/assets/icons/notification-icon.png'
      });
    }
  };

  // Get notifications based on active tab
  const getFilteredNotifications = () => {
    switch (activeTab) {
      case 1:
        return notifications.filter(n => !n.read);
      case 2:
        return notifications.filter(n => n.actionable);
      case 3:
        return notifications.filter(n => n.type === 'info');
      case 4:
        return notifications.filter(n => n.type === 'warning');
      case 5:
        return notifications.filter(n => n.type === 'error');
      case 6:
        return notifications.filter(n => n.type === 'success');
      default:
        return notifications;
    }
  };

  // Mark notification as read
  const markAsRead = (id: string) => {
    setNotifications(prev =>
      prev.map(n => (n.id === id ? { ...n, read: true } : n))
    );
  };

  // Mark all notifications as read
  const markAllAsRead = () => {
    setNotifications(prev => prev.map(n => ({ ...n, read: true })));
  };

  // Remove a notification
  const removeNotification = (id: string) => {
    setNotifications(prev => prev.filter(n => n.id !== id));
  };

  // Clear all notifications
  const clearAllNotifications = () => {
    // TODO: Add confirmation dialog with MUI
        setNotifications([]);
  };

  // Show notification details
  const showNotificationDetails = (notification: Notification) => {
    setSelectedNotification(notification);
    setDetailsVisible(true);
    
    // Auto-mark as read if enabled
    if (autoReadOnOpen && !notification.read) {
      markAsRead(notification.id);
    }
  };

  // Execute notification action
  const executeAction = (notification: Notification) => {
    if (notification.action) {
      if (notification.action.callback) {
      notification.action.callback();
      } else if (notification.action.url) {
        window.open(notification.action.url, '_blank');
    }
    setDetailsVisible(false);
    }
  };

  // Get notification icon
  const getNotificationIcon = (type: string): React.ReactNode => {
    const iconColor = {
      info: 'info.main',
      warning: 'warning.main',
      error: 'error.main',
      success: 'success.main'
    }[type] || 'info.main';

    const IconComponent = {
      info: InfoCircleIcon,
      warning: ExclamationCircleIcon,
      error: CloseCircleIcon,
      success: CheckCircleIcon
    }[type] || InfoCircleIcon;

    return (
      <Avatar sx={{ bgcolor: 'transparent' }}>
        <IconComponent sx={{ color: iconColor }} />
      </Avatar>
    );
  };

  // Format date
  const formatDate = (timestamp: number): string => {
    return new Date(timestamp).toLocaleString();
  };

  // Get time description (relative time)
  const getTimeDescription = (timestamp: number): string => {
    const now = Date.now();
    const diffInMinutes = Math.floor((now - timestamp) / (1000 * 60));
    
    if (diffInMinutes < 1) return 'Just now';
    if (diffInMinutes < 60) return `${diffInMinutes}m ago`;
    
    const diffInHours = Math.floor(diffInMinutes / 60);
    if (diffInHours < 24) return `${diffInHours}h ago`;
    
    const diffInDays = Math.floor(diffInHours / 24);
    if (diffInDays < 7) return `${diffInDays}d ago`;
    
      return formatDate(timestamp);
  };

  // Settings component
  const NotificationSettings = () => (
    <Box sx={{ padding: 2 }}>
      <Typography variant="h6" gutterBottom>
        Notification Settings
      </Typography>
      <Stack sx={{ marginBottom: 2 }}>
        <FormControlLabel
          control={
            <Switch
              checked={autoReadOnOpen}
              onChange={(e) => setAutoReadOnOpen(e.target.checked)}
            />
          }
          label="Auto-mark as read when viewed"
        />
        <FormControlLabel
          control={
            <Switch
              checked={soundEnabled}
              onChange={(e) => setSoundEnabled(e.target.checked)}
            />
          }
          label="Enable notification sounds"
        />
        <FormControlLabel
          control={
            <Switch
              checked={showPopup}
              onChange={(e) => setShowPopup(e.target.checked)}
            />
          }
          label="Show popup for important alerts"
        />
      </Stack>
      <Button 
        fullWidth
        variant="contained" 
        onClick={() => setSettingsVisible(false)}
      >
        Save Settings
      </Button>
    </Box>
  );

  // Only show the badge with count if requested
  if (onlyShowBadge) {
    return (
      <Badge badgeContent={unreadCount} color="primary">
        <BellIcon />
      </Badge>
    );
  }

  // Handle tab change
  const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
    setActiveTab(newValue);
  };

  // Content component for the notification drawer
  const NotificationContent = () => (
    <Box sx={{ padding: 2 }}>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 2 }}>
        <Typography variant="h5">Notifications</Typography>
        <Stack direction="row">
          <Tooltip title="Mark all as read">
            <IconButton 
            size="small" 
            onClick={markAllAsRead}
            disabled={unreadCount === 0}
          >
              <EyeIcon />
            </IconButton>
          </Tooltip>
          <Tooltip title="Clear all">
            <IconButton 
            size="small" 
            onClick={clearAllNotifications}
            disabled={notifications.length === 0}
              color="error"
            >
              <DeleteIcon />
            </IconButton>
          </Tooltip>
          <Tooltip title="Settings">
            <IconButton
            size="small"
            onClick={() => setSettingsVisible(true)}
            >
              <SettingsIcon />
            </IconButton>
          </Tooltip>
        </Stack>
      </Box>

      <Box sx={{ borderBottom: 1, borderColor: 'divider', marginBottom: 2 }}>
      <Tabs 
          value={activeTab}
          onChange={handleTabChange}
          variant="scrollable"
          scrollButtons="auto"
        >
          <Tab label={`All (${notifications.length})`} />
          <Tab label={`Unread (${unreadCount})`} />
          <Tab label="Actionable" />
          <Tab label="Info" />
          <Tab label="Warning" />
          <Tab label="Error" />
          <Tab label="Success" />
      </Tabs>
      </Box>

      {!isConnected && (
        <Alert
          severity="warning"
          sx={{ marginBottom: 2 }}
        >
          Not connected to notification service. Notifications might not be up to date.
        </Alert>
      )}

      {/* Show mock data banner if using mock data */}
      {isMock && (
        <DataSourceBanner
          dataSource={dataSource}
          mockReason={mockReason}
          serviceName="Notifications"
        />
      )}

      {loadingNotifications ? (
        <Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'center', padding: 3 }}>
          <CircularProgress />
          <Typography variant="body2" sx={{ marginTop: 2 }}>
            Loading notifications...
          </Typography>
        </Box>
      ) : getFilteredNotifications().length === 0 ? (
        <EmptyState 
          title="No notifications" 
          icon={<BellIcon />}
        />
      ) : (
        <List>
          {getFilteredNotifications().map(notification => (
            <ListItem
              key={notification.id}
              sx={{ 
                backgroundColor: notification.read ? 'transparent' : 'action.hover',
                borderRadius: 1,
                marginBottom: 1,
                cursor: 'pointer',
                '&:hover': {
                  backgroundColor: 'action.selected',
                },
              }}
              onClick={() => showNotificationDetails(notification)}
            >
              <ListItemAvatar>
                {getNotificationIcon(notification.type)}
              </ListItemAvatar>
              <ListItemText
                primary={
                  <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, flexWrap: 'wrap' }}>
                    <Typography variant="subtitle2" sx={{ fontWeight: 600 }}>
                      {notification.title}
                    </Typography>
                    {!notification.read && (
                      <StatusChip status="info" label="New" size="small" />
                    )}
                    {notification.actionable && (
                      <StatusChip status="warning" label="Action Required" size="small" />
                    )}
                    {notification.category && (
                      <Chip label={notification.category} size="small" variant="outlined" />
                    )}
                  </Box>
                }
                secondary={
                  <Box>
                    <Typography variant="body2" color="text.secondary">
                      {notification.message}
                    </Typography>
                    <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, marginTop: 0.5 }}>
                      <ClockCircleIcon fontSize="small" />
                      <Typography variant="caption" color="text.secondary">
                          {getTimeDescription(notification.timestamp)}
                      </Typography>
                        {notification.source && (
                        <>
                          <Divider orientation="vertical" flexItem />
                          <Typography variant="caption" color="text.secondary">
                            Source: {notification.source}
                          </Typography>
                        </>
                      )}
                    </Box>
                  </Box>
                }
              />
              <ListItemSecondaryAction>
                <Stack direction="row" spacing={0.5}>
                  {notification.actionable && (
                    <Tooltip title="Take action">
                      <IconButton
                        size="small"
                        color="primary"
                        onClick={(e) => {
                          e.stopPropagation();
                          showNotificationDetails(notification);
                        }}
                      >
                        <RightCircleIcon />
                      </IconButton>
                    </Tooltip>
                  )}
                  <Tooltip title="Mark as read">
                    <IconButton 
                      size="small" 
                      onClick={(e) => {
                        e.stopPropagation();
                        markAsRead(notification.id);
                      }} 
                      disabled={notification.read}
                    >
                      <EyeIcon />
                    </IconButton>
                  </Tooltip>
                  <Tooltip title="Delete">
                    <IconButton 
                      size="small" 
                      color="error"
                      onClick={(e) => {
                        e.stopPropagation();
                        removeNotification(notification.id);
                      }} 
                    >
                      <DeleteIcon />
                    </IconButton>
                  </Tooltip>
                </Stack>
              </ListItemSecondaryAction>
            </ListItem>
          ))}
        </List>
      )}
    </Box>
  );

  // Handle popover
  const handlePopoverClick = (event: React.MouseEvent<HTMLElement>) => {
    setPopoverAnchorEl(event.currentTarget);
  };

  const handlePopoverClose = () => {
    setPopoverAnchorEl(null);
  };

  const popoverOpen = Boolean(popoverAnchorEl);

  // Drawer version - full notification center in a drawer
  if (!onlyShowBadge) {
    return (
      <>
        <Badge badgeContent={unreadCount} color="primary">
          <IconButton 
            size={buttonSize}
            onClick={() => setDrawerVisible(true)}
            aria-label="Notifications"
          >
            <BellIcon />
          </IconButton>
        </Badge>

        <Drawer
          anchor="right"
          open={drawerVisible}
          onClose={() => setDrawerVisible(false)}
          PaperProps={{
            sx: { width: 380 }
          }}
        >
          <NotificationContent />
        </Drawer>

        <Dialog
          open={settingsVisible}
          onClose={() => setSettingsVisible(false)}
          maxWidth="sm"
          fullWidth
        >
          <DialogTitle>Notification Settings</DialogTitle>
          <DialogContent>
          <NotificationSettings />
          </DialogContent>
        </Dialog>

        <Dialog
          open={detailsVisible}
          onClose={() => setDetailsVisible(false)}
          maxWidth="sm"
          fullWidth
        >
          <DialogTitle>
            {selectedNotification && (
              <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                {getNotificationIcon(selectedNotification.type)}
                <span>{selectedNotification?.title}</span>
              </Box>
            )}
          </DialogTitle>
          <DialogContent>
            {selectedNotification && (
              <Box>
                <Typography variant="body1" paragraph>
                  {selectedNotification.message}
                </Typography>
                {selectedNotification.detail && (
                  <Card variant="outlined" sx={{ padding: 1, marginY: 1 }}>
                    <Typography variant="body2">
                      {selectedNotification.detail}
                    </Typography>
                  </Card>
                )}
                <Box sx={{ marginTop: 2 }}>
                  <Typography variant="body2" color="text.secondary">
                    Source: {selectedNotification.source || 'System'}
                  </Typography>
                  <Typography variant="body2" color="text.secondary">
                    Time: {formatDate(selectedNotification.timestamp)}
                  </Typography>
                  {selectedNotification.category && (
                    <Typography variant="body2" color="text.secondary">
                      Category: {selectedNotification.category}
                    </Typography>
                  )}
                </Box>
              </Box>
            )}
          </DialogContent>
          <DialogActions>
            <Button onClick={() => setDetailsVisible(false)}>
              Close
            </Button>
            {selectedNotification?.actionable && selectedNotification?.action && (
              <Button 
                variant="contained"
                onClick={() => executeAction(selectedNotification)}
              >
                {selectedNotification.action.label || 'Take Action'}
              </Button>
            )}
          </DialogActions>
        </Dialog>
      </>
    );
  }

  // Popover version - show a simpler notification list in a popover
  return (
    <>
      <Badge badgeContent={unreadCount} color="primary">
        <IconButton 
          size={buttonSize}
          onClick={handlePopoverClick}
          aria-label="Notifications"
        >
          <BellIcon />
        </IconButton>
      </Badge>

      <Popover
        open={popoverOpen}
        anchorEl={popoverAnchorEl}
        onClose={handlePopoverClose}
        anchorOrigin={{
          vertical: 'bottom',
          horizontal: 'right',
        }}
        transformOrigin={{
          vertical: 'top',
          horizontal: 'right',
        }}
        PaperProps={{
          sx: { width: 300, maxHeight: 400, overflow: 'auto' }
        }}
      >
        <NotificationContent />
      </Popover>

      <Dialog
        open={settingsVisible}
        onClose={() => setSettingsVisible(false)}
        maxWidth="sm"
        fullWidth
      >
        <DialogTitle>Notification Settings</DialogTitle>
        <DialogContent>
        <NotificationSettings />
        </DialogContent>
      </Dialog>

      <Dialog
        open={detailsVisible}
        onClose={() => setDetailsVisible(false)}
        maxWidth="sm"
        fullWidth
      >
        <DialogTitle>
          {selectedNotification && (
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
              {getNotificationIcon(selectedNotification.type)}
              <span>{selectedNotification?.title}</span>
            </Box>
          )}
        </DialogTitle>
        <DialogContent>
          {selectedNotification && (
            <Box>
              <Typography variant="body1" paragraph>
                {selectedNotification.message}
              </Typography>
              {selectedNotification.detail && (
                <Card variant="outlined" sx={{ padding: 1, marginY: 1 }}>
                  <Typography variant="body2">
                    {selectedNotification.detail}
                  </Typography>
                </Card>
              )}
              <Box sx={{ marginTop: 2 }}>
                <Typography variant="body2" color="text.secondary">
                  Source: {selectedNotification.source || 'System'}
                </Typography>
                <Typography variant="body2" color="text.secondary">
                  Time: {formatDate(selectedNotification.timestamp)}
                </Typography>
                {selectedNotification.category && (
                  <Typography variant="body2" color="text.secondary">
                    Category: {selectedNotification.category}
                  </Typography>
                )}
              </Box>
            </Box>
          )}
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setDetailsVisible(false)}>
            Close
          </Button>
          {selectedNotification?.actionable && selectedNotification?.action && (
            <Button 
              variant="contained"
              onClick={() => executeAction(selectedNotification)}
            >
              {selectedNotification.action.label || 'Take Action'}
            </Button>
          )}
        </DialogActions>
      </Dialog>
    </>
  );
};

export default NotificationCenter; 
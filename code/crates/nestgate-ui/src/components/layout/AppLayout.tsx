import React, { useState, useContext } from 'react';
import { Outlet, useNavigate, useLocation } from 'react-router-dom';
import {
  Box,
  AppBar,
  Toolbar,
  IconButton,
  Avatar,
  Menu,
  MenuItem,
  ListItemIcon,
  ListItemText,
  Divider,
  Typography,
  useTheme,
  alpha,
} from '@mui/material';
import {
  Dashboard as DashboardIcon,
  Storage as DatabaseIcon,
  CameraAlt as CameraIcon,
  Settings as SettingsIcon,
  Person as UserIcon,
  Logout as LogoutIcon,
  Menu as MenuIcon,
  MenuOpen as MenuOpenIcon,
  Computer as HddIcon,
  BarChart as BarChartIcon,
  Group as TeamIcon,
  Public as GlobalIcon,
  Save as SaveIcon,
  ShowChart as LineChartIcon,
} from '@mui/icons-material';
import NotificationCenter from '../notifications/NotificationCenter';
import { AuthContext } from '../../contexts/AuthContext';
import Sidebar from './Sidebar';

/**
 * Main application layout with sidebar, header, and content area
 */
const AppLayout: React.FC = () => {
  const [collapsed, setCollapsed] = useState(false);
  const [userMenuAnchorEl, setUserMenuAnchorEl] = useState<null | HTMLElement>(null);
  const navigate = useNavigate();
  const location = useLocation();
  const theme = useTheme();
  const { logout, user } = useContext(AuthContext);

  // Handle user menu clicks
  const handleUserMenuClick = (key: string) => {
    switch (key) {
      case 'profile':
        navigate('/profile');
        break;
      case 'settings':
        navigate('/settings');
        break;
      case 'logout':
        logout();
        break;
      default:
        break;
    }
    setUserMenuAnchorEl(null);
  };

  const handleUserMenuOpen = (event: React.MouseEvent<HTMLElement>) => {
    setUserMenuAnchorEl(event.currentTarget);
  };

  const handleUserMenuClose = () => {
    setUserMenuAnchorEl(null);
  };

  // Get the current path for highlighting the active menu item
  const getCurrentPath = () => {
    const path = location.pathname;
    if (path.startsWith('/dashboard')) return 'dashboard';
    if (path.startsWith('/storage')) return 'storage';
    if (path.startsWith('/snapshots')) return 'snapshots';
    if (path.startsWith('/settings')) return 'settings';
    if (path.startsWith('/disk-health')) return 'disk-health';
    if (path.startsWith('/performance')) return 'performance';
    if (path.startsWith('/users')) return 'users';
    if (path.startsWith('/network')) return 'network';
    if (path.startsWith('/backup')) return 'backup';
    if (path.startsWith('/profile')) return 'profile';
    if (path.startsWith('/monitoring')) return 'monitoring';
    return 'dashboard'; // Default to dashboard
  };

  const menuItems = [
    {
      key: 'dashboard',
      icon: <DashboardIcon />,
      label: 'Dashboard',
      onClick: () => navigate('/dashboard'),
    },
    {
      key: 'monitoring',
      icon: <LineChartIcon />,
      label: 'Monitoring',
      onClick: () => navigate('/monitoring'),
    },
    {
      key: 'storage',
      icon: <DatabaseIcon />,
      label: 'Storage',
      onClick: () => navigate('/storage'),
    },
    {
      key: 'snapshots',
      icon: <CameraIcon />,
      label: 'Snapshots',
      onClick: () => navigate('/snapshots'),
    },
    {
      key: 'disk-health',
      icon: <HddIcon />,
      label: 'Disk Health',
      onClick: () => navigate('/disk-health'),
    },
    {
      key: 'network',
      icon: <GlobalIcon />,
      label: 'Network',
      onClick: () => navigate('/network'),
    },
    {
      key: 'performance',
      icon: <BarChartIcon />,
      label: 'Performance',
      onClick: () => navigate('/performance'),
    },
    {
      key: 'backup',
      icon: <SaveIcon />,
      label: 'Backup',
      onClick: () => navigate('/backup'),
    },
    {
      key: 'users',
      icon: <TeamIcon />,
      label: 'Users',
      onClick: () => navigate('/users'),
    },
    {
      key: 'settings',
      icon: <SettingsIcon />,
      label: 'Settings',
      onClick: () => navigate('/settings'),
    },
  ];

  return (
    <Box sx={{ display: 'flex', minHeight: '100vh' }}>
      <Sidebar collapsed={collapsed} />
      <Box sx={{ flex: 1, display: 'flex', flexDirection: 'column' }}>
        <AppBar 
          position="fixed" 
          sx={{ 
            zIndex: theme.zIndex.drawer + 1,
            backgroundColor: theme.palette.background.paper,
            color: theme.palette.text.primary,
            boxShadow: `0 1px 2px 0 ${alpha(theme.palette.common.black, 0.05)}`,
          }}
        >
          <Toolbar sx={{ justifyContent: 'space-between' }}>
            <IconButton
              edge="start"
              color="inherit"
              aria-label="toggle sidebar"
            onClick={() => setCollapsed(!collapsed)}
              sx={{ marginRight: 2 }}
            >
              {collapsed ? <MenuIcon /> : <MenuOpenIcon />}
            </IconButton>
            
            <Box sx={{ display: 'flex', alignItems: 'center' }}>
            {/* Notification Center Component */}
            <NotificationCenter maxItems={50} />
            
              <Box sx={{ display: 'flex', alignItems: 'center', marginLeft: 2 }}>
                <IconButton
                  onClick={handleUserMenuOpen}
                  aria-controls="user-menu"
                  aria-haspopup="true"
                  sx={{ padding: 0 }}
                >
                  <Avatar sx={{ width: 32, height: 32 }}>
                    <UserIcon />
                  </Avatar>
                </IconButton>
                {!collapsed && (
                  <Typography variant="body2" sx={{ marginLeft: 1 }}>
                    {user?.username || 'User'}
                  </Typography>
                )}
              </Box>
              
              <Menu
                id="user-menu"
                anchorEl={userMenuAnchorEl}
                open={Boolean(userMenuAnchorEl)}
                onClose={handleUserMenuClose}
                anchorOrigin={{
                  vertical: 'bottom',
                  horizontal: 'right',
                }}
                transformOrigin={{
                  vertical: 'top',
                  horizontal: 'right',
                }}
              >
                <MenuItem onClick={() => handleUserMenuClick('profile')}>
                  <ListItemIcon>
                    <UserIcon fontSize="small" />
                  </ListItemIcon>
                  <ListItemText>Profile</ListItemText>
                </MenuItem>
                <MenuItem onClick={() => handleUserMenuClick('settings')}>
                  <ListItemIcon>
                    <SettingsIcon fontSize="small" />
                  </ListItemIcon>
                  <ListItemText>Settings</ListItemText>
                </MenuItem>
                <Divider />
                <MenuItem onClick={() => handleUserMenuClick('logout')}>
                  <ListItemIcon>
                    <LogoutIcon fontSize="small" />
                  </ListItemIcon>
                  <ListItemText>Logout</ListItemText>
                </MenuItem>
              </Menu>
            </Box>
          </Toolbar>
        </AppBar>
        
        <Box
          component="main"
          sx={{
            flexGrow: 1,
            padding: 3,
            marginTop: '64px', // AppBar height
            backgroundColor: alpha(theme.palette.background.default, 0.5),
          }}
        >
          <Outlet />
        </Box>
      </Box>
    </Box>
  );
};

export default AppLayout; 
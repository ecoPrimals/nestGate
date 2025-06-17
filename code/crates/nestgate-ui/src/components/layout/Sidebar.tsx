import React, { useState } from 'react';
import {
  Drawer,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Collapse,
  Box,
  Typography,
  useTheme,
  alpha,
} from '@mui/material';
import { useNavigate, useLocation } from 'react-router-dom';
import { 
  Dashboard as DashboardIcon, 
  Storage as DatabaseIcon, 
  CameraAlt as CameraIcon, 
  ShowChart as LineChartIcon, 
  Public as GlobalIcon, 
  Settings as SettingsIcon, 
  Group as TeamIcon,
  Science as ExperimentIcon,
  ExpandLess,
  ExpandMore,
} from '@mui/icons-material';

const DRAWER_WIDTH = 220;
const DRAWER_WIDTH_COLLAPSED = 64;

type MenuItem = {
  key: string;
  icon?: React.ReactNode;
  label: React.ReactNode;
  children?: MenuItem[];
};

interface SidebarProps {
  collapsed: boolean;
}

const Sidebar: React.FC<SidebarProps> = ({ collapsed }) => {
  const navigate = useNavigate();
  const location = useLocation();
  const theme = useTheme();
  const [expandedItems, setExpandedItems] = useState<string[]>(['examples']);
  
  // Define menu items
  const items: MenuItem[] = [
    {
      key: '/dashboard',
      icon: <DashboardIcon />,
      label: 'Dashboard',
    },
    {
      key: '/storage',
      icon: <DatabaseIcon />,
      label: 'Storage',
    },
    {
      key: '/snapshots',
      icon: <CameraIcon />,
      label: 'Snapshots',
    },
    {
      key: '/performance',
      icon: <LineChartIcon />,
      label: 'Performance',
    },
    {
      key: '/network',
      icon: <GlobalIcon />,
      label: 'Network',
    },
    {
      key: '/users',
      icon: <TeamIcon />,
      label: 'Users',
    },
    {
      key: 'examples',
      icon: <ExperimentIcon />,
      label: 'Examples',
      children: [
        {
          key: '/examples/live-data',
          label: 'Live Data',
        },
      ],
    },
  ];
  
  // Handle menu item clicks
  const handleMenuClick = (key: string, hasChildren?: boolean) => {
    if (hasChildren) {
      // Toggle expansion for items with children
      setExpandedItems(prev => 
        prev.includes(key) 
          ? prev.filter(item => item !== key)
          : [...prev, key]
      );
    } else {
      navigate(key);
    }
  };
  
  // Check if a menu item is selected
  const isSelected = (key: string) => {
    if (key === 'examples') {
      return location.pathname.startsWith('/examples/');
    }
    return location.pathname === key;
  };

  // Check if a parent item should be expanded
  const shouldExpand = (key: string) => {
    if (collapsed) return false;
    if (key === 'examples' && location.pathname.startsWith('/examples/')) {
      return true;
    }
    return expandedItems.includes(key);
  };

  const renderMenuItem = (item: MenuItem, depth = 0) => {
    const hasChildren = item.children && item.children.length > 0;
    const selected = isSelected(item.key);
    const expanded = shouldExpand(item.key);

    return (
      <React.Fragment key={item.key}>
        <ListItem disablePadding>
          <ListItemButton
            selected={selected}
            onClick={() => handleMenuClick(item.key, hasChildren)}
            sx={{
              minHeight: 48,
              paddingLeft: depth === 0 ? 2 : 4,
              paddingRight: 2,
              '&.Mui-selected': {
                backgroundColor: alpha(theme.palette.primary.main, 0.1),
                '&:hover': {
                  backgroundColor: alpha(theme.palette.primary.main, 0.15),
                },
              },
            }}
          >
            {item.icon && (
              <ListItemIcon
                sx={{
                  minWidth: collapsed ? 0 : 40,
                  justifyContent: 'center',
                  color: selected ? theme.palette.primary.main : 'inherit',
                }}
              >
                {item.icon}
              </ListItemIcon>
            )}
            {!collapsed && (
              <>
                <ListItemText
                  primary={item.label}
                  sx={{
                    '& .MuiListItemText-primary': {
                      fontSize: '0.875rem',
                      fontWeight: selected ? 600 : 400,
                      color: selected ? theme.palette.primary.main : 'inherit',
                    },
                  }}
                />
                {hasChildren && (expanded ? <ExpandLess /> : <ExpandMore />)}
              </>
            )}
          </ListItemButton>
        </ListItem>
        
        {hasChildren && !collapsed && (
          <Collapse in={expanded} timeout="auto" unmountOnExit>
            <List component="div" disablePadding>
              {item.children?.map(child => renderMenuItem(child, depth + 1))}
            </List>
          </Collapse>
        )}
      </React.Fragment>
    );
  };
  
  return (
    <Drawer
      variant="permanent"
      sx={{
        width: collapsed ? DRAWER_WIDTH_COLLAPSED : DRAWER_WIDTH,
        flexShrink: 0,
        '& .MuiDrawer-paper': {
          width: collapsed ? DRAWER_WIDTH_COLLAPSED : DRAWER_WIDTH,
          boxSizing: 'border-box',
          borderRight: `1px solid ${theme.palette.divider}`,
          backgroundColor: theme.palette.background.paper,
          transition: theme.transitions.create('width', {
            easing: theme.transitions.easing.sharp,
            duration: theme.transitions.duration.standard,
          }),
          overflowX: 'hidden',
        },
      }}
      data-testid="sidebar"
    >
      {/* Logo */}
      <Box 
        sx={{ 
          height: 64, 
          display: 'flex', 
          alignItems: 'center', 
          justifyContent: 'center',
          borderBottom: `1px solid ${theme.palette.divider}`,
        }}
      >
        <Typography 
          variant="h6" 
          sx={{ 
            fontWeight: 600,
            color: theme.palette.primary.main,
            transition: theme.transitions.create('opacity', {
              easing: theme.transitions.easing.sharp,
              duration: theme.transitions.duration.standard,
            }),
          }}
        >
          {collapsed ? 'NG' : 'NestGate'}
        </Typography>
      </Box>
      
      {/* Menu Items */}
      <List data-testid="sidebar-menu" sx={{ paddingTop: 1 }}>
        {items.map(item => renderMenuItem(item))}
      </List>
    </Drawer>
  );
};

export default Sidebar; 
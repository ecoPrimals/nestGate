import React from 'react';
import { Card, CardHeader, CardContent, Typography, List, ListItem, ListItemAvatar, ListItemText, Avatar, Box } from '@mui/material';
import { 
  History as HistoryIcon,
  CheckCircle as CheckCircleIcon,
  Warning as WarningIcon,
  Cancel as CloseCircleIcon,
  Sync as SyncIcon,
  Build as ToolIcon
} from '@mui/icons-material';
import StatusChip from '../common/StatusChip';

interface ActivityItem {
  id: string;
  timestamp: number;
  type: 'snapshot' | 'scrub' | 'resilver' | 'warning' | 'error' | 'info';
  message: string;
  details: string;
  status: 'completed' | 'in_progress' | 'failed' | 'warning';
}

const RecentActivityCard: React.FC = () => {
  // Mock recent activities
  const recentActivities: ActivityItem[] = [
    {
      id: '1',
      timestamp: Date.now() - 1000 * 60 * 5, // 5 minutes ago
      type: 'snapshot',
      message: 'Snapshot created',
      details: 'pool/dataset@auto-2024-11-30-1200',
      status: 'completed'
    },
    {
      id: '2',
      timestamp: Date.now() - 1000 * 60 * 30, // 30 minutes ago
      type: 'scrub',
      message: 'Scrub started',
      details: 'mainpool',
      status: 'in_progress'
    },
    {
      id: '3',
      timestamp: Date.now() - 1000 * 60 * 60 * 2, // 2 hours ago
      type: 'warning',
      message: 'Disk temperature warning',
      details: 'ada3 (WDC WD1002FAEX)',
      status: 'warning'
    },
    {
      id: '4',
      timestamp: Date.now() - 1000 * 60 * 60 * 12, // 12 hours ago
      type: 'resilver',
      message: 'Resilver completed',
      details: 'backup_pool',
      status: 'completed'
    },
    {
      id: '5',
      timestamp: Date.now() - 1000 * 60 * 60 * 24, // 1 day ago
      type: 'error',
      message: 'Dataset mount failed',
      details: 'pool/dataset2',
      status: 'failed'
    }
  ];
  
  // Format timestamp to human readable
  const formatTimestamp = (timestamp: number): string => {
    const now = Date.now();
    const diffInMinutes = Math.floor((now - timestamp) / (1000 * 60));
    
    if (diffInMinutes < 1) return 'Just now';
    if (diffInMinutes < 60) return `${diffInMinutes}m ago`;
    
    const diffInHours = Math.floor(diffInMinutes / 60);
    if (diffInHours < 24) return `${diffInHours}h ago`;
    
    const diffInDays = Math.floor(diffInHours / 24);
    return `${diffInDays}d ago`;
  };
  
  // Get icon based on activity type
  const getActivityIcon = (type: string): React.ReactNode => {
    switch (type) {
      case 'snapshot':
        return <HistoryIcon sx={{ color: 'primary.main' }} />;
      case 'scrub':
        return <ToolIcon sx={{ color: 'primary.main' }} />;
      case 'resilver':
        return <SyncIcon sx={{ color: 'primary.main' }} />;
      case 'warning':
        return <WarningIcon sx={{ color: 'warning.main' }} />;
      case 'error':
        return <CloseCircleIcon sx={{ color: 'error.main' }} />;
      case 'info':
        return <CheckCircleIcon sx={{ color: 'success.main' }} />;
      default:
        return <HistoryIcon sx={{ color: 'primary.main' }} />;
    }
  };
  
  // Get status for StatusChip
  const getStatusChipStatus = (status: string): string => {
    switch (status) {
      case 'completed':
        return 'success';
      case 'in_progress':
        return 'info';
      case 'failed':
        return 'error';
      case 'warning':
        return 'warning';
      default:
        return 'default';
    }
  };

  const getStatusLabel = (status: string): string => {
    switch (status) {
      case 'completed':
        return 'Completed';
      case 'in_progress':
        return 'In Progress';
      case 'failed':
        return 'Failed';
      case 'warning':
        return 'Warning';
      default:
        return status;
    }
  };
  
  return (
    <Card>
      <CardHeader 
        title={
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
            <HistoryIcon />
            Recent Activity
          </Box>
        }
      />
      <CardContent sx={{ paddingTop: 0 }}>
        <List disablePadding>
          {recentActivities.map((item, index) => (
            <ListItem 
              key={item.id}
              divider={index < recentActivities.length - 1}
              sx={{ paddingLeft: 0, paddingRight: 0 }}
            >
              <ListItemAvatar>
                <Avatar sx={{ bgcolor: 'transparent' }}>
                  {getActivityIcon(item.type)}
                </Avatar>
              </ListItemAvatar>
              <ListItemText
                primary={
                  <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, flexWrap: 'wrap' }}>
                    <Typography variant="body2">
                      {item.message}
                    </Typography>
                    <Typography variant="caption" color="text.secondary">
                      {formatTimestamp(item.timestamp)}
                    </Typography>
                  </Box>
                }
                secondary={
                  <Typography variant="body2" color="text.secondary">
                    {item.details}
                  </Typography>
                }
              />
              <Box sx={{ marginLeft: 1 }}>
                <StatusChip 
                  status={getStatusChipStatus(item.status)} 
                  label={getStatusLabel(item.status)}
                  size="small"
                />
              </Box>
            </ListItem>
          ))}
        </List>
      </CardContent>
    </Card>
  );
};

export default RecentActivityCard; 
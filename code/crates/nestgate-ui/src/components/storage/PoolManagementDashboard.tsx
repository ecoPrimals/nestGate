import React, { useState, useEffect } from 'react';
import {
  Box,
  Typography,
  Card,
  CardContent,
  CardHeader,
  Button,
  Grid,
  Chip,
  LinearProgress,
  Alert,
  IconButton,
  Menu,
  MenuItem,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  FormControl,
  InputLabel,
  Select,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
  Tooltip,
  CircularProgress,
  Fab,
} from '@mui/material';
import {
  Add as AddIcon,
  Refresh as RefreshIcon,
  MoreVert as MoreVertIcon,
  Storage as StorageIcon,
  Warning as WarningIcon,
  CheckCircle as CheckCircleIcon,
  Error as ErrorIcon,
  PlayArrow as PlayIcon,
  Stop as StopIcon,
  ImportExport as ImportIcon,
  Delete as DeleteIcon,
  Settings as SettingsIcon,
  Timeline as TimelineIcon,
} from '@mui/icons-material';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import PoolCreationWizard from './PoolCreationWizard';
import ZFSPoolMonitor from '../monitoring/ZFSPoolMonitor';
import { ZfsPoolService, ZfsPool } from '../../services/zfs-pool.service';
import { formatBytes, formatPercent } from '../../utils/format.utils';

interface PoolAction {
  id: string;
  label: string;
  icon: React.ReactNode;
  action: (pool: ZfsPool) => void;
  color?: 'primary' | 'secondary' | 'error' | 'warning';
  disabled?: (pool: ZfsPool) => boolean;
}

interface ScrubStatus {
  poolName: string;
  isRunning: boolean;
  progress: number;
  startTime?: string;
  endTime?: string;
  errors: number;
}

interface PoolManagementDashboardProps {
  onPoolSelect?: (poolName: string) => void;
}

const PoolManagementDashboard: React.FC<PoolManagementDashboardProps> = ({ onPoolSelect }) => {
  // State
  const [createWizardOpen, setCreateWizardOpen] = useState(false);
  const [selectedPool, setSelectedPool] = useState<ZfsPool | null>(null);
  const [actionMenuAnchor, setActionMenuAnchor] = useState<null | HTMLElement>(null);
  const [scrubStatuses, setScrubStatuses] = useState<Map<string, ScrubStatus>>(new Map());
  const [confirmDialog, setConfirmDialog] = useState<{
    open: boolean;
    title: string;
    message: string;
    action: () => void;
  }>({ open: false, title: '', message: '', action: () => {} });

  const queryClient = useQueryClient();

  // Queries
  const {
    data: pools = [],
    isLoading,
    error,
    refetch
  } = useQuery({
    queryKey: ['zfs-pools'],
    queryFn: () => ZfsPoolService.getPools(),
    refetchInterval: 30000, // Refresh every 30 seconds
  });

  // Mutations
  const destroyPoolMutation = useMutation({
    mutationFn: (poolName: string) => ZfsPoolService.destroyPool(poolName),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['zfs-pools'] });
    },
  });

  const scrubPoolMutation = useMutation({
    mutationFn: (poolName: string) => ZfsPoolService.scrubPool(poolName),
    onSuccess: (_, poolName) => {
      // Update scrub status
      setScrubStatuses(prev => new Map(prev.set(poolName, {
        poolName,
        isRunning: true,
        progress: 0,
        startTime: new Date().toISOString(),
        errors: 0
      })));
    },
  });

  const importPoolMutation = useMutation({
    mutationFn: (poolName: string) => ZfsPoolService.importPool(poolName),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['zfs-pools'] });
    },
  });

  // Pool actions configuration
  const poolActions: PoolAction[] = [
    {
      id: 'scrub',
      label: 'Start Scrub',
      icon: <PlayIcon />,
      action: (pool) => handleScrubPool(pool.name),
      disabled: (pool) => scrubStatuses.get(pool.name)?.isRunning || false,
    },
    {
      id: 'import',
      label: 'Import Pool',
      icon: <ImportIcon />,
      action: (pool) => handleImportPool(pool.name),
      disabled: (pool) => pool.status === 'ONLINE',
    },
    {
      id: 'properties',
      label: 'Properties',
      icon: <SettingsIcon />,
      action: (pool) => handlePoolProperties(pool),
    },
    {
      id: 'destroy',
      label: 'Destroy Pool',
      icon: <DeleteIcon />,
      color: 'error' as const,
      action: (pool) => handleDestroyPool(pool),
    },
  ];

  // Event handlers
  const handleScrubPool = (poolName: string) => {
    scrubPoolMutation.mutate(poolName);
    setActionMenuAnchor(null);
  };

  const handleImportPool = (poolName: string) => {
    importPoolMutation.mutate(poolName);
    setActionMenuAnchor(null);
  };

  const handlePoolProperties = (pool: ZfsPool) => {
    // TODO: Open pool properties dialog
    console.log('Opening properties for pool:', pool.name);
    setActionMenuAnchor(null);
  };

  const handleDestroyPool = (pool: ZfsPool) => {
    setConfirmDialog({
      open: true,
      title: 'Destroy Pool',
      message: `Are you sure you want to destroy pool "${pool.name}"? This action cannot be undone and all data will be lost.`,
      action: () => {
        destroyPoolMutation.mutate(pool.name);
        setConfirmDialog(prev => ({ ...prev, open: false }));
      }
    });
    setActionMenuAnchor(null);
  };

  const handleActionMenuOpen = (event: React.MouseEvent<HTMLElement>, pool: ZfsPool) => {
    setActionMenuAnchor(event.currentTarget);
    setSelectedPool(pool);
  };

  const handleActionMenuClose = () => {
    setActionMenuAnchor(null);
    setSelectedPool(null);
  };

  const getPoolStatusColor = (status: string) => {
    switch (status.toUpperCase()) {
      case 'ONLINE':
        return 'success';
      case 'DEGRADED':
        return 'warning';
      case 'FAULTED':
      case 'OFFLINE':
        return 'error';
      default:
        return 'default';
    }
  };

  const getPoolStatusIcon = (status: string) => {
    switch (status.toUpperCase()) {
      case 'ONLINE':
        return <CheckCircleIcon color="success" />;
      case 'DEGRADED':
        return <WarningIcon color="warning" />;
      case 'FAULTED':
      case 'OFFLINE':
        return <ErrorIcon color="error" />;
      default:
        return <StorageIcon />;
    }
  };

  if (error) {
    return (
      <Alert severity="error" sx={{ m: 2 }}>
        Failed to load ZFS pools: {error instanceof Error ? error.message : 'Unknown error'}
      </Alert>
    );
  }

  return (
    <Box sx={{ p: 3 }}>
      {/* Header */}
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Typography variant="h4" component="h1">
          ZFS Pool Management
        </Typography>
        <Box display="flex" gap={1}>
          <Button
            variant="outlined"
            startIcon={<RefreshIcon />}
            onClick={() => refetch()}
            disabled={isLoading}
          >
            Refresh
          </Button>
          <Button
            variant="contained"
            startIcon={<AddIcon />}
            onClick={() => setCreateWizardOpen(true)}
          >
            Create Pool
          </Button>
        </Box>
      </Box>

      {/* Loading State */}
      {isLoading && (
        <Box display="flex" justifyContent="center" py={4}>
          <CircularProgress />
        </Box>
      )}

      {/* Pools Overview Cards */}
      {!isLoading && pools.length > 0 && (
        <Grid container spacing={3} sx={{ mb: 4 }}>
          {pools.map((pool) => {
            const usagePercent = (pool.allocated / pool.size) * 100;
            const scrubStatus = scrubStatuses.get(pool.name);
            
            return (
              <Grid item xs={12} md={6} lg={4} key={pool.name}>
                <Card 
                  sx={{ cursor: 'pointer' }}
                  onClick={() => onPoolSelect?.(pool.name)}
                >
                  <CardHeader
                    avatar={getPoolStatusIcon(pool.status)}
                    title={pool.name}
                    subheader={
                      <Chip
                        label={pool.status}
                        color={getPoolStatusColor(pool.status)}
                        size="small"
                      />
                    }
                    action={
                      <IconButton
                        onClick={(e) => handleActionMenuOpen(e, pool)}
                        size="small"
                      >
                        <MoreVertIcon />
                      </IconButton>
                    }
                  />
                  <CardContent>
                    <Box mb={2}>
                      <Box display="flex" justifyContent="space-between" mb={1}>
                        <Typography variant="body2" color="text.secondary">
                          Storage Usage
                        </Typography>
                        <Typography variant="body2" color="text.secondary">
                          {formatBytes(pool.allocated)} / {formatBytes(pool.size)}
                        </Typography>
                      </Box>
                      <LinearProgress
                        variant="determinate"
                        value={usagePercent}
                        color={usagePercent > 80 ? 'error' : usagePercent > 60 ? 'warning' : 'primary'}
                        sx={{ height: 8, borderRadius: 1 }}
                      />
                      <Typography variant="caption" color="text.secondary">
                        {formatPercent(usagePercent)} used
                      </Typography>
                    </Box>

                    {/* Scrub Status */}
                    {scrubStatus?.isRunning && (
                      <Box mb={2}>
                        <Typography variant="body2" color="text.secondary" gutterBottom>
                          Scrub in Progress
                        </Typography>
                        <LinearProgress
                          variant="determinate"
                          value={scrubStatus.progress}
                          sx={{ height: 6, borderRadius: 1 }}
                        />
                        <Typography variant="caption" color="text.secondary">
                          {scrubStatus.progress.toFixed(1)}% complete
                        </Typography>
                      </Box>
                    )}

                    {/* Pool Health */}
                    <Box display="flex" justifyContent="space-between" alignItems="center">
                      <Typography variant="body2">
                        Health: <strong>{pool.health || 'Unknown'}</strong>
                      </Typography>
                      <Typography variant="body2" color="text.secondary">
                        {pool.fragmentation && `${pool.fragmentation}% frag`}
                      </Typography>
                    </Box>
                  </CardContent>
                </Card>
              </Grid>
            );
          })}
        </Grid>
      )}

      {/* Detailed Pool Table */}
      {!isLoading && pools.length > 0 && (
        <Card>
          <CardHeader
            title="Pool Details"
            subheader="Detailed information about all ZFS pools"
          />
          <TableContainer>
            <Table>
              <TableHead>
                <TableRow>
                  <TableCell>Pool Name</TableCell>
                  <TableCell>Status</TableCell>
                  <TableCell>Size</TableCell>
                  <TableCell>Allocated</TableCell>
                  <TableCell>Free</TableCell>
                  <TableCell>Fragmentation</TableCell>
                  <TableCell>Health</TableCell>
                  <TableCell align="right">Actions</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {pools.map((pool) => (
                  <TableRow key={pool.name} hover>
                    <TableCell>
                      <Box display="flex" alignItems="center" gap={1}>
                        <StorageIcon fontSize="small" />
                        <Typography variant="subtitle2">{pool.name}</Typography>
                      </Box>
                    </TableCell>
                    <TableCell>
                      <Chip
                        label={pool.status}
                        color={getPoolStatusColor(pool.status)}
                        size="small"
                      />
                    </TableCell>
                    <TableCell>{formatBytes(pool.size)}</TableCell>
                    <TableCell>{formatBytes(pool.allocated)}</TableCell>
                    <TableCell>{formatBytes(pool.free)}</TableCell>
                    <TableCell>
                      {pool.fragmentation ? `${pool.fragmentation}%` : 'N/A'}
                    </TableCell>
                    <TableCell>
                      <Box display="flex" alignItems="center" gap={1}>
                        {getPoolStatusIcon(pool.health || 'UNKNOWN')}
                        <Typography variant="body2">
                          {pool.health || 'Unknown'}
                        </Typography>
                      </Box>
                    </TableCell>
                    <TableCell align="right">
                      <IconButton
                        size="small"
                        onClick={(e) => handleActionMenuOpen(e, pool)}
                      >
                        <MoreVertIcon />
                      </IconButton>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </TableContainer>
        </Card>
      )}

      {/* Empty State */}
      {!isLoading && pools.length === 0 && (
        <Card sx={{ textAlign: 'center', py: 8 }}>
          <CardContent>
            <StorageIcon sx={{ fontSize: 64, color: 'text.secondary', mb: 2 }} />
            <Typography variant="h6" gutterBottom>
              No ZFS Pools Found
            </Typography>
            <Typography variant="body2" color="text.secondary" paragraph>
              Create your first ZFS pool to get started with advanced storage management.
            </Typography>
            <Button
              variant="contained"
              startIcon={<AddIcon />}
              onClick={() => setCreateWizardOpen(true)}
            >
              Create Pool
            </Button>
          </CardContent>
        </Card>
      )}

      {/* ZFS Pool Monitor Integration */}
      {pools.length > 0 && (
        <Box mt={4}>
          <Typography variant="h5" gutterBottom>
            Real-time Monitoring
          </Typography>
          <ZFSPoolMonitor showAllPools={true} showDetailed={true} />
        </Box>
      )}

      {/* Action Menu */}
      <Menu
        anchorEl={actionMenuAnchor}
        open={Boolean(actionMenuAnchor)}
        onClose={handleActionMenuClose}
      >
        {selectedPool && poolActions.map((action) => (
          <MenuItem
            key={action.id}
            onClick={() => action.action(selectedPool)}
            disabled={action.disabled?.(selectedPool)}
            sx={{ color: action.color === 'error' ? 'error.main' : 'inherit' }}
          >
            <Box display="flex" alignItems="center" gap={1}>
              {action.icon}
              {action.label}
            </Box>
          </MenuItem>
        ))}
      </Menu>

      {/* Confirmation Dialog */}
      <Dialog
        open={confirmDialog.open}
        onClose={() => setConfirmDialog(prev => ({ ...prev, open: false }))}
      >
        <DialogTitle>{confirmDialog.title}</DialogTitle>
        <DialogContent>
          <Typography>{confirmDialog.message}</Typography>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setConfirmDialog(prev => ({ ...prev, open: false }))}>
            Cancel
          </Button>
          <Button
            onClick={confirmDialog.action}
            color="error"
            variant="contained"
          >
            Confirm
          </Button>
        </DialogActions>
      </Dialog>

      {/* Pool Creation Wizard */}
      <PoolCreationWizard
        open={createWizardOpen}
        onClose={() => setCreateWizardOpen(false)}
        onPoolCreated={(poolName) => {
          console.log('Pool created:', poolName);
          refetch();
        }}
      />
    </Box>
  );
};

export default PoolManagementDashboard; 